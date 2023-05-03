use std::collections::HashMap;

use super::semgrep_generic_rule::{GenericRule, GenericRuleFile};
use crate::error::{Error, Result};
use crate::utils::{find_files, read_file_to_string};

use log::error;

// ----- START GenericRuleIndex

#[allow(dead_code)]
pub struct GenericRuleIndex {
    index: HashMap<String, GenericRule>,
    complete: bool,
}

impl GenericRuleIndex {
    pub fn new(complete: bool) -> GenericRuleIndex {
        GenericRuleIndex {
            index: HashMap::new(),
            complete,
        }
    }

    pub fn get_index(&self) -> &HashMap<String, GenericRule> {
        &self.index
    }

    pub fn get_ids(&self) -> Vec<String> {
        self.index.keys().map(|k| k.to_string()).collect()
    }

    pub fn len(&self) -> usize {
        self.get_ids().len()
    }

    // create and return a new GenericRuleIndex from the files in paths.
    pub fn from_paths_simple(path: Vec<&str>) -> Result<GenericRuleIndex> {
        GenericRuleIndex::from_paths(path, None, None, false)
    }

    // create and return a new GenericRuleIndex from the files in paths.
    pub fn from_paths(
        path: Vec<&str>,
        include: Option<Vec<&str>>,
        exclude: Option<Vec<&str>>,
        complete: bool,
    ) -> Result<GenericRuleIndex> {
        // let mut gri = GenericRuleIndex::new(complete);
        // match create_generic_rule_index(&path, include, exclude, complete) {
        //     Ok(index) => gri.index = index,
        //     Err(e) => return Error::wrap_string(e.to_string()),
        // };
        // Ok(gri)

        generic_rule_index_from_paths(path, include, exclude, complete).map(|index| {
            let mut gri = GenericRuleIndex::new(complete);
            gri.index = index;
            gri
        })
        // .map_err(|e| Error::new(e.to_string()))
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
        // What happens if the rule is not in the index? An error.
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

    // combine all the rules in the index into one file and return.
    pub fn get_all(&self) -> GenericRuleFile {
        // instead of iterating and adding all rules, we use create_policy with
        // all rule IDs in the index.
        self.create_policy(&self.get_ids())
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
fn generic_rule_index_from_paths(
    paths: Vec<&str>,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
    complete: bool,
) -> Result<HashMap<String, GenericRule>> {
    // check the path.
    // TODO is this needed? Supposedly we will check the path before calling this function.
    // utils::check_path(&path)?;

    let mut rule_files: Vec<String> = Vec::new();

    for p in paths {
        rule_files.extend(find_files(p, &include, &exclude));
    }
    generic_rule_index_from_files(rule_files, complete)
}

fn generic_rule_index_from_files(
    rule_files: Vec<String>,
    complete: bool,
) -> Result<HashMap<String, GenericRule>> {
    let mut rule_index: HashMap<String, GenericRule> = HashMap::new();

    for r in rule_files {
        let content = match read_file_to_string(&r) {
            Ok(cn) => cn,
            Err(e) => {
                // TODO: need better error logging
                error!("Error reading file: {}", e.to_string());
                continue;
            }
        };

        // create a rule file from the string
        let rule_file = match GenericRuleFile::from_yaml(&content) {
            Ok(rf) => rf,
            Err(e) => {
                // TODO: need better error logging
                error!("Error deserializing file: {}", e.to_string());
                continue;
            }
        };

        // get the file index
        let file_index: HashMap<String, GenericRule> = rule_file.create_index(&r, complete);

        // merge it into the main index
        rule_index.extend(file_index);
    }

    if rule_index.keys().len() == 0 {
        return Error::wrap_str("Rule index is empty.");
    }
    Ok(rule_index)
}
