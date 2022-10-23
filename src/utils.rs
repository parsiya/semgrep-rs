use std::{fmt, path::Path, error::Error, io, fs};

use walkdir::{DirEntry, WalkDir};

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

// check if a path exists and if it's a directory, otherwise return an error.
pub(crate) fn check_path(path: &str) -> Result<bool, PathError>  {

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


// check if a path exists and if it's a directory, otherwise panic.
pub(crate) fn check_path_panic(path: &str) {

    let path_path = Path::new(&path);

    // check if path exists.
    if !Path::exists(path_path) {
        panic!("{} doesn't exist.", path);
    }
    // check if path is a directory. Technically, we could have just done this
    // check but we wouldn't know if the path existed vs. is not a directory.
    if !Path::is_dir(path_path) {
        panic!("{} is not a directory.", path)
    }
}

// ----- START find_rules

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
    let rule_extensions: Vec<&str> = vec!["yml", "yaml"];
    // file ending in `.test.yaml`, `.test.yml` and `.test.fixed.yaml` are test yaml files and not rules.
    let test_extensions: Vec<&str> = vec![".test.yml", ".test.yaml", ".test.fixed.yaml"];

    // use the default values if include and exclude are not provided.
    let include_extensions = include.unwrap_or_else(|| rule_extensions);

    let exclude_extensions = exclude.unwrap_or_else(|| test_extensions);

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
                    // convert the file path to a string
                    let file_path_string = file_path.to_string_lossy();

                    // check if the file ends with exclude_extensions
                    for excluded in &exclude_extensions {
                        if file_path_string.ends_with(excluded) {
                            continue;
                        }
                    }
                    results.push(file_path.to_string_lossy().as_ref().to_string());
                }
            }
        }
    }
    results
}

#[test]
fn test_find_rules() {

    let control_files: [&str; 9] = [
        "tests/multiple-rules.yml",
        "tests/multiple-rules.yaml",
        "tests/cpp/arrays-out-of-bounds-access.yaml",
        "tests/cpp/arrays-passed-to-functions.yaml",
        "tests/cpp/encode-decode-function-name.yaml",
        "tests/cpp/encrypt-decrypt-function-name.yaml",
        "tests/cpp/memcpy-insecure-use.yaml",
        "tests/cpp/potentially-uninitialized-pointer.yaml",
        "tests/cpp/snprintf-insecure-use.yaml",
    ];
    let mut control = control_files.map(String::from).to_vec();

    let mut results = find_rules("tests".to_string(), None, None);

    // sort the results before comparison because order of file read is not guaranteed.
    assert_eq!(results.sort(), control.sort());
}

#[test]
fn test_find_rules_include() {

    let control_files: [&str; 8] = [
        "tests/multiple-rules.yaml",
        "tests/cpp/arrays-out-of-bounds-access.yaml",
        "tests/cpp/arrays-passed-to-functions.yaml",
        "tests/cpp/encode-decode-function-name.yaml",
        "tests/cpp/encrypt-decrypt-function-name.yaml",
        "tests/cpp/memcpy-insecure-use.yaml",
        "tests/cpp/potentially-uninitialized-pointer.yaml",
        "tests/cpp/snprintf-insecure-use.yaml",
    ];
    let mut control = control_files.map(String::from).to_vec();

    // only include the yaml extension.
    let include: Vec<&str> = vec!["yaml"];
    let mut results = find_rules("tests".to_string(), Some(include), None);

    // sort the results before comparison because order of file read is not guaranteed.
    assert_eq!(results.sort(), control.sort());
}

// simple version of find_rules with default values.
pub fn find_rules_simple(path: String) -> Vec<String> {
    return find_rules(path, None, None);
}

// ----- END find_rules



// ----- START read_file_to_string

// read a file and return a String.
pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

#[test]
fn test_read_file_to_string() {
    let content = read_file_to_string("tests/not-a-rule-1.test.yaml").unwrap();
    assert_eq!(content, "# this is not a rule and should not be found in tests");
}

// ----- END read_file_to_string
