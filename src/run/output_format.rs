/// values for the semgrep output formats.
pub enum OutputFormat {
    Text,
    Emacs,
    JSON,
    GitLabSAST,
    GitLabSecrets,
    JUnitXML,
    SARIF,
    Vim,
}

impl OutputFormat {
    /// return the output format as a str.
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Text => "--text",
            OutputFormat::Emacs => "--emacs",
            OutputFormat::JSON => "--json",
            OutputFormat::GitLabSAST => "--gitlab-sast",
            OutputFormat::GitLabSecrets => "--gitlab-secrets",
            OutputFormat::JUnitXML => "--junit-xml",
            OutputFormat::SARIF => "--sarif",
            OutputFormat::Vim => "--vim",
        }
    }

    /// return the output format as a String.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// convert a string to an OutputFormat.
    pub fn from_str(s: &str) -> Result<OutputFormat, String> {
        match s {
            "text" => Ok(OutputFormat::Text),
            "emacs" => Ok(OutputFormat::Emacs),
            "json" => Ok(OutputFormat::JSON),
            "gitlab-sast" => Ok(OutputFormat::GitLabSAST),
            "gitlab-secrets" => Ok(OutputFormat::GitLabSecrets),
            "junit-xml" => Ok(OutputFormat::JUnitXML),
            "sarif" => Ok(OutputFormat::SARIF),
            "vim" => Ok(OutputFormat::Vim),
            _ => Err(format!("invalid output format: {}", s)),
        }
    }
}
