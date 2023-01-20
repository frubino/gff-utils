use super::super::utils::{file_or_stdin, file_or_stdout};
use bio_rascal::io::open_file;
use super::TableCommand;
use anyhow::{Result, bail};
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use bio_rascal::gff::GffReader;
use log::info;

type ValueTable = HashMap<String, Vec<String>>;

fn read_table<P: AsRef<Path>, C: AsRef<str>>(file_name: P, skip_lines: usize, comment_char: C, n_fields: usize) -> Result<ValueTable> {
    
    let file_handle = open_file(file_name)?;
    // needs to get a concrete type for `str::starts_with`
    let skip_char = comment_char.as_ref();
    
    // Let's define at least 100 elements
    let mut value_table = ValueTable::with_capacity(100);
    
    // Skips also the number of lines specified
    for line in file_handle.lines().skip(skip_lines) {
        let line = line?;
        
        // checks that the line is not a comment
        if line.starts_with(skip_char) {
            continue
        }
        
        let fields: Vec<String> = line.split('\t').map(|s| String::from_str(s).ok()).flatten().collect();
        // checks that enough columns are present
        if fields.len() < n_fields {
            bail!("Expected {} columns including the key, instead found {}", n_fields, fields.len())
        }
        //value_table.insert(fields[0].into(), fields);
    }
    
    todo!()
}

pub fn table_command(options: &TableCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    info!("Reading table from file {}", &options.table_file.display());
    
    let reader = GffReader::from_reader(input_file);
    
    for mut annotation in reader {
        todo!()
    }
    
    Ok(())
}
