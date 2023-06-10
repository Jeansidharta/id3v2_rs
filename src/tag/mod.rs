use std::io::Read;

use thiserror::Error;

use crate::tag::header::HeaderFlagType;

use self::{
    extended_header::{ExtendedHeader, ExtendedHeaderReadError},
    frame::{Frame, FrameReadError},
    header::{Header, HeaderReadError},
};

mod encoding;
mod extended_header;
mod frame;
mod header;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Tag {
    header: Header,
    extended_header: Option<ExtendedHeader>,
    frames: Vec<Frame>,
    footer: Option<Box<Tag>>,
}

#[derive(PartialEq, Clone, Debug, Error)]
pub enum TagReadError {
    #[error("Error while parsing header: {0}")]
    HeaderError(HeaderReadError),
    #[error("Error while parsing extended header: {0}")]
    ExtendedHeaderError(ExtendedHeaderReadError),
    #[error("Error while parsing frame {0}: {1}")]
    FrameReadError(u32, FrameReadError),
}

impl From<HeaderReadError> for TagReadError {
    fn from(value: HeaderReadError) -> Self {
        Self::HeaderError(value)
    }
}

impl From<ExtendedHeaderReadError> for TagReadError {
    fn from(value: ExtendedHeaderReadError) -> Self {
        Self::ExtendedHeaderError(value)
    }
}

impl Tag {
    pub fn read(reader: &mut impl Read) -> Result<Tag, TagReadError> {
        let header = Header::read(reader)?;

        let extended_header = if header.is_flag_set(&HeaderFlagType::ExtendedHeader) {
            Some(ExtendedHeader::read(reader)?)
        } else {
            None
        };

        let footer = if header.is_flag_set(&HeaderFlagType::ExtendedHeader) {
            todo!();
            #[allow(unreachable_code)]
            Some(Box::from(Tag::read(reader)?))
        } else {
            None
        };

        let mut frames: Vec<Frame> = vec![];
        let mut bytes_read = header.bytes_size()
            + extended_header
                .as_ref()
                .map(|xheader| xheader.bytes_size())
                .unwrap_or(0);

        while bytes_read < header.tag_size() {
            let new_frame = match Frame::read(reader) {
                Err(FrameReadError::InvalidFrameID(_)) => break,
                Err(frame_read_error) => {
                    return Err(TagReadError::FrameReadError(
                        frames.len() as u32 + 1,
                        frame_read_error,
                    ));
                }
                Ok(frame) => frame,
            };

            bytes_read += new_frame.bytes_size();
            frames.push(new_frame);
        }

        Ok(Tag {
            header,
            extended_header,
            footer,
            frames,
        })
    }
}
