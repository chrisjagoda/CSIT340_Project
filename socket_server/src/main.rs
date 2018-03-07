use std::net::{SocketAddr};
use std::str::FromStr;

mod tcp_server;
mod udp_server;

fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:8085").unwrap();
    
    udp_server::listen(&addr);
}