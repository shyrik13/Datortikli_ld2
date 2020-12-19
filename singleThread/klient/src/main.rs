#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::net::{TcpStream};
use std::io::{self, Read, BufRead};
use std::str::from_utf8;
use std::env;


fn main() {
    // argumentu sanemsana no terminala
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        // 0 - programmas nosaukums, 1 - host:port
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
        panic!("Jabut ievadits domens un ports (piemeram 0.0.0.0:0000)!");
    }
}

// funkcija kura tiek veikta klienta savienosanas ar serveri un datus sanemsana no serveri
fn connect(connect_to : &str, test: bool) {

    let count = if test { 10 } else { 1 };

    for i in 0..count {
        println!("Savienojums {}", i+1);

        // klienta savienosanas ar serveri realizacija pec ievadita host:port
        match TcpStream::connect(connect_to.to_owned()) {
            Ok(mut stream) => { // ja ir veiksmigs savienojums
                println!("Veiksmigi savienots ar {}", connect_to.to_owned());
                println!("Gaidu atbilde...");

                let mut data = [0 as u8; 50]; // 50 byte biferis
                stream.read(&mut data); // nolasit servera atbilde
                let text = from_utf8(&data).unwrap(); // parveidot simbolu virkne
                println!("{}", text);
            },
            Err(e) => { // ja nav veiksmigs savienojums
                println!("Savienosanas kluda: {}", e);
            }
        }
    }
}