use std::{fmt, path::Path, error::Error};

// start error

#[derive(Debug)]
pub enum PathError {
    DoesntExist,
    NotDirectory,
    FileReadError,
}

impl Error for PathError {}

impl fmt::Display for PathError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathError::DoesntExist => write!(f, "path doesn't exist."),
            PathError::NotDirectory => write!(f, "path is not a directory"),
            PathError::FileReadError => write!(f, "could not read file"),
        }
    }
}

// end error

// check if a path exists and if it's a directory, otherwise panic.
pub fn check_path(path: &str) -> Result<bool, PathError>  {

    let path_path = Path::new(&path);

    // check if path exists.
    if !Path::exists(path_path) {
        // return with an error.
        return Err(PathError::DoesntExist);
    } 
    // check if path is a directory. Technically, we could have just done this
    // check but we wouldn't know if the path existed vs. is not a directory.
    if !Path::is_dir(path_path) {
        // return with an error.
        return Err(PathError::NotDirectory);
    }
    Ok(true)
}
