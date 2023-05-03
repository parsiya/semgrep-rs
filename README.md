# semgrep-rs
Rust library crate to interact with [Semgrep][semgrep]. It allows you to:

1. Parse and combine Semgrep rules.
2. Create and populate and a new construct (policy) that imitates rulesets.
3. Parse Semgrep's JSON output.
4. Run Semgrep CLI by passing the rules as a string and get the results in a struct.

I have used it in my [Personal Semgrep Server][server] project. See also the
[examples][ex].

[semgrep]: https://semgrep.dev
[server]: https://github.com/parsiya/personal-semgrep-server
[ex]: https://github.com/parsiya/semgrep-rs/blob/dev/

## Future Plans:

1. Add detailed structs and the ability to programmatically create rules.
2. ~~Handle running and executing Semgrep CLI.~~
3. ~~Structs to parse the Semgrep output and do post-processing on the results.~~
4. Simple plugin system.

**Note: Work in progress.** The interface is subject to change and might break
backwards compatibility.

## Add it to Your Rust Project
I have not published the crate yet. You can use the library in two ways right
now:

### git Submodule

1. Add it as a submodule to your repository.
    1. `git submodule add -b dev https://github.com/parsiya/semgrep-rs src/semgrep-rs`
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

# How do I Use This?
See the [examples][ex] or https://github.com/parsiya/personal-semgrep-server.

## Serialize and Deserialize Rules

```rust
// Deserialize the rule file into a GenericRuleFile struct.
let rule_file: GenericRuleFile =
    GenericRuleFile::from_file("tests/rules/cpp/memcpy-insecure-use.yaml").unwrap();
// A rule file can have multiple rules, the process is the same.
let rule_file2: GenericRuleFile =
    GenericRuleFile::from_file("tests/rules/multiple-rules.yaml").unwrap();

// Iterate through all the rules in the file and get their ID.
for rule in &rule_file2.rules {
    print!("{}", rule.get_id().unwrap());
}

// You can serialize them back to YAML.
let yaml_string: String = rule_file.to_string().unwrap();

// Usually rule files only have one rule, if you have a GenericRuleFile
// with multiple rules, you can create rule files with only one rule per file
// with .split().
let rules: Vec<GenericRuleFile> = rule_file2.split();
for rule in &rules {
    let mut path = "tests/".to_string();
    // Each rule file has only one rule so we will just use that one's ID.
    path.push_str(rule.rules[0].get_id().unwrap());
    // Store the result in a file.
    let err = utils::write_string_to_file(&path, &rule.to_string().unwrap());
}
```

I have more detailed structs in the [dev-old branch][dev-old-structs]. These
structs allow you to extract all the rule fields (e.g., `pattern` or
`metavariable`). The plan is to add them after I have the generic structs
production ready. It will allow you to parse every field in a rule file and also
create rules dynamically.

[dev-old-structs]: https://github.com/parsiya/semgrep-rs/blob/dev-old/src/semgrep_rule.rs

## Create a Rule Index
A rule index contains all the rules and uses their ID as an index. You can
create a rule index that contains all the rules in a specific path and its
children. If there are files with multiple rules, the library will split them
and add each one individually.

**Note:** The index doesn't care about rule ID collisions and will happily
overwrite them. See below for a solution.

```rust
let simple_gri: GenericRuleIndex =
    GenericRuleIndex::from_path_simple("test/rules").unwrap();
// Get all rules IDs in the index.
let ids: Vec<String> = simple_gri.get_ids();
```

The `_simple` methods only index files with `.yaml` and `.yml` extensions and
ignores rule test files ending in `.test.yaml`, `.test.yml` and
`.test.fixed.yaml`. 

You can use your own `Option<Vec<&str>>` for include and exclude.

```rust
// Don't include the dot for the include vector.
let include = vec!["ext1", "ext2"];
// For exclude, you need to specify how the file ends.
// Including the dot here helps prevent skipping files like "overflow-test.yml".
let exclude = vec![".test.ext1", ".test.ext2"];

let custom_gri = GenericRuleIndex::from_path(
    "test/rules",
    Option<include>,
    Option<exclude>,
    false,
).unwrap();
```

You can also pass multiple paths by using the `from_paths` and
`from_paths_simple` methods.

