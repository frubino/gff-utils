mod cli;

use anyhow::Result;
use clap::Parser;
use cli::*;
use cli::add::add_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cli::Commands::Add(options)) => add_command(&options),
        _ => Ok(()),
    }
}
