#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 64];
                let _bytes_read = stream.read(&mut buffer).unwrap();
                let client_request: &str = std::str::from_utf8(&buffer).unwrap();
                println!("read string: {client_request}");
                handle_client_request(&mut stream, client_request);
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}

fn handle_client_request(stream: &mut TcpStream, request: &str) {
    let lines: Vec<&str> = request.split("\r\n").collect();
    let request_line = lines[0];
    let words: Vec<&str> = request_line.split_whitespace().collect();
    let request_target = words[1];
    println!("request target: {request_target} size: {}", request_target.len());
    if request_target.len() > 1 {
        let result = stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n");
        match result {
            Ok(_result) => println!("success"),
            Err(e) => println!("error: {e}")
        }
    } else {
        let result = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
        match result {
            Ok(_result) => println!("success"),
            Err(e) => println!("error: {e}")
        }
    }
}
