use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                on_connection(stream, addr);
            }
            Err(e) => {

            }
        }

    }
}

fn on_connection(mut stream: TcpStream, sock_addr: SocketAddr) {
    let out = String::from("hello!!!!! MOO!!!\n");
    println!("got new connection from {}", sock_addr.ip());
    stream.write(out.as_bytes());
}
