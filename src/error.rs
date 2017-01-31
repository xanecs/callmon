use std::io;
use std::num;
use std::convert::From;
use rustc_serialize::json;
use chrono;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    Time(chrono::ParseError),
    Str(String),
    JsonDecode(json::DecoderError),
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error::Parse(e)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Error::Time(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<json::DecoderError> for Error {
    fn from(e: json::DecoderError) -> Error {
        Error::JsonDecode(e)
    }
}
