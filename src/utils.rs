use anyhow::{bail, Result};
use log::{error, info};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Checks passed Option<PathBuf> and returns an open file, if the Option is
/// `None` in which case the `stdout` is used.
pub fn file_or_stdout(output_file: &Option<PathBuf>) -> Result<Box<dyn Write>> {
    let result = match output_file {
        None => {
            info!("Opening stdout");
            Box::new(std::io::stdout()) as Box<dyn Write>
        }
        Some(value) => match File::create(value) {
            Err(err) => {
                error!("Cannot create file {}", value.display());
                bail!("{}", err.to_string())
            }
            Ok(handle) => {
                info!("Opening file: {:?}", &output_file);
                Box::new(handle)
            }
        },
    };
    Ok(result)
}

/// Checks passed Option<PathBuf> and returns an open file, if the Option is
/// `None` in which case the `stdin` is used.
pub fn file_or_stdin(input_file: &Option<PathBuf>) -> Result<Box<dyn Read>> {
    let result = match input_file {
        None => {
            info!("Opening stdin");
            Box::new(std::io::stdin().lock()) as Box<dyn Read>
        }
        Some(value) => match File::open(value) {
            Err(err) => {
                error!("Cannot read file {}", value.display());
                bail!("{}", err.to_string())
            }
            Ok(handle) => {
                info!("Opening file: {:?}", &input_file);
                Box::new(handle)
            }
        },
    };
    Ok(result)
}

pub fn read_uid_file<P: AsRef<Path>>(uid_file: &Option<P>) -> Result<HashSet<String>> {
    // Makes the set for UIDs
    let mut uid_set: HashSet<String> = HashSet::new();
    if let Some(uid_file) = uid_file {
        info!("Reading file with list of UIDs: {:?}", uid_file.as_ref());
        let file_handle = BufReader::new(File::open(uid_file)?);
        for line in file_handle.lines() {
            uid_set.insert(line?);
        }
        info!("Adding attributes to {} annotations", uid_set.len());
    }

    Ok(uid_set)
}
