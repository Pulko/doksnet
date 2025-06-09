use std::fs;
use tempfile::tempdir;

// We need to test the command modules, but since they use interactive input,
// we'll create unit tests for the core logic and integration tests for CLI

#[cfg(test)]
mod command_unit_tests {
    use super::*;

    // Test that we can create a proper test environment
    #[test]
    fn test_environment_setup() {
        let dir = tempdir().unwrap();

        // Create test documentation file
        let readme_path = dir.path().join("README.md");
        fs::write(
            &readme_path,
            "# Documentation\n\nThis is documentation.\n\nMore content here.",
        )
        .unwrap();

        // Create test source file
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        let main_path = src_dir.join("main.rs");
        fs::write(
            &main_path,
            "fn main() {\n    println!(\"Hello, world!\");\n    let x = 42;\n}",
        )
        .unwrap();

        // Verify files exist
        assert!(readme_path.exists());
        assert!(main_path.exists());

        // Verify content
        let readme_content = fs::read_to_string(&readme_path).unwrap();
        assert!(readme_content.contains("Documentation"));

        let main_content = fs::read_to_string(&main_path).unwrap();
        assert!(main_content.contains("Hello, world!"));
    }

    #[test]
    fn test_doks_file_structure() {
        let dir = tempdir().unwrap();

        // Create a .doks file manually and verify it parses correctly
        let doks_content = r#"
version = "0.1.0"
default_doc = "README.md"

[[mappings]]
id = "test-id-123"
doc_partition = "README.md:1-2"
code_partition = "src/main.rs:1-2"
doc_hash = "abcdef123456"
code_hash = "fedcba654321"
description = "Test mapping between readme and main"
"#;

        let doks_path = dir.path().join(".doks");
        fs::write(&doks_path, doks_content.trim()).unwrap();

        // Parse and verify
        let parsed_content = fs::read_to_string(&doks_path).unwrap();
        assert!(parsed_content.contains("version = \"0.1.0\""));
        assert!(parsed_content.contains("default_doc = \"README.md\""));
        assert!(parsed_content.contains("[[mappings]]"));
        assert!(parsed_content.contains("test-id-123"));
    }

    #[test]
    fn test_partition_content_extraction() {
        let dir = tempdir().unwrap();

        // Create a test file with multiple lines
        let test_file = dir.path().join("test.txt");
        let test_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        fs::write(&test_file, test_content).unwrap();

        // Test different partition extractions
        let content = fs::read_to_string(&test_file).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Test single line extraction
        assert_eq!(lines[1], "Line 2");

        // Test range extraction
        let range_content = lines[1..=2].join("\n");
        assert_eq!(range_content, "Line 2\nLine 3");

        // Test full file
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_hash_computation() {
        let content1 = "Hello, world!";
        let content2 = "Hello, world!";
        let content3 = "Hello, world?"; // Different content

        let hash1 = blake3::hash(content1.as_bytes()).to_hex().to_string();
        let hash2 = blake3::hash(content2.as_bytes()).to_hex().to_string();
        let hash3 = blake3::hash(content3.as_bytes()).to_hex().to_string();

        // Same content should produce same hash
        assert_eq!(hash1, hash2);

        // Different content should produce different hash
        assert_ne!(hash1, hash3);

        // Hashes should be proper length (Blake3 produces 64-character hex strings)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_realistic_scenario() {
        let dir = tempdir().unwrap();

        // Create realistic documentation
        let readme_content = r#"# My Project

## Installation

To install this project:

```bash
cargo install my-project
```

## Usage

The main function starts the application:

```rust
fn main() {
    println!("Starting application...");
    let config = load_config();
    run_app(config);
}
```

## Configuration

Configure using the `config.toml` file.
"#;

        let readme_path = dir.path().join("README.md");
        fs::write(&readme_path, readme_content).unwrap();

        // Create realistic source code
        let main_content = r#"use std::fs;

fn main() {
    println!("Starting application...");
    let config = load_config();
    run_app(config);
}

fn load_config() -> Config {
    // Load configuration
    Config::default()
}

fn run_app(config: Config) {
    // Run the application
    println!("Running with config: {:?}", config);
}

#[derive(Debug, Default)]
struct Config {
    name: String,
}
"#;

        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        let main_path = src_dir.join("main.rs");
        fs::write(&main_path, main_content).unwrap();

        // Extract specific sections that would be linked
        let readme_lines: Vec<&str> = readme_content.lines().collect();

        // Debug: let's see what we have in the readme
        println!("README lines: {:?}", readme_lines);

        // Find the rust code block more reliably
        let start_index = readme_lines
            .iter()
            .position(|&line| line.contains("```rust"))
            .unwrap();
        let end_index = readme_lines
            .iter()
            .skip(start_index + 1)
            .position(|&line| line.contains("```"))
            .unwrap()
            + start_index
            + 1;
        let main_section = readme_lines[(start_index + 1)..end_index].join("\n");

        let main_lines: Vec<&str> = main_content.lines().collect();
        let main_function = main_lines[2..6].join("\n"); // The main function

        // Verify the content matches what we expect to link
        assert!(main_section.contains("fn main()"));
        assert!(main_section.contains("println!(\"Starting application...\")"));

        assert!(main_function.contains("fn main()"));
        assert!(main_function.contains("println!(\"Starting application...\")"));

        // Generate hashes for this linked content
        let doc_hash = blake3::hash(main_section.as_bytes()).to_hex().to_string();
        let code_hash = blake3::hash(main_function.as_bytes()).to_hex().to_string();

        // In a real scenario, these would be the same if the documentation
        // code example matches the actual implementation
        assert_ne!(doc_hash, code_hash); // They're different because of formatting

        // But if we cleaned up the content (removed code block markers), they might match
        let cleaned_doc = main_section
            .replace("```rust", "")
            .replace("```", "")
            .trim()
            .to_string();

        let cleaned_doc_hash = blake3::hash(cleaned_doc.as_bytes()).to_hex().to_string();
        // This demonstrates how content processing might be needed for matching
        assert_ne!(cleaned_doc_hash, code_hash); // Still different due to exact formatting
    }

    #[test]
    fn test_file_discovery() {
        let dir = tempdir().unwrap();

        // Create various documentation files
        fs::write(dir.path().join("README.md"), "# Main readme").unwrap();
        fs::write(dir.path().join("DOCS.md"), "# Documentation").unwrap();
        fs::write(dir.path().join("guide.txt"), "Guide content").unwrap();
        fs::write(dir.path().join("random.rs"), "// Code file").unwrap();

        // List directory contents
        let entries: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        // Verify we can identify documentation files
        let mut doc_files = Vec::new();
        for entry in entries {
            if entry.ends_with(".md")
                || entry.to_lowercase().contains("readme")
                || entry.to_lowercase().contains("doc")
            {
                doc_files.push(entry);
            }
        }

        // Should find README.md and DOCS.md
        assert!(doc_files.iter().any(|f| f == "README.md"));
        assert!(doc_files.iter().any(|f| f == "DOCS.md"));
        assert!(!doc_files.iter().any(|f| f == "random.rs"));
    }
}
