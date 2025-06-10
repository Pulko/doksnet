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
    New {
        path: Option<PathBuf>,
    },
    Add,
    Edit {
        id: String,
    },
    RemoveFailed,
    Test,
    TestInteractive,
}
