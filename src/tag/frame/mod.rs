use std::{io::Read, rc::Rc};

use thiserror::Error;

use crate::utils::read_syncsafe_integer;

use self::{frame_id::FrameID, frame_type::FrameType};

mod frame_id;
mod frame_type;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Frame {
    frame_type: FrameType,
    frame_id: FrameID,
    frame_size: u32,
    flags_byte: (u8, u8),
    data: Rc<Vec<u8>>,
}

#[derive(PartialEq, Clone, Debug, Error)]
pub enum FrameReadError {
    #[error("While reading the tag header, no ID3 character sequence was found")]
    ID3NotFound,
    #[error("The byte stream ended before the minimum 10 bytes could be read")]
    NotEnoughBytes,
    #[error("The frame id {0} must only have capital letters or numbers")]
    InvalidFrameID(FrameID),
}

impl Frame {
    pub fn bytes_size(&self) -> u32 {
        return self.frame_size + 10;
    }

    fn read_id(reader: &mut impl Read) -> Result<FrameID, FrameReadError> {
        let mut frame_id = [0u8; 4];

        reader
            .read_exact(&mut frame_id)
            .map_err(|_| FrameReadError::NotEnoughBytes)?;

        FrameID::try_from(frame_id).map_err(|err| FrameReadError::InvalidFrameID(err))
    }

    pub fn read(reader: &mut impl Read) -> Result<Frame, FrameReadError> {
        let frame_id = Frame::read_id(reader)?;

        let mut buffer = [0u8; 6];

        reader
            .read_exact(&mut buffer)
            .map_err(|_| FrameReadError::NotEnoughBytes)?;

        let frame_size = read_syncsafe_integer([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let flags_byte = (buffer[4], buffer[5]);

        let mut data = Vec::new();
        data.resize(frame_size as usize, 0);

        reader
            .read_exact(&mut data[..])
            .map_err(|_| FrameReadError::NotEnoughBytes)?;

        Ok(Frame {
            frame_type: FrameType::Unknown,
            frame_id,
            frame_size,
            flags_byte,
            data: Rc::new(data),
        })
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct FrameFlags {}
