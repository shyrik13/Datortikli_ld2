#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::{thread, time};
use std::net::{TcpListener};
use std::io::{Write, Result};
use chrono::{Local};
use std::net::Shutdown::Both;
use std::env;

fn handle_client(mut stream: &std::net::TcpStream)-> Result<()> {
    loop {
        let now = Local::now();
        stream.write(now.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes())?;
        stream.flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let addr = &args[1].to_owned();
        create(addr);
    } else {
        panic!("Must be included ip and port as (0.0.0.0:0000)!");
    }
}

fn create(addr : &str) {
    let listener = TcpListener::bind(addr.to_owned()).unwrap();

    // accept connections and process them
    println!("Server running on {}", addr.to_owned());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let addr = stream.peer_addr().unwrap();
                    println!("New connection: {}", addr);
                    match handle_client(&stream) {
                        Err(_) => {
                            stream.shutdown(Both);
                            println!("Lost connection: {}", addr);
                        }
                        _ => {}
                    }
                });

            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
}
