use anyhow::Result;
use regex::Regex;
use std::{io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}};

// we assume that http packets contain content length and don't use chunked transfer encoding
// since http1.1 packets don't have termination chars we use the content length to determine when to stop reading
pub fn read_http_packet_tcp_stream(stream: &mut TcpStream) -> Result<Vec<String>> {
    println!("Reading HTTP packet from TCP stream");
    let mut lines: Vec<String> = Vec::new();
    let mut reader = BufReader::new(stream);

    let content_length_pattern = r"Content-Length:\s+(\d+)\s+";
    let content_length_regex = Regex::new(content_length_pattern)?;

    let mut content_length: usize = 0;
    let mut body_reading_started = false;

    loop {
        let mut raw_buffer = vec![0u8; 65536];
        let mut line = String::new();
        match body_reading_started {
            true => {
                reader.read_exact(&mut raw_buffer[..content_length])?;
                line = String::from_utf8_lossy(&raw_buffer[..content_length]).to_string();
                lines.push(line);
                return Ok(lines);
            }
            false => {
                reader.read_line(&mut line)?;
                match content_length_regex.captures(&line) {
                    Some(captures) => {
                        content_length = captures[1].parse()?;
                    }
                    None => {}
                }
                match line == "\r\n" || line == "\n" {
                    true => {
                        body_reading_started = true;
                    }
                    false => {
                        lines.push(line.trim().to_string());
                    }
                }
            }
        }
    }
}

