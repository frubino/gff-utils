pub mod add;
pub mod fields;
pub mod remove;
pub mod table;
pub mod view;

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
    Gtf(GtfCommand),
    // Json(JsonCommand),
    // Import(ImportCommand),
    // Filter(FilterCommand)
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

/// Removes attributes from a GFF file
#[derive(Debug, Args)]
pub struct RmCommand {
    /// Attributes to remove
    ///
    /// Multiple attributes can be passed, by using the option multiple times
    /// or separating them by commas `,`
    #[arg(short, long, required = true, value_delimiter = ',')]
    attributes: Vec<String>,
    /// Use a file with UIDs to limit the removal to specific annotations
    ///
    /// The file needs to have a UID per line
    #[arg(short, long)]
    uid_file: Option<PathBuf>,
    /// Input file, without value the stdin is used
    input_file: Option<PathBuf>,
    /// Output file, without value the stdout is used
    output_file: Option<PathBuf>,
}

/// Views a GFF as a table, using the fields requested
///
/// The table is `tab` separated and the order of the columns
/// is the same as the attributes requested.
#[derive(Debug, Args)]
pub struct ViewCommand {
    /// Prints header
    ///
    /// Line starts with `#` and includes all attributes requested
    #[arg(short = 'e', long)]
    header: bool,
    /// The default is to remove from the output annotations where no attributes were found
    /// 
    /// This options allows to keep them instead.
    #[arg(short, long)]
    keep_empty: bool,
    /// Attributes to print
    ///
    /// Multiple attributes can be passed, by using the option multiple times
    /// or separating them by commas `,`
    #[arg(short, long, required = true, value_delimiter = ',')]
    attributes: Vec<String>,
    /// Input file, without value the stdin is used
    input_file: Option<PathBuf>,
    /// Output file, without value the stdout is used
    output_file: Option<PathBuf>,
}

/// Adds attributes to a GFF using a file containing a table
///
/// The table needs to include the key to decide which annotations
/// to modify as the first column, be tab separated and values
/// to modify in each column must correspond to the order the
/// `attributes` options are passed.
#[derive(Debug, Args)]
pub struct TableCommand {
    /// Attribute in a GFF file to modify it
    ///
    /// Corresponds to the first column in the table
    #[arg(short, long, default_value = "uid")]
    key: Option<String>,
    /// Attributes, one per each column
    ///
    /// Corresponds to column 2 onwards in the table
    #[arg(short, long, required = true)]
    attributes: Vec<String>,
    /// Only output the modified annotations
    #[arg(short, long)]
    only_edited: bool,
    /// Skips lines starting with this character
    ///
    /// By default, lines starting with `#` in the table are skipped
    #[arg(short, long, default_value = "#")]
    comment_char: String,
    /// File with table of changes
    ///
    /// TODO: a few more notes
    #[arg(short, long, required = true)]
    table_file: PathBuf,
    /// Use a key compatible with Prodigal sequence files
    ///
    /// The file must contain one column for the key and one for each
    /// attribute to modify. Additional columns will be ignored
    /// TODO: example
    #[arg(short, long)]
    prodigal_gene: bool,
    /// Skips a number a lines from the table file
    #[arg(short, long, default_value_t = 0)]
    skip_rows: usize,
    /// Input file, without value the stdin is used
    input_file: Option<PathBuf>,
    /// Output file, without value the stdout is used
    output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct GtfCommand {
    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}

/// Generates the completion for the specified shell
///
/// Slightly modified from example
pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
