use std::{vec, collections::HashMap};

use serde::{Serialize, Deserialize};

type GenericRule = serde_yaml::Mapping;

// This allows us to split the rules without caring about their contents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericRuleFile {
    pub rules: Vec<GenericRule>,
}

impl GenericRuleFile {

    // split the rules in a file and return each one as a GenericRuleFiles.
    pub fn split(&self) -> Vec<GenericRuleFile> {
        let mut rule_files: Vec<GenericRuleFile> = Vec::new();

        for rule in self.rules.clone() {
            let new_rules: Vec<GenericRule> = vec![rule];
            rule_files.push(GenericRuleFile { rules: new_rules });
        }
        rule_files
    }

    // create an index from the rules in the GenericRuleFile.
    pub fn index(&self) -> HashMap<String, GenericRule> {
        let mut index: HashMap<String, GenericRule> = HashMap::new();

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