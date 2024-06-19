use crate::Client;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

pub fn handle_update(mut stream: &TcpStream, clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let clients = clients.lock().unwrap();
    let response = format!(
        "{:?}\n",
        clients.get(&stream.peer_addr().unwrap()).unwrap().addr
    );
    stream
        .write(response.as_bytes())
        .expect("couldn't send update");
}

pub fn handle_quit(mut stream: &TcpStream) {
    stream
        .write("goodbye\n".as_bytes())
        .expect("message couldnt be sent");
}
pub fn handle_number(mut stream: &TcpStream, clients: &Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let clients = clients.lock().unwrap();
    let response = format!("{:?}\n", clients.len());
    stream
        .write(response.as_bytes())
        .expect("couldnt send number");
}
