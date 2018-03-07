extern crate bufstream;
extern crate calc;

use self::bufstream::BufStream;
use self::calc::eval;
use std::io::{BufRead, ErrorKind, Read, Write};
use std::net::{SocketAddr, UdpSocket};

pub fn listen(addr: &SocketAddr) {
    let listener = UdpSocket::bind(addr).unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut buf = [0; 10];
    loop {
        match listener.recv_from(&mut buf) {
            Ok(n) => {
                break n;
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {

            }
            Err(e) => {
                panic!("IO error: {}", e)
            }
        }
    };
}

fn handle_connection<T>(stream: &mut T) where T: Read + Write {
    let mut stream = BufStream::new(stream);
    stream.write(b"Please enter a mathematical problem\n").unwrap();
    stream.flush().unwrap();
    loop {
        let mut problem = String::new();
        stream.read_line(&mut problem).unwrap();
        if problem.trim().len() != 0 {
            let result = eval(&problem);
            match result {
                Ok(n) => {
                    stream.write_fmt(format_args!("Result: {}\n", n.to_string())).unwrap();
                }
                Err(e) => {
                    stream.write_fmt(format_args!("Input error: {}\n", e.to_string())).unwrap();
                }
            }
            stream.flush().unwrap();
        }
    }
}