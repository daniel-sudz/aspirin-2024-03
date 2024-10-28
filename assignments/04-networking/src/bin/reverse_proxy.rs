use std::{env, io::Write, net::{TcpListener, TcpStream}};
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
