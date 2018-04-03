use std::net::{SocketAddr};
use std::str::FromStr;

mod tcp_client;
mod udp_client;

fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    let host_addr = SocketAddr::from_str("127.0.0.1:8085").unwrap();

    tcp_client::connect(&host_addr);
    udp_client::connect(&addr, &host_addr);
}