use super::router::Router;
use http::http_request::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
  socket_addr: &'a str,
}

impl<'a> Server<'a> {
  pub fn new(socket_addr: &'a str) -> Self {
    Self { socket_addr }
  }

  pub fn run(&self) {
    let connection_listioner = TcpListener::bind(self.socket_addr).unwrap();
    println!("Running on {}", self.socket_addr);

    for stream in connection_listioner.incoming() {
      let mut stream = stream.unwrap();
      println!("Connection established");

      let mut read_buffer = [0; 200];
      stream.read(&mut read_buffer).unwrap();
      let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
      Router::route(req, &mut stream);
    }
  }
}
