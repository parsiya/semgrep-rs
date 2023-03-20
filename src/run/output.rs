use crate::{CliOutput, Result};
use std::process;

/// The result of a Semgrep CLI execution.
pub struct Output {
    /// The status (exit code) of the process copied from the command result.
    pub status: process::ExitStatus,
    /// Deserialized output of Semgrep.
    pub clioutput: CliOutput,
}

impl Output {
    /// Convert a std::process:Output to our semgrep_rs::Output.
    pub(crate) fn from_result(r: &process::Output) -> Result<Output> {
        // deserialize the command output in stdout.
        let output_string = String::from_utf8_lossy(&r.stdout).to_string();

        CliOutput::from_json(&output_string).map(|out| Output {
            status: r.status,
            clioutput: out,
        })
    }
}
