use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use std::fs::File;

const REQUEST_BUFFER_SIZE: usize = 8192;

fn main() {
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        let (stream, addr) =  listener.accept().unwrap();
        on_connection(stream, addr);
    }
}

fn on_connection(mut stream: TcpStream, sock_addr: SocketAddr) {

    let mut buffer: Vec<u8> = Vec::with_capacity(REQUEST_BUFFER_SIZE);
    buffer.resize(REQUEST_BUFFER_SIZE, 0);

    // get request data
    println!("got new connection from {}", sock_addr.ip());
    let n = stream.read(&mut buffer[..]).unwrap();
    println!("Read {} bytes from socket", n);
    buffer.truncate(n);

    // parse path from request data
    let mut path_string = String::new();
    find_path(buffer, &mut path_string);

    // make a path from path String
    let req_path = Path::new(&path_string);
    let path = if req_path.exists() {
        req_path
    } else {
        Path::new("404.html")
    };

    // println!("path is {}", path.display());

    // load file from path
    let mut fd = File::open(path).unwrap();
    let mut file_data = String::new();
    let r = fd.read_to_string(&mut file_data).unwrap();
    println!("read to string result {}", r);

    // add headers
    let mut out = String::from("HTTP/1.0 200 OK\r\n\r\n");
    out.push_str(file_data.as_str());

    // send file back
    let n = stream.write(out.as_bytes()).unwrap();
    println!("wrote {} bytes", n);
}

fn find_path(buffer: Vec<u8>, path_out: &mut String) {
    let req = String::from_utf8(buffer).unwrap();
    let v: Vec<&str> = req.split("\r\n").collect();
    let get_line = &v[0];
    let tmp: Vec<&str> = get_line.split(" ").collect();
    let path: &str = tmp[1];
    path_out.push_str(path);
    path_out.remove(0);
}
