use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use serde_json::json;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:6969";
    let mut buffer = [0; 1024];
    let mut stream = TcpStream::connect(addr).unwrap();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        if input == "quit\n" {
            break;
        }

        let json = json!({
            "command":input.trim(),
        });

        let _ = stream.write(json.to_string().as_bytes());

        // read from stream
        let mut request = String::new();
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                println!("bytes read: {}", bytes_read.to_string());
                request = String::from_utf8_lossy(&buffer[..bytes_read])
                    .trim()
                    .to_string();
                println!("request: {}", request);
            }
            Err(e) => eprintln!("error be like {}", e),
        };
    }
    Ok(())
}
