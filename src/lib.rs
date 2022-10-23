use std::collections::HashMap;

mod utils;
mod generic_rule_index;
mod semgrep_generic_rule;

pub type GenericRuleIndex = generic_rule_index::GenericRuleIndex;
pub type GenericRuleFile = semgrep_generic_rule::GenericRuleFile;
pub type GenericRule = serde_yaml::Mapping;


pub fn create_generic_rule_index(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) ->
Result<HashMap<String, GenericRule>, utils::PathError> {
    return generic_rule_index::generic_rule_index(path, include, exclude);
}
