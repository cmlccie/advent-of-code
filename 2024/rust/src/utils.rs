/*-------------------------------------------------------------------------------------------------
  Crate Utilities
-------------------------------------------------------------------------------------------------*/

use anyhow::{Error, Result};

#[cfg(test)]
use anyhow::anyhow;

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
pub(crate) fn solution<P>(file_path: &P) -> Result<i64>
where
    P: AsRef<Path> + ?Sized,
{
    Ok(std::fs::read_to_string(file_path)?
        .lines()
        .next()
        .ok_or(anyhow!("Empty first line"))?
        .parse::<i64>()?)
}
