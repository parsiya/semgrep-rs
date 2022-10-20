use serde::{Serialize, Deserialize};
use serde_with::{serde_as, OneOrMany, skip_serializing_none};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleFile {
    pub rules: Vec<Rule>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    // Rule ID to attach to findings
    pub id: String,
    // Version of rule
    pub version: Option<String>,
    // Description to attach to findings
    pub message: Option<String>,

    pub mode: Option<String>,

    // Languages this pattern should run on
    pub languages: Vec<String>,

    // Path globs this pattern should run on
    pub paths: Option<Paths>,

    // Severity to report alongside this finding
    pub severity: Option<String>,

    #[serde(rename = "pattern-sinks")]
    pub pattern_sinks: Option<Vec<TaintContent>>,

    #[serde(rename = "pattern-sources")]
    pub pattern_sources: Option<Vec<TaintContent>>,

    #[serde(rename = "pattern-propagators")]
    pub pattern_propagators: Option<Vec<TaintContent>>,

    #[serde(rename = "pattern-sanitizers")]
    pub pattern_sanitizers: Option<Vec<TaintContent>>,

    pub join: Option<Join>,

    // Replacement text to fix matched code. Can use matched metavariables.
    pub fix: Option<String>,

    // Replacement regex to fix matched code.
    #[serde(rename = "fix-regex")]
    pub fix_regex: Option<FixRegex>,

    // Arbitrary structured data for your own reference
    pub metadata: Option<serde_yaml::Mapping>,

    // Options object to enable/disable certain matching features in semgrep-core
    pub options: Option<serde_yaml::Mapping>,

    // Return finding where Semgrep pattern matches exactly
    pub pattern: Option<String>,

    // Return finding where regular expression matches exactly
    #[serde(rename = "pattern-regex")]
    pub pattern_regex: Option<String>,

    // patterns
    pub patterns: Option<Vec<PatternsContent>>,

    // pattern-either
    #[serde(rename = "pattern-either")]
    pub pattern_either: Option<Vec<PatternEitherContent>>,

    #[serde(rename = "r2c-internal-project-depends-on")]
    pub r2c_internal_project_depends_on: Option<R2CInternalProjectDependsOnContent>,

    // Metavariable whose content to use as the extracted result for subsequent rules
    pub extract: Option<String>,

    // Language to process the extracted result of this rule as
    #[serde(rename = "dest-language")]
    pub dest_language: Option<String>,

    // Method of intrafile match reduction
    pub reduce: Option<String>,

}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paths {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}


// what's inside "taint-content:" in the schema.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaintContent {
    // Return finding where Semgrep pattern matches exactly
    pattern: Option<String>,

    #[serde(rename = "pattern-regex")]
    pattern_regex: Option<String>,

    patterns: Option<Vec<PatternsContent>>,

    #[serde(rename = "pattern-either")]
    pattern_either: Option<Vec<PatternEitherContent>>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixRegex {
    // Replace up to this many regex matches
    count: Option<i32>,
    // Regular expression to find in matched code
    regex: String,
    // Code to replace the regular expression match with. Can use capture groups.
    replacement: String,
}


