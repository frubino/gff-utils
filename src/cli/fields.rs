use super::super::utils::file_or_stdin;
use super::FieldsCommand;
use anyhow::Result;
use bio_rascal::gff::GffReader;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::option;
use log::info;
use std::collections::{HashMap, HashSet};

pub fn fields_command(options: &FieldsCommand) -> Result<()> {
    let input_file = file_or_stdin(&options.input_file)?;
    
    let mut fields: HashSet<String> = HashSet::new();
    fields.insert("uid".into());
    fields.insert("seq_id".into());
    fields.insert("source".into());
    fields.insert("feature_type".into());
    fields.insert("start".into());
    fields.insert("end".into());
    fields.insert("score".into());
    fields.insert("strand".into());
    fields.insert("phase".into());
    fields.insert("length".into());
    fields.insert("uid".into());
    
    let mut reader = GffReader::from_reader(input_file);

    let mut count = 0;
    // Using a while loop, since None will be the end
    while let Some(annotation) = reader.next() {
        if count >= options.num_ann {
            break;
        }
        for attribute in annotation.attributes.keys() {
            // to avoid cloning unnecessarly
            if !fields.contains(attribute) {
                fields.insert(attribute.clone());
            }
        }
        count += 1;
    }
    
    info!("Found {} attributes from {} annotations", fields.len(), count);
    
    let mut stdout = std::io::stdout().lock();
    
    for field in fields {
        writeln!(stdout, "{}", field)?;
    }
    
    Ok(())
}
