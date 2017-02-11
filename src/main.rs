use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};

const REQUEST_BUFFER_SIZE: usize = 8192;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                on_connection(stream, addr);
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

fn on_connection(mut stream: TcpStream, sock_addr: SocketAddr) {
    let out = String::from("hi");
    let mut buffer: Vec<u8> = Vec::with_capacity(REQUEST_BUFFER_SIZE);
    buffer.resize(REQUEST_BUFFER_SIZE, 0);
    println!("got new connection from {}", sock_addr.ip());
    match stream.read(&mut buffer[..]) {
        Ok(n) => {
            println!("Read {} bytes from socket", n);
            buffer.truncate(n);
            let buf_str = String::from_utf8(buffer).unwrap();
            for 
            println!("{}", buf_str);
        }
        Err(e) => {
            println!("Error reading socket: {}", e);
        }
    }

    match stream.write(out.as_bytes()) {
        Ok(n) => {
            println!("wrote {} bytes", n);
        }
        Err(e) => {
            println!("Error writing to socket: {}", e);
        }
    };
}
