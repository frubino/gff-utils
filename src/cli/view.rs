use super::super::utils::{file_or_stdin, file_or_stdout};
use super::ViewCommand;
use anyhow::Result;
use bio_rascal::gff::GffReader;
use bio_rascal::taxon::ROOT_TAXON;
use itertools::Itertools;
use log::info;

pub fn view_command(options: &ViewCommand) -> Result<()> {
    // first check the input and output files
    let input_file = file_or_stdin(&options.input_file)?;
    let mut output_file = file_or_stdout(&options.output_file)?;

    let reader = GffReader::from_reader(input_file);

    info!("{} attributes will be written", options.attributes.len());

    if options.header {
        info!("Writing header");
        writeln!(output_file, "#{}", options.attributes.iter().join("\t"))?;
    }

    let mut count = 0;
    // Using a while loop, since None will be the end
    for annotation in reader {
        count += 1;
        let mut values: Vec<String> = Vec::new();

        for attribute in &options.attributes {
            let value = match attribute.as_str() {
                "uid" => annotation.uid.to_string(),
                "taxon_id" => match annotation.taxon_id {
                    ROOT_TAXON.. => annotation.taxon_id.to_string(),
                    _ => "".into(),
                },
                "seq_id" => annotation.seq_id.clone(),
                "source" => annotation.source.clone(),
                "feature_type" => annotation.feature_type.clone(),
                "start" => annotation.start.to_string(),
                "end" => annotation.end.to_string(),
                "score" => annotation.score.to_string(),
                "strand" => annotation.strand.to_string(),
                "phase" => annotation.phase.to_string(),
                "length" => annotation.length().to_string(),
                _ => match annotation.attributes.get(attribute) {
                    Some(value) => value.clone(),
                    None => {
                        if options.keep_empty {
                            "".into()
                        } else {
                            continue;
                        }
                    },
                },
            };
            values.push(value);
        }
        if values.is_empty() && !options.keep_empty {
            continue;
        }
        writeln!(output_file, "{}", values.join("\t"))?;
    }
    
    info!("Read {} annotations", count);

    Ok(())
}
