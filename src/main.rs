mod message;
mod error;
extern crate chrono;

use std::net::TcpStream;
use std::io::{BufRead, BufReader};


fn main() {
    let stream = TcpStream::connect("10.0.0.1:1012").unwrap();
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);
        match result {
            Ok(_) => {
                let m = message::Message::parse_from_str(&line);
                println!("{:?}", m);
            }
            Err(e) => {
                println!("Err: {}", e);
            }

        }
    }
}
