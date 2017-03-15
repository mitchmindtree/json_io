//! 
//! Functions for simplifying the process of serializing types to JSON files.
//!
//! Supports both rustc-serialize (by default) and serde via the `--features="serde_serialization"
//! --no-default-features` flags.
//!


#[cfg(feature="rustc-serialize")]
#[cfg(not(feature="serde_serialization"))]
pub use rustc_serialize::{Error, load, save};

#[cfg(feature="serde_serialization")]
#[cfg(not(feature="rustc_serialization"))]
pub use serde::{Error, load, save};


#[cfg(feature="serde_serialization")]
mod serde {
    extern crate serde;
    extern crate serde_json;

    use std;

    /// Represents the different kinds of errors returned by Librar.
    #[derive(Debug)]
    pub enum Error {
        /// Some std::io Error.
        IO(std::io::Error),
        /// Occurs when trying to create a `str` from a slice of supposedly Utf8 bytes.
        Utf8(std::str::Utf8Error),
        /// This type represents all possible errors that can occur when serializing or
        /// deserializing a value into JSON (returned by the serde_json crate).
        Json(serde_json::error::Error),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            match *self {
                Error::IO(ref err) => std::fmt::Display::fmt(err, f),
                Error::Utf8(ref err) => std::fmt::Display::fmt(err, f),
                Error::Json(ref err) => std::fmt::Display::fmt(err, f),
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::IO(ref err) => std::error::Error::description(err),
                Error::Utf8(ref err) => std::error::Error::description(err),
                Error::Json(ref err) => std::error::Error::description(err),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(err: std::io::Error) -> Self {
            Error::IO(err)
        }
    }

    impl From<std::str::Utf8Error> for Error {
        fn from(err: std::str::Utf8Error) -> Self {
            Error::Utf8(err)
        }
    }

    impl From<serde_json::error::Error> for Error {
        fn from(err: serde_json::error::Error) -> Self {
            Error::Json(err)
        }
    }

    /// Construct a Deserializable type from a JSON file at the given path.
    ///
    /// json_io will first try and open the file with the path exactly as given.
    ///
    /// If the file isn't found, it will set the extension to .json and try again.
    pub fn load<P, T>(path: P) -> Result<T, Error>
        where P: AsRef<std::path::Path>,
              T: serde::Deserialize,
    {
        let path = path.as_ref();
        let mut file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound =>
                    try!(std::fs::File::open(&path.with_extension("json"))),
                _ => return Err(err.into()),
            },
        };
        let mut contents = Vec::new();
        try!(std::io::Read::read_to_end(&mut file, &mut contents));
        let json_str = try!(std::str::from_utf8(&contents[..]));
        let t: T = try!(serde_json::from_str(&json_str));
        Ok(t)
    }

    /// Save an Encodable type to a JSON file at the given path.
    ///
    /// The file will be saved with the ".json" extension whether or not it was given with the Path.
    pub fn save<P, T>(path: P, t: &T) -> Result<(), Error>
        where P: AsRef<std::path::Path>,
              T: serde::Serialize,
    {
        let path = path.as_ref();
        let json_string = try!(serde_json::to_string(&t));
        let mut file = try!(std::fs::File::create(&path.with_extension("json")));
        try!(std::io::Write::write_all(&mut file, json_string.as_bytes()));
        Ok(())
    }
}

#[cfg(not(feature="serde_serialization"))]
mod rustc_serialize {
    extern crate rustc_serialize;

    use self::rustc_serialize::{json, Decodable, Encodable};
    use std;

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

    impl From<std::io::Error> for Error {
        fn from(err: std::io::Error) -> Self {
            Error::IO(err)
        }
    }

    impl From<json::ParserError> for Error {
        fn from(err: json::ParserError) -> Self {
            Error::JsonParserError(err)
        }
    }

    impl From<json::DecoderError> for Error {
        fn from(err: json::DecoderError) -> Self {
            Error::JsonDecoderError(err)
        }
    }

    impl From<json::EncoderError> for Error {
        fn from(err: json::EncoderError) -> Self {
            Error::JsonEncoderError(err)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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
    ///
    /// json_io will first try and open the file with the path exactly as given.
    ///
    /// If the file isn't found, it will set the extension to .json and try again.
    #[deprecated(since="0.3.0", note="rustc-serialize has been deprecated - use serde instead")]
    pub fn load<T: Decodable>(path: &Path) -> Result<T, Error> {
        let mut file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound =>
                    try!(std::fs::File::open(&path.with_extension("json"))),
                _ => return Err(err.into()),
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
    ///
    /// The file will be saved with the ".json" extension whether or not it was given with the Path.
    #[deprecated(since="0.3.0", note="rustc-serialize has been deprecated - use serde instead")]
    pub fn save<T: Encodable>(path: &Path, t: &T) -> Result<(), Error> {
        let json_string = try!(json::encode(&t).map_err(|err| Error::JsonEncoderError(err)));
        let mut file = try!(File::create(&path.with_extension("json")).map_err(|err| Error::IO(err)));
        io::Write::write_all(&mut file, json_string.as_bytes()).map_err(|err| Error::IO(err))
    }
}
