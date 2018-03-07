use std::io::{ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn listen(addr: &String) {
    
}

fn handle_connection(stream: &mut TcpStream) {
    //let mut msg: String = String::new();
    println!("Connected with ip: {}", stream.local_addr().expect("Error"));
    stream.write(b"Hello World\r\n").unwrap();    
}