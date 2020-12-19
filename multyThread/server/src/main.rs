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

// funkcija kura tiek veikta bezgaliga laika parsutisana no servera uz klienta puse
// kamer nebutu partraukta savienosana
fn handle_client(mut stream: &std::net::TcpStream)-> Result<()> {
    loop {
        let now = Local::now();
        // sagatavot atbilde klientam
        stream.write(now.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes())?;
        // izpildit nosutisanu
        stream.flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // argumentu sanemsana no terminala
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        // 0 - programmas nosaukums, 1 - host:port
        let addr = &args[1].to_owned();
        create(addr);
    } else {
        panic!("Jabut ievadits domens un ports (piemeram 0.0.0.0:0000)!");
    }
}

// funkcija kura tiek izveidots serveris uz ievadita no terminala host:port konfiguracijam
fn create(addr : &str) {
    let listener = TcpListener::bind(addr.to_owned()).unwrap();

    println!("Serveris strada uz {}", addr.to_owned());
    // prieks katra jauna savienojuma
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { // ja ir veiksmigs savienojums
                thread::spawn(move || { // pavadienu izveidosana
                    let addr = stream.peer_addr().unwrap();
                    println!("Jauns savienojums: {}", addr);
                    match handle_client(&stream) {
                        Err(_) => { // kad savienosanas sessija bija partraukta
                            stream.shutdown(Both); // ciet savienojumu un nemeginat savienoties
                            println!("Savienojums beidzas: {}", addr);
                        }
                        _ => {}
                    }
                });

            }
            Err(e) => { // ja nav veiksmigs savienojums
                println!("Kluda: {}", e);
            }
        }
    }

    // ciet soket serveri
    drop(listener);
}
