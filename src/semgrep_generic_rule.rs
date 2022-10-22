use std::{vec, collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_yaml:: Mapping;

// This allows us to split the rules without caring about their contents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericRuleFile {
    pub rules: Vec<Mapping>,
}

impl GenericRuleFile {

    // split the rules in a file and return each one as a GenericRuleFiles.
    pub fn split(&self) -> Vec<GenericRuleFile> {
        let mut rule_files: Vec<GenericRuleFile> = Vec::new();

        for rule in self.rules.clone() {
            let new_rules: Vec<Mapping> = vec![rule];
            rule_files.push(GenericRuleFile { rules: new_rules });
        }
        rule_files
    }

    pub fn index(&self) -> HashMap<String, Mapping> {
        let mut index: HashMap<String, Mapping> = HashMap::new();

        for rule in self.rules.clone() {
            index.insert(rule.get("id").unwrap().as_str().unwrap().to_string(), rule);
        }
        index
    }

    // convert a YAML string to a GenericRuleFile.
    pub fn from_yaml(yaml: String) -> serde_yaml::Result<GenericRuleFile> {

        // deserialize the rule
        Ok(serde_yaml::from_str::<GenericRuleFile>(&yaml)?)
    }
}