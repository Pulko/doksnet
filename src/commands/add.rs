use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input};
use uuid::Uuid;

use crate::config::{DoksConfig, Mapping};
use crate::hash::hash_content;
use crate::partition::Partition;

pub fn handle() -> Result<()> {
    // Find the .doks file
    let doks_file_path = DoksConfig::find_doks_file()
        .ok_or_else(|| anyhow!("No .doks file found. Run 'doksnet new' first."))?;

    let mut config = DoksConfig::from_file(&doks_file_path)?;

    println!("📝 Adding new documentation-code mapping");
    println!("Current default documentation file: {}", config.default_doc);

    let doc_partition_str: String = Input::new()
        .with_prompt("Documentation partition (e.g., README.md:10-20 or README.md:10-20@5-15)")
        .with_initial_text(format!("{}:", config.default_doc))
        .interact_text()?;

    let doc_partition = Partition::parse(&doc_partition_str)?;
    let doc_content = doc_partition
        .extract_content()
        .map_err(|e| anyhow!("Failed to extract documentation content: {}", e))?;

    println!("\n📄 Documentation content preview:");
    println!("---");
    println!("{}", doc_content.chars().take(200).collect::<String>());
    if doc_content.len() > 200 {
        println!("... (truncated)");
    }
    println!("---");

    let confirm_doc = Confirm::new()
        .with_prompt("Is this the correct documentation content?")
        .default(true)
        .interact()?;

    if !confirm_doc {
        println!("❌ Documentation selection cancelled");
        return Ok(());
    }

    let code_partition_str: String = Input::new()
        .with_prompt("Code partition (e.g., src/main.rs:15-30 or src/lib.rs:5-25@10-50)")
        .interact_text()?;

    let code_partition = Partition::parse(&code_partition_str)?;
    let code_content = code_partition
        .extract_content()
        .map_err(|e| anyhow!("Failed to extract code content: {}", e))?;

    println!("\n💻 Code content preview:");
    println!("---");
    println!("{}", code_content.chars().take(200).collect::<String>());
    if code_content.len() > 200 {
        println!("... (truncated)");
    }
    println!("---");

    let confirm_code = Confirm::new()
        .with_prompt("Is this the correct code content?")
        .default(true)
        .interact()?;

    if !confirm_code {
        println!("❌ Code selection cancelled");
        return Ok(());
    }

    let description: String = Input::new()
        .with_prompt("Optional description for this mapping")
        .allow_empty(true)
        .interact_text()?;

    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    let doc_hash = hash_content(&doc_content);
    let code_hash = hash_content(&code_content);

    let mapping = Mapping {
        id: Uuid::new_v4().to_string(),
        doc_partition: doc_partition_str,
        code_partition: code_partition_str,
        doc_hash,
        code_hash,
        description,
    };

    config.add_mapping(mapping);
    config.to_file(&doks_file_path)?;

    println!("✅ Successfully added mapping!");
    println!("📊 Total mappings: {}", config.mappings.len());

    Ok(())
}
