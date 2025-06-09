use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod hash;
mod partition;

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
