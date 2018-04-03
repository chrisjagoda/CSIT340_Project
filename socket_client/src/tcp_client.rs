extern crate bufstream;

use self::bufstream::BufStream;
use std::io::{Read, Write, stdin};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::str;

pub fn connect(addr: &SocketAddr) {
    let stream: TcpStream = TcpStream::connect(addr).unwrap();
    let mut input_stream = stream.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            let mut client_buf = [0; 128];
            match input_stream.read(&mut client_buf) {
                Ok(n) => {
                    if n == 0 {
                        return 0;
                    }
                    else {
                        println!("{}", str::from_utf8(&client_buf).unwrap());
                    }
                }
                Err(e) => { 
                    panic!("{}", e);   
                }
            }
        }
    });
    let mut output_stream = BufStream::new(stream);
    let mut user_buf: String;
    loop {
        user_buf = String::new();
        stdin().read_line(&mut user_buf).unwrap();      
        output_stream.write(user_buf.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}