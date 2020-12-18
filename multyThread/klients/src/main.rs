#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

// TODO: run on Linux

use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::str::from_utf8;
use std::env;


fn main() {
    println!("TCP klienta puse");
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let connect_to = &args[1].to_owned();
        connect(connect_to);
    } else {
        panic!("Must be included ip and port as (0.0.0.0:0000)!");
    }

}

fn connect(connect_to : &str) -> Result<(), Error> {
    match TcpStream::connect(connect_to.to_owned()) {
        Ok(mut stream) => {
            println!("Successfully connected to server {}", connect_to.to_owned());

            loop {
                let mut data = [0 as u8; 50]; // using 50 byte buffer
                stream.read(&mut data);
                let text = from_utf8(&data).unwrap();
                println!("{}", text);
            }

        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    
    Ok(())
}
