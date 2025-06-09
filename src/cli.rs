use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "doksnet")]
#[command(about = "A CLI tool for documentation-code mapping verification")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new .doks file
    New {
        /// Path where the .doks file should be created (default: current directory)
        path: Option<PathBuf>,
    },
    /// Add a mapping between documentation and code
    Add,
    /// Edit an existing mapping by ID
    Edit {
        /// The ID of the mapping to edit (first 8 characters are sufficient)
        id: String,
    },
    /// Remove all failed mappings
    RemoveFailed,
    /// Test all mappings and verify hashes (non-interactive, CI/CD friendly)
    Test,
    /// Interactive testing with change preview and edit/remove options
    TestInteractive,
}
