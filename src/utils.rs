use anyhow::{bail, Result};
use log::{error, info};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

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
