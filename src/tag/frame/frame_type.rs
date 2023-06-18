use std::rc::Rc;

use thiserror::Error;

use crate::{
    tag::encoding::{Encoding, EncodingError},
    utils::latin1_to_string,
};

use super::Frame;

#[derive(PartialEq, Clone, Debug, Error)]
pub enum TextInformationError {
    #[error("Could not read enough bytes to parse the data")]
    MissingData,
    #[error("Encoding Error: {0}")]
    EncodingError(EncodingError),
}

#[derive(PartialEq, Clone, Debug)]
pub struct TextInformation {
    encoding: Encoding,
    strings: Vec<String>,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum FrameType {
    #[default]
    Unknown,
    UniqueFileIdentifier {
        owner_identifier: String,
        identifier: Rc<[u8]>,
    },
    TextInformation(Result<TextInformation, TextInformationError>),
    Experimental,
}

impl From<Frame> for FrameType {
    fn from(value: Frame) -> Self {
        let id = value.frame_id.bytes();
        let data = value.data;

        match id {
            b"UFID" => {
                let position = data.iter().position(|e| *e == 0).unwrap_or(data.len());
                let owner_identifier = latin1_to_string(&data[..position]);
                let identifier = Rc::from(&data[position + 1..]);
                FrameType::UniqueFileIdentifier {
                    owner_identifier,
                    identifier,
                }
            }
            _ if id[0] == b'X' || id[0] == b'Y' || id[0] == b'Z' => FrameType::Experimental,
            _ if id[1] == b'T' => FrameType::TextInformation('struct_result: {
                let encoding = match Encoding::extract_from_vec(&data) {
                    Ok(encoding) => encoding,
                    Err(err) => break 'struct_result Err(TextInformationError::EncodingError(err)),
                };
                let data = &data[encoding.bytes_length()..];
                let splits = encoding.split_bytes_by_string_separator(data);
                let strings = splits
                    .into_iter()
                    .map(|split| encoding.make_string(split))
                    .collect();

                Ok(TextInformation { encoding, strings })
            }),
            _ => FrameType::Unknown,
        }
    }
}

impl FrameType {}