### Note About Errors
If a file is not accessible or it cannot be deserialized into a struct, the
crate logs it with `error!` and continues.

### Complete Rule IDs
By default, both Semgrep and the portal use complete rule IDs to avoid
collisions. A complete rule ID contains the complete path of the rule in
addition to the ID in the file.

For example, the normal ID of the rule in
`tests/rules/cpp/arrays-out-of-bounds-access.yaml` is
`arrays-out-of-bounds-access`. The complete rule ID will be
`tests.rules.cpp.arrays-out-of-bounds-access.arrays-out-of-bounds-access`.

Using complete rule IDs is a hassle when we create policies by hand (see
policies below). So you can specify if you want simple or complete rule IDs.

**In both cases you have to make sure you don't have rule ID collisions.**

```rust
let custom_gri = GenericRuleIndex::from_path(
    "test/rules",
    include,
    exclude,
    true,   // Create complete rule IDs.
).unwrap();
```

## Policies
A rule index by itself is not that useful. A policy is a collection of one or
multiple rules. This is not a Semgrep construct and you cannot pass it to the
CLI. The crate is trying to imitate rulesets in the Semgrep portal. You can
define a policy by creating a YAML file.

```yaml
name: policy1 # this should be unique
rules:
- arrays-out-of-bounds-access
- potentially-uninitialized-pointer
- snprintf-insecure-use
```

Rules in a policy are defined by rule ID. If you have created your rule index
with complete IDs then you should use the same version here. E.g.,
`path.to.file.id`.

### Serialize and Deserialize Policies
Similar to rules you can create a `Policy` object from a YAML string and
serialize it back to YAML, again.

```rust
// Read a policy from a file.
let policy1: Policy = Policy::from_file("tests/policies/policy1.yaml").unwrap();
// We can also create it from a string.
let content: String = utils::read_file_to_string("tests/policies/policy2.yaml").unwrap();
let policy2: Policy = Policy::from_string(content).unwrap();

// Serialize it to a YAML string (note this doesn't create a rule file).
let policy1_string: String = policy1.to_yaml().unwrap();

// Write it to a file (note this doesn't create a rule file).
let res = policy2.to_file("tests/policies/policy2-copy.yaml");
```

As we saw above, we can create the same YAML file with rule names. This cannot
be passed to the Semgrep CLI because we want to pass policies as a collection of
rules. The `GenericRuleIndex` that we created before comes into play. We can
`populate` a policy by using the index.

```rust
// Note: this method ignores rules that are not found (writes to stderr) and
// panics on YAML de/serialization errors.
policy2.populate(&simple_gri);
```

This will create a YAML string that contains a rule file with all rules in the
policy that are found in the rule index. This is stored in the `content` field.
You can write this to a file to pass to Semgrep.

```rust
// You can write this to a file to pass to Semgrep.
let rule_file: String = policy2.get_content();
```

## Policy Index
This is another custom construct. It's an index of all policies in a path where
policy name is the key. Users should avoid duplicate names here as the library
does not check for collisions. If there's a need to implement complete IDs
(similar to rules) here, it can be done quickly.

```rust
// Create a policy index from all rules in a path. This will panic on YAML
// de/serialization errors and if there are no valid policies in the path.
let simple_pi: PolicyIndex = PolicyIndex::from_paths_simple("test/rules").unwrap();

// Get a policy by ID.
let pol: Policy = simple_pi.get_policy("policy1").unwrap();
// Get the rule file for it.
let content: String = pol.get_content();

// You can also iuterate through every policy in the index.
for (id, pol) in simple_pi.get_index() {
    assert_eq!(id, pol.get_name());
}
```

The simple version only deserializes files with `.yaml/.yml` extensions. You can
set custom include and exclude extensions like the rule index example above.

```rust
// No need to include the dot.
let include = vec!["ext1", "ext2"];
// For exclude, you need to specify how the file ends.
let exclude = vec![".test.ext1", ".test.ext2"];

let custom_pi: PolicyIndex::from_paths(
    "test/rules",
    Option<include>,
    Option<exclude>,
).unwrap();
```

The policy index is useful if you want to create your own server and serve
policies to Semgrep.

You can also pass multiple paths by using the `from_paths` and
`from_paths_simple` methods.

