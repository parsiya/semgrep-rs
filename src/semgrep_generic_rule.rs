use log::error;
use std::{collections::HashMap, path::Path, vec};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

const RULE_SEPARATOR: &str = ".";

// ----- START GenericRule

pub type GenericRule = serde_yaml::Mapping;

// extension trait
pub trait GenericRuleExt {
    fn get_id(&self) -> Result<&str>;
    fn to_string(&self) -> Result<String>;
}

impl GenericRuleExt for GenericRule {
    fn get_id(&self) -> Result<&str> {
        // original
        // return self.get("id").unwrap().as_str().unwrap();

        // trying error handling
        // let i1 = self.get("id");
        // let i2 = match i1 {
        //     Some(i) => i.as_str(),
        //     None => return Error::wrap_str("The rule doesn't have and `id` field."),
        // };
        // let i3 = i2.ok_or(Error::new("Cannot convert rule's `id` field to string.".to_string()));
        // i3

        // using combinators
        self.get("id")
            .ok_or_else(|| Error::new("The rule doesn't have and `id` field.".to_string()))
            .and_then(|i| {
                i.as_str().ok_or_else(|| {
                    Error::new("Cannot convert rule's `id` field to string.".to_string())
                })
            })
    }

    // create a GenericRuleFile with just this GenericRule, serialize it to YAML
    // and return it.
    fn to_string(&self) -> Result<String> {
        // let rules: Vec<GenericRule> = vec![self.clone()];
        // let rf = GenericRuleFile{rules};
        // rf.to_string();
        GenericRuleFile {
            rules: vec![self.clone()],
        }
        .to_string()
    }
}

// ----- END GenericRule

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
    //
    // If `complete` it true, this function uses the same ID that Semgrep uses which
    // contains the path followed by the rule ID in the file. E.g., if the
    // /rules/cpp/security/buffer-overflow.cpp file contains the rule with ID
    // buffer-overflow, the complete ruleID will be
    // rules.cpp/security.buffer-overflow.buffer-overflow. Hence, rule ID is very
    // much dependent on the path of the registry passed to the server.
    //
    // If `complete` is false, just the rule ID from the file will be used.
    pub fn create_index(&self, path: &str, complete: bool) -> HashMap<String, GenericRule> {
        let mut index: HashMap<String, GenericRule> = HashMap::new();

        for rule in &self.rules {
            let mut path_string: String;

            match complete {
                true => {
                    // create the complete rule ID.
                    path_string = Path::new(path)
                        // 1. remove the extension (if any)
                        .with_extension("")
                        .to_string_lossy()
                        .to_string()
                        // 2. replace the path separator with `.`.
                        .replace(std::path::MAIN_SEPARATOR, RULE_SEPARATOR);
                    path_string.push_str(RULE_SEPARATOR);
                }
                false => {
                    let id = match rule.get_id() {
                        Err(e) => {
                            // log the error and continue.
                            error!("error getting rule's id: {}", e.to_string());
                            continue;
                        }
                        Ok(i) => i,
                    };
                    // just use the rule ID from the file.
                    path_string = id.to_string();
                }
            }
            index.insert(path_string, rule.to_owned());
        }
        index
    }

    // deserialize a YAML string into a GenericRuleFile.
    pub fn from_yaml(yaml: String) -> Result<GenericRuleFile> {
        // deserialize the rule.
        serde_yaml::from_str::<GenericRuleFile>(&yaml).map_err(|e| Error::new(e.to_string()))
    }

    // serialize a GenericRuleFile to a YAML string.
    pub fn to_string(&self) -> Result<String> {
        serde_yaml::to_string(&self).map_err(|e| Error::new(e.to_string()))
    }
}
