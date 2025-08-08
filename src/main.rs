use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let result = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
                match result {
                    Ok(_result) => println!("success"),
                    Err(e) => println!("error: {}", e)
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