// search for "r2c-internal-project-depends-on-content:".
// One or more dependencies that the project contains in a lock file
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct R2CInternalProjectDependsOnContent {

    namespace: Option<String>,
    package: Option<String>,
    version: Option<String>,

    #[serde(rename = "depends-on-either")]
    depends_on_either: Option<Vec<DependsOnEither>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependsOnEither {

    namespace: Option<String>,
    package: Option<String>,
    version: Option<String>,
}


// patterns-content
#[serde_as] // serde_as
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternsContent {
    
    patterns: Option<Vec<PatternsContent>>,

    // Return finding where any of the nested conditions are true
    #[serde(rename = "pattern-either")]
    pattern_either: Option<Vec<PatternEitherContent>>,
    
    // "focus-metavariable:"
    // Focus on what a given metavariable is matching
    #[serde_as(as = "Option<OneOrMany<_>>")] // serde_as
    #[serde(rename = "focus-metavariable")]
    focus_metavariable: Option<Vec<String>>,
    
    // "pattern-inside:"
    // Focus on what a given metavariable is matching
    #[serde(rename = "pattern-inside")]
    pattern_inside: Option<String>,

    // "pattern-not-inside:"
    // Do not return findings from within snippets Semgrep pattern matches
    #[serde(rename = "pattern-not-inside")]
    pattern_not_inside: Option<String>,

    // "pattern-not:"
    // Do not return finding where Semgrep pattern matches exactly
    #[serde(rename = "pattern-not")]
    pattern_not: Option<String>,

    // "pattern:"
    // Return finding where Semgrep pattern matches exactly
    pattern: Option<String>,

    // "pattern-regex:"
    #[serde(rename = "pattern-regex")]
    pattern_regex: Option<String>,

    // "pattern-not-regex:"
    #[serde(rename = "pattern-not-regex")]
    pattern_not_regex: Option<String>,

    // "pattern-where-python:"
    // Return finding where Python expression returns true
    #[serde(rename = "pattern-where-python")]
    pattern_where_python: Option<String>,

    #[serde(rename = "metavariable-analysis")]
    metavariable_analysis: Option<MetavariableAnalysis>,

    #[serde(rename = "metavariable-regex")]
    metavariable_regex: Option<MetavariableRegex>,

    #[serde(rename = "metavariable-pattern")]
    metavariable_pattern: Option<Box<MetavariablePattern>>,

    #[serde(rename = "metavariable-comparison")]
    metavariable_comparison: Option<MetavariableComparison>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternEitherContent {

    patterns: Option<Vec<PatternsContent>>,

    #[serde(rename = "pattern-either")]
    pattern_either: Option<Vec<PatternEitherContent>>,

    #[serde(rename = "pattern-inside")]
    pattern_inside: Option<String>,
    
    pattern: Option<String>,

    #[serde(rename = "pattern-regex")]
    pattern_regex: Option<String>,
}


// "metavariable-analysis:"
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetavariableAnalysis {
    analyzer: String,
    metavariable: String,
    // freeform attribute
    options: Option<serde_yaml::Value>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetavariableRegex {
    metavariable: String,
    regex: String,
    #[serde(rename = "constant-propagation")]
    constant_propagation: Option<bool>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetavariablePattern {
    metavariable: String,

    language: Option<String>,

    // Return finding where Semgrep pattern matches exactly
    pattern: Option<String>,

    // Return finding where regular expression matches exactly
    #[serde(rename = "pattern-regex")]
    pattern_regex: Option<String>,
    
    patterns: Option<Vec<PatternsContent>>,

    #[serde(rename = "pattern-either")]
    pattern_either: Option<Vec<PatternEitherContent>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetavariableComparison {

    comparison: Option<String>,
    strip: Option<bool>,
    metavariable: Option<String>,
    base: Option<i64>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Join {

    #[serde(rename = "refs")]
    refs: Vec<Ref>,

    #[serde(rename = "on")]
    on: Vec<String>,

    #[serde(rename = "rules")]
    rules: Vec<JoinRule>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ref {

    #[serde(rename = "rule", skip_serializing_if = "Option::is_none")]
    rule: Option<String>,

    #[serde(rename = "renames", skip_serializing_if = "Option::is_none")]
    renames: Option<Vec<Rename>>,

    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    ref_as: Option<String>,
}


// the complete file which is an array of rules.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRule {

    // required
    id: String,
    languages: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    patterns: Option<Vec<PatternsContent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,

    #[serde(rename = "pattern-sources", skip_serializing_if = "Option::is_none")]
    pattern_sources: Option<TaintContent>,
    
    #[serde(rename = "pattern-propagators", skip_serializing_if = "Option::is_none")]
    pattern_propagators: Option<TaintContent>,

    #[serde(rename = "pattern-sinks", skip_serializing_if = "Option::is_none")]
    pattern_sinks: Option<TaintContent>,

    #[serde(rename = "pattern-sanitizers", skip_serializing_if = "Option::is_none")]
    pattern_sanitizers: Option<TaintContent>,
}


#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rename {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}


impl RuleFile {

    // split the rules in a file and return each one as a RuleFiles.
    fn split(&self) -> Vec<RuleFile> {
        
        let mut rule_files: Vec<RuleFile> = Vec::new();

        for rule in self.rules.clone() {
            let new_rules: Vec<Rule> = vec![rule];
            rule_files.push(RuleFile { rules: new_rules });
        }
        rule_files
    }
}