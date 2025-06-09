use anyhow::{anyhow, Result};
use dialoguer::{Select, Input};
use std::path::PathBuf;

use crate::config::{DoksConfig, DOKS_FILE_NAME};

pub fn handle(path: Option<PathBuf>) -> Result<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    let doks_file_path = target_path.join(DOKS_FILE_NAME);

    // Check if .doks file already exists
    if doks_file_path.exists() {
        return Err(anyhow!("A .doks file already exists in this directory"));
    }

    println!("🚀 Initializing new doksnet project in: {}", target_path.display());

    // Find potential documentation files
    let doc_files = find_documentation_files(&target_path)?;
    
    let default_doc = if doc_files.is_empty() {
        // No documentation files found, ask user to specify
        let input: String = Input::new()
            .with_prompt("No documentation files found. Please specify a documentation file")
            .with_initial_text("README.md")
            .interact_text()?;
        input
    } else if doc_files.len() == 1 {
        // Only one documentation file found, use it
        let doc_file = &doc_files[0];
        println!("📄 Found documentation file: {}", doc_file);
        doc_file.clone()
    } else {
        // Multiple documentation files found, let user choose
        println!("📚 Found multiple documentation files:");
        let selection = Select::new()
            .with_prompt("Select the default documentation file")
            .items(&doc_files)
            .default(0)
            .interact()?;
        doc_files[selection].clone()
    };

    // Create the .doks configuration
    let config = DoksConfig::new(default_doc.clone());
    config.to_file(&doks_file_path)?;

    println!("✅ Created .doks file with default documentation: {}", default_doc);
    println!("📝 You can now use 'doksnet add' to create mappings between documentation and code");

    Ok(())
}

fn find_documentation_files(path: &PathBuf) -> Result<Vec<String>> {
    let mut doc_files = Vec::new();
    
    // Common documentation file patterns
    let doc_patterns = [
        "README.md", "readme.md", "README.rst", "readme.rst",
        "README.txt", "readme.txt", "README", "readme",
        "DOCS.md", "docs.md", "DOCUMENTATION.md", "documentation.md",
        "GUIDE.md", "guide.md", "MANUAL.md", "manual.md"
    ];

    // Walk through the directory (not recursively for documentation files)
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if entry.file_type()?.is_file() {
            // Check if it matches any documentation pattern
            for pattern in &doc_patterns {
                if file_name_str.eq_ignore_ascii_case(pattern) {
                    doc_files.push(file_name_str.to_string());
                    break;
                }
            }
            
            // Also check for .md files in general
            if file_name_str.ends_with(".md") && !doc_files.contains(&file_name_str.to_string()) {
                doc_files.push(file_name_str.to_string());
            }
        }
    }

    // Sort the documentation files, prioritizing README files
    doc_files.sort_by(|a, b| {
        let a_is_readme = a.to_lowercase().starts_with("readme");
        let b_is_readme = b.to_lowercase().starts_with("readme");
        
        match (a_is_readme, b_is_readme) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.cmp(b),
        }
    });

    Ok(doc_files)
} 