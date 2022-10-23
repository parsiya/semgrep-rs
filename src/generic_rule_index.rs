use std::collections::HashMap;

use crate::utils::{self, find_rules, read_file_to_string};
use crate::semgrep_generic_rule::GenericRuleFile;

type GenericRule = serde_yaml::Mapping;

// ----- START GenericRuleIndex

pub struct GenericRuleIndex {
    index: HashMap<String, GenericRule>
}

impl GenericRuleIndex {

    pub fn new() -> GenericRuleIndex {
        let index: HashMap<String, GenericRule> = HashMap::new();
        let gri: GenericRuleIndex = GenericRuleIndex { index };
        gri
    }

    pub fn from_path_simple(path: String) -> GenericRuleIndex {
        return GenericRuleIndex::from_path(path, None, None);
    }

    // create and return a new GenericRuleIndex.
    pub fn from_path(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> GenericRuleIndex {
        let mut gri = GenericRuleIndex::new();
        gri.populate_from_path(path, include, exclude);
        gri
    }

    pub fn populate_from_path(&mut self, path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) {
        // ZZZ add error handling
        self.index = generic_rule_index(path, include, exclude).unwrap();

    }

    // creates a RuleFile with all the rule IDs.
    pub fn create_ruleset(&self, rule_ids: Vec<String>) -> GenericRuleFile {
        let mut rules: Vec<GenericRule> = Vec::new();

        for id in rule_ids {
            // check if the key exists
            match self.index.contains_key(&id) {
                true => rules.push(self.index[&id].clone()),
                false => continue,
            }
        }

        let grf: GenericRuleFile = GenericRuleFile { rules };
        grf
    }

}


// ----- END GenericRuleIndex


// ----- START index_rules

// return an index of rules where the key is rule ID and the value is the rule.
pub(crate) fn generic_rule_index(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) ->
    Result<HashMap<String, GenericRule>, utils::PathError> {
    
    // check the path.
    // ZZZ is this needed? Supposedly we will checke the path before calling this function.
    // utils::check_path(&path)?;

    let rule_paths: Vec<String> = find_rules(path, include, exclude);

    let mut rule_index: HashMap<String, GenericRule> = HashMap::new();

    for rule_file in rule_paths {

        let content = match read_file_to_string(&rule_file) {
            Ok(cn) => cn,
            Err(e) => {
                // ZZZ need error logging
                println!("Error reading file: {}", e.to_string());
                continue;
            }
        };

        // create a rule file from the string
        let rule_file = match GenericRuleFile::from_yaml(content) {
            Ok(rf) => rf,
            Err(e) => {
                // ZZZ need error logging
                println!("Error deserializing file: {}", e.to_string());
                continue;
            }
        };

        // get the file index
        let file_index: HashMap<String, GenericRule> = rule_file.index();

        // merge it into the main index
        rule_index.extend(file_index);
    }

    Ok(rule_index)
}


// ----- END index_rules