use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::semgrep_generic_rule::GenericRuleFile;
use crate::utils::{find_files, read_file_to_string};
use crate::GenericRule;

use log::error;

// ----- START GenericRuleIndex

#[allow(dead_code)]
pub struct GenericRuleIndex {
    index: HashMap<String, GenericRule>,
    complete: bool,
}

impl GenericRuleIndex {
    fn new(complete: bool) -> GenericRuleIndex {
        let index: HashMap<String, GenericRule> = HashMap::new();
        let gri: GenericRuleIndex = GenericRuleIndex { index, complete };
        gri
    }

    pub fn get_index(&self) -> &HashMap<String, GenericRule> {
        return &self.index;
    }

    pub fn get_ids(&self) -> Vec<String> {
        let out: Vec<String> = self.index.keys().map(|k| k.to_string()).collect();
        out
    }

    pub fn from_path_simple(path: &str) -> GenericRuleIndex {
        return GenericRuleIndex::from_path(path, None, None, false);
    }

    // create and return a new GenericRuleIndex.
    pub fn from_path(
        path: &str,
        include: Option<Vec<&str>>,
        exclude: Option<Vec<&str>>,
        complete: bool,
    ) -> GenericRuleIndex {
        let mut gri = GenericRuleIndex::new(complete);

        // ZZZ add error handling?
        // we will panic here if there are errors but I don't think we care, we
        // want to know if our rule index was not created successfully so the
        // server can shut down and the user can fix the error.
        gri.index = create_generic_rule_index(&path, include, exclude, complete).unwrap();
        gri
    }

    // creates a RuleFile (that represents a Policy) with the provided rule IDs.
    pub fn create_policy(&self, rule_ids: &Vec<String>) -> GenericRuleFile {
        // let mut rules: Vec<GenericRule> = Vec::new();
        // for id in &rule_ids {
        //     if let Some(rule) = self.index.get(id) {
        //         rules.push(rule.clone());
        //     }
        // }

        // ChatGPT rewrite with combinators.
        // What happens if the rule is not in the index?.
        let rules: Vec<GenericRule> = rule_ids
            .iter()
            .filter_map(|id| self.index.get(id))
            .cloned()
            .collect();

        GenericRuleFile { rules }
    }

    // returns a rule if it exists in the index, otherwise, returns None.
    pub fn get_rule(&self, rule_id: &str) -> Option<GenericRule> {
        self.index.get(rule_id).cloned()
    }
}

// ----- END GenericRuleIndex

// return an index of rules where the key is the rule ID and the value is the
// rule.
//
// If `complete` it true, this function uses the same ID that Semgrep uses which
// contains the path followed by the rule ID in the file. E.g., if the
// /rules/cpp/security/buffer-overflow.cpp file contains the rule with ID
// buffer-overflow, the complete ruleID will be
// rules.cpp/security.buffer-overflow.buffer-overflow. Hence, rule ID is very
// much dependent on the path of the registry passed to the server.
//
// If `complete` is false, just the rule ID from the file will be used.
pub(crate) fn create_generic_rule_index(
    path: &str,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
    complete: bool,
) -> Result<HashMap<String, GenericRule>> {
    // check the path.
    // ZZZ is this needed? Supposedly we will check the path before calling this function.
    // utils::check_path(&path)?;

    let rule_paths: Vec<String> = find_files(path, include, exclude);

    let mut rule_index: HashMap<String, GenericRule> = HashMap::new();

    for rule_file_path in rule_paths {
        let content = match read_file_to_string(&rule_file_path) {
            Ok(cn) => cn,
            Err(e) => {
                // ZZZ need error logging
                error!("Error reading file: {}", e.to_string());
                continue;
            }
        };

        // create a rule file from the string
        let rule_file = match GenericRuleFile::from_yaml(content) {
            Ok(rf) => rf,
            Err(e) => {
                // ZZZ need error logging
                error!("Error deserializing file: {}", e.to_string());
                continue;
            }
        };

        // get the file index
        let file_index: HashMap<String, GenericRule> =
            rule_file.create_index(&rule_file_path, complete);

        // merge it into the main index
        rule_index.extend(file_index);
    }

    if rule_index.keys().len() == 0 {
        return Error::wrap_str("Rule index is empty.");
    }

    Ok(rule_index)
}
