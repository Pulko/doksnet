use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input, Select};

use crate::config::DoksConfig;
use crate::hash::hash_content;
use crate::partition::Partition;

pub fn handle(id: String) -> Result<()> {
    // Find the .doks file
    let doks_file_path = DoksConfig::find_doks_file()
        .ok_or_else(|| anyhow!("No .doks file found. Run 'doksnet new' first."))?;

    let mut config = DoksConfig::from_file(&doks_file_path)?;

    if config.mappings.is_empty() {
        println!("📭 No mappings found. Use 'doksnet add' to create some first.");
        return Ok(());
    }

    // Find the mapping by ID (allow partial matching)
    let mapping_index = config
        .mappings
        .iter()
        .position(|m| m.id.starts_with(&id))
        .ok_or_else(|| anyhow!("No mapping found with ID starting with '{}'", id))?;

    let mapping = &mut config.mappings[mapping_index];

    println!("✏️  Editing mapping: {}", mapping.id);
    println!("Current values:");
    println!("📄 Documentation: {}", mapping.doc_partition);
    println!("💻 Code: {}", mapping.code_partition);
    if let Some(desc) = &mapping.description {
        println!("📝 Description: {}", desc);
    } else {
        println!("📝 Description: (none)");
    }
    println!();

    // What would you like to edit?
    let options = vec![
        "Documentation partition",
        "Code partition",
        "Description",
        "Both documentation and code partitions",
        "Cancel",
    ];

    let selection = Select::new()
        .with_prompt("What would you like to edit?")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => edit_doc_partition(mapping)?,
        1 => edit_code_partition(mapping)?,
        2 => edit_description(mapping)?,
        3 => {
            edit_doc_partition(mapping)?;
            edit_code_partition(mapping)?;
        }
        4 => {
            println!("❌ Edit cancelled");
            return Ok(());
        }
        _ => unreachable!(),
    }

    // Save the updated config
    config.to_file(&doks_file_path)?;
    println!("✅ Successfully updated mapping!");

    Ok(())
}

fn edit_doc_partition(mapping: &mut crate::config::Mapping) -> Result<()> {
    println!("\n📄 Editing documentation partition");
    println!("Current value: {}", mapping.doc_partition);

    let new_partition: String = Input::new()
        .with_prompt("New documentation partition")
        .with_initial_text(&mapping.doc_partition)
        .interact_text()?;

    if new_partition != mapping.doc_partition {
        // Validate and extract content
        let partition = Partition::parse(&new_partition)?;
        let content = partition
            .extract_content()
            .map_err(|e| anyhow!("Failed to extract documentation content: {}", e))?;

        println!("\n📄 New documentation content preview:");
        println!("---");
        println!("{}", content.chars().take(200).collect::<String>());
        if content.len() > 200 {
            println!("... (truncated)");
        }
        println!("---");

        let confirm = Confirm::new()
            .with_prompt("Apply this change?")
            .default(true)
            .interact()?;

        if confirm {
            mapping.doc_partition = new_partition;
            mapping.doc_hash = hash_content(&content);
            println!("✅ Documentation partition updated");
        } else {
            println!("❌ Documentation partition change cancelled");
        }
    } else {
        println!("ℹ️  No changes made to documentation partition");
    }

    Ok(())
}

fn edit_code_partition(mapping: &mut crate::config::Mapping) -> Result<()> {
    println!("\n💻 Editing code partition");
    println!("Current value: {}", mapping.code_partition);

    let new_partition: String = Input::new()
        .with_prompt("New code partition")
        .with_initial_text(&mapping.code_partition)
        .interact_text()?;

    if new_partition != mapping.code_partition {
        // Validate and extract content
        let partition = Partition::parse(&new_partition)?;
        let content = partition
            .extract_content()
            .map_err(|e| anyhow!("Failed to extract code content: {}", e))?;

        println!("\n💻 New code content preview:");
        println!("---");
        println!("{}", content.chars().take(200).collect::<String>());
        if content.len() > 200 {
            println!("... (truncated)");
        }
        println!("---");

        let confirm = Confirm::new()
            .with_prompt("Apply this change?")
            .default(true)
            .interact()?;

        if confirm {
            mapping.code_partition = new_partition;
            mapping.code_hash = hash_content(&content);
            println!("✅ Code partition updated");
        } else {
            println!("❌ Code partition change cancelled");
        }
    } else {
        println!("ℹ️  No changes made to code partition");
    }

    Ok(())
}

fn edit_description(mapping: &mut crate::config::Mapping) -> Result<()> {
    println!("\n📝 Editing description");
    let current_desc = mapping.description.as_deref().unwrap_or("");
    println!(
        "Current value: {}",
        if current_desc.is_empty() {
            "(none)"
        } else {
            current_desc
        }
    );

    let new_description: String = Input::new()
        .with_prompt("New description (leave empty to remove)")
        .with_initial_text(current_desc)
        .allow_empty(true)
        .interact_text()?;

    let new_description = if new_description.trim().is_empty() {
        None
    } else {
        Some(new_description.trim().to_string())
    };

    if new_description != mapping.description {
        mapping.description = new_description;
        println!("✅ Description updated");
    } else {
        println!("ℹ️  No changes made to description");
    }

    Ok(())
}
