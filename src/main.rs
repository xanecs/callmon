mod message;
mod error;
mod config;

extern crate chrono;
extern crate redis;
extern crate rustc_serialize;

use std::net::{TcpStream};
use std::io::{BufRead, BufReader};
use rustc_serialize::json::ToJson;
use config::Config;


fn main() {
    let config = Config::load_from_file("config.json").expect("Could not load config file");
    let topic: &str = &config.topic;
    let redis_address: &str = &config.redis;
    let redis_client = redis::Client::open(redis_address).expect("Could not connect to redis");
    let redis_con = redis_client.get_connection().expect("Could not initialize redis connection");

    let callmon_address: &str = &config.callmon;
    let stream = TcpStream::connect(callmon_address)
        .expect("Error connecting to call monitor");
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);
        match result {
            Ok(_) => {
                match message::Message::parse_from_str(&line) {
                    Ok(m) => {
                        let j = m.to_json().to_string();
                        println!("{}", j);
                        let result = redis::cmd("PUBLISH").arg(topic).arg(j).query::<i32>(&redis_con);
                        match result {
                            Err(e) => { println!("Error: {:?}", e); }
                            Ok(_) => {}
                        }
                    }
                    Err(e) => { println!("Error: {:?}", e); }
                }

            }
            Err(e) => {
                println!("Err: {}", e);
            }

        }
    }
}