### The Special "all" Policy
The crate automatically creates an special policy named `all`. This policy
contains every rule in the rule index. If you have a policy named `all`, it will
be overwritten by this policy. The `all` policy is useful when you want to throw
every rule at the code.

```rust
let all_policy: Policy = custom_pi.get_policy("all").unwrap();
// Get the rule file that has every rule!
let all_rules: String = all_policy.get_content();
```

## Semgrep Output
The crate supports parsing Semgrep's output in JSON (not the SARIF one). Use the
`--json` flag: `semgrep --config p/default --json --output my-results.json`.

```rust
use semgrep_rs::CliOutput;

// Parse the file.
let res = CliOutput::from_json_file("result.json").unwrap();

// Print all scanned paths.
for p in res.paths.scanned {
    print!("{}", p);
}

// Go through all results.
for r in res.results {
    print!("{}", r.check_id);   // Print the rule ID and path for each result.
    print!("{}", r.path);
}
```

The structs are based on
https://github.com/returntocorp/semgrep-interfaces/blob/main/semgrep_output_v1.jsonschema.
I have created an annotated version of it in
[source-schemas/semgrep_output_v1.jsonschema](source-schemas/semgrep_output_v1.jsonschema).

A few structs are parsed as `serde_json::Value` because I did not know how to
implement the enums with multiple different field types.

For example `core_match_call_trace` or `CoreMatchCallTrace` is an enum that can
either be:

1. An array with at least two items. The first item must be the string `CoreLoc`
   and the second is an object of type `Location`.
2. An array with at least 2 items:
    1. The first item is the string `CoreCall`.
    2. The second item is another array with at least 3 items:
        1. The first item is `Location`.
        2. Second item is a `Vec<CoreMatchIntermediateVar>` (another array).
        3. Third item is a `CoreMatchCallTrace` (which is the parent object).

I can make Rust enums but I have no idea how to make these other objects, yet.

Another one is `core_error_kind` or `CoreErrorKind`. It can be one of:

1. 14 hardcoded strings.
2. An array with at least two items:
    1. The first item is the string `Pattern parse error`.
    2. The second item is a `Vec<String>`.
3. An array with at least two items:
    1. The first item is the string `PartialParsing`.
    2. The second item is a `Vec<Location>`.

The problem here is with choices 2 and 3. We're looking at something like
`{"Pattern parse error", {"foo", "bar", "baz"}}`. I don't know how to create a
type like this in the Rust enum. There are no field names so I cannot make an
object.

## Running Semgrep
You can also use this crate to run the Semgrep CLI. You can pass the rules as a
string and get the results in a struct. Create an instance of `Args`:

```rust
pub struct Args {
    /// Semgrep rules as a string.
    pub rules: String,
    /// value of the Semgrep `metrics` CLI argument, default is `off`
    /// (`--metrics=off`). Note metrics will be collected regardless of this
    /// field on certain invocations like `-c=p/default`. See the docs at:
    /// https://semgrep.dev/docs/metrics/.
    metrics: Metrics,
    /// other flags, passed to the tool as-is before scan_paths and separated by
    /// space.
    pub extra: Option<Vec<String>>,
    /// paths scanned with Semgrep.
    pub paths: Vec<String>,
    /// the output format
    pub output_format: OutputFormat,
}
```

Like this:

```rust

let rule_as_string = "a string with one or more rules";
let paths = vec!["scan/code1/".to_string(), "scan/code2/".to_string()];
let metrics = false;

let format = semgrep_rs::OutputFormat::from_str("json").unwrap();
// Or just make a format using one of the enums.
let format2 = semgrep_rs::OutputFormat::JSON;

// pass any other cli switches.
let extra = vec!["--time".to_string(), "--timeout=10".to_string()];

let args = semgrep_rs::Args::new(
    rules_as_string, paths, metrics, format, extra
    );

// See the final command as a string and inspect it.
print!("{}", args.to_string());

// Run Semgrep and get the result in a struct.
let results: semgrep_rs::CliOutput = args.execute().unwrap();

// You can also convert it back to JSON to write to disk.
let bytes: Vec<u8> = results.to_json_bytes().unwrap();
```

# License
Rust likes dual-licensing like this so here we go.

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
