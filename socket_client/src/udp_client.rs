use std::io::{ErrorKind, stdin};
use std::net::{SocketAddr, UdpSocket};
use std::str;

pub fn connect(addr: &SocketAddr, host_addr: &SocketAddr) {
    let socket = UdpSocket::bind(addr).unwrap();
    socket.connect(host_addr).unwrap();
    loop {
        let mut user_buf = String::new();
        let mut buf = [0; 128];
        stdin().read_line(&mut user_buf).unwrap();
        user_buf = user_buf.trim().to_string();
        socket.send(user_buf.as_bytes()).unwrap();
        match socket.recv_from(&mut buf) {
            Ok(_n) => {
                println!("{}", str::from_utf8(&buf).unwrap());
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                println!("Blocking error: {}", e);
            }
            Err(e) => {
                panic!("IO error: {}", e)
            }
        }
    };
}