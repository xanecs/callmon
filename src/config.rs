use error::Error;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json;

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Config {
    pub mqtt: String,
    pub callmon: String,
    pub topic: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config, Error> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string (&mut s));
        let c = try!(json::decode::<Config>(&s));
        Ok(c)
    }
}
