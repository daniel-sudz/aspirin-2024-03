use anyhow::Result;
use aspirin_eats::{error::AspirinEatsError, http::HttpResponse, tcp::read_http_packet_tcp_stream};
use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    process::{Child, Command},
    time::Duration,
};

/// Handles an incoming connection from a client by:
/// 1. Connecting to the origin server
/// 2. Reading the client's request
/// 3. Forwarding the request to the origin
/// 4. Reading the origin's response
/// 5. Forwarding the response back to the client
fn handle_connection(stream: &mut TcpStream, origin_addr: &str) -> Result<()> {
    let mut origin_stream = TcpStream::connect(origin_addr)?;
    origin_stream.set_read_timeout(Some(Duration::from_millis(1000)))?;
    origin_stream.set_write_timeout(Some(Duration::from_millis(1000)))?;

    // read client request to proxy
    let request = match read_http_packet_tcp_stream(stream) {
        Ok(req) => req,
        Err(e) => {
            let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
            let _ = stream.write(response.to_string().as_bytes());
            return Err(AspirinEatsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
            .into());
        }
    };
    // send request to origin
    origin_stream.write(request.join("\n").as_bytes())?;
    // read response from origin
    let response = read_http_packet_tcp_stream(&mut origin_stream)?;
    // send response to client
    stream.write(response.join("\n").as_bytes())?;
    println!("[Reverse Proxy] Terminated connection sucessfully");
    Ok(())
}

/// Main entry point for the reverse proxy server.
/// Takes command line arguments for proxy address and origin address.
/// Starts a TCP listener and forwards connections to the origin server.
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
                // set timeouts for the stream
                stream.set_read_timeout(Some(Duration::from_millis(1000)))?;
                stream.set_write_timeout(Some(Duration::from_millis(1000)))?;

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
    use serial_test::serial;

    use super::*;
    use std::thread;
    use std::time::Duration;

    struct TestServer {
        origin: Child,
        proxy: Child,
    }

    impl TestServer {
        /// Creates a new test server environment by:
        /// 1. Starting an origin server on port 8000
        /// 2. Starting a reverse proxy on port 8001 that forwards to the origin
        /// 3. Waiting for both servers to start up
        fn new() -> Result<Self> {
            // Start origin server on port 8000
            let origin = Command::new("cargo")
                .args(["run", "--bin", "origin"])
                .spawn()?;

            // Start reverse proxy on port 8001, forwarding to origin
            let proxy = Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "proxy",
                    "--",
                    "127.0.0.1:8081",
                    "127.0.0.1:8080",
                ])
                .spawn()?;

            // Give servers time to start up
            thread::sleep(Duration::from_secs(1));

            Ok(TestServer { origin, proxy })
        }
    }

    impl Drop for TestServer {
        /// Cleans up the test server by killing both the origin and proxy processes
        fn drop(&mut self) {
            self.origin.kill().unwrap();
            self.proxy.kill().unwrap();
        }
    }

    /// Tests that the proxy correctly forwards basic GET requests to the origin
    /// and returns responses back to the client
    #[test]
    #[serial]
    fn test_proxy_forwards_requests() -> Result<()> {
        let _server = TestServer::new()?;

        // Connect to proxy
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;

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

    /// Tests a complex flow of HTTP requests through the proxy including:
    /// - GET requests to root endpoint
    /// - DELETE requests to clear orders
    /// - GET requests to verify empty orders
    /// - POST requests to create orders
    /// - GET requests to verify order creation
    /// - DELETE requests to remove specific orders
    /// - Error handling for invalid requests
    #[test]
    #[serial]
    fn test_proxy_complex_flow() -> Result<()> {
        let _server = TestServer::new()?;

        // Test root endpoint
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("Welcome to Aspirin Eats!"));

        // Clear out anything previously in the origin server
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        let request = "DELETE /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("reset all orders"));

        // Test getting empty orders list
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "GET /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        println!("response: {}", response);
        assert!(response.contains("[]"));

        // Test adding an order
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let order = r#"{"customer":"Test Customer 🍔","food":[{"Burger":{"bun":"Sesame","patty":"Beef","toppings":["Lettuce"]}}]}"#;
        let request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\n\r\n{}",
            order.len(),
            order
        );
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("created order with id 1"));

        // Test getting specific order
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "GET /orders/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("Test Customer 🍔"));

        // Test getting all orders
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "GET /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("Test Customer 🍔"));

        // Test deleting specific order
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "DELETE /orders/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("deleted order with id 1"));

        // Add another order and test delete all
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        stream.write(request.as_bytes())?;
        let request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\n\r\n{}",
            order.len(),
            order
        );
        stream.write(request.as_bytes())?;

        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "DELETE /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("200 OK"));
        assert!(response.contains("reset all orders"));

        // Test error handling with invalid request
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "INVALID /wrong HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        assert!(response.contains("400 Bad Request"));

        Ok(())
    }

    /// Tests that the proxy correctly handles malformed HTTP requests
    #[test]
    #[serial]
    fn test_invalid_http() -> Result<()> {
        let server = TestServer::new()?;

        // Test malformed HTTP request with no newline ending
        let mut stream = TcpStream::connect("127.0.0.1:8081")?;
        let request = "GET /orders HTTP/1.1 Host: localhost"; // No final \r\n\r\n
        stream.write(request.as_bytes())?;
        let response = read_http_packet_tcp_stream(&mut stream)?.join("\n");
        println!("response: {}", response);
        assert!(response.contains("400 Bad Request"));

        Ok(())
    }
}
