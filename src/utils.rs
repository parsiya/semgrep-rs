use std::io::Write;
use std::{fs, io, path::Path};

use walkdir::{DirEntry, WalkDir};

use crate::error::{Error, Result};

// check if a path exists and if it's a directory, otherwise return an error.
pub fn check_path(path: &str) -> Result<bool> {
    let path_path = Path::new(&path);

    // check if path exists.
    if !Path::exists(path_path) {
        // return with an error.
        return Err(Error::new(format!("{} doesn't exist.", path.to_string())));
    }
    // check if path is a directory. Technically, we could have just done this
    // check but we wouldn't know if the path existed vs. is not a directory.
    if !Path::is_dir(path_path) {
        // return with an error.
        return Err(Error::new(format!("{} is not a directory.", path)));
    }
    Ok(true)
}

// check if a path exists and if it's a directory, otherwise panic.
pub fn check_path_panic(path: &str) {
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
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// return the relative path to all files in path (recursive).
// include: extensions to look for.
// exclude: skip files that end in this (regardless of extension).
pub fn find_files(
    path: &str,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
) -> Vec<String> {
    // use the default values if include and exclude are not provided.
    let include_extensions = include.unwrap_or_else(|| rule_extensions());

    let exclude_extensions = exclude.unwrap_or_else(|| test_extensions());

    let mut results: Vec<String> = Vec::new();

    let walker = WalkDir::new(path).into_iter();

    // ignore errors and skip hidden files/directories
    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
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

// rule_extensions returns the default file extensions for Semgrep rules.
fn rule_extensions() -> Vec<&'static str> {
    return vec!["yml", "yaml"];
}

// test_extensions returns the default file extensions for Semgrep rule tests.
fn test_extensions() -> Vec<&'static str> {
    return vec![".test.yml", ".test.yaml", ".test.fixed.yaml"];
}

// simple version of find_files with default values.
pub fn find_files_simple(path: &str) -> Vec<String> {
    return find_files(path, None, None);
}

// ----- END find_rules

// ----- START read_file_to_string

// read a file and return a String.
pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

// ----- END read_file_to_string

// ----- START write_string_to_file
pub fn write_string_to_file(filename: &str, data: &str) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
// ----- END write_string_to_file

// pub fn convert_result<T>(result: io::Result<T>) -> serde_yaml::Result<T> {
//     match result {
//         Ok(value) => Ok(value),
//         Err(error) => Err(serde_yaml::Error(error)),
//     }
// }

// ----- START mod tests
#[cfg(test)]
mod tests {

    const control_files: [&str; 9] = [
        "tests/multiple-rules.yml",
        "tests/multiple-rules.yaml",
        "tests/rules/cpp/arrays-out-of-bounds-access.yaml",
        "tests/rules/cpp/arrays-passed-to-functions.yaml",
        "tests/rules/cpp/encode-decode-function-name.yaml",
        "tests/rules/cpp/encrypt-decrypt-function-name.yaml",
        "tests/rules/cpp/memcpy-insecure-use.yaml",
        "tests/rules/cpp/potentially-uninitialized-pointer.yaml",
        "tests/rules/cpp/snprintf-insecure-use.yaml",
    ];

    // test for find_files().
    #[test]
    fn test_find_files() {
        let mut control = control_files.map(String::from).to_vec();

        let mut results = find_files("tests", None, None);

        // sort the results before comparison because order of file read is not guaranteed.
        assert_eq!(results.sort(), control.sort());
    }

    // test for find_files() when `include` is provided.
    #[test]
    fn test_find_files_include() {
        // let control_files: [&str; 8] = [
        //     "tests/multiple-rules.yaml",
        //     "tests/rules/cpp/arrays-out-of-bounds-access.yaml",
        //     "tests/rules/cpp/arrays-passed-to-functions.yaml",
        //     "tests/rules/cpp/encode-decode-function-name.yaml",
        //     "tests/rules/cpp/encrypt-decrypt-function-name.yaml",
        //     "tests/rules/cpp/memcpy-insecure-use.yaml",
        //     "tests/rules/cpp/potentially-uninitialized-pointer.yaml",
        //     "tests/rules/cpp/snprintf-insecure-use.yaml",
        // ];
        let mut control = control_files.map(String::from).to_vec();

        let mut results = find_files("tests", Some(rule_extensions()), None);

        // sort the results before comparison because order of file read is not guaranteed.
        assert_eq!(results.sort(), control.sort());
    }

    // test for read_file_to_string().
    #[test]
    fn test_read_file_to_string() {
        let content = read_file_to_string("tests/not-a-rule-1.test.yaml").unwrap();
        assert_eq!(
            content,
            "# this is not a rule and should not be found in tests"
        );
    }
}

// ----- END mod tests
