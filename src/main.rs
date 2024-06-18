use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let addr: &str = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).expect("couldnt bind to {addr}");
    println!("server listening on {addr}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e)
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    stream
        .write("gimme a message: ".as_bytes())
        .expect("message couldnt be sent");
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("failed to read from client");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("received request: {}", request);
    let response = "whats up beeach".as_bytes();
    stream.write(response).expect("message couldnt be sent");
}
