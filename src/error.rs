use std::{fmt, io, string};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    StringError(String),
    IoError(io::Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    Utf8Error(string::FromUtf8Error),
}

// impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StringError(e) => write!(f, "{}", e),
            Error::IoError(e) => write!(f, "IO Error: {}", e),
            Error::JsonError(e) => write!(f, "JSON Error: {}", e),
            Error::YamlError(e) => write!(f, "YAML error: {}", e),
            Error::Utf8Error(e) => write!(f, "Utf8 error: {}", e.utf8_error()),
        }
    }
}

// impl std::error::Error for Error {
//     fn description(&self) -> &str {
//         &self.message
//     }
// }

impl Error {
    /// create a new error with a message. E.g.,
    /// Error::StringError(format!("{}", err))
    pub fn new(message: String) -> Error {
        Error::StringError(message)
    }

    /// create a new string error with just a string and no formatting.
    pub fn wrap_string<T>(msg: String) -> std::result::Result<T, Error> {
        Err(Error::StringError(msg))
    }
    /// create a new string error with just a &str and no formatting.
    pub fn wrap_str<T>(msg: &str) -> std::result::Result<T, Error> {
        Err(Error::StringError(msg.to_string()))
    }
}

// implement traits to convert from each encapsulated error to our error type.

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::YamlError(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::Utf8Error(err)
    }
}
