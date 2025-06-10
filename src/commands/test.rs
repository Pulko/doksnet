use anyhow::{anyhow, Result};
use std::process;

use crate::config::DoksConfig;
use crate::hash::{hash_content, verify_hash};
use crate::partition::Partition;

pub fn handle() -> Result<()> {
    let doks_file_path = DoksConfig::find_doks_file()
        .ok_or_else(|| anyhow!("No .doks file found. Run 'doksnet new' first."))?;

    let config = DoksConfig::from_file(&doks_file_path)?;

    if config.mappings.is_empty() {
        println!("📭 No mappings found. Use 'doksnet add' to create some first.");
        return Ok(());
    }

    println!(
        "🧪 Testing {} documentation-code mappings",
        config.mappings.len()
    );
    println!("📄 Default documentation file: {}", config.default_doc);
    println!();

    let mut failed_mappings = Vec::new();
    let mut success_count = 0;

    for (index, mapping) in config.mappings.iter().enumerate() {
        let mapping_num = index + 1;
        println!(
            "🔍 Testing mapping {}/{}: {}",
            mapping_num,
            config.mappings.len(),
            mapping.id
        );

        if let Some(desc) = &mapping.description {
            println!("   📝 Description: {}", desc);
        }

        println!("   📄 Doc: {}", mapping.doc_partition);
        println!("   💻 Code: {}", mapping.code_partition);

        let doc_result = test_partition(&mapping.doc_partition, &mapping.doc_hash, "documentation");

        let code_result = test_partition(&mapping.code_partition, &mapping.code_hash, "code");

        match (doc_result, code_result) {
            (Ok(()), Ok(())) => {
                println!("   ✅ PASS");
                success_count += 1;
            }
            (doc_err, code_err) => {
                println!("   ❌ FAIL");

                let mut error_details = Vec::new();
                if let Err(e) = doc_err {
                    error_details.push(format!("Documentation: {}", e));
                }
                if let Err(e) = code_err {
                    error_details.push(format!("Code: {}", e));
                }

                failed_mappings.push((mapping_num, mapping.id.clone(), error_details));
            }
        }

        println!();
    }

    println!("📊 Test Results Summary:");
    println!("   ✅ Passed: {}/{}", success_count, config.mappings.len());
    println!(
        "   ❌ Failed: {}/{}",
        failed_mappings.len(),
        config.mappings.len()
    );

    if !failed_mappings.is_empty() {
        println!("\n🚨 Failed Mappings Details:");
        for (mapping_num, id, errors) in failed_mappings {
            println!("   {}. {} (ID: {})", mapping_num, id, &id[..8]);
            for error in errors {
                println!("      • {}", error);
            }
        }

        println!("\n💡 Tip: Use 'doksnet edit <id>' to fix broken mappings");

        process::exit(1);
    } else {
        println!("\n🎉 All mappings are up to date!");
    }

    Ok(())
}

fn test_partition(partition_str: &str, expected_hash: &str, content_type: &str) -> Result<()> {
    let partition = Partition::parse(partition_str).map_err(|e| {
        anyhow!(
            "Failed to parse {} partition '{}': {}",
            content_type,
            partition_str,
            e
        )
    })?;

    let content = partition
        .extract_content()
        .map_err(|e| anyhow!("Failed to extract {} content: {}", content_type, e))?;

    if !verify_hash(&content, expected_hash) {
        let current_hash = hash_content(&content);
        return Err(anyhow!(
            "{} content has changed (expected: {}..., actual: {}...)",
            content_type,
            &expected_hash[..8],
            &current_hash[..8]
        ));
    }

    Ok(())
}
