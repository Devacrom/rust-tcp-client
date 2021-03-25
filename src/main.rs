use std::io::prelude::*;
use std::net::TcpStream;

pub mod messenger;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
    .expect("Couldn't connect to the server...");

    fn connect(mut stream: &TcpStream)-> std::io::Result<()> {
        let mut buffer = [0; 2048];
        let message = "greetings from rust client";
        stream.write(message.as_bytes())?;
        stream.read(&mut buffer)?;
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        Ok(())
        
    }
    
    
    match connect(&stream) {
        Err(e) => println!("{:?}", e),
        _ => loop {
            let mut buffer = [0; 2048];
            messenger::send_message(&stream);
            stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        }
    }
        

} 
