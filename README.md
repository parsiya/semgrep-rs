# semgrep-rs
Rust library crate to parse and interact with Semgrep. Currently, it only
supports interacting with Semgrep rules.

## Future Plans:

1. Handle running and executing Semgrep CLI.
2. Structs to parse the Semgrep output and do post-processing on the results.

**Note: Work in progress.** The interface is subject to change and might break
backwards compatibility.

## Usage
I have not published the crate yet. You can use the library in two ways right
now:

### git Submodule

1. Add it as a submodule to your repository.
    1. `git clone https://github.com/parsiya/semgrep-rs src/semgrep-rs`
2. Add the following to your project's `Cargo.toml`.
    ```ini
    [dependencies]
    semgrep-rs = { path = "src/semgrep-rs" }
    ```

### Directly Pointing to GitHub
Add the following to your project's `Cargo.toml`.

```ini
[dependencies]
semgrep-rs = { git = "https://github.com/parsiya/semgrep-rs", branch = "dev" }
```

## Serialize and Deserialize Rules

```rust
// deserialize the rule file into a GenericRuleFile struct.
let rule_file: GenericRuleFile =
    GenericRuleFile::from_file("tests/rules/cpp/memcpy-insecure-use.yaml").unwrap();
// a rule file can have multiple rules, the process is the same.
let rule_file2: GenericRuleFile =
    GenericRuleFile::from_file("tests/rules/multiple-rules.yaml").unwrap();

// iterate through all the rules in the file and get their ID.
for rule in &rule_file2.rules {
    print!("{}", rule.get_id().unwrap());
}

// you can serialize a rule file back to YAML.
let yaml_string: String = rule_file.to_string().unwrap();

// generally rule files only have one rule in them, if you have a GenericRuleFile
// with multiple rules, you can create separate rule files that only have a
// single rule.
let rules: Vec<GenericRuleFile> = rule_file2.split();
for rule in &rules {
    let mut path = "tests/".to_string();
    // each rule file has only one rule so we will just use that one's ID.
    path.push_str(rule.rules[0].get_id().unwrap());
    // store the result in a file.
    let err = utils::write_string_to_file(&path, &rule.to_string().unwrap());
}
```

## Create a Rule Index
A rule index contains all the rules and uses their ID as an index. You can
create a rule index that contains all the rules in a specific path and its
children. If there are files with multiple rules, the library will split them
and add each one individually.

**Note:** The index doesn't care about rule ID collisions and will happily
overwrite them. See below for a solution.

```rust
// create a simple rule index.
let simple_gri: GenericRuleIndex = GenericRuleIndex::from_path_simple("tests/rules");
// get all rules IDs in the index.
let ids: Vec<String> = simple_gri.get_ids();
```

The simple methods only indexes files with `.yaml` and `.yml` extensions and
ignores rule test files ending in `.test.yaml`, `.test.yml` and
`.test.fixed.yaml`.

You can use your own `Option<Vec<&str>>` for include and exclude.

```rust
// no need to include the dot.
let include = vec!["ext1", "ext2"];
// for exclude, you need to specify how the file ends.
// including the dot here helps prevent skipping files like "overflow-test.yml".
let exclude = vec![".test.ext1", ".test.ext2"];

let custom_gri = GenericRuleIndex::from_path(
    "tests/rules",
    Option<include>,
    Option<exclude>,
    false,
);
```

A rule index by itself is not that useful. You can just grab and serve rules by
ID.

### Complete Rule IDs
By default, both Semgrep and the portal use complete rule IDs to avoid collisions.
A complete rule ID contains the complete path of the rule in addition to the ID
in the file.

For example, the normal ID of the rule in
`tests/rules/cpp/arrays-out-of-bounds-access.yaml` is
`arrays-out-of-bounds-access`. The complete rule ID will be
`tests.rules.cpp.arrays-out-of-bounds-access.arrays-out-of-bounds-access`.

Using complete rule IDs is a hassle when we create policies by hand (see
policies below). So you can specify if you want simple or complete rule IDs. If
you don't, you need to make sure you do not have rule ID collisions.

```rust
let custom_gri = GenericRuleIndex::from_path(
    "tests/rules",
    include,
    exclude,
    true,   // create complete rule IDs.
);
```

## Serialize and Deserialize Policies
A policy is a collection of one or multiple rules. This is not a Semgrep
construct and you cannot pass it to the CLI. I am trying to imitate the Semgrep
portal . You can define a policy by creating a YAML file.

```yaml
name: policy1 # this should be unique
rules:
- arrays-out-of-bounds-access
- potentially-uninitialized-pointer
- snprintf-insecure-use
```

Rules in a policy are defined by rule ID. If you have created your rule index
with complete IDs then you should use the same version here.

### Serialize and Deserialize Policies
Similar to rules you can create a `Policy` object from a YAML string and
serialize it back to YAML.

```rust
// read a policy from a file.
let policy1: Policy = Policy::from_file("tests/policies/policy1.yaml").unwrap();
// we can also create it from a string.
let content: String = utils::read_file_to_string("tests/policies/policy2.yaml").unwrap();
let policy2: Policy = Policy::from_string(content).unwrap();

// serialzie it to a YAML string (note this doesn't create a rule file).
let policy1_string: String = policy1.to_yaml().unwrap();

// write it to a file (note this doesn't create a rule file).
let res = policy2.to_file("tests/policies/policy2-copy.yaml");
```

As we saw above, we can create the same YAML file with rule names. This is
useless because we want to pass policies as a collection of rules to Semgrep
CLI. This is when the `GenericRuleIndex` created before comes into play. We can
`populate` a policy by using the index.

```rust
// note: ignores rules that are not found (writes to stderr) and panics on YAML
// de/serialization errors.
policy2.populate(&simple_gri);
```

This will create a YAML string that contains a rule file with all rules in the
policy that are found in the rule index. This is stored in the `content` field.
You can write this to a file to pass to Semgrep.

```rust
// you can write this to a file for Semgrep usages.
let rule_file: String = policy2.get_content();
```

## Policy Index
This is another custom construct. It's an index of all policies in a path where
policy name is the key. Users should avoid duplicate names here as the library
does not check for collisions. If there's a need to implement complete IDs
(similar to rules) here, it can be done quickly.

```rust
// create a policy index from all rules in a path. This will panic on YAML
// de/serialization errors and if there are no valid policies in the path.
let simple_pi: PolicyIndex = PolicyIndex::from_path_simple("tests/policies");

// get a policy by ID.
let pol: Policy = simple_pi.get_policy("policy1").unwrap();
// get the rule file for it.
let content: String = pol.get_content();

// you can also iuterate through every policy in the index.
for (id, pol) in simple_pi.get_index() {
    assert_eq!(id, pol.get_name());
}
```

The simple version only deserializes files with `.yaml/.yml` extensions. You can
set custom include and exclude extensions like the rule index example above.

```rust
// no need to include the dot.
let include = vec!["ext1", "ext2"];
// for exclude, you need to specify how the file ends.
let exclude = vec![".test.ext1", ".test.ext2"];

let custom_pi: PolicyIndex::from_path(
    "tests/policies",
    Option<include>,
    Option<exclude>,
);
```

The policy index is very useful if you want to create your own server and serve
policies to Semgrep.

# License
Rust likes dual-licensing like this so here we go.

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
