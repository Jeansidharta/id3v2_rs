use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::PathBuf,
};
use tag::{Tag, TagReadError};
use thiserror::Error;

pub mod errors;
pub mod tag;
pub mod utils;

#[derive(PartialEq, Clone, Debug, Error)]
pub enum FileReadError {
    #[error("File {0} not found")]
    FileNotFound(String),
    #[error("Missing read permission on file {0}")]
    MissingReadPermissions(String),
    #[error("File {0} not found")]
    FileSystemError(String),
    #[error("While reading file {0}, error parsing it's ID3v2 tag: {1}")]
    TagReadingError(String, TagReadError),
}

pub fn read_file(filename: &PathBuf) -> Result<Tag, FileReadError> {
    let mut file = File::open(filename).map_err(|fserror| match fserror.kind() {
        ErrorKind::NotFound => FileReadError::FileNotFound(filename.to_string_lossy().to_string()),
        ErrorKind::PermissionDenied => {
            FileReadError::MissingReadPermissions(filename.to_string_lossy().to_string())
        }
        _ => FileReadError::FileSystemError(filename.to_string_lossy().to_string()),
    })?;

    let tag = tag::Tag::read(&mut file).map_err(|err| {
        FileReadError::TagReadingError(filename.to_string_lossy().to_string(), err)
    })?;

    Ok(tag)
}
