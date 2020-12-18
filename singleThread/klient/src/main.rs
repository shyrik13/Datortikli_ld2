#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

// TODO: run on Linux

use std::net::{TcpStream};
use std::io::{self, Read, BufRead};
use std::str::from_utf8;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let connect_to = &args[1].to_owned();

        println!("TCP klienta puse");
        println!("1 - sanemt laiku no servera");
        println!("2 - testet serveri");
        println!("3 - ciet programmu");

        loop {
            let stdin = io::stdin(); // izveidot jaunu ievad teksta lasitaju
            let input = stdin.lock().lines().next().unwrap().unwrap(); // saglabat klienta ievadito tekstu

            if input.trim() == "1" || input.trim() == "2" {
                connect(connect_to, input.trim() == "2");
            } else if input.trim() == "3" {
                break;
            }
        }

    } else {
        panic!("Must be included ip and port as (0.0.0.0:0000)!");
    }
}

fn connect(connect_to : &str, test: bool) {

    let count = if test { 10 } else { 1 };

    for i in 0..count {
        println!("Attempt {}", i+1);

        match TcpStream::connect(connect_to.to_owned()) {
            Ok(mut stream) => {
                println!("Successfully connected to {}", connect_to.to_owned());
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