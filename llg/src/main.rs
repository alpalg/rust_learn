use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{Write, Read};



fn main() {
    let host: Vec<_> = "httpbin.org:80".to_socket_addrs().unwrap().collect();
    let mut socket = TcpStream::connect_timeout(
        &host[0], Duration::from_millis(500)).unwrap();
    let mut header = format!("GET /ip HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n").into_bytes();
    socket.write_all(&mut header).unwrap();
    let mut response = String::new();
    socket.read_to_string(&mut response).expect("Nresp");
    println!("{}", response);

}
