use super::super::utils::{file_or_stdin, file_or_stdout};
use super::TableCommand;
use anyhow::{bail, Result};
use bio_rascal::gff::GffReader;
use bio_rascal::io::open_file;
use log::{info, warn};
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

type ValueTable = HashMap<String, Vec<String>>;

fn read_table<P: AsRef<Path>, C: AsRef<str>>(
    file_name: P,
    skip_lines: &usize,
    comment_char: &C,
    n_fields: usize,
) -> Result<ValueTable> {
    let file_handle = open_file(file_name.as_ref())?;
    // needs to get a concrete type for `str::starts_with`
    let skip_char = comment_char.as_ref();

    // Let's define at least 100 elements
    let mut value_table = ValueTable::with_capacity(100);

    let mut count = 0;
    // Skips also the number of lines specified
    for line in file_handle.lines().skip(*skip_lines) {
        let line = line?;

        // checks that the line is not a comment
        if line.starts_with(skip_char) {
            continue;
        }

        let fields: Vec<String> = line
            .split('\t')
            .map(|s| String::from_str(s).ok())
            .flatten()
            .collect();
        // checks that enough columns are present
        if fields.len() < n_fields {
            bail!(
                "Expected {} columns including the key, instead found {}",
                n_fields,
                fields.len()
            )
        }
        value_table.insert(
            fields[0].clone(),
            fields[1..n_fields].iter().cloned().collect(),
        );
        count += 1;
    }

    info!("Read {} line(s) from the table", count);

    Ok(value_table)
}

pub fn table_command(options: &TableCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    info!("Reading table from file {}", &options.table_file.display());
    let value_table = read_table(
        &options.table_file,
        &options.skip_rows,
        &options.comment_char,
        options.attributes.len() + 1,
    )?;

    let key = match &options.key {
        None => "uid".to_string(),
        Some(value) => value.clone(),
    };

    if options.prodigal_gene {
        info!("Using key from Prodigal sequences")
    } else {
        info!(
            "Using '{}' as key and attributes: {}",
            key,
            options.attributes.join(", ")
        );
    }

    let reader = GffReader::from_reader(input_file);

    for mut annotation in reader {
        let key_value;
        // check if the key is in the value_table
        if options.prodigal_gene {
            key_value = match annotation.get_attr("ID") {
                None => String::new(),
                Some(value) => format!("{}_{}", annotation.seq_id, value),
            }
        // gets the value from the attributes
        } else {
            key_value = match annotation.get_attr(&key) {
                None => String::new(),
                Some(value) => value,
            };
        }
        match value_table.get(&key_value) {
            Some(value_vec) => {
                // start to add/change attributes
                for (key, value) in options.attributes.iter().zip(value_vec) {
                    match key.as_str() {
                        "taxon_id" => annotation.taxon_id = value.parse()?,
                        "uid" => warn!("UID changing is not allowed"),
                        _ => _ = annotation.attributes.insert(key.clone(), value.clone()),
                    }
                }
            }
            None => {
                if options.only_edited {
                    continue;
                }
            }
        };

        // Writes to the output file
        writeln!(output_file, "{}", annotation.to_string())?;
    }

    Ok(())
}
