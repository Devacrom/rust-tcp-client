use std::io::prelude::*;
use std::net:: TcpStream;
use std::thread;


pub mod messenger;

fn main() {
    let ip ="127.0.0.1:3000";
    let stream = TcpStream::connect(ip)
    .expect("Couldn't connect to the server...");

    fn connect(mut stream: &TcpStream)-> std::io::Result<()> {
        let message = "greetings from rust client";
        stream.write(message.as_bytes())?;
        let rstream = stream.try_clone().unwrap();
        thread::spawn(move ||{            
             loop {
                    messenger::read_message(&rstream).expect("connection interrupted");
                }
            });
        Ok(())
        
    }
    
    
    match connect(&stream) {
        Err(e) => println!("{:?}", e),
        _ => loop {
                messenger::send_message(&stream);
           }
    }
        

} 
