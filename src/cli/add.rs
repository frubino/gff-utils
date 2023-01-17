use super::super::utils::{file_or_stdin, file_or_stdout};
use super::AddCommand;
use anyhow::Result;
use bio_rascal::gff::GffReader;
use std::fs::File;
use std::io::{BufReader, BufRead};
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
    let mut uid_set: HashSet<String> = HashSet::new();
    if let Some(uid_file) = &options.uid_file {
        info!("Reading file with list of UIDs: {:?}", uid_file);
        let file_handle = BufReader::new(File::open(uid_file)?);
        for line in file_handle.lines() {
            uid_set.insert(line?);
        }
        info!("Adding attributes to {} annotations", uid_set.len());
    }

    // Init the reader as mutable
    let mut reader = GffReader::from_reader(input_file);

    // Using a while loop, since None will be the end
    while let Some(mut annotation) = reader.next() {
        // If the uid_set is empty or the UID is contained, modify the
        // annotation
        if uid_set.is_empty() || uid_set.contains(&annotation.uid.to_string()) {
            for (key, value) in attributes.iter() {
                // if the annotation already has the key but the overwrite
                // flag is not set, skip the change
                if annotation.attributes.contains_key(key) && !options.overwrite {
                    continue
                }
                annotation.attributes.insert(key.clone(), value.clone());
            }
        }
        // Writes to the output file
        writeln!(output_file, "{}", annotation.to_string())?;
    }

    Ok(())
}
