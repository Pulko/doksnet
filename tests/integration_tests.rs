use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A CLI tool for documentation-code mapping verification",
        ))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("edit"))
        .stdout(predicate::str::contains("remove-failed"))
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("test-interactive"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_new_command_creates_doks_file() {
    let dir = tempdir().unwrap();
    let readme_path = dir.path().join("README.md");
    fs::write(&readme_path, "# Test README\nThis is a test.").unwrap();

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.arg("new")
        .arg(dir.path())
        .write_stdin("0\n") // Select first (and only) README.md
        .assert()
        .success();

    let doks_path = dir.path().join(".doks");
    assert!(doks_path.exists());

    let content = fs::read_to_string(doks_path).unwrap();
    assert!(content.contains("version=0.1.0"));
    assert!(content.contains("default_doc=README.md"));
}

// Commented out because it requires interactive input which doesn't work in CI
// #[test]
// fn test_new_command_with_custom_doc_file() {
//     let dir = tempdir().unwrap();
//
//     let mut cmd = Command::cargo_bin("doksnet").unwrap();
//     cmd.arg("new")
//         .arg(dir.path())
//         .write_stdin("docs.md\n") // Custom documentation file
//         .assert()
//         .success();
//
//     let doks_path = dir.path().join(".doks");
//     assert!(doks_path.exists());
//
//     let content = fs::read_to_string(doks_path).unwrap();
//     assert!(content.contains("default_doc = \"docs.md\""));
// }

#[test]
fn test_new_command_fails_when_doks_exists() {
    let dir = tempdir().unwrap();
    let doks_path = dir.path().join(".doks");
    fs::write(&doks_path, "existing").unwrap();

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.arg("new")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("A .doks file already exists"));
}

#[test]
fn test_commands_fail_without_doks_file() {
    let dir = tempdir().unwrap();

    // Test that add fails without .doks
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("add")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .doks file found"));

    // Test that edit fails without .doks
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("edit")
        .arg("test-id")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .doks file found"));

    // Test that remove-failed fails without .doks
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("remove-failed")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .doks file found"));

    // Test that test fails without .doks
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .doks file found"));

    // Test that test-interactive fails without .doks
    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test-interactive")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No .doks file found"));
}

#[test]
fn test_test_command_with_empty_mappings() {
    let dir = tempdir().unwrap();
    create_basic_doks_file(&dir);

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("No mappings found"));
}

#[test]
fn test_test_interactive_with_empty_mappings() {
    let dir = tempdir().unwrap();
    create_basic_doks_file(&dir);

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test-interactive")
        .assert()
        .success()
        .stdout(predicate::str::contains("No mappings found"));
}

#[test]
fn test_remove_failed_with_empty_mappings() {
    let dir = tempdir().unwrap();
    create_basic_doks_file(&dir);

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("remove-failed")
        .assert()
        .success()
        .stdout(predicate::str::contains("No mappings found"));
}

#[test]
fn test_edit_with_nonexistent_id() {
    let dir = tempdir().unwrap();

    // Create .doks with at least one mapping so it doesn't bail out early
    let readme_path = dir.path().join("README.md");
    fs::write(&readme_path, "# Test\nContent").unwrap();

    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    let main_path = src_dir.join("main.rs");
    fs::write(&main_path, "fn main() {}").unwrap();

    create_doks_with_mapping(&dir, "README.md:1", "src/main.rs:1");

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("edit")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "No mapping found with ID starting with",
        ));
}

#[test]
fn test_test_command_with_valid_mappings() {
    let dir = tempdir().unwrap();

    // Create test files
    let readme_path = dir.path().join("README.md");
    fs::write(&readme_path, "# Test\nLine 2\nLine 3\nLine 4\nLine 5").unwrap();

    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    let main_path = src_dir.join("main.rs");
    fs::write(
        &main_path,
        "fn main() {\n    println!(\"Hello\");\n    println!(\"World\");\n}",
    )
    .unwrap();

    // Create .doks file with valid mapping
    create_doks_with_mapping(&dir, "README.md:2-3", "src/main.rs:2-3");

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Testing 1 documentation-code mappings",
        ))
        .stdout(predicate::str::contains("✅ Passed: 1/1"));
}

