mod message;
mod error;
mod config;

extern crate chrono;
extern crate mqtt3;
extern crate netopt;
extern crate mqttc;
extern crate rustc_serialize;

use std::net::{TcpStream};
use std::io::{BufRead, BufReader};
use std::time::Duration;
use rustc_serialize::json::ToJson;
use mqttc::{PubOpt, PubSub};
use config::Config;


fn main() {
    let config = Config::load_from_file("config.json").expect("Could not load config file");

    let netopts = netopt::NetworkOptions::new();
    let mut mqtt_opts = mqttc::ClientOptions::new();
    mqtt_opts.set_reconnect(mqttc::ReconnectMethod::ReconnectAfter(Duration::from_secs(1)));
    let mqtt_address: &str = &config.mqtt;
    let mut mqtt_client = mqtt_opts.connect(mqtt_address, netopts)
        .expect("Error connecting to server");

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
                        let j = m.to_json();
                        println!("{}", j);

                        let topic: &str = &config.topic;
                        match mqtt_client.publish(topic, j.to_string(), PubOpt::at_least_once()) {
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
