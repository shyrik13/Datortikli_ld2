#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

// TODO: run on Linux

use std::net::{TcpStream};
use std::io::{self, Read, BufRead};
use std::str::from_utf8;

fn main() {
    println!("TCP klienta puse");
    println!("1 - sanemt laiku no servera");
    println!("2 - testet serveri");
    println!("3 - ciet programmu");

    loop {
        let stdin = io::stdin(); // izveidot jaunu ievad teksta lasitaju
        let input = stdin.lock().lines().next().unwrap().unwrap(); // saglabat klienta ievadito tekstu

        if input.trim() == "1" || input.trim() == "2" {
            connect(input.trim() == "2");
        } else if input.trim() == "3" {
            break;
        }
    }
}

fn connect(test: bool) {

    let count = if test { 10 } else { 1 };

    for i in 0..count {
        println!("Attempt {}", i+1);

        match TcpStream::connect("127.0.0.1:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");
                println!("awaiting reply...");

                let mut data = [0 as u8; 50]; // using 50 byte buffer
                stream.read(&mut data);
                let text = from_utf8(&data).unwrap();
                println!("{}", text);
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}