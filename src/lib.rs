pub mod utils;
pub use utils::{check_path, check_path_panic};

mod error;
pub use error::{Error, Result};

mod rules;
pub use rules::generic_rule_index::GenericRuleIndex;
pub use rules::policy::{Policy, PolicyIndex};
pub use rules::semgrep_generic_rule::{GenericRule, GenericRuleExt, GenericRuleFile};
