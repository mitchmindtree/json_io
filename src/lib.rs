//! 
//! Functions for simplifying the process of serializing types to JSON files.
//!

extern crate rustc_serialize;

use rustc_serialize::{json, Decodable, Encodable};
use std::fs::File;
use std::io;
use std::path::Path;


/// Represents the different kinds of errors returned by Librar.
#[derive(Debug)]
pub enum Error {
    /// Some std::io Error.
    IO(std::io::Error),
    /// An error returned by the JSON parser.
    JsonParserError(json::ParserError),
    /// An error returned by the JSON decoder.
    JsonDecoderError(json::DecoderError),
    /// An error returned by the JSON encoder.
    JsonEncoderError(json::EncoderError),
}


impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        writeln!(f, "{:?}", *self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use std::error::Error as StdError;
        match *self {
            Error::IO(ref err)               => err.description(),
            Error::JsonParserError(ref err)  => err.description(),
            Error::JsonDecoderError(ref err) => err.description(),
            Error::JsonEncoderError(ref err) => err.description(),
        }
    }
}


/// Construct a Library from a JSON file.
/// The `Path` should be the absolute path of the file without the ".json" extension.
pub fn load<T: Decodable>(path: &Path) -> Result<T, Error> {
    let mut path = path.to_path_buf();
    path.set_extension("json");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };
    let mut contents = Vec::new();
    if let Err(err) = io::Read::read_to_end(&mut file, &mut contents) {
        return Err(Error::IO(err));
    }
    let json_object = match json::Json::from_str(std::str::from_utf8(&contents[..]).unwrap()) {
        Ok(json_object) => json_object,
        Err(err) => return Err(Error::JsonParserError(err)),
    };
    let mut decoder = json::Decoder::new(json_object);
    match Decodable::decode(&mut decoder) {
        Ok(t) => Ok(t),
        Err(err) => Err(Error::JsonDecoderError(err)),
    }
}

/// Save a Library to a JSON file.
/// The `Path` should be the absolute path of the file without the ".json" extension.
pub fn save<T: Encodable>(path: &Path, t: &T) -> Result<(), Error> {
    let mut path = path.to_path_buf();
    path.set_extension("json");
    let json_string = match json::encode(&t) {
        Ok(x) => x,
        Err(err) => return Err(Error::JsonEncoderError(err)),
    };
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };
    match io::Write::write_all(&mut file, json_string.as_bytes()) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}

