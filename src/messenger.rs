use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

//pub struct Message {
//    content: String,
//}

pub fn send_message(mut stream: &TcpStream) {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    stream.write(input.trim().as_bytes()).expect("ERROR: impossible to send message to ip, check the connection");
}