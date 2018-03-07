extern crate bufstream;
extern crate calc;

use self::bufstream::BufStream;
use self::calc::eval;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::thread::spawn;
use std::io::BufRead;

pub fn listen(addr: &SocketAddr) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connected to {} from {}",
                         stream.local_addr().unwrap(),
                         stream.peer_addr().unwrap());
                spawn(move|| {
                    let mut stream = BufStream::new(stream);
                    handle_connection(&mut stream);
                });
            }
            Err(e) => { println!("Listening error: {}", e.to_string()); }
        }
    }
}

fn handle_connection(stream: &mut BufStream<TcpStream>) {
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