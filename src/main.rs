use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
pub mod request;
use request::Request;
use std::fmt;
fn main() {
    let k = world(13);
    let w = world(9);
    work(&k);
    work(&w);
    /*
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection establish");
        handle_connection(stream);
    }*/
}
fn handle_connection(mut stream :TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    //let request = String::from_utf8_lossy(&buffer[..]);
    let content = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn world(k:u32) -> Option<String>{
    if k > 12 {
        Option::None
    }else {
        Some(format!("the number is {}",k))
    }
}
fn work(value:&Option<String>) {
    match value {
        None => println!("no value"),
        Some(x) => println!("the value is {}",x)
    }
}
