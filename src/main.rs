use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};

const REQUEST_BUFFER_SIZE: usize = 8192;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        let (stream, addr) =  listener.accept().unwrap();
        on_connection(stream, addr);
    }
}

fn on_connection(mut stream: TcpStream, sock_addr: SocketAddr) {
    let out = String::from("<!DOCTYPE HTML><body><h1>Hi</h1></body>");
    let mut buffer: Vec<u8> = Vec::with_capacity(REQUEST_BUFFER_SIZE);
    buffer.resize(REQUEST_BUFFER_SIZE, 0);

    println!("got new connection from {}", sock_addr.ip());
    let n = stream.read(&mut buffer[..]).unwrap();
    println!("Read {} bytes from socket", n);
    buffer.truncate(n);
    let buf_str = String::from_utf8(buffer).unwrap();
    println!("{}", buf_str);

    let n = stream.write(out.as_bytes()).unwrap();
    println!("wrote {} bytes", n);
}
