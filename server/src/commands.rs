use crate::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum Actions {
    Quit,
    Update,
    Number,
    Invalid,
    AuthFailure,
    AuthSuccessful,
}

fn handle_update(mut stream: &TcpStream, clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let clients = clients.lock().unwrap();
    let response = format!(
        "{:?}\n",
        clients.get(&stream.peer_addr().unwrap()).unwrap().addr
    );
    stream
        .write(response.as_bytes())
        .expect("couldn't send update");
}

fn handle_quit(mut stream: &TcpStream) {
    stream
        .write("goodbye\n".as_bytes())
        .expect("message couldnt be sent");
}

fn handle_auth(
    mut stream: &TcpStream,
    clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>,
    authcode: String,
) -> bool {
    if clients
        .lock()
        .unwrap()
        .get(&stream.peer_addr().unwrap())
        .unwrap()
        .isauth
    {
        stream.write("already authed\n".as_bytes()).unwrap();
        return true;
    } else {
        let mut buffer = [0; 1024];
        stream.write("enter authcode\n".as_bytes()).unwrap();
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                let v: Value = serde_json::from_str(&request).unwrap();
                let requestcode = v["command"].as_str().unwrap();

                if requestcode == authcode {
                    clients
                        .lock()
                        .unwrap()
                        .get_mut(&stream.peer_addr().unwrap())
                        .unwrap()
                        .isauth = true;
                    stream.write("auth SUCCESSFUL\n".as_bytes()).unwrap();

                    {
                        let mut client_list = clients.lock().unwrap();
                        client_list
                            .get_mut(&stream.peer_addr().unwrap())
                            .unwrap()
                            .isauth = true;
                    }
                    return true;
                } else {
                    stream.write("wrong authcode\n".as_bytes()).unwrap();
                    stream.write("auth FAILURE".as_bytes()).unwrap();
                    return false;
                }
            }
            Err(e) => eprintln!("error be like {}", e),
        };
    }
    return false;
}

fn handle_number(mut stream: &TcpStream, clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let clients = clients.lock().unwrap();
    let response = format!("{:?}\n", clients.len());
    stream
        .write(response.as_bytes())
        .expect("couldnt send number");
}

pub fn handle_request(
    mut stream: &TcpStream,
    v: Value,
    mut clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>,
    authcode: String,
) -> Actions {
    match v["command"].as_str().unwrap() {
        "quit" => {
            handle_quit(&stream);
            return Actions::Quit;
        }
        "update" => {
            handle_update(&stream, &clients);
            return Actions::Update;
        }
        "number" => {
            handle_number(&stream, &clients);
            return Actions::Number;
        }
        "auth" => {
            let auth_success = handle_auth(&stream, &mut clients, authcode);
            if auth_success {
                return Actions::AuthSuccessful;
            }
            return Actions::AuthFailure;
        }
        _ => {
            stream.write("invalid command\n".as_bytes()).unwrap();
            return Actions::Invalid;
        }
    }
}
