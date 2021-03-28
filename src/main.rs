use std::io::prelude::*;
use std::net:: TcpStream;
use std::thread;
use std::process;

pub mod messenger;

fn main() {
    let ip ="127.0.0.1:3000";
    let stream = TcpStream::connect(ip)
    .expect("Couldn't connect to the server...");

    fn connect(mut stream: &TcpStream)-> std::io::Result<()> {
        let message = "greetings from rust client";
        stream.write(message.as_bytes())?;
        let mut rstream = stream.try_clone().unwrap();
        thread::spawn(move ||{ 
            let mut client_buffer = [0; 2048];           
             loop {
                 match rstream.read(&mut client_buffer) {
                    Ok(n) => {
                        if n == 0 {
                            process::exit(0);
                        }
                        messenger::read_message(&rstream).expect("connection interrupted");
                    }
                    Err(error)=> println!("{}",error),
                 }
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
