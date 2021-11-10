use lazy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;

mod commands;
use crate::commands::process_client_request;

type STORE = Mutex<HashMap<String, String>>;

// 使用它作为内存数据库来存储客户端发送的键/值对
lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

fn main() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    println!("rudis_sync listening on {} ...", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New connection from: {:?}", stream);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("New connection from: {:?}", stream);
            thread::spawn(|| handle_client(stream));
        }
    }
}

fn handle_client(stream: TcpStream) {
    // 将客户端 stream包装到 BufReader 中，然后将其作为可变引用传递给 resp 软件包的 Decoder::new 方法
    // Decoder 会从 stream 中读取字节以创建 RESP 的 Value 类型
    // 然后有一个匹配代码块来检查我们的解码是否成功
    let mut stream = BufReader::new(stream);
    let decoder = Decoder::new(&mut stream).decode();
    match decoder {
        Ok(v) => {
            //handle_client 函数在 steam 变量中接收客户端 TcpStream 套接字
            let reply = process_client_request(v);
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            println!("Invalid command: {:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    }
}
