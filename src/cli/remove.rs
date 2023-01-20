use super::super::utils::{file_or_stdin, file_or_stdout, read_uid_file};
use super::RmCommand;
use anyhow::Result;
use bio_rascal::gff::GffReader;
use log::info;
use std::collections::HashSet;

pub fn remove_command(options: &RmCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    // Stores the key:value changes into a HashMap
    let attributes: HashSet<String> = HashSet::from_iter(options.attributes.iter().cloned());
    info!("Remove {} attributes", attributes.len());

    let uid_set = read_uid_file(&options.uid_file)?;

    let reader = GffReader::from_reader(input_file);

    // Using a while loop, since None will be the end
    for mut annotation in reader {
        if uid_set.is_empty() || uid_set.contains(&annotation.uid.to_string()) {
            for attribute in &attributes {
                // taxon_id is part of the structure
                if attribute == "taxon_id" {
                    annotation.taxon_id = 0;
                } else {
                    annotation.attributes.remove(attribute);
                }
            }
        }

        // Writes to the output file
        writeln!(output_file, "{}", annotation.to_string())?;
    }

    Ok(())
}
