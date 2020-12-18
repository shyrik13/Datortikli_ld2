#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

// TODO: run on Linux

use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::str::from_utf8;

fn main() {
    println!("TCP klienta puse");

    connect();
}

fn connect() -> Result<(), Error> {
    match TcpStream::connect("127.0.0.1:4444") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 4444");

            loop {
                stream.write("waiting".as_bytes()).unwrap();
                println!("awaiting reply...");

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
