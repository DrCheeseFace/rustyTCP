use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Client {
    fn new(stream: TcpStream) -> Self {
        let addr = stream.peer_addr().unwrap();
        Self { stream, addr }
    }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let mut buffer = [0; 1024];
    loop {
        stream
            .write("send a message: ".as_bytes())
            .expect("message couldnt be sent");

        //get message from client
        let bytes_read = stream
            .read(&mut buffer)
            .expect("failed to read from client");
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!(
            "message from: {}: {}",
            stream.peer_addr().unwrap().to_string(),
            request.trim()
        );

        //command to terminate connection
        if request.trim() == "quit" {
            stream
                .write("goodbye\n".as_bytes())
                .expect("message couldnt be sent");
            break;
        }

        //command to see all connected clients
        if request.trim() == "update" {
            let clients = clients.lock().unwrap();
            let response = format!(
                "{:?}\n",
                clients.get(&stream.peer_addr().unwrap()).unwrap().addr
            );
            stream
                .write(response.as_bytes())
                .expect("message couldnt be sent");
            continue;
        }

        let remoteaddr: String = stream.peer_addr().unwrap().to_string();
        let response = format!("HIIII {}", remoteaddr);
        stream
            .write(response.as_bytes())
            .expect("message couldnt be sent");
    }
}

fn main() {
    let clients: Arc<Mutex<HashMap<SocketAddr, Client>>> = Arc::new(Mutex::new(HashMap::new()));
    let addr: &str = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).expect("couldnt bind to {addr}");
    println!("server listening on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connected to {}", stream.peer_addr().unwrap());

                {
                    let mut client_list = clients.lock().unwrap();
                    client_list.insert(
                        stream.peer_addr().unwrap(),
                        Client::new(stream.try_clone().unwrap()),
                    );
                }
                let clients = clients.clone();
                std::thread::spawn(move || handle_client(stream, clients));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e)
            }
        }
    }
}
