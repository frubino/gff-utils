use super::super::utils::{file_or_stdin, file_or_stdout, read_uid_file};
use super::AddCommand;
use anyhow::{Context, Result};
use bio_rascal::gff::GffReader;
use log::info;
use std::collections::{HashMap, HashSet};

pub fn add_command(options: &AddCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    // Stores the key:value changes into a HashMap
    let attributes: HashMap<String, String> =
        HashMap::from_iter(options.attributes.iter().cloned());
    info!("Adding {} attributes", attributes.len());

    // Makes the set for UIDs
    let uid_set: HashSet<String> = read_uid_file(&options.uid_file)?;

    // Init the reader as mutable
    let reader = GffReader::from_reader(input_file);

    // Using a while loop, since None will be the end
    for mut annotation in reader {
        // If the uid_set is empty or the UID is contained, modify the
        // annotation
        if uid_set.is_empty() || uid_set.contains(&annotation.uid.to_string()) {
            for (key, value) in &attributes {
                // if the annotation already has the key but the overwrite
                // flag is not set, skip the change
                if annotation.attributes.contains_key(key) && !options.overwrite {
                    continue;
                }
                if key == "taxon_id" {
                    annotation.taxon_id = value
                        .parse()
                        .context("Failed to convert the taxon_id passed to a number")?;
                } else {
                    annotation.attributes.insert(key.clone(), value.clone());
                }
            }
        }
        // Writes to the output file
        writeln!(output_file, "{}", annotation.to_string())?;
    }

    Ok(())
}
