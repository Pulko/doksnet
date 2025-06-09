use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

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

    pub fn find_mapping_by_id(&mut self, id: &str) -> Option<&mut Mapping> {
        self.mappings.iter_mut().find(|m| m.id == id)
    }
} 