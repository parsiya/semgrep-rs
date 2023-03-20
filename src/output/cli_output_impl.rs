use crate::error::{Error, Result};
use crate::utils::read_file_to_string;
use crate::CliOutput;

impl CliOutput {
    // Deserialize a JSON string into a CliOutput object.
    pub fn from_json(str: &str) -> Result<CliOutput> {
        serde_json::from_str::<CliOutput>(str).map_err(Error::from)
    }

    // Read the JSON string from a file.
    pub fn from_json_file(file_path: &str) -> Result<CliOutput> {
        read_file_to_string(file_path).map(|str| CliOutput::from_json(&str))?

        // read_file_to_string(file_path)
        //     .map_err(|e| Error::new(e.to_string()))
        //     .and_then(|str| CliOutput::from_json(&str))
    }
}

#[cfg(test)]
mod tests {

    use super::CliOutput;

    #[test]
    fn test_cli_output_from() {
        let js = CliOutput::from_json_file("tests/outputs/juice-shop-default.json").unwrap();

        // Rust will hide the output when running tests unless something goes
        // bad. But we're trying to figure out if we can just make it again.
        print!("{}", serde_json::to_string(&js).unwrap());

        // Check some of the errors.
        assert_eq!(js.errors[0].code, 3);
        assert_eq!(js.errors[2].type_, "Syntax error");
        assert_eq!(js.errors[3].spans.as_ref().unwrap()[0].end.col, 44);

        // Check some of the scanned paths.
        assert_eq!(js.paths.scanned[0], "juice-shop/.codeclimate.yml"); // First path
        assert_eq!(
            js.paths.scanned[js.paths.scanned.len() - 1],
            "juice-shop/views/userProfile.pug"
        ); // Last path

        // Check some of the results.
        assert_eq!(js.results[0].end.col, 61);
        assert_eq!(js.results[0].path, "juice-shop/lib/insecurity.ts");
        assert_eq!(js.results[js.results.len() - 1].end.offset, 85);
    }

    #[test]
    fn test_cli_output_from2() {
        let js = CliOutput::from_json_file("tests/outputs/juice-shop-verbose.json").unwrap();

        // Rust will hide the output when running tests unless something goes
        // bad. But we're trying to figure out if we can just make it again.
        print!("{}", serde_json::to_string(&js).unwrap());

        // Check some of the skipped paths.
        let skipped_paths = js.paths.skipped.as_ref().unwrap();
        // Check first skipped path.
        assert_eq!(skipped_paths[0].path, "/tmp/tmp-26472-22c6eb.mvar-pattern");

        assert_eq!(
            skipped_paths[skipped_paths.len() - 1].path,
            "juice-shop/test/smoke/smoke-test.sh"
        );
    }
}
