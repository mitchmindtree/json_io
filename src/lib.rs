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


/// Construct a Decodable type from a JSON file at the given path.
/// json_io will first try and open the file with the path exactly as given.
/// If the file isn't found, it will set the extension to .json and try again.
pub fn load<T: Decodable>(path: &Path) -> Result<T, Error> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound =>
                    try!(File::open(&path.with_extension("json")).map_err(|err| Error::IO(err))),
                _ => return Err(Error::IO(err)),
            }
        },
    };
    let mut contents = Vec::new();
    try!(io::Read::read_to_end(&mut file, &mut contents).map_err(|err| Error::IO(err)));
    let json_object = try!(json::Json::from_str(std::str::from_utf8(&contents[..]).unwrap())
        .map_err(|err| Error::JsonParserError(err)));
    let mut decoder = json::Decoder::new(json_object);
    T::decode(&mut decoder).map_err(|err| Error::JsonDecoderError(err))
}

/// Save an Encodable type to a JSON file at the given path.
/// The file will be saved with the ".json" extension whether or not it was given with the Path.
pub fn save<T: Encodable>(path: &Path, t: &T) -> Result<(), Error> {
    let json_string = try!(json::encode(&t).map_err(|err| Error::JsonEncoderError(err)));
    let mut file = try!(File::create(&path.with_extension("json")).map_err(|err| Error::IO(err)));
    io::Write::write_all(&mut file, json_string.as_bytes()).map_err(|err| Error::IO(err))
}