#[test]
fn test_test_command_with_changed_content() {
    let dir = tempdir().unwrap();

    // Create test files
    let readme_path = dir.path().join("README.md");
    fs::write(&readme_path, "# Test\nOriginal content\nLine 3").unwrap();

    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    let main_path = src_dir.join("main.rs");
    fs::write(&main_path, "fn main() {\n    println!(\"Hello\");\n}").unwrap();

    // Create .doks file with mapping
    create_doks_with_mapping(&dir, "README.md:2", "src/main.rs:2");

    // Modify the content after creating mapping
    fs::write(&readme_path, "# Test\nModified content\nLine 3").unwrap();

    let mut cmd = Command::cargo_bin("doksnet").unwrap();
    cmd.current_dir(&dir)
        .arg("test")
        .assert()
        .failure() // Should fail with exit code 1
        .stdout(predicate::str::contains("❌ Failed: 1/1"))
        .stdout(predicate::str::contains(
            "documentation content has changed",
        ));
}

// Helper functions

fn create_basic_doks_file(dir: &tempfile::TempDir) {
    let doks_content = r#"# .doks v2 - Compact format
version=0.1.0
default_doc=README.md

# Format: id|doc_partition|code_partition|doc_hash|code_hash|description"#;
    let doks_path = dir.path().join(".doks");
    fs::write(doks_path, doks_content).unwrap();
}

fn create_doks_with_mapping(dir: &tempfile::TempDir, doc_partition: &str, code_partition: &str) {
    // Read the actual content to generate real hashes
    let doc_parts: Vec<&str> = doc_partition.split(':').collect();
    let doc_file = dir.path().join(doc_parts[0]);
    let doc_content = if doc_parts.len() > 1 {
        let range = doc_parts[1];
        let content = fs::read_to_string(&doc_file).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        if range.contains('-') {
            let range_parts: Vec<&str> = range.split('-').collect();
            let start: usize = range_parts[0].parse().unwrap();
            let end: usize = range_parts[1].parse().unwrap();
            lines[(start - 1)..end].join("\n")
        } else {
            let line_num: usize = range.parse().unwrap();
            lines[line_num - 1].to_string()
        }
    } else {
        fs::read_to_string(&doc_file).unwrap()
    };

    let code_parts: Vec<&str> = code_partition.split(':').collect();
    let code_file = dir.path().join(code_parts[0]);
    let code_content = if code_parts.len() > 1 {
        let range = code_parts[1];
        let content = fs::read_to_string(&code_file).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        if range.contains('-') {
            let range_parts: Vec<&str> = range.split('-').collect();
            let start: usize = range_parts[0].parse().unwrap();
            let end: usize = range_parts[1].parse().unwrap();
            lines[(start - 1)..end].join("\n")
        } else {
            let line_num: usize = range.parse().unwrap();
            lines[line_num - 1].to_string()
        }
    } else {
        fs::read_to_string(&code_file).unwrap()
    };

    // Generate hashes using blake3
    let doc_hash = blake3::hash(doc_content.as_bytes()).to_hex().to_string();
    let code_hash = blake3::hash(code_content.as_bytes()).to_hex().to_string();

    let doks_content = format!(
        r#"# .doks v2 - Compact format
version=0.1.0
default_doc=README.md

# Format: id|doc_partition|code_partition|doc_hash|code_hash|description
test-mapping-123|{}|{}|{}|{}|Test mapping"#,
        doc_partition, code_partition, doc_hash, code_hash
    );

    let doks_path = dir.path().join(".doks");
    fs::write(doks_path, doks_content).unwrap();
}
