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
