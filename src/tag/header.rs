use std::io::Read;

use thiserror::Error;

use crate::utils::read_syncsafe_integer;

#[derive(PartialEq, Clone, Debug)]
pub enum HeaderFlagType {
    Unsynchronisation = 0b1000000,
    ExtendedHeader = 0b0100000,
    ExperimentalIndicator = 0b0010000,
    FooterPresent = 0b0001000,
}

impl HeaderFlagType {
    pub fn binary_representation(&self) -> u8 {
        match self {
            Self::Unsynchronisation => 0b1000000,
            Self::ExtendedHeader => 0b0100000,
            Self::ExperimentalIndicator => 0b0010000,
            Self::FooterPresent => 0b0001000,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Header {
    version: u16,
    flags_byte: u8,
    tag_size: u32,
}

#[derive(PartialEq, Clone, Debug, Error)]
pub enum HeaderReadError {
    #[error("While reading the tag header, no ID3 character sequence was found")]
    ID3NotFound,
    #[error("The byte stream ended before the minimum 10 bytes could be read")]
    NotEnoughBytes,
}

impl Header {
    pub fn bytes_size(&self) -> u32 {
        10
    }

    pub fn tag_size(&self) -> u32 {
        self.tag_size
    }

    pub fn is_flag_set(&self, flag_type: &HeaderFlagType) -> bool {
        self.flags_byte & flag_type.binary_representation() != 0
    }

    pub fn read(reader: &mut impl Read) -> Result<Header, HeaderReadError> {
        let mut buffer = [0u8; 10];
        reader
            .read_exact(&mut buffer)
            .map_err(|_| HeaderReadError::NotEnoughBytes)?;

        if &buffer[0..3] != b"ID3" {
            return Err(HeaderReadError::ID3NotFound);
        }

        let version = ((buffer[4] as u16) << 8) | buffer[3] as u16;

        let tag_size = read_syncsafe_integer([buffer[6], buffer[7], buffer[8], buffer[9]]);

        Ok(Header {
            version,
            flags_byte: buffer[5],
            tag_size,
        })
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: 4,
            flags_byte: 0,
            tag_size: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_header() {
        let bytes: [u8; 10] = [
            b'I', b'D', b'3', 4, 0, 0b01010000, 0, 0, 0b00000001, 0b01111111,
        ];
        let header = Header::read(&mut Cursor::new(bytes));
        assert_eq!(
            header,
            Ok(Header {
                version: 4,
                flags_byte: 0b01010000,
                tag_size: 255
            })
        );
        let header = header.unwrap();
        assert!(!header.is_flag_set(&HeaderFlagType::Unsynchronisation));
        assert!(header.is_flag_set(&HeaderFlagType::FooterPresent));
        assert!(header.is_flag_set(&HeaderFlagType::ExtendedHeader));
        assert!(!header.is_flag_set(&HeaderFlagType::ExperimentalIndicator));
    }
}
