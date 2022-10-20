use std::vec;

use serde::{Serialize, Deserialize};
use serde_yaml:: Mapping;

// This allows us to split the rules without caring about their contents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericRuleFile {
    pub rules: Vec<Mapping>,
}

impl GenericRuleFile {

    // split the rules in a file and return each one as a GenericRuleFiles.
    fn split(&self) -> Vec<GenericRuleFile> {
        
        let mut rule_files: Vec<GenericRuleFile> = Vec::new();

        for rule in self.rules.clone() {
            let new_rules: Vec<Mapping> = vec![rule];
            rule_files.push(GenericRuleFile { rules: new_rules });
        }
        rule_files
    }
}