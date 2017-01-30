use std::io;
use std::num;
use std::convert::From;
use chrono;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    Time(chrono::ParseError),
    Str(String)
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
