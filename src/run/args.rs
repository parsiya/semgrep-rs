use crate::error::{Error, Result};
use crate::run::exec;
use crate::CliOutput;

/// arguments passed to Semgrep. Note this is a small subset of possible
/// command-line arguments. You can add any arguments in Extra. For the complete
/// list run `semgrep scan --help`.
///
/// The final command will look like:
/// semgrep -c=tmp_file_with_rules --json --metrics=on/off [extra...] scan_paths.
pub struct Args {
    /// Semgrep rules as a string instead of a file.
    pub rules: String,
    /// value of the Semgrep `metrics` CLI argument, default is `off`
    /// (`--metrics=off`). Note metrics will be collected regardless of this
    /// field on certain invocations like `-c=p/default`. See the docs at:
    /// https://semgrep.dev/docs/metrics/.
    metrics: Metrics,
    /// add other flags here, the contents will be passed to the tool as-is
    /// before scan_paths and separated by space.
    extra: Option<Vec<String>>,
    /// paths scanned with Semgrep.
    paths: Vec<String>,
}

impl Args {
    /// return an instance of SemgrepArgs wth default values.
    pub fn new(rules: String, paths: Vec<String>) -> Args {
        Args {
            rules,
            metrics: Metrics::Off,
            extra: None,
            paths,
        }
    }

    /// enable metrics (e.g., pass --metrics=on to the Semgrep CLI).
    pub fn enable_metrics(&mut self) {
        self.metrics = Metrics::On;
    }

    /// return the arguments (except rules) as a string separated by ` `.
    pub fn to_string(&self) -> String {
        self.to_vector().join(" ")
    }

    /// return the arguments (except rules) as a Vec<String>.
    pub fn to_vector(&self) -> Vec<String> {
        // start with `--json`.
        let mut out: Vec<String> = vec!["--json".to_string()];
        // skipping config because the rules are stored in a String that should
        // be written to a file before execution.

        // `--metrics on/off`
        out.push("--metrics".to_string());
        out.push(self.metrics.to_string());

        // add arguments in extra.
        if let Some(args) = &self.extra {
            out.extend(args.to_owned());
        }

        // add code paths.
        out.extend(self.paths.to_owned());

        out
    }

    /// run Semgrep and return the results.
    pub fn execute(&self) -> Result<CliOutput> {
        let res = exec::execute(self)?;
        // if Semgrep executed successfully but with errors (exit code !=0) then
        // stderr will be empty. We need to read the `errors` key in the output
        // result to read the errors.

        // try and deserialize the output.
        let strr = String::from_utf8(res.stdout)?;
        CliOutput::from_json(&strr)
    }
}

/// values for the Semgrep `metrics` CLI argument. Could have been a simple
/// boolean but I wanted to practice using Rust's string enums.
enum Metrics {
    On,
    Off,
}

impl Metrics {
    /// return "on" or "off".
    fn as_str(&self) -> &'static str {
        match self {
            Metrics::On => "on",
            Metrics::Off => "off",
        }
    }
    /// return "on" or "off" as String. AKA, a Rube Goldberg machine.
    fn to_string(&self) -> String {
        // self.as_str().to_string()
        match self {
            Metrics::On => "on".to_string(),
            Metrics::Off => "off".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_args() {
        // not used here, just used to create this object.
        let rules = "Some random string".to_string();
        let paths = vec!["path1".to_string(), "path2".to_string()];

        // convert it to a string.
        let mut args = Args::new(rules, paths);
        assert_eq!(args.to_string(), "--metrics off path1 path2");

        // enable metrics.
        args.enable_metrics();
        assert_eq!(args.to_string(), "--metrics on path1 path2");
    }

    #[test]
    fn test_metrics() {
        assert_eq!(Metrics::On.as_str(), "on");
        assert_eq!(Metrics::On.to_string(), "on".to_string());

        assert_eq!(Metrics::Off.as_str(), "off");
        assert_eq!(Metrics::Off.to_string(), "off".to_string());
    }
}
