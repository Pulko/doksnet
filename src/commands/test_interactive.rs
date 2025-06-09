use anyhow::{anyhow, Result};
use dialoguer::{Select, Confirm};

use crate::config::DoksConfig;
use crate::partition::Partition;
use crate::hash::{hash_content, verify_hash};

pub fn handle() -> Result<()> {
    // Find the .doks file
    let doks_file_path = DoksConfig::find_doks_file()
        .ok_or_else(|| anyhow!("No .doks file found. Run 'doksnet new' first."))?;

    let mut config = DoksConfig::from_file(&doks_file_path)?;

    if config.mappings.is_empty() {
        println!("📭 No mappings found. Use 'doksnet add' to create some first.");
        return Ok(());
    }

    println!("🧪 Interactive Testing Mode - {} mappings", config.mappings.len());
    println!("📄 Default documentation file: {}", config.default_doc);
    println!();

    let mut failed_mappings = Vec::new();
    let mut passed_count = 0;
    let mut modified = false;

    // Test all mappings first
    for (index, mapping) in config.mappings.iter().enumerate() {
        let mapping_num = index + 1;
        println!("🔍 Testing mapping {}/{}: {}", mapping_num, config.mappings.len(), &mapping.id[..8]);
        
        if let Some(desc) = &mapping.description {
            println!("   📝 Description: {}", desc);
        }
        
        println!("   📄 Doc: {}", mapping.doc_partition);
        println!("   💻 Code: {}", mapping.code_partition);

        // Test both partitions
        let doc_result = test_partition_detailed(&mapping.doc_partition, &mapping.doc_hash, "documentation");
        let code_result = test_partition_detailed(&mapping.code_partition, &mapping.code_hash, "code");

        match (doc_result, code_result) {
            (Ok(_), Ok(_)) => {
                println!("   ✅ PASS");
                passed_count += 1;
            }
            (doc_result, code_result) => {
                println!("   ❌ FAIL");
                failed_mappings.push((index, mapping.clone(), doc_result, code_result));
            }
        }
        
        println!();
    }

    // Summary
    println!("📊 Test Results Summary:");
    println!("   ✅ Passed: {}/{}", passed_count, config.mappings.len());
    println!("   ❌ Failed: {}/{}", failed_mappings.len(), config.mappings.len());
    println!();

    if failed_mappings.is_empty() {
        println!("🎉 All mappings are up to date!");
        return Ok(());
    }

    // Handle failed mappings interactively
    println!("🛠️  Let's fix the failed mappings...");
    
    for (original_index, mapping, doc_result, code_result) in failed_mappings {
        // Find current index (may have changed due to removals)
        let current_index = config.mappings.iter().position(|m| m.id == mapping.id);
        
        if current_index.is_none() {
            continue; // Mapping was already removed
        }
        let current_index = current_index.unwrap();

        println!("\n🚨 Failed mapping: {} ({}...)", mapping.id, &mapping.id[..8]);
        if let Some(desc) = &mapping.description {
            println!("📝 Description: {}", desc);
        }
        println!("📄 Doc: {}", mapping.doc_partition);
        println!("💻 Code: {}", mapping.code_partition);

        // Show what changed
        show_changes(&mapping, &doc_result, &code_result)?;

        // Ask what to do
        let options = vec![
            "Update hashes (accept current content)",
            "Edit this mapping",
            "Remove this mapping", 
            "Skip (leave as-is)",
        ];

        let action = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match action {
            0 => {
                // Update hashes
                if let Err(ref e) = doc_result {
                    if let Some(content) = extract_content_if_possible(&mapping.doc_partition) {
                        config.mappings[current_index].doc_hash = hash_content(&content);
                        println!("✅ Updated documentation hash");
                    }
                }
                if let Err(ref e) = code_result {
                    if let Some(content) = extract_content_if_possible(&mapping.code_partition) {
                        config.mappings[current_index].code_hash = hash_content(&content);
                        println!("✅ Updated code hash");
                    }
                }
                modified = true;
            }
            1 => {
                // Edit mapping - redirect to edit functionality
                println!("💡 Use 'doksnet edit {}' to edit this mapping", &mapping.id[..8]);
            }
            2 => {
                // Remove mapping
                let confirm = Confirm::new()
                    .with_prompt("Are you sure you want to remove this mapping?")
                    .default(false)
                    .interact()?;
                
                if confirm {
                    config.mappings.remove(current_index);
                    println!("✅ Mapping removed");
                    modified = true;
                }
            }
            3 => {
                // Skip
                println!("⏭️  Skipped");
            }
            _ => unreachable!(),
        }
    }

    // Save changes if any were made
    if modified {
        config.to_file(&doks_file_path)?;
        println!("\n💾 Changes saved to .doks file");
    }

    println!("\n🏁 Interactive testing complete!");
    
    Ok(())
}

fn test_partition_detailed(partition_str: &str, expected_hash: &str, content_type: &str) -> Result<(), String> {
    // Parse the partition
    let partition = match Partition::parse(partition_str) {
        Ok(p) => p,
        Err(e) => return Err(format!("Failed to parse {} partition: {}", content_type, e)),
    };

    // Extract content
    let content = match partition.extract_content() {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to extract {} content: {}", content_type, e)),
    };

    // Verify hash
    if !verify_hash(&content, expected_hash) {
        let current_hash = hash_content(&content);
        return Err(format!(
            "{} content has changed (expected: {}..., actual: {}...)",
            content_type,
            &expected_hash[..8],
            &current_hash[..8]
        ));
    }

    Ok(())
}

fn show_changes(mapping: &crate::config::Mapping, doc_result: &Result<(), String>, code_result: &Result<(), String>) -> Result<()> {
    println!("\n📋 Changes detected:");

    if let Err(_) = doc_result {
        println!("\n📄 Documentation content has changed:");
        if let Some(content) = extract_content_if_possible(&mapping.doc_partition) {
            println!("--- Current content ---");
            println!("{}", content.chars().take(300).collect::<String>());
            if content.len() > 300 {
                println!("... (truncated)");
            }
        } else {
            println!("⚠️  Could not extract current documentation content");
        }
    }

    if let Err(_) = code_result {
        println!("\n💻 Code content has changed:");
        if let Some(content) = extract_content_if_possible(&mapping.code_partition) {
            println!("--- Current content ---");
            println!("{}", content.chars().take(300).collect::<String>());
            if content.len() > 300 {
                println!("... (truncated)");
            }
        } else {
            println!("⚠️  Could not extract current code content");
        }
    }

    Ok(())
}

fn extract_content_if_possible(partition_str: &str) -> Option<String> {
    Partition::parse(partition_str)
        .ok()
        .and_then(|p| p.extract_content().ok())
} 