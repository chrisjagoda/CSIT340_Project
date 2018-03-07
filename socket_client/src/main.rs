extern crate bufstream;

use bufstream::BufStream;
use std::io::*;
use std::net::TcpStream;
use std::thread;

fn main() {
    let stream: TcpStream = TcpStream::connect("127.0.0.1:8085").unwrap();
    let mut input_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        //let mut input_stream = BufStream::new(input_stream);
        let mut client_buf = [0u8, 128];

        loop {
            match input_stream.read(&mut client_buf) {
                Ok(n) => {
                    if n == 0 {
                        return 0;
                    }
                    else {
                        stdout().write(&client_buf).unwrap();
                        stdout().flush().unwrap();
                    }
                }
                Err(e) => { println!("{}", e.to_string()) }
            }
        }
    });

    let mut output_stream = BufStream::new(stream);    
    loop {
        let mut user_buf = String::new();
        stdin().read_line(&mut user_buf).unwrap();        
        output_stream.write(user_buf.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}