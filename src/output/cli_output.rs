use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct CliOutput {
    pub errors: Vec<CliError>,
    pub results: Vec<CliMatch>,
    pub paths: CliPaths,
    pub version: Option<String>,
    pub time: Option<CliTiming>,
    pub explanations: Option<Vec<MatchingExplanation>>,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub col: i64,
    pub line: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub path: String,
    pub start: Position,
    pub end: Position,
}

// An enum without data is serialized as a string so we just need to add the
// renames.
// https://users.rust-lang.org/t/serde-serialize-enum-as-string/37549/2
#[derive(Serialize, Deserialize)]
pub enum EngineKind {
    #[serde(rename = "OSSMatch")]
    OSSMatch,
    #[serde(rename = "ProMatch")]
    ProMatch,
}

type RuleId = String;

#[derive(Serialize, Deserialize)]
pub struct CoreMatch {
    pub rule_id: RuleId,
    pub location: Location,
    pub extra: CoreMatchExtra,
}

#[derive(Serialize, Deserialize)]
pub struct CoreMatchExtra {
    pub metavars: Metavars,
    // pub dataflow_trace: CoreMatchDataflowTrace,
    pub dataflow_trace: serde_json::Value,
    pub message: Option<String>,
    pub rendered_fix: Option<String>,
    pub engine_kind: Option<EngineKind>,
}

// #[derive(Serialize, Deserialize)]
// pub struct CoreMatchCallTrace {
//     // skipping this.
// }

#[derive(Serialize, Deserialize)]
pub struct CoreMatchDataflowTrace {
    // pub taint_source: Option<CoreMatchCallTrace>,
    pub taint_source: Option<serde_json::Value>,
    pub intermediate_vars: Option<Vec<CoreMatchIntermediateVar>>,
    // pub taint_sink: Option<CoreMatchCallTrace>,
    pub taint_sink: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct CoreMatchIntermediateVar {
    pub location: Location,
}

#[derive(Serialize, Deserialize)]
pub struct Metavars {
    #[serde(rename = "additionalProperties")]
    additional_properties: Option<MetavarValue>,
}

#[derive(Serialize, Deserialize)]
pub struct MetavarValue {
    pub start: Position,
    pub end: Position,
    pub abstract_content: String,
    pub propagated_value: Option<SValueValue>,
}

#[derive(Serialize, Deserialize)]
pub struct SValueValue {
    pub svalue_start: Option<Position>,
    pub svalue_end: Option<Position>,
    pub svalue_abstract_content: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoreError {
    // pub error_type: CoreErrorKind,
    pub error_type: serde_json::Value,
    pub severity: CoreSeverity,
    pub location: Location,
    pub message: String,
    pub rule_id: Option<RuleId>,
    pub details: Option<String>,
}

// #[derive(Serialize, Deserialize)]
// pub enum CoreErrorKind {
//     // Missing two types here which have the type: {"const string", Vec<String>}
//     // {"some const string", {"str1", "str2", "str3"}}
//     #[serde(rename = "Lexical error")]
//     LexicalError,
//     #[serde(rename = "Syntax error")]
//     SyntaxError,
//     #[serde(rename = "Other syntax error")]
//     OtherSyntaxError,
//     #[serde(rename = "AST builder error")]
//     ASTBuilderError,
//     #[serde(rename = "Rule parse error")]
//     RuleParseError,
//     #[serde(rename = "Invalid YAML")]
//     InvalidYAML,
//     #[serde(rename = "Internal matching error")]
//     InternalMatchingError,
//     #[serde(rename = "Semgrep match found")]
//     SemgrepMatchFound,
//     #[serde(rename = "Too many matches")]
//     TooManyMatches,
//     #[serde(rename = "Fatal error")]
//     FatalError,
//     #[serde(rename = "Timeout")]
//     Timeout,
//     #[serde(rename = "Out of memory")]
//     OutOfMemory,
//     #[serde(rename = "Timeout during interfile analysis")]
//     TimeoutDuringInterfileAnalysis,
//     #[serde(rename = "OOM during interfile analysis")]
//     OOMDuringInterfileAnalysis,
// }

#[derive(Serialize, Deserialize)]
pub enum CoreSeverity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
}

#[derive(Serialize, Deserialize)]
pub struct CoreStats {
    #[serde(rename = "okfiles")]
    pub ok_files: i64,
    #[serde(rename = "errorfiles")]
    pub error_files: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SkippedTarget {
    pub path: String,
    pub reason: SkipReason,
    pub details: String,
    pub rule_id: Option<RuleId>,
}

#[derive(Serialize, Deserialize)]
pub enum SkipReason {
    #[serde(rename = "excluded_by_config")]
    ExcludedByConfig,
    #[serde(rename = "wrong_language")]
    WrongLanguage,
    #[serde(rename = "too_big")]
    TooBig,
    #[serde(rename = "minified")]
    Minified,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "irrelevant_rule")]
    IrrelevantRule,
    #[serde(rename = "too_many_matches")]
    TooManyMatches,
}

