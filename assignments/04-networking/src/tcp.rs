use anyhow::Result;
use regex::Regex;
use std::{io::Read, net::TcpStream};

/// Reads an HTTP packet from a TCP stream by parsing headers and body based on Content-Length
/// Returns a vector of strings containing the lines of the HTTP packet
pub fn read_http_packet_tcp_stream(stream: &mut TcpStream) -> Result<Vec<String>> {
    println!("Reading HTTP packet from TCP stream");
    let mut lines: Vec<String> = Vec::new();

    let content_length_pattern = r"Content-Length:\s+(\d+)\s+";
    let content_length_regex = Regex::new(content_length_pattern)?;

    let mut content_length: usize = 0;
    let mut body_reading_started = false;

    loop {
        let mut raw_buffer = vec![0u8; 65536];
        let line: String;
        match body_reading_started {
            true => {
                stream.read_exact(&mut raw_buffer[..content_length])?;
                line = String::from_utf8_lossy(&raw_buffer[..content_length]).to_string();
                lines.push(line);
                return Ok(lines);
            }
            false => {
                let mut buf = [0u8; 1];
                let mut line_bytes = Vec::new();
                while buf[0] != b'\n' {
                    stream.read_exact(&mut buf)?;
                    line_bytes.push(buf[0]);
                }
                line = String::from_utf8_lossy(&line_bytes).to_string();
                if let Some(captures) = content_length_regex.captures(&line) {
                    content_length = captures[1].parse()?;
                }
                if line == "\r\n" || line == "\n" {
                    body_reading_started = true;
                }
                lines.push(line.trim().to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    /// Tests reading an HTTP packet with a Content-Length header and body
    #[test]
    fn test_read_http_packet() {
        // Start a TCP server in a separate thread
        thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
            let (mut stream, _) = listener.accept().unwrap();

            // Write a sample HTTP request
            let request = "POST /orders HTTP/1.1\r\nContent-Length: 13\r\n\r\nHello, World!";
            stream.write_all(request.as_bytes()).unwrap();
        });

        // Connect to the server
        thread::sleep(std::time::Duration::from_millis(100)); // Give server time to start
        let mut client = TcpStream::connect("127.0.0.1:8081").unwrap();

        // Read and verify the HTTP packet
        let lines = read_http_packet_tcp_stream(&mut client).unwrap();

        assert_eq!(lines[0], "POST /orders HTTP/1.1");
        assert_eq!(lines[1], "Content-Length: 13");
        assert_eq!(lines[2], "");
        assert_eq!(lines[3], "Hello, World!");
    }

    /// Tests reading an HTTP packet without a Content-Length header
    #[test]
    fn test_read_http_packet_no_content_length() {
        thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
            let (mut stream, _) = listener.accept().unwrap();

            let request = "GET / HTTP/1.1\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
        });

        thread::sleep(std::time::Duration::from_millis(100));
        let mut client = TcpStream::connect("127.0.0.1:8082").unwrap();

        let lines = read_http_packet_tcp_stream(&mut client).unwrap();
        assert_eq!(lines[0], "GET / HTTP/1.1");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "");
        assert_eq!(lines.len(), 3);
    }
}
