use chrono::NaiveDateTime;
use chrono;
use error::Error;
use std::str::FromStr;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

#[derive(Debug)]
enum Event {
    Ring {from: String, to: String},
    Disconnect {duration: u32},
    Connect {extension: String, from: String},
    Call {extension: String, from: String, to: String}
}

#[derive(Debug)]
pub struct Message {
    timestamp: NaiveDateTime,
    connection_id: u32,
    event: Event,
}

impl Message {
    pub fn parse_from_str(s: &str) -> Result<Message, Error> {
        let mut line = String::from(s.trim());
        line.pop();
        let mut fields: Vec<&str> = line.split(';').collect();

        let datetime = try!(parse_datetime(fields[0]));
        let connection_id = try!(u32::from_str(fields[2]));
        let event = try!(Event::parse(fields.split_off(1)));
        Ok(Message{timestamp: datetime, connection_id: connection_id, event: event})
    }
}

impl Event {
    fn parse(v: Vec<&str>) -> Result<Event, Error> {
        let event = match v[0] {
            "RING" => {Event::Ring{from: String::from(v[2]), to: String::from(v[3])}}
            "DISCONNECT" => {Event::Disconnect{duration: try!(u32::from_str(v[2]))}}
            "CONNECT" => {Event::Connect{extension: String::from(v[2]), from: String::from(v[3])}}
            "CALL" => {Event::Call{extension: String::from(v[2]), from: String::from(v[3]), to: String::from(v[4])}}
            _ => return Err(Error::Str(String::from("Invalid Event type")))

        };
        Ok(event)
    }
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match *self {
            Event::Call {ref extension, ref from, ref to} => {
                d.insert("type".to_string(), "call".to_json());
                d.insert("extension".to_string(), extension.to_json());
                d.insert("from".to_string(), from.to_json());
                d.insert("to".to_string(), to.to_json());
            }
            Event::Ring { ref from, ref to} => {
                d.insert("type".to_string(), "ring".to_json());
                d.insert("from".to_string(), from.to_json());
                d.insert("to".to_string(), to.to_json());
            }
            Event::Connect {ref extension, ref from} => {
                d.insert("type".to_string(), "connect".to_json());
                d.insert("extension".to_string(), extension.to_json());
                d.insert("from".to_string(), from.to_json());
            }
            Event::Disconnect {ref duration} => {
                d.insert("type".to_string(), "disconnect".to_json());
                d.insert("duration".to_string(), duration.to_json());
            }
        };
        Json::Object(d)
    }
}

impl ToJson for Message {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("timestamp".to_string(), self.timestamp.to_string().to_json());
        d.insert("connectionId".to_string(), self.connection_id.to_json());
        d.insert("event".to_string(), self.event.to_json());
        Json::Object(d)
    }
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(s, "%d.%m.%Y %T")
}
