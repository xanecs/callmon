mod message;
mod error;
extern crate chrono;
/*
extern crate mqtt3;
extern crate netopt;
extern crate mqttc;
*/
extern crate rustc_serialize;

use std::net::TcpStream;
use std::io::{BufRead, BufReader};
//use std::time::Duration;
use rustc_serialize::json::ToJson;


fn main() {
    /*
    let netopts = netopt::NetworkOptions::new();
    let mut mqtt_opts = mqttc::ClientOptions::new();
    mqtt_opts.set_reconnect(mqttc::ReconnectMethod::ReconnectAfter(Duration::from_secs(1)));
    let mut mqtt_client = mqtt_opts.connect("iot.eclipse.org:1883", netopts).expect("Error connecting to server");
    */

    let stream = TcpStream::connect("127.0.0.1:1012").unwrap();
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);
        match result {
            Ok(_) => {
                match message::Message::parse_from_str(&line) {
                    Ok(m) => { println!("{}", m.to_json()); }
                    Err(e) => { println!("Error: {:?}", e); }
                }

            }
            Err(e) => {
                println!("Err: {}", e);
            }

        }
    }
}
