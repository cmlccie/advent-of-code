/*-------------------------------------------------------------------------------------------------
  Crate Utilities
-------------------------------------------------------------------------------------------------*/

use anyhow::{Error, Result};

#[cfg(test)]
use std::path::Path;

/*--------------------------------------------------------------------------------------
  Logging Functions
--------------------------------------------------------------------------------------*/

pub(crate) fn log_error(error: &Error) {
    log::error!("{}", error);
}

pub(crate) fn log_if_error<T>(result: &Result<T>) {
    if let Err(error) = result {
        log_error(error);
    }
}

/*--------------------------------------------------------------------------------------
  Get a Solution from a Data File
--------------------------------------------------------------------------------------*/

/// Get the solution from a file. The file should contain a single line with the solution.
#[cfg(test)]
pub(crate) fn solution<P>(file_path: &P) -> String
where
    P: AsRef<Path> + ?Sized,
{
    std::fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .to_string()
}
