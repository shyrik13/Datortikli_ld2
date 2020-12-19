#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use chrono::{Local};
use std::env;
use std::net::Shutdown::Both;

// funkcija kura tiek veikta laika parsutisana no servera uz klienta puse
fn handle_client(mut stream: &std::net::TcpStream) {
    let now = Local::now();

    // atbildet klientam
    stream.write(now.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes()).unwrap();
    thread::sleep(time::Duration::from_millis(1000));
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
    // servera izveidosana un pieprasijumu klausitaja sanemsana
    let listener = TcpListener::bind(addr.to_owned()).unwrap();

    println!("Serveris strada uz {}", addr.to_owned());
    // prieks katra jauna savienojuma
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { // ja ir veiksmigs savienojums
                let addr = stream.peer_addr().unwrap();
                println!("Jauns savienojums: {}", addr);
                // apstradat klienta pieprasijumu
                handle_client(&stream);
                stream.shutdown(Both); // ciet savienojumu un nemeginat savienoties
                println!("Savienojums beidzas: {}", addr);
            }
            Err(e) => { // ja nav veiksmigs savienojums
                println!("Kluda: {}", e);
            }
        }
    }
    // ciet soket serveri
    drop(listener);
}