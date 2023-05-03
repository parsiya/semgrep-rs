// ----- START Policy

use super::generic_rule_index::GenericRuleIndex;
use crate::error::{Error, Result};
use crate::utils::{find_files, read_file_to_string, write_string_to_file};

use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    name: String,
    rules: Vec<String>,
    #[serde(skip)]
    // we don't want this field in the actual policy file.
    content: String,
}

impl Policy {
    // create a new Policy.
    pub fn new(name: String, rules: Vec<String>) -> Policy {
        Policy {
            name,
            rules,
            content: "".to_string(),
        }
    }

    // create a new Policy from a YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Policy> {
        serde_yaml::from_str::<Policy>(&yaml).map_err(Error::from)
    }

    // create a new Policy from a file.
    pub fn from_file(file: &str) -> Result<Policy> {
        // match read_file_to_string(file.as_str()) {
        //     Err(e) => return Error::wrap_string(e.to_string()),
        //     Ok(str) => match serde_yaml::from_str::<Policy>(&str) {
        //         Err(e) => return Error::wrap_string(e.to_string()),
        //         Ok(rs) => Ok(rs),
        //     },
        // }
        let content = read_file_to_string(file)?;
        Policy::from_yaml(&content)
    }

    // serialize the Policy as a YAML string.
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(&self).map_err(Error::from)
    }

    // write the policy to a YAML file.
    pub fn to_file(&self, path: &str) -> io::Result<()> {
        // match self.to_yaml() {
        //     Err(e) => Err::<(), io::Error>(io::Error::new(io::ErrorKind::InvalidData, e)),
        //     Ok(yaml) => utils::write_string_to_file(path, yaml),
        // }

        self.to_yaml()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
            .and_then(|yaml| write_string_to_file(path, &yaml))
    }

    // populate the policy from the rules index and store it in content.
    pub fn populate(&mut self, ri: &GenericRuleIndex) -> Result<()> {
        // match ri.create_policy(&self.rules).to_string() {
        //     Ok(str) => {
        //         self.content = str;
        //         Ok(())
        //     }
        //     Err(e) => Error::wrap_string(e.to_string()),
        // }

        ri.create_policy(&self.rules)
            .to_string()
            // if successful, store it in self.content.
            .map(|str| self.content = str)
        // otherwise, return any errors.
        // .map_err(|e| Error::new(e.to_string()))
    }

    // returns the policy content that can be passed to Semgrep.
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    // returns the policy name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

// ----- END Policy

// an index of policies, key: policy name, value: the Policy obj.
pub struct PolicyIndex {
    index: HashMap<String, Policy>,
    // keys: Vec<String>,
}

impl PolicyIndex {
    // create a new PolicyIndex.
    fn new() -> PolicyIndex {
        let index: HashMap<String, Policy> = HashMap::new();
        PolicyIndex { index }
    }

    // return a policy from the index.
    pub fn get_policy(&self, policy_name: &str) -> Option<Policy> {
        self.index.get(policy_name).map(|p| p.clone())
    }

    // return all policies in the index.
    pub fn get_index(&self) -> HashMap<String, Policy> {
        self.index.clone()
    }

    // return all the policy IDs in the index.
    pub fn get_ids(&self) -> Vec<String> {
        self.index.keys().map(|k| k.to_string()).collect()
    }

    // return the number of policies in the index.
    pub fn len(&self) -> usize {
        self.index.keys().len()
    }

    // return a new PolicyIndex populated with policies in the paths.
    // only index extensions in include and no files that end in exclude.
    // Deserialize them into a Policy and store them in the index. Key: policy
    // name, Value: the Policy object.
    pub fn from_paths(
        paths: Vec<&str>,
        include: Option<Vec<&str>>,
        exclude: Option<Vec<&str>>,
        ri: &GenericRuleIndex,
    ) -> Result<PolicyIndex> {
        let mut pi = PolicyIndex::new();

        match create_policy_index(paths, include, exclude, ri) {
            Ok(index) => pi.index = index,
            Err(e) => return Error::wrap_string(e.to_string()),
        };
        // these can be moved into the Ok() arm of the match, too.

        // create the "all" policy that contains all the rules.
        let all_policy = create_all_policy(ri)?;
        // add it to the index.
        pi.index.insert("all".to_string(), all_policy);
        Ok(pi)
    }

    // same as from_paths but uses the default policy file extensions.
    pub fn from_paths_simple(paths: Vec<&str>, ri: &GenericRuleIndex) -> Result<PolicyIndex> {
        PolicyIndex::from_paths(paths, None, None, ri)
    }

    // creates a policy index that only contains the p/all policy.
    pub fn empty(ri: &GenericRuleIndex) -> Result<PolicyIndex> {
        let mut pi = PolicyIndex::new();
        let all_policy = create_all_policy(ri)?;
        // add it to the index.
        pi.index.insert("all".to_string(), all_policy);
        Ok(pi)
    }
}

// find all policies with extensions in include and no files that end in
// exclude. Deserialize each into a Policy and store them in the index where
// key: policy name and value: the Policy object.
fn create_policy_index(
    paths: Vec<&str>,
    include: Option<Vec<&str>>,
    exclude: Option<Vec<&str>>,
    ri: &GenericRuleIndex,
) -> Result<HashMap<String, Policy>> {
    let mut policy_index: HashMap<String, Policy> = HashMap::new();

    let mut policy_files: Vec<String> = Vec::new();

    for p in paths {
        policy_files.extend(find_files(p, &include, &exclude));
    }

    for policy_file_path in policy_files {
        let policy_text = match read_file_to_string(&policy_file_path) {
            Ok(cn) => cn,
            Err(e) => {
                error!("Error reading file: {}", e.to_string());
                continue;
            }
        };

        // create a Policy object from the string.
        let mut policy_object = match Policy::from_yaml(&policy_text) {
            Ok(rf) => rf,
            Err(e) => {
                error!("Error deserializing file: {}", e.to_string());
                continue;
            }
        };

        // populate the Policy.
        policy_object.populate(ri)?;

        // add it to the main index.
        policy_index.insert(policy_object.name.clone(), policy_object);
    }
    // return an error if the index is empty.
    if policy_index.keys().len() == 0 {
        return Error::wrap_str("Policy index is empty.");
    }
    Ok(policy_index)
}

// creates a policy named `all` from all the rules in the index.
fn create_all_policy(ri: &GenericRuleIndex) -> Result<Policy> {
    let mut all_policy = Policy::new("all".to_string(), ri.get_ids());
    // populate the `all` policy.
    all_policy.populate(ri)?;
    Ok(all_policy)
}
