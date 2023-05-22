use std::{io::{BufReader, Read, BufRead}, collections::HashMap, str::FromStr, path::{Path, PathBuf}};
use log::{error, info};
use super::cli::GtfCommand;
use uuid::Uuid;
use anyhow::{Result, Context};
use bio_rascal::gff::{Phase, Strand, Annotation};

fn parse_gtf_attributes(attributes_line: &str) -> (Uuid, HashMap<String, String>) {
    let mut uid: Uuid = Uuid::nil();
    let mut attributes: HashMap<String, String> = HashMap::new();
    
    for field in attributes_line.split(';').map(|f| f.trim()) {
        if let Some((mut key, mut value)) = field.split_once(' ') {
            key = key.trim();
            value = value.trim().trim_matches('"');
            match key {
                "uid" => uid = Uuid::from_str(value).context("Cannot convert Uuid").unwrap(),
                _ => _ = attributes.insert(key.into(), value.into()),
            }
        } else {
            error!("Cannot parse attribute: {:?}", field);
        }
    }
    if uid.is_nil() {
        uid = Uuid::new_v4();
    }
    (uid, attributes)
}

fn parse_gtf_line(line: &str) -> Annotation {
    let fields: Vec<&str> = line.trim().trim_end_matches(';').splitn(9, '\t').map(|f| f.trim()).collect();
    
    let (uid, attributes) = parse_gtf_attributes(fields[8]);
    
    Annotation {
        seq_id: fields[0].to_owned(),
        source: fields[1].to_owned(),
        feature_type: fields[2].to_owned(),
        start: fields[3].parse().context("Parsing Start field failed").unwrap(),
        end: fields[4].parse().context("Parsing End field failed").unwrap(),
        score: fields[5].parse().unwrap_or(0.),
        strand: Strand::from_value(fields[6]),
        phase: Phase::from_value(fields[7]).context("Cannot parse Phase").unwrap(),
        uid,
        attributes,
        taxon_id: 0,
    }
}

struct GtfReader {
    reader: BufReader<Box<dyn Read>>
}

impl GtfReader {
    pub fn from_reader(reader: Box<dyn Read>) -> Self {
        GtfReader {
            reader: BufReader::new(reader),
        }
    }
}

impl Iterator for GtfReader {
    type Item = Annotation;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        
        if let Ok(read_size) = self.reader.read_line(&mut buffer) {
            if read_size == 0 {
                None
            } else if buffer.starts_with('#') {
                // Goes to next iteration
                self.next()
            // starts of sequence, stop reading
            } else if buffer.starts_with('>') {
                None
            } else {
                Some(parse_gtf_line(&buffer))
            }
        } else {
            None
        }
    }
}

pub fn gtf_command(options: GtfCommand) -> Result<()> {
    let input_file = super::utils::file_or_stdin(&options.input_file)?;
    let mut output_file = super::utils::file_or_stdout(&options.output_file)?;
    
    if let Some(path) = options.input_file {
        info!("Reading GTF from file {}", path.display());
    }
    
    if let Some(path) = options.output_file {
        info!("Writing GFF to file {}", path.display());
    }
    
    let reader = GtfReader::from_reader(input_file);
    
    for annotation in reader {
        write!(&mut output_file, "{}\n", annotation.to_string())?;
    }
    
    Ok(())
}
