use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Server is running on port 3000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connect established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
