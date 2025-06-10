use anyhow::{anyhow, Result};
use std::path::Path;

pub const DOKS_FILE_NAME: &str = ".doks";

#[derive(Debug, Clone)]
pub struct DoksConfig {
    pub default_doc: String,
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub id: String,
    pub doc_partition: String,
    pub code_partition: String,
    pub doc_hash: String,
    pub code_hash: String,
    pub description: Option<String>,
}

impl DoksConfig {
    pub fn new(default_doc: String) -> Self {
        Self {
            default_doc,
            mappings: Vec::new(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = self.to_string();
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn parse(content: &str) -> Result<Self> {
        let mut default_doc = String::new();
        let mut mappings = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("default_doc=") {
                default_doc = line.strip_prefix("default_doc=").unwrap().to_string();
            } else if line.contains('|') {
                // Parse mapping line: id|doc_partition|code_partition|doc_hash|code_hash|description
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 5 {
                    return Err(anyhow!(
                        "Invalid mapping line: {} (expected at least 5 parts)",
                        line
                    ));
                }

                let description = if parts.len() > 5 && !parts[5].trim().is_empty() {
                    Some(parts[5].trim().to_string())
                } else {
                    None
                };

                mappings.push(Mapping {
                    id: parts[0].trim().to_string(),
                    doc_partition: parts[1].trim().to_string(),
                    code_partition: parts[2].trim().to_string(),
                    doc_hash: parts[3].trim().to_string(),
                    code_hash: parts[4].trim().to_string(),
                    description,
                });
            }
        }

        if default_doc.is_empty() {
            return Err(anyhow!("Missing default_doc in .doks file"));
        }

        Ok(Self {
            default_doc,
            mappings,
        })
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut content = String::new();

        content.push_str("# .doks - Mapping doks to code \n");
        content.push_str(&format!("default_doc={}\n", self.default_doc));
        content.push('\n');

        if !self.mappings.is_empty() {
            content.push_str(
                "# Format: id|doc_partition|code_partition|doc_hash|code_hash|description\n",
            );

            for mapping in &self.mappings {
                let description = mapping.description.as_deref().unwrap_or("");
                content.push_str(&format!(
                    "{}|{}|{}|{}|{}|{}\n",
                    mapping.id,
                    mapping.doc_partition,
                    mapping.code_partition,
                    mapping.doc_hash,
                    mapping.code_hash,
                    description
                ));
            }
        }

        content
    }

    pub fn find_doks_file() -> Option<std::path::PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        loop {
            let doks_path = current.join(DOKS_FILE_NAME);
            if doks_path.exists() {
                return Some(doks_path);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    pub fn add_mapping(&mut self, mapping: Mapping) {
        self.mappings.push(mapping);
    }

    #[allow(dead_code)]
    pub fn find_mapping_by_id(&mut self, id: &str) -> Option<&mut Mapping> {
        self.mappings.iter_mut().find(|m| m.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_mapping() -> Mapping {
        Mapping {
            id: "test-id-123".to_string(),
            doc_partition: "README.md:1-5".to_string(),
            code_partition: "src/main.rs:10-20".to_string(),
            doc_hash: "abc123".to_string(),
            code_hash: "def456".to_string(),
            description: Some("Test mapping".to_string()),
        }
    }

    #[test]
    fn test_doks_config_new() {
        let config = DoksConfig::new("README.md".to_string());
        assert_eq!(config.default_doc, "README.md");
        assert!(config.mappings.is_empty());
    }

    #[test]
    fn test_add_mapping() {
        let mut config = DoksConfig::new("README.md".to_string());
        let mapping = create_test_mapping();

        config.add_mapping(mapping.clone());
        assert_eq!(config.mappings.len(), 1);
        assert_eq!(config.mappings[0].id, mapping.id);
    }

    #[test]
    fn test_find_mapping_by_id() {
        let mut config = DoksConfig::new("README.md".to_string());
        let mapping = create_test_mapping();
        let id = mapping.id.clone();

        config.add_mapping(mapping);

        let found = config.find_mapping_by_id(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, id);

        let not_found = config.find_mapping_by_id("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_to_file_and_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".doks");

        let mut config = DoksConfig::new("README.md".to_string());
        config.add_mapping(create_test_mapping());

        config.to_file(&file_path).unwrap();
        assert!(file_path.exists());

        let loaded_config = DoksConfig::from_file(&file_path).unwrap();
        assert_eq!(loaded_config.default_doc, config.default_doc);
        assert_eq!(loaded_config.mappings.len(), 1);
        assert_eq!(loaded_config.mappings[0].id, config.mappings[0].id);
    }

    #[test]
    fn test_from_file_not_found() {
        let result = DoksConfig::from_file("nonexistent.doks");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_doks_file() {
        let dir = tempdir().unwrap();
        let doks_path = dir.path().join(DOKS_FILE_NAME);

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        assert!(DoksConfig::find_doks_file().is_none());

        fs::write(&doks_path, "default_doc=README.md\n").unwrap();
        let found = DoksConfig::find_doks_file();
        assert!(found.is_some());

        let found_path = found.unwrap();
        assert!(found_path.ends_with(DOKS_FILE_NAME));
        assert!(found_path.exists());

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_serialization_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".doks");

        let mut config = DoksConfig::new("README.md".to_string());
        config.add_mapping(create_test_mapping());

        config.to_file(&file_path).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("# .doks"));
        assert!(content.contains("default_doc=README.md"));
        assert!(content
            .contains("test-id-123|README.md:1-5|src/main.rs:10-20|abc123|def456|Test mapping"));
    }

    #[test]
    fn test_mapping_serialization() {
        let mapping = create_test_mapping();
        let mut config = DoksConfig::new("README.md".to_string());
        config.add_mapping(mapping.clone());

        let serialized = config.to_string();
        let deserialized = DoksConfig::parse(&serialized).unwrap();

        assert_eq!(deserialized.mappings.len(), 1);
        let parsed_mapping = &deserialized.mappings[0];
        assert_eq!(mapping.id, parsed_mapping.id);
        assert_eq!(mapping.doc_partition, parsed_mapping.doc_partition);
        assert_eq!(mapping.code_partition, parsed_mapping.code_partition);
        assert_eq!(mapping.doc_hash, parsed_mapping.doc_hash);
        assert_eq!(mapping.code_hash, parsed_mapping.code_hash);
        assert_eq!(mapping.description, parsed_mapping.description);
    }

    #[test]
    fn test_parse_compact_format() {
        let content = r#"
# .doks
default_doc=README.md

# Format: id|doc_partition|code_partition|doc_hash|code_hash|description
test-1|README.md:1-5|src/main.rs:10-20|abc123|def456|Test mapping
test-2|docs/api.md:5-10|src/lib.rs:1-10|fedcba|654321|
        "#;

        let config = DoksConfig::parse(content).unwrap();
        assert_eq!(config.default_doc, "README.md");
        assert_eq!(config.mappings.len(), 2);

        assert_eq!(config.mappings[0].id, "test-1");
        assert_eq!(
            config.mappings[0].description,
            Some("Test mapping".to_string())
        );

        assert_eq!(config.mappings[1].id, "test-2");
        assert_eq!(config.mappings[1].description, None);
    }

    #[test]
    fn test_parse_invalid_format() {
        let content = "invalid|format";
        let result = DoksConfig::parse(content);
        assert!(result.is_err());

        let content = "# missing default_doc";
        let result = DoksConfig::parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_description() {
        let mut config = DoksConfig::new("README.md".to_string());
        let mapping = Mapping {
            id: "test".to_string(),
            doc_partition: "README.md:1".to_string(),
            code_partition: "src/main.rs:1".to_string(),
            doc_hash: "abc".to_string(),
            code_hash: "def".to_string(),
            description: None,
        };
        config.add_mapping(mapping);

        let serialized = config.to_string();
        let parsed = DoksConfig::parse(&serialized).unwrap();

        assert_eq!(parsed.mappings[0].description, None);
    }
}
