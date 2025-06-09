use anyhow::{anyhow, Result};
use dialoguer::Confirm;

use crate::config::DoksConfig;
use crate::hash::verify_hash;
use crate::partition::Partition;

pub fn handle() -> Result<()> {
    // Find the .doks file
    let doks_file_path = DoksConfig::find_doks_file()
        .ok_or_else(|| anyhow!("No .doks file found. Run 'doksnet new' first."))?;

    let mut config = DoksConfig::from_file(&doks_file_path)?;

    if config.mappings.is_empty() {
        println!("📭 No mappings found. Use 'doksnet add' to create some first.");
        return Ok(());
    }

    println!(
        "🔍 Checking {} mappings for failures...",
        config.mappings.len()
    );

    // Identify failed mappings
    let mut failed_indices = Vec::new();
    let mut failed_details = Vec::new();

    for (index, mapping) in config.mappings.iter().enumerate() {
        let doc_failed = !test_partition_validity(&mapping.doc_partition, &mapping.doc_hash);
        let code_failed = !test_partition_validity(&mapping.code_partition, &mapping.code_hash);

        if doc_failed || code_failed {
            let mut failure_reasons = Vec::new();
            if doc_failed {
                failure_reasons.push("documentation");
            }
            if code_failed {
                failure_reasons.push("code");
            }

            failed_indices.push(index);
            failed_details.push((
                mapping.id.clone(),
                mapping.doc_partition.clone(),
                mapping.code_partition.clone(),
                mapping.description.clone(),
                failure_reasons,
            ));
        }
    }

    if failed_indices.is_empty() {
        println!("✅ No failed mappings found! All mappings are up to date.");
        return Ok(());
    }

    println!("\n🚨 Found {} failed mapping(s):", failed_indices.len());
    for (id, doc_partition, code_partition, description, reasons) in &failed_details {
        println!("   📍 ID: {} ({}...)", &id[..8], id);
        println!("      📄 Doc: {}", doc_partition);
        println!("      💻 Code: {}", code_partition);
        if let Some(desc) = description {
            println!("      📝 Description: {}", desc);
        }
        println!("      ❌ Failed: {}", reasons.join(", "));
        println!();
    }

    println!("💡 These mappings have content that no longer matches their stored hashes.");

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Remove all {} failed mapping(s)?",
            failed_indices.len()
        ))
        .default(false)
        .interact()?;

    if confirm {
        // Remove failed mappings (iterate in reverse to preserve indices)
        for &index in failed_indices.iter().rev() {
            config.mappings.remove(index);
        }

        config.to_file(&doks_file_path)?;

        println!(
            "✅ Successfully removed {} failed mapping(s)",
            failed_indices.len()
        );
        println!("📊 Remaining mappings: {}", config.mappings.len());

        if config.mappings.is_empty() {
            println!("💡 No mappings remain. Use 'doksnet add' to create new ones.");
        }
    } else {
        println!("❌ Removal cancelled. Failed mappings remain.");
        println!("💡 Tip: Use 'doksnet edit <id>' to fix individual mappings");
        println!("💡 Tip: Use 'doksnet test-interactive' for guided fixing");
    }

    Ok(())
}

fn test_partition_validity(partition_str: &str, expected_hash: &str) -> bool {
    // Try to parse and extract content, then verify hash
    match Partition::parse(partition_str) {
        Ok(partition) => {
            match partition.extract_content() {
                Ok(content) => verify_hash(&content, expected_hash),
                Err(_) => false, // File not found or content extraction failed
            }
        }
        Err(_) => false, // Partition parsing failed
    }
}
