use crate::error::Result;
use crate::CliOutput;
use crate::OutputFormat;

use super::exec;

/// arguments passed to Semgrep. Note this is a small subset of possible
/// command-line arguments. You can add any arguments in Extra. For the complete
/// list run `semgrep scan --help`.
///
/// The final command will look like:
/// semgrep -c=tmp_file_with_rules --json --metrics=on/off [extra...] scan_paths.
pub struct Args {
    /// Semgrep rules as a string.
    pub rules: String,
    /// value of the Semgrep `metrics` CLI argument, default is `off`
    /// (`--metrics=off`). Note metrics will be collected regardless of this
    /// field on certain invocations like `-c=p/default`. See the docs at:
    /// https://semgrep.dev/docs/metrics/.
    metrics: Metrics,
    /// other flags, passed to the tool as-is before scan_paths and separated by
    /// space.
    pub extra: Option<Vec<String>>,
    /// paths scanned with Semgrep.
    pub paths: Vec<String>,
    /// the output format
    pub output_format: OutputFormat,
}

impl Args {
    /// return a new instance of Args.
    /// # Arguments
    /// * `rules` - Semgrep rules as a string.
    /// * `paths` - paths scanned with Semgrep.
    /// * `metrics` - value of the Semgrep `metrics` CLI argument.
    /// * `output_format` - the output format (e.g., JSON)
    /// * `extra` - other flags, passed to the tool as-is.
    pub fn new(
        rules: String,
        paths: Vec<String>,
        metrics: bool,
        output_format: OutputFormat,
        extra: Option<Vec<String>>,
    ) -> Args {
        Args {
            rules,
            paths,
            metrics: Metrics::from_bool(metrics),
            output_format,
            extra,
        }
    }

    /// return an instance of SemgrepArgs wth default values.
    pub fn default(rules: String, paths: Vec<String>) -> Args {
        // convert paths to a Vec<String>.
        Args {
            rules,
            paths,
            metrics: Metrics::Off,
            output_format: OutputFormat::JSON,
            extra: None,
        }
    }

    /// enable metrics (e.g., pass --metrics=on to the Semgrep CLI).
    pub fn enable_metrics(&mut self) {
        self.metrics = Metrics::On;
    }

    /// disable metrics (e.g., pass --metrics=off to the Semgrep CLI). This is
    /// the default value for metrics.
    pub fn disable_metrics(&mut self) {
        self.metrics = Metrics::Off;
    }

    /// add extra arguments to the Semgrep CLI. This will be appended to the
    /// current extra arguments.
    pub fn add_extra(&mut self, extra: Vec<String>) {
        if let Some(args) = &mut self.extra {
            args.extend(extra);
        } else {
            self.extra = Some(extra);
        }
    }

    /// return the arguments (except rules) as a string separated by ` `.
    pub fn to_string(&self) -> String {
        self.to_vec().join(" ")
    }

    /// return the arguments (except rules) as a Vec<String>.
    pub fn to_vec(&self) -> Vec<String> {
        // start with the output format.
        let mut out: Vec<String> = vec![self.output_format.to_string()];
        // `--metrics=on/off`
        out.push(self.metrics.to_string());

        // add the arguments in extra.
        if let Some(args) = &self.extra {
            out.extend(args.to_owned());
        }

        // add code paths.
        out.extend(self.paths.to_owned());

        out
    }

    /// run Semgrep and return the results.
    pub fn execute(&self) -> Result<CliOutput> {
        let res = exec::internal_exec(self)?;
        // if Semgrep executed successfully but with errors (exit code !=0) then
        // stderr will be empty. We need to read the `errors` key in the output
        // result to read the errors.

        // try and deserialize the output.
        let strr = String::from_utf8(res.stdout)?;
        CliOutput::from_json(&strr)
    }
}

/// values for the Semgrep `metrics` CLI argument. Could have been a simple
/// boolean but I wanted to practice using Rust's enums. It's a Rube Goldberg
/// machine.
enum Metrics {
    On,
    Off,
}

impl Metrics {
    /// return "--metrics=on" or "--metrics=off".
    #[allow(dead_code)] // make it undead code, har har!
    fn as_str(&self) -> &'static str {
        match self {
            Metrics::On => "--metrics=on",
            Metrics::Off => "--metrics=off",
        }
    }
    /// return "--metrics=on" or "--metrics=off" as String.
    fn to_string(&self) -> String {
        // self.as_str().to_string()
        match self {
            Metrics::On => "--metrics=on".to_string(),
            Metrics::Off => "--metrics=off".to_string(),
        }
    }
    /// from_bool returns a Metrics enum from a boolean.
    fn from_bool(b: bool) -> Metrics {
        if b {
            Metrics::On
        } else {
            Metrics::Off
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
        let mut args = Args::default(rules, paths);
        assert_eq!(args.to_string(), "--json --metrics=off path1 path2");

        // enable metrics.
        args.enable_metrics();
        assert_eq!(args.to_string(), "--json --metrics=on path1 path2");
    }

    #[test]
    fn test_metrics() {
        assert_eq!(Metrics::On.as_str(), "--metrics=on");
        assert_eq!(Metrics::On.to_string(), "--metrics=on".to_string());

        assert_eq!(Metrics::Off.as_str(), "--metrics=off");
        assert_eq!(Metrics::Off.to_string(), "--metrics=off".to_string());
    }
}
