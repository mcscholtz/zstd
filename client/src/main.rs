use std::net::TcpStream;
use std::io::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4011").unwrap();
    let mut buf = Vec::with_capacity(256);
    let count = stream.read_to_end(&mut buf).unwrap();
    println!("Got {} bytes", count);
    let msg = String::from_utf8(buf).unwrap();
    println!("msg: {}", msg);
}