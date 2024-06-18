use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

struct Client {
    stream: TcpStream,
    addr: String,
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<Client>>>) {
    let mut buffer = [0; 1024];
    loop {
        stream
            .write("gimme a message: ".as_bytes())
            .expect("message couldnt be sent");

        //get message from client
        let bytes_read = stream
            .read(&mut buffer)
            .expect("failed to read from client");
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("received request: {}", request);

        //command to terminate connection
        if request.trim() == "quit" {
            stream
                .write("goodbye".as_bytes())
                .expect("message couldnt be sent");
            break;
        }

        //command to see all connected clients
        if request.trim() == "update" {
            let clients = clients.lock().unwrap();
            for client in clients.iter() {
                let response = format!("{}\n", client.addr);
                stream
                    .write(response.as_bytes())
                    .expect("message couldnt be sent");
            }
            continue;
        }

        let remoteaddr: String = stream.peer_addr().unwrap().to_string();
        let response = format!("{}\n", remoteaddr);
        stream
            .write(response.as_bytes())
            .expect("message couldnt be sent");
    }
}

fn main() {
    let clients = Arc::new(Mutex::new(Vec::new()));
    let addr: &str = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).expect("couldnt bind to {addr}");
    for stream in listener.incoming() {
        println!("server listening on {addr}");
        match stream {
            Ok(stream) => {
                //dafak????
                let clients = clients.clone();

                //there HAS to be a better way of doing this
                {
                    let mut clients = clients.lock().unwrap();
                    clients.push(Client {
                        stream: stream.try_clone().expect("couldnt clone stream"),
                        addr: stream.peer_addr().unwrap().to_string(),
                    });
                }
                std::thread::spawn(|| handle_client(stream, clients));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e)
            }
        }
    }
}
