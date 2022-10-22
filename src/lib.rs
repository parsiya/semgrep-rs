use std::{fs, collections::HashMap, io};

use semgrep_rule::RuleFile;
use walkdir::{DirEntry, WalkDir};

mod semgrep_rule;
mod utils;

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


// return an index of rules where the key is rule ID and the value is the rule.
fn index_rules(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) ->
    Result<HashMap<String, semgrep_rule::Rule>, utils::PathError> {
    
    // check the path.
    utils::check_path(&path)?;

    let rule_paths: Vec<String> = find_rules(path, include, exclude);

    let mut rule_index: HashMap<String, semgrep_rule::Rule> = HashMap::new();

    for rule_file in rule_paths {

        let content = match read_file_to_string(&rule_file) {
            Ok(cn) => cn,
            Err(_) => {
                // ZZZ need error logging
                // println!("Error reading file: {}", e.to_string());
                continue;
            }
        };

        // create a rule file from the string
        let rule_file = match RuleFile::from_yaml(content) {
            Ok(rf) => rf,
            Err(_) => {
                // ZZZ need error logging
                continue;
            }
        };

        // get the file index
        let file_index: HashMap<String, semgrep_rule::Rule> = rule_file.index();

        // merge it into the main index
        rule_index.extend(file_index);
    }

    Ok(rule_index)
}
