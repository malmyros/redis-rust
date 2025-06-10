use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    
    let response = "+PONG\r\n";
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if let Err(e) = reader.get_mut().write_all(response.as_bytes()) {
                    eprintln!("Error writing to stream: {}", e);
                    break;
                }
                line.clear();
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
            }
        }
    }
}
