use serde_json::Value;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
mod commands;

pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let addr = stream.peer_addr().unwrap();
        Self { stream, addr }
    }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let mut buffer = [0; 1024];
    loop {
        //get message from client
        let mut request = String::new();
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("client disconnected");
                    break;
                }
                request = String::from_utf8_lossy(&buffer[..bytes_read])
                    .trim()
                    .to_string();
                println!(
                    "message from: {}: {}",
                    stream.peer_addr().unwrap().to_string(),
                    request
                );
            }
            Err(e) => eprintln!("error be like {}", e),
        };
        let v: Value = serde_json::from_str(request.as_str()).unwrap();
        let action = commands::handle_request(&stream, v, &clients);
        if action == commands::Actions::Quit{
            break;
        }
    }

    //remove disconnected clients from client list
    let addr = stream.peer_addr().unwrap();
    {
        let mut clients = clients.lock().unwrap();
        clients.remove(&addr);
        println!("Client {} removed. Total clients: {}", addr, clients.len());
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
