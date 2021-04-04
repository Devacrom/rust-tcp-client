use std::io::prelude::*;
use std::net:: TcpStream;
use std::thread;
use std::env;
use utils::Address;
use std::process;


pub mod messenger;

fn main() {
    let ip_input =Address::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let ip = format!("{}:{}",ip_input.ip, ip_input.port);

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
