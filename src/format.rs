//! Emotiva Format - Loader for `.emotiva.ron` rig files

use crate::CharRig;
use ron::de::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur while loading a character rig file.
#[derive(Debug, Error)]
pub enum RigLoadError {
    #[error("Failed to open file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

/// Loads a character rig from a `.ron` file path.
pub fn load_rig_from_file<P: AsRef<Path>>(path: P) -> Result<CharRig, RigLoadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let rig = from_reader(reader)?;
    Ok(rig)
}
