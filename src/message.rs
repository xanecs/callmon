use chrono::NaiveDateTime;
use chrono;
use error::Error;
use std::str::FromStr;

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

fn parse_datetime(s: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(s, "%d.%m.%Y %T")
}
