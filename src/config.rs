use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const DOKS_FILE_NAME: &str = ".doks";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoksConfig {
    /// Version of the doks format
    pub version: String,
    /// Default documentation file
    pub default_doc: String,
    /// Mappings between documentation and code partitions
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mapping {
    /// Unique identifier for this mapping
    pub id: String,
    /// Documentation partition reference
    pub doc_partition: String,
    /// Code partition reference
    pub code_partition: String,
    /// Hash of the documentation content
    pub doc_hash: String,
    /// Hash of the code content
    pub code_hash: String,
    /// Optional description
    pub description: Option<String>,
}

impl DoksConfig {
    pub fn new(default_doc: String) -> Self {
        Self {
            version: "0.1.0".to_string(),
            default_doc,
            mappings: Vec::new(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DoksConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
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
        assert_eq!(config.version, "0.1.0");
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

        // Write to file
        config.to_file(&file_path).unwrap();
        assert!(file_path.exists());

        // Read from file
        let loaded_config = DoksConfig::from_file(&file_path).unwrap();
        assert_eq!(loaded_config.version, config.version);
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

        // Change to temp directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        // No .doks file exists
        assert!(DoksConfig::find_doks_file().is_none());

        // Create .doks file
        fs::write(&doks_path, "test").unwrap();
        let found = DoksConfig::find_doks_file();
        assert!(found.is_some());

        // Check that the filename matches (handle potential symlink differences on macOS)
        let found_path = found.unwrap();
        assert!(found_path.ends_with(DOKS_FILE_NAME));
        assert!(found_path.exists());

        // Restore original directory
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
        assert!(content.contains("version = \"0.1.0\""));
        assert!(content.contains("default_doc = \"README.md\""));
        assert!(content.contains("[[mappings]]"));
        assert!(content.contains("id = \"test-id-123\""));
    }

    #[test]
    fn test_mapping_serialization() {
        let mapping = create_test_mapping();
        let serialized = toml::to_string(&mapping).unwrap();
        let deserialized: Mapping = toml::from_str(&serialized).unwrap();

        assert_eq!(mapping.id, deserialized.id);
        assert_eq!(mapping.doc_partition, deserialized.doc_partition);
        assert_eq!(mapping.code_partition, deserialized.code_partition);
        assert_eq!(mapping.doc_hash, deserialized.doc_hash);
        assert_eq!(mapping.code_hash, deserialized.code_hash);
        assert_eq!(mapping.description, deserialized.description);
    }
}
