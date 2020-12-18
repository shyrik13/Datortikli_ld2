#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use chrono::{Local};

fn handle_client(mut stream: TcpStream) {
    let now = Local::now();

    stream.write(now.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes()).unwrap();
    thread::sleep(time::Duration::from_millis(1000));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    // accept connections and process them
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
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