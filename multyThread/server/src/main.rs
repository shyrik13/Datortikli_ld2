#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::{thread, time};
use std::net::{TcpListener};
use std::io::{Write, Result};
use chrono::{Local};
use std::net::Shutdown::Both;

fn handle_client(mut stream: &std::net::TcpStream)-> Result<()> {
    loop {
        let now = Local::now();
        stream.write(now.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes())?;
        stream.flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:4444").unwrap();

    // accept connections and process them
    println!("Server listening on port 4444");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    match handle_client(&stream) {
                        Err(_) => {
                            stream.shutdown(Both);
                            println!("Lost connection: {}", stream.peer_addr().unwrap());
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
