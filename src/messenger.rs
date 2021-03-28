use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process;

pub fn send_message(mut stream: &TcpStream) {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    stream.write(input.trim().as_bytes()).expect("ERROR: impossible to send message to ip, check the connection");
    stream.flush().unwrap();
}

pub fn read_message(mut stream: &TcpStream)-> std::io::Result<()> {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).expect("connection interrupted");

    if buffer == [0;2048] {
        process::exit(0);
    } else{
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
                     
    Ok(())
}