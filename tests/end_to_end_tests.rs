use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

/// End-to-end test that simulates a real project workflow
#[test]
fn test_complete_workflow() {
    let dir = tempdir().unwrap();

    // 1. Create a realistic project structure
    create_realistic_project(&dir);

    // 2. Initialize doksnet project
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("new")
        .arg(".")
        .write_stdin("0\n") // Select README.md
        .assert()
        .success();

    // Verify .doks file was created
    let doks_path = dir.path().join(".doks");
    assert!(doks_path.exists());

    // 3. Manually add a mapping (simulating what 'doksnet add' would do)
    add_realistic_mapping(&dir);

    // 4. Test that the mapping passes verification
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Passed: 1/1"));

    // 5. Modify the code to break the mapping
    modify_code_file(&dir);

    // 6. Test should now fail
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .failure()
        .stdout(predicate::str::contains("❌ Failed: 1/1"));

    // 7. Verify that we can detect the failure (skip interactive remove-failed for CI)
    // The test command already showed the failure, which is the main functionality we want to test
}

#[test]
fn test_multiple_mappings_scenario() {
    let dir = tempdir().unwrap();

    // Create project with multiple files
    create_multi_file_project(&dir);

    // Initialize project
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("new")
        .arg(".")
        .write_stdin("0\n") // Select README.md
        .assert()
        .success();

    // Add multiple mappings
    add_multiple_mappings(&dir);

    // All should pass initially
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Passed: 2/2"));

    // Break one mapping
    let lib_path = dir.path().join("src/lib.rs");
    fs::write(&lib_path, "// Modified library\npub fn different() {}\n").unwrap();

    // Should now show 1 pass, 1 fail
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .failure()
        .stdout(predicate::str::contains("✅ Passed: 1/2"))
        .stdout(predicate::str::contains("❌ Failed: 1/2"));
}

// Helper functions

fn create_realistic_project(dir: &tempfile::TempDir) {
    // Create README.md
    let readme_content = r#"# Awesome Rust CLI

A demonstration CLI tool built with Rust.

## Installation

```bash
cargo install awesome-cli
```

## Usage

The main function initializes the application:

```rust
fn main() {
    println!("Starting application...");
    let config = Config::default();
    run_app(config);
}
```

This will start the application with default configuration.
"#;
    fs::write(dir.path().join("README.md"), readme_content).unwrap();

    // Create src/main.rs
    let main_content = r#"use std::env;

fn main() {
    println!("Starting application...");
    let config = Config::default();
    run_app(config);
}

#[derive(Default)]
struct Config {
    debug: bool,
}

fn run_app(config: Config) {
    if config.debug {
        println!("Debug mode enabled");
    }
    println!("Application running!");
}
"#;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    fs::write(src_dir.join("main.rs"), main_content).unwrap();

    // Create Cargo.toml
    let cargo_content = r#"[package]
name = "awesome-cli"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(dir.path().join("Cargo.toml"), cargo_content).unwrap();
}

fn add_realistic_mapping(dir: &tempfile::TempDir) {
    // Read the actual content from files to generate correct hashes
    let readme_content = fs::read_to_string(dir.path().join("README.md")).unwrap();
    let main_content = fs::read_to_string(dir.path().join("src/main.rs")).unwrap();

    let readme_lines: Vec<&str> = readme_content.lines().collect();
    let main_lines: Vec<&str> = main_content.lines().collect();

    // Extract the actual content that will be compared
    let doc_content = readme_lines[10..15].join("\n"); // Lines 11-15 (0-indexed 10-14)
    let code_content = main_lines[2..7].join("\n"); // Lines 3-7 (0-indexed 2-6)

    let doc_hash = blake3::hash(doc_content.as_bytes()).to_hex().to_string();
    let code_hash = blake3::hash(code_content.as_bytes()).to_hex().to_string();

    let doks_content = format!(
        r#"
version = "0.1.0"
default_doc = "README.md"

[[mappings]]
id = "main-function-example"
doc_partition = "README.md:11-15"
code_partition = "src/main.rs:3-7"
doc_hash = "{}"
code_hash = "{}"
description = "Main function documentation example"
"#,
        doc_hash, code_hash
    );

    let doks_path = dir.path().join(".doks");
    fs::write(doks_path, doks_content.trim()).unwrap();
}

fn modify_code_file(dir: &tempfile::TempDir) {
    let main_content = r#"use std::env;

fn main() {
    println!("Modified application startup...");
    let config = Config::default();
    run_app(config);
}

#[derive(Default)]
struct Config {
    debug: bool,
}

fn run_app(config: Config) {
    if config.debug {
        println!("Debug mode enabled");
    }
    println!("Application running!");
}
"#;
    let main_path = dir.path().join("src/main.rs");
    fs::write(main_path, main_content).unwrap();
}

fn create_multi_file_project(dir: &tempfile::TempDir) {
    // README
    let readme_content = r#"# Multi-File Project

## Core Functions

The `add` function:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The main function:
```rust
fn main() {
    println!("Hello, world!");
}
```
"#;
    fs::write(dir.path().join("README.md"), readme_content).unwrap();

    // src/main.rs
    let main_content = r#"use multi_file::add;

fn main() {
    println!("Hello, world!");
    let result = add(2, 3);
    println!("2 + 3 = {}", result);
}
"#;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    fs::write(src_dir.join("main.rs"), main_content).unwrap();

    // src/lib.rs
    let lib_content = r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
"#;
    fs::write(src_dir.join("lib.rs"), lib_content).unwrap();
}

fn add_multiple_mappings(dir: &tempfile::TempDir) {
    // Read actual content from files
    let readme_content = fs::read_to_string(dir.path().join("README.md")).unwrap();
    let lib_content = fs::read_to_string(dir.path().join("src/lib.rs")).unwrap();
    let main_content = fs::read_to_string(dir.path().join("src/main.rs")).unwrap();

    let readme_lines: Vec<&str> = readme_content.lines().collect();
    let lib_lines: Vec<&str> = lib_content.lines().collect();
    let main_lines: Vec<&str> = main_content.lines().collect();

    // Extract actual content for mappings
    let add_doc_content = readme_lines[4..7].join("\n"); // Lines 5-7
    let add_code_content = lib_lines[0..3].join("\n"); // Lines 1-3

    let main_doc_content = readme_lines[10..13].join("\n"); // Lines 11-13
    let main_code_content = main_lines[2..5].join("\n"); // Lines 3-5

    let add_doc_hash = blake3::hash(add_doc_content.as_bytes())
        .to_hex()
        .to_string();
    let add_code_hash = blake3::hash(add_code_content.as_bytes())
        .to_hex()
        .to_string();
    let main_doc_hash = blake3::hash(main_doc_content.as_bytes())
        .to_hex()
        .to_string();
    let main_code_hash = blake3::hash(main_code_content.as_bytes())
        .to_hex()
        .to_string();

    let doks_content = format!(
        r#"
version = "0.1.0"
default_doc = "README.md"

[[mappings]]
id = "add-function-example"
doc_partition = "README.md:5-7"
code_partition = "src/lib.rs:1-3"
doc_hash = "{}"
code_hash = "{}"
description = "Add function example"

[[mappings]]
id = "main-function-example"
doc_partition = "README.md:11-13"
code_partition = "src/main.rs:3-5"
doc_hash = "{}"
code_hash = "{}"
description = "Main function example"
"#,
        add_doc_hash, add_code_hash, main_doc_hash, main_code_hash
    );

    let doks_path = dir.path().join(".doks");
    fs::write(doks_path, doks_content.trim()).unwrap();
}
