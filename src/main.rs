use clap::Parser;
use anyhow::Result;

mod cli;
mod config;
mod partition;
mod hash;
mod commands;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        cli::Commands::New { path } => commands::new::handle(path),
        cli::Commands::Add => commands::add::handle(),
        cli::Commands::Edit { id } => commands::edit::handle(id),
        cli::Commands::RemoveFailed => commands::remove_failed::handle(),
        cli::Commands::Test => commands::test::handle(),
        cli::Commands::TestInteractive => commands::test_interactive::handle(),
    }
}
