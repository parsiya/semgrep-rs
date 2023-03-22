// Example that creates a CLI applicatio (using clap) to use the library.

use clap::{Parser, Subcommand};
use log::info;
use std::fs;

// clap CLI struct.
#[derive(Parser, Debug)]
// #[command(override_usage = "./cli ZZZZ -r path/to/rules/ [-p path/to/policies/] [-s 9090] [-q]")]
#[command(version = "0.1")]
#[command(about = "semgrep-rs usage example", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

// subcommands
#[derive(Subcommand, Debug)]
enum Action {
    /// combines rules into one file
    Combine {
        /// paths to the rules directories or files
        paths: Vec<String>,

        /// path to the output file
        #[arg(short, long = "output")]
        output: String,
    },
    /// runs Semgrep
    Run {
        /// path(s) to the code to scan
        paths: Vec<String>,

        /// path to the rule(s) file or directory
        #[arg(short, long = "config")]
        config: String,

        /// send metrics to Semgrep (default is off)
        #[arg(short, long = "metrics")]
        metrics: bool,

        /// store the results in a file
        #[arg(short, long = "output")]
        output: String,

        /// output format
        #[arg(short, long = "format")]
        format: String,

        /// other flags passed to Semgrep as-is. E.g., `--extra "--force-color" --extra "--jobs 4"`
        /// you can repeat this flag multiple times.
        #[arg(short, long = "other")]
        extra: Option<Vec<String>>,
    },
}

fn main() {
    // check is Semgrep is installed, if not, return an error.
    if !semgrep_rs::is_installed() {
        eprintln!("Semgrep is not installed or detected, try `python3 -m pip install semgrep`.");
        std::process::exit(1);
    }

    // parse stuff
    let cli = Cli::parse();
    match cli.action {
        Action::Combine {
            paths: rules,
            output,
        } => run_combine(&rules, &output),
        Action::Run {
            paths,
            config,
            metrics,
            output,
            format,
            extra,
        } => run(paths, &config, metrics, &output, &format, extra),
    };
}

// Combine rules in paths and write it to output.
fn run_combine(paths: &Vec<String>, output: &str) {
    // Convert Vec<String> to Vec<&str>.
    let r: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    // Create a rule index from all paths. Inaccessible and hidden files/paths
    // will be ignored. Panic on fatal errors.
    let rule_index = semgrep_rs::GenericRuleIndex::from_paths_simple(r).unwrap();
    // Create a YAML file from all rules and panic on errors.
    let content = rule_index.get_all().to_string().unwrap();
    // Write the yaml file to disk.
    fs::write(output, content).expect("couldn't write the rule file");
    info!("Wrote the combined rule file to: {}", output);
}

// run semgrep with the given paths and config and return the results.
fn run(
    paths: Vec<String>,
    config: &str,
    metrics: bool,
    output: &str,
    format: &str,
    extra: Option<Vec<String>>,
) {
    // Read all the rules from the config file.
    // Create a rule index.
    let ri = semgrep_rs::GenericRuleIndex::from_paths_simple(vec![config]).unwrap();
    // Create a rule file from all the rules in the index.
    let all_rules = ri.get_all().to_string().unwrap();

    // Convert format to OutputFormat.
    let format = semgrep_rs::OutputFormat::from_str(&format).unwrap();

    // Create an instance of args.
    let args = semgrep_rs::Args::new(all_rules, paths, metrics, format, extra);

    // Get the command as a text.
    info!("Running: {}", args.to_string());

    // Run semgrep with the given paths and config and return the results.
    let results = args.execute().unwrap();
    let bytes = results.to_json_bytes().unwrap();
    // Write the results to disk.
    fs::write(output, bytes).expect("couldn't write the results file");
    info!("Wrote the results to: {}", output);
}
