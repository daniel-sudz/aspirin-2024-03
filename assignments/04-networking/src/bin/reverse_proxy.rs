use std::{env, io::Write, net::{TcpListener, TcpStream}, process::{Command, Child}};
use anyhow::Result;
use aspirin_eats::{error::AspirinEatsError, tcp::read_http_packet_tcp_stream};

fn handle_connection(stream: &mut TcpStream, origin_addr: &str) -> Result<()> {
    let mut origin_stream = TcpStream::connect(origin_addr)?;
    // read client request to proxy
    let request = read_http_packet_tcp_stream(stream).map_err(|e| {
        AspirinEatsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    // send request to origin
    origin_stream.write(request.join("\n").as_bytes())?;
    // read response from origin
    let response = read_http_packet_tcp_stream(&mut origin_stream)?;
    // send response to client 
    stream.write(response.join("\n").as_bytes())?;
    println!("[Reverse Proxy] Terminated connection sucessfully");
    Ok(())
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        return Err(anyhow::anyhow!("Invalid arguments"));
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    // start a TCP listener on proxy_addr
    let listener = TcpListener::bind(proxy_addr)?;

    // accept connections and process them
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // client can drop while handling which is fine
                let res = handle_connection(&mut stream, origin_addr);
                match res {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error while handling connection: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    struct TestServer {
        origin: Child,
        proxy: Child,
    }

    impl TestServer {
        fn new() -> Result<Self> {
            // Start origin server on port 8000
            let origin = Command::new("cargo")
                .args(["run", "--bin", "server", "--", "127.0.0.1:8000"])
                .spawn()?;

            // Start reverse proxy on port 8001, forwarding to origin
            let proxy = Command::new("cargo")
                .args(["run", "--bin", "reverse_proxy", "--", "127.0.0.1:8001", "127.0.0.1:8000"])
                .spawn()?;

            // Give servers time to start up
            thread::sleep(Duration::from_secs(1));

            Ok(TestServer { origin, proxy })
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            self.origin.kill().unwrap();
            self.proxy.kill().unwrap();
        }
    }

    #[test]
    fn test_proxy_forwards_requests() -> Result<()> {
        let _server = TestServer::new()?;

        // Connect to proxy
        let mut stream = TcpStream::connect("127.0.0.1:8001")?;

        // Send GET request
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;

        // Read response
        let response = read_http_packet_tcp_stream(&mut stream)?;
        let response = response.join("\n");

        // Verify response contains expected content
        assert!(response.contains("200 OK"));
        assert!(response.contains("Welcome to Aspirin Eats!"));

        Ok(())
    }
}
