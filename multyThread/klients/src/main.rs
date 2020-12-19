#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;

use std::net::{TcpStream};
use std::io::{Read, Write, Error};
use std::str::from_utf8;
use std::env;


fn main() {
    println!("TCP klienta puse");
    // argumentu sanemsana no terminala
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        // 0 - programmas nosaukums, 1 - host:port
        let connect_to = &args[1].to_owned();
        connect(connect_to);
    } else {
        panic!("Jabut ievadits domens un ports (piemeram 0.0.0.0:0000)!");
    }

}

// funkcija kura tiek veikta klienta savienosanas ar serveri un datus sanemsana no serveri
fn connect(connect_to : &str) -> Result<(), Error> {

    // klienta savienosanas ar serveri realizacija pec ievadita host:port
    match TcpStream::connect(connect_to.to_owned()) {
        Ok(mut stream) => { // ja ir veiksmigs savienojums
            println!("Veiksmigi savienots ar {}", connect_to.to_owned());

            loop { // bezgaligi klausit serveri
                let mut data = [0 as u8; 50]; // 50 byte biferis
                stream.read(&mut data); // nolasit servera atbilde
                let text = from_utf8(&data).unwrap(); // parveidot simbolu virkne
                println!("{}", text);
            }

        },
        Err(e) => { // ja nav veiksmigs savienojums
            println!("Savienosanas kluda: {}", e);
        }
    }
    
    Ok(())
}
