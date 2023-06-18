use std::{fs::File, io::ErrorKind, path::PathBuf};
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

pub fn find_substring(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.len() == 0 {
        return None;
    }

    let mut iter = bytes.iter();
    while let Some(byte_position) = iter.position(|b| *b == pattern[0]) {
        let max_position = std::cmp::min(pattern.len() + byte_position, bytes.len());
        let bytes = &(bytes[byte_position..max_position]);
        dbg!(bytes);
        if bytes == pattern {
            return Some(byte_position);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn make_bytes(string: &str) -> Vec<u8> {
        string.bytes().collect::<Vec<u8>>()
    }

    #[test]
    fn find_substring_simple() {
        let bytes = &make_bytes("hello, world!")[..];
        let pattern = &make_bytes("o, w")[..];

        assert_eq!(find_substring(bytes, pattern), Some(4))
    }
}
