use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
mod commands;

pub struct Client {
    addr: SocketAddr,
    stream: TcpStream,
    isauth: bool,
}

#[derive(Clone, Debug)]
pub struct AppState {
    authcode: String,
}

impl AppState {
    pub fn new(authcode: String) -> Self {
        Self { authcode }
    }
    pub fn get_authcode(&self) -> String {
        self.authcode.clone()
    }
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let addr = stream.peer_addr().unwrap();
        Self {
            stream,
            addr,
            isauth: false,
        }
    }
    pub fn auth(&mut self) {
        self.isauth = true;
    }
}

fn handle_client(
    mut stream: TcpStream,
    mut clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    authcode: String,
) {
    let mut buffer = [0; 1024];
    loop {
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

        let v = serde_json::from_str(request.as_str());
        match v {
            Ok(v) => {
                let action = commands::handle_request(&stream, v, &mut clients, authcode.clone());
                match action {
                    commands::Actions::Quit => break,
                    commands::Actions::Update => println!("LOG: Sent update"),
                    commands::Actions::Number => println!("LOG: Sent number"),
                    commands::Actions::Invalid => println!("LOG: invalid command received"),
                    commands::Actions::AuthFailure => println!("LOG: auth FALIURE"),
                    commands::Actions::AuthSuccessful => println!("LOG: auth SUCCESS"),
                }
            }
            Err(_) => {
                stream.write("invalid json\n".as_bytes()).unwrap();
            }
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

    let app = AppState::new(
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect(),
    );

    println!("session code: {}", app.authcode);
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
                let code = app.get_authcode();
                std::thread::spawn(move || handle_client(stream, clients, code));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e)
            }
        }
    }
}
