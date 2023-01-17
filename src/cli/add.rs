use super::super::utils::{file_or_stdin, file_or_stdout};
use super::AddCommand;
use anyhow::Result;
use bio_rascal::gff::GffReader;
use log::info;
use std::collections::HashMap;

pub fn add_command(options: &AddCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    let attributes: HashMap<String, String> =
        HashMap::from_iter(options.attributes.iter().cloned());
    info!("Adding {} attributes", attributes.len());

    let reader = GffReader::from_reader(input_file);

    for line in reader {
        let mut annotation = line.clone();
        for (key, value) in attributes.iter() {
            annotation.attributes.insert(key.clone(), value.clone());
        }
        writeln!(output_file, "{}", annotation.to_string())?;
    }

    Ok(())
}