#[derive(Serialize, Deserialize)]
pub struct SkippedRule {
    pub rule_id: RuleId,
    pub details: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize)]
pub struct CoreTiming {
    pub targets: Vec<TargetTime>,
    pub rules: Vec<String>,
    pub max_memory_bytes: i64,
    pub rules_parse_time: Option<f64>, // JSON Schema type: `number`.
}

#[derive(Serialize, Deserialize)]
pub struct TargetTime {
    pub path: String,
    pub rule_times: Vec<RuleTimes>,
    pub run_time: f64,
}

#[derive(Serialize, Deserialize)]
pub struct RuleTimes {
    pub rule_id: RuleId,
    pub parse_time: f64,
    pub match_time: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MatchingExplanation {
    // pub op: MatchingOperation,
    pub op: serde_json::Value,
    pub children: Vec<MatchingExplanation>,
    pub matches: Vec<CoreMatch>,
    pub loc: Location,
}

// #[derive(Serialize, Deserialize)]
// pub enum MatchingOperation {}

#[derive(Serialize, Deserialize)]
pub struct CveResult {
    // Not used anywhere other than CveResults (also unused)
    pub url: String,
    pub filename: String,
    pub funcnames: Vec<String>,
}

// Type alias.
type CveResults = Vec<CveResult>; // Not used anywhere in the schema.

#[derive(Serialize, Deserialize)]
pub struct CoreMatchResults {
    pub matches: Vec<CoreMatch>,
    pub errors: Vec<CoreError>,
    pub stats: CoreStats,
    pub skipped: Option<Vec<SkippedTarget>>,
    pub skipped_rules: Option<Vec<SkippedRule>>,
    pub explanations: Option<Vec<MatchingExplanation>>,
    pub time: Option<CoreTiming>,
}

#[derive(Serialize, Deserialize)]
pub struct CliError {
    pub code: i64,
    pub level: String,
    #[serde(rename = "type")] // can't have a field named type.
    pub type_: String,
    pub rule_id: Option<RuleId>,
    pub message: Option<String>,
    pub path: Option<String>,
    pub long_msg: Option<String>,
    pub short_msg: Option<String>,
    pub spans: Option<Vec<ErrorSpan>>,
    pub help: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorSpan {
    pub file: String,
    pub start: PositionBis,
    pub end: PositionBis,
    pub source_hash: Option<String>,
    pub config_start: Option<PositionBis>,
    pub config_end: Option<PositionBis>,
    pub config_path: Option<Vec<String>>, // "type": [ "array", "null" ],
    pub context_start: Option<PositionBis>,
    pub context_end: Option<PositionBis>,
}

#[derive(Serialize, Deserialize)]
pub struct PositionBis {
    pub line: i64,
    pub col: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CliMatchCallTrace {
    // Skipping this.
}

#[derive(Serialize, Deserialize)]
pub struct CliMatchDataflowTrace {
    pub taint_source: Option<CliMatchCallTrace>,
    pub intermediate_vars: Option<Vec<CliMatchIntermediateVar>>,
    pub taint_sink: Option<CliMatchCallTrace>,
}

#[derive(Serialize, Deserialize)]
pub struct CliMatchTaintSource {
    pub location: Location,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct CliMatchIntermediateVar {
    pub location: Location,
    pub content: String,
}

// Search for `"cli_match"` in the JSON schema. Value of "results" in the output.
#[derive(Serialize, Deserialize)]
pub struct CliMatch {
    pub check_id: RuleId, // Type in the schema is rule_id which is a String.
    pub path: String,
    pub start: Position,
    pub end: Position,
    pub extra: CliMatchExtra,
}

#[derive(Serialize, Deserialize)]
pub struct CliMatchExtra {
    pub fingerprint: String,
    pub lines: String,
    pub message: String,
    pub metadata: serde_json::Value, // raw_json
    pub severity: String,
    pub engine_kind: EngineKind,
    pub is_ignored: Option<bool>,
    pub metavars: Option<Metavars>,
    pub fix: Option<String>,
    pub fix_regex: Option<FixRegex>,
    pub sca_info: Option<ScaInfo>,
    pub fixed_lines: Option<Vec<String>>,
    pub dataflow_trace: Option<CliMatchDataflowTrace>,
}

#[derive(Serialize, Deserialize)]
pub struct FixRegex {
    pub regex: String,
    pub replacement: String,
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CliOutputExtra {
    pub paths: CliPaths, // Different from CliPath!
    pub time: Option<CliTiming>,
    pub explanations: Option<Vec<MatchingExplanation>>,
}

#[derive(Serialize, Deserialize)]
pub struct CliPaths {
    pub scanned: Vec<String>,
    #[serde(rename = "_comment")]
    pub comment: Option<String>,
    pub skipped: Option<Vec<CliSkippedTarget>>,
}

#[derive(Serialize, Deserialize)]
pub struct CliSkippedTarget {
    pub path: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct CliTiming {
    pub rules: Vec<RuleIdDict>,
    pub rules_parse_time: f64,
    pub profiling_times: Vec<f64>,
    pub targets: Vec<CliTargetTimes>,
    pub total_bytes: f64,
    pub max_memory_bytes: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RuleIdDict {
    pub id: RuleId,
}

#[derive(Serialize, Deserialize)]
pub struct CliTargetTimes {
    pub path: String,
    pub num_bytes: f64,
    pub match_times: Vec<f64>,
    pub parse_times: Vec<f64>,
    pub run_time: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ScaInfo {
    pub reachable: bool,
    pub reachability_rule: bool,
    pub sca_finding_schema: i64,
    pub dependency_match: DependencyMatch,
}

#[derive(Serialize, Deserialize)]
pub struct DependencyMatch {
    pub dependency_pattern: DependencyPattern,
    pub found_dependency: FoundDependency,
    pub lockfile: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // converts everything to lower case
pub enum Ecosystem {
    Npm, // npm
    Pypi,
    Gem,
    Gomod,
    Cargo,
    Maven,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // converts everything to lower case
pub enum Transitivity {
    Direct,
    Transitive,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct DependencyPattern {
    pub ecosystem: Ecosystem,
    pub package: String,
    pub semver_range: String,
}

#[derive(Serialize, Deserialize)]
pub struct FoundDependency {
    pub package: String,
    pub version: String,
    pub ecosystem: Ecosystem,
    pub allowed_hashes: AllowedHashes,
    pub transitivity: Transitivity,
    pub resolved_url: Option<String>,
    pub line_number: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct AllowedHashes {
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiScansFindings {
    pub findings: Vec<Finding>,
    pub token: Option<String>,
    pub gitlab_token: Option<String>,
    pub searched_paths: Vec<String>,
    pub rule_ids: Vec<String>,
    pub cai_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Finding {
    pub check_id: RuleId,
    pub path: String,
    pub line: f64,
    pub column: f64,
    pub end_line: f64,
    pub end_column: f64,
    pub message: String,
    pub severity: f64,
    pub index: f64,
    pub commit_date: String,
    pub syntactic_id: String,
    pub match_based_id: Option<String>,
    pub metadata: serde_yaml::Value,
    pub is_blocking: bool,
    pub fixed_lines: Option<Vec<String>>,
    pub sca_info: Option<ScaInfo>,
    pub dataflow_trace: Option<CliMatchDataflowTrace>,
}
