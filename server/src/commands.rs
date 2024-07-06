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
    clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>,
) -> Actions {
    match v["command"].as_str().unwrap() {
        "quit" => {
            handle_quit(&stream);
            return Actions::Exit;
        }
        "update" => {
            handle_update(&stream, &clients);
            return Actions::Update;
        }
        "number" => {
            handle_number(&stream, &clients);
            return Actions::Number;
        }
        _ => {
            stream.write("invalid command\n".as_bytes()).unwrap();
            return Actions::Invalid;
        }
    }
}
