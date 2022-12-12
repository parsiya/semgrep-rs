use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    // create a new custom error with a message.
    // Error::new(format!("{}", err))
    pub fn new(message: String) -> Error {
        Error { message }
    }

    // as I am usign map_err, are these even used?
    // TODO: Remove if not used by the end of the first prototype.

    // create a new custom error with just a string and no formatting.
    pub fn wrap_string<T>(msg: String) -> std::result::Result<T, Error> {
        Err(Error::new(msg))
    }
    // create a new custom error with just a &str and no formatting.
    pub fn wrap_str<T>(msg: &str) -> std::result::Result<T, Error> {
        Err(Error::new(msg.to_string()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

// implement traits to convert from each to our error type.

// impl From<std::io::Error> for Error {
//     fn From(err: std::io::Error) -> self {
//         Error::Io(err)
//     }
// }

// impl From<serde_yaml::Error> for Error {
//     fn From(err: serde_yaml::Error) -> self {
//         Error::Yaml(err)
//     }
// }
