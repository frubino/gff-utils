pub mod add;
pub mod fields;

use anyhow::{bail, Result};
use clap::{Args, Command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::path::PathBuf;

/// Taxonomy Utilities
#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// Generates Shell completion code
    ///
    /// It prints the code to the standard output and the way to
    /// use depends on the Shell. For Fish, redirect to a file
    /// with `.fish` extension in `~/.config/fish/completion`.
    #[arg(long)]
    pub complete: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(AddCommand),
    Fields(FieldsCommand),
    Rm(RmCommand),
    View(ViewCommand),
    Table(TableCommand),
    // Json(JsonCommand),
    // Import(ImportCommand),
}

fn key_value_parser(arg: &str) -> Result<(String, String)> {
    match arg.split_once(':') {
        None => bail!("Cannot parse 'key:value' argument: {}", arg),
        Some((key, value)) => Ok((key.into(), value.into())),
    }
}

/// Adds attributes to a GFF file
#[derive(Debug, Args)]
pub struct AddCommand {
    /// Attributes and value in the form attr:value
    /// 
    /// if the keys are duplicated, the value from the last one
    /// is used
    /// This can be passed multiple times, either by
    /// the option multiple times or passing multiple
    /// values separated by commas. These are equivalent:
    /// -a attr1:value -a attr2:value <-> -a attr1:value,attr2:value
    #[arg(short, long, value_parser = key_value_parser, required = true, value_delimiter = ',')]
    attributes: Vec<(String, String)>,
    /// If an attribute is already present overwrite its value
    #[arg(short, long)]
    overwrite: bool,
    /// Only changes annotations with uids contained in the file
    /// 
    /// One uid per line
    #[arg(short, long)]
    uid_file: Option<PathBuf>,
    /// Input file, without value the stdin is used
    input_file: Option<PathBuf>,
    /// Output file, without value the stdout is used
    output_file: Option<PathBuf>,
}

/// Scans a GFF file to list the attributes contained
#[derive(Debug, Args)]
pub struct FieldsCommand {
    /// Number of lines to read, before printing
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u16).range(1..))]
    num_ann: u16,
    /// Input file, without value the stdin is used
    input_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct RmCommand {
    #[arg(short, long, required = true)]
    attribute: Vec<String>,
    #[arg(short, long)]
    uid_file: Option<PathBuf>,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    /// Prints header (only Text file)
    #[arg(short = 'e', long)]
    header: bool,
    /// Keeps annotations where not all attributes were found
    #[arg(short, long)]
    keep_empty: bool,
    /// Attributes to print
    #[arg(short, long, required = true, value_delimiter = ',')]
    attributes: Vec<String>,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct TableCommand {
    #[arg(short, long, default_value = "uid")]
    key: Option<String>,
    #[arg(short, long, required = true)]
    attribute: Vec<String>,
    #[arg(short, long)]
    only_edited: bool,
    #[arg(short, long, default_value = "#")]
    comment_char: String,
    #[arg(short, long, required = true)]
    table_file: PathBuf,
    #[arg(short, long)]
    prodigal_gene: bool,
    #[arg(short, long, default_value_t = 0)]
    skip_rows: usize,
    #[arg(short, long)]
    default_value: Option<String>,
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

/// Generates the completion for the specified shell
///
/// Slightly modified from example
pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
