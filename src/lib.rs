use std::env;

pub struct Address{
    pub ip: String,
    pub port: String,
}

impl Address {
    pub fn new(mut args: env::Args) -> Result<Address, &'static str> {
        args.next();

        let ip = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a ip string"),
        };

        let port = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a port string"),
        };

        Ok(Address{ip, port})
    }
}