use tempfile::NamedTempFile;

use crate::{
    error::{Error, Result},
    Args, GenericRuleFile,
};

use std::process::{Command, Output};
use std::{fs, io::Write};

/// Run Semgrep.
pub(crate) fn execute(args: &Args) -> Result<std::process::Output> {
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

    // pass the file name to Semgrep and execute it.

    // add --config `path-to-tmp_file`.
    let mut new_args: Vec<String> = vec!["--config".to_string()];
    new_args.push(tmp_file_path.clone());
    // append the rest of the arguments. `--json --metrics on/off [extra]... code_paths...`
    new_args.append(&mut args.to_vector());

    // run Semgrep.
    // internal_exec(args_str);
    let result = internal_exec(new_args);

    // delete the temporary file.
    fs::remove_file(tmp_file_path)?;
    result
}

/// run Semgrep
fn internal_exec(args: Vec<String>) -> Result<Output> {
    Command::new("semgrep")
        .args(args)
        .output()
        .map_err(Error::from)
}

/// return true if the Semgrep command is available.
fn check_installation() -> bool {
    match internal_exec(vec!["--version".to_string()]) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_installation() {
        match check_installation() {
            false => println!("semgrep is not installed"),
            true => println!("semgrep is installed"),
        }
    }
}
