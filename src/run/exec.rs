use tempfile::NamedTempFile;

use crate::{
    error::{Error, Result},
    Args, GenericRuleFile,
};

use std::process::{Command, Output};
use std::{fs, io::Write};

/// Run Semgrep.
pub(crate) fn internal_exec(args: &Args) -> Result<std::process::Output> {
    // load the rule and check that it can be deserialized. We don't need the
    // result here.
    GenericRuleFile::from_yaml(&args.rules)?;

    // create a temp file and write the rule string to it.
    // temp files created this way might be destroyed but we only want the file
    // to be valid for a few minutes at most.
    // See: https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html
    let mut tmp_file = NamedTempFile::new()?;
    write!(tmp_file, "{}", args.rules)?;
    let tmp_file_path = tmp_file.path().to_string_lossy().to_string();

    // add --config `path-to-tmp_file`.
    let mut new_args: Vec<String> = vec!["--config".to_string()];
    new_args.push(tmp_file_path.clone());
    // append the rest of the arguments. `--[output-format] --metrics on/off [extra]... code_paths...`
    new_args.append(&mut args.to_vec());
    // convert the vector to a slice of &str.
    let new_args: Vec<&str> = new_args.iter().map(|s| s.as_str()).collect();

    // run Semgrep and get the result.
    let result = run_semgrep(&new_args.as_slice());

    // delete the temporary file.
    fs::remove_file(tmp_file_path)?;
    result
}

// run semgrep and return the result.
fn run_semgrep(args: &[&str]) -> Result<Output> {
    Command::new("semgrep")
        .args(args)
        .output()
        .map_err(Error::from)
}

/// return true if the Semgrep command is available.
pub fn is_installed() -> bool {
    // match run_semgrep(&["--version"]) {
    //     Ok(_) => true,
    //     Err(_) => false,
    // }

    run_semgrep(&["--version"]).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // I know this is not a real test.
    #[test]
    fn test_check_installation() {
        match is_installed() {
            false => println!("semgrep is not installed"),
            true => println!("semgrep is installed"),
        }
    }
}
