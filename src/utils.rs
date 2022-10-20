use std::fs;
use walkdir::{DirEntry, WalkDir};

// TODO: replace this panic with error?!
pub fn read_file_to_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

// returns true if a DirEntry (file or directory) is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}


// return the relative path to all files in path (recursive).
// include: extensions to look for.
// exclude: extensions to skip.
pub fn find_rules(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> Vec<String> {

    // TODO: How do I make these constants without all the issues that start when I do?
    // rule file extensions.
    let RULE_EXTENSIONS: Vec<&str> = vec!["yml", "yaml"];
    // file ending in `.test.yaml`, `.test.yml` and `.test.fixed.yaml` are test yaml files and not rules.
    let TEST_EXTENSIONS: Vec<&str> = vec![".test.yml", ".test.yaml", ".test.fixed.yaml"];

    // use the default values if include and exclude are not provided.
    let include_extensions = include.unwrap_or_else(|| RULE_EXTENSIONS);

    let exclude_extensions = exclude.unwrap_or_else(|| TEST_EXTENSIONS);

    let mut results: Vec<String> = Vec::new();

    let walker = WalkDir::new(path).into_iter();

    // ignore errors and skip hidden files/directories
    for entry in walker.filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        let file_path = entry.path();

        // get the extension
        if let Some(extension) = file_path.extension() {
            // convert the extension from &OsStr to &str
            if let Some(ext_str) = extension.to_str() {
                // check if the file is a rule
                if include_extensions.contains(&ext_str) {
                    // check if the file is yaml test file
                    // check if the file path ends with TEST_EXTENSION
                    let file_path_string = file_path.to_string_lossy();

                    // check if the file ends with exclude_extensions
                    for excluded in &exclude_extensions {
                        if file_path_string.ends_with(excluded) {
                            continue;
                        }
                    }

                    // println!("{:?} is a rule.", file_path.as_os_str());
                    results.push(file_path.to_string_lossy().as_ref().to_string());
                }
            }
        }
    }
    results
}

// start error

#[derive(Debug)]
pub enum PathError {
    DoesntExist,
    NotDirectory,
}

use std::fmt;

impl Error for PathError {}

impl fmt::Display for PathError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathError::DoesntExist => write!(f, "path doesn't exist."),
            PathError::NotDirectory => write!(f, "path is not a directory"),
        }
    }
}

// end error

use std::path::Path;
use std::error::Error;

// check if a path exists and if it's a directory, otherwise panic.
pub fn check_registry_path(path: &str) -> Result<bool, PathError>  {

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
