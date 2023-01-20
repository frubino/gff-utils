mod cli;
mod utils;

use anyhow::{Ok, Result};
use clap::{CommandFactory, Parser}; // CommandFactory is necessary for Cli::command()
use cli::add::add_command;
use cli::fields::fields_command;
use cli::remove::remove_command;
use cli::*;
use cli::view::view_command;
use env_logger::Env;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(generator) = cli.complete {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
    } else if let Some(command) = cli.command {
        // Starts the logging
        // possible to define log level with RUST_LOG
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format_timestamp_millis()
            .init();

        let result = match command {
            cli::Commands::Add(options) => add_command(&options),
            cli::Commands::Fields(options) => fields_command(&options),
            cli::Commands::Rm(options) => remove_command(&options),
            cli::Commands::View(options) => view_command(&options),
            _ => todo!(),
        };

        return result;
    }

    Ok(())
}
