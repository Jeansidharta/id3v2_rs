use std::io::Read;

use thiserror::Error;

use crate::utils::{read_syncsafe_integer, BitPosition};

#[derive(PartialEq, Clone, Debug)]
pub struct ExtendedHeader {
    extended_header_size: u32,
    number_of_flag_bytes: u8,
    flag_bytes: Vec<u8>,
    flag_data: Vec<ExtendedHeaderFlagData>,
}

#[derive(PartialEq, Clone, Debug, Error)]
pub enum ExtendedHeaderReadError {
    #[error("The byte stream ended before the minimum 6 bytes could be read")]
    NotEnoughBytes,
    #[error("The flag size byte provided for the CRC should be 5, but it was {0}")]
    InvalidTagCRCSize(u8),
    #[error("The flag size byte provided for the Tag Restrictions should be 1, but it was {0}")]
    InvalidFlagTagRestrictionSize(u8),
}

impl ExtendedHeader {
    pub fn bytes_size(&self) -> u32 {
        let minimum_bytes = 5u32;
        let flags_bytes: u32 = { self.flag_data.iter().map(|flag| flag.size as u32 + 1).sum() };

        minimum_bytes + self.number_of_flag_bytes as u32 + flags_bytes
    }

    pub fn read(reader: &mut impl Read) -> Result<ExtendedHeader, ExtendedHeaderReadError> {
        let mut buffer = [0u8; 5];
        reader
            .read_exact(&mut buffer)
            .map_err(|_| ExtendedHeaderReadError::NotEnoughBytes)?;
        let extended_header_size =
            read_syncsafe_integer([buffer[0], buffer[1], buffer[2], buffer[3]]);

        let number_of_flag_bytes = buffer[4];
        let mut flag_bytes = vec![];

        flag_bytes.resize(number_of_flag_bytes as usize, 0);

        reader
            .read_exact(&mut flag_bytes[..])
            .map_err(|_| ExtendedHeaderReadError::NotEnoughBytes)?;

        let flag_data = Self::parse_flags_data(&flag_bytes, reader)?;

        Ok(ExtendedHeader {
            extended_header_size,
            number_of_flag_bytes,
            flag_bytes,
            flag_data,
        })
    }

    fn parse_flags_data(
        flags_bytes: &Vec<u8>,
        reader: &mut impl Read,
    ) -> Result<Vec<ExtendedHeaderFlagData>, ExtendedHeaderReadError> {
        let mut flags_data: Vec<ExtendedHeaderFlagData> = vec![];

        let set_positions = flags_bytes
            .iter()
            .flat_map(|flags_byte| {
                BitPosition::iter_right()
                    .filter(|pos| pos.is_set_on(*flags_byte))
                    .map(|pos| (*flags_byte, pos))
            })
            .collect::<Vec<(u8, BitPosition)>>();

        for (byte_position, bit_position) in set_positions {
            let mut buffer = [0];
            reader
                .read_exact(&mut buffer)
                .map_err(|_| ExtendedHeaderReadError::NotEnoughBytes)?;

            let size = buffer[0];
            let mut data = vec![];
            data.resize(size as usize, 0);

            reader
                .read_exact(&mut data)
                .map_err(|_| ExtendedHeaderReadError::NotEnoughBytes)?;

            let typ = match (byte_position, &bit_position) {
                (0, BitPosition::LSBPlus6) => ExtendedHeaderFlagDataType::TagIsAnUpdate,
                (0, BitPosition::LSBPlus5) => ExtendedHeaderFlagDataType::CrcDataPresent,
                (0, BitPosition::LSBPlus4) => {
                    let data_byte = data.get(0).unwrap_or(&0);
                    let tag_size_restrictions_byte = (data_byte & 0b11000000) >> 6;
                    let text_encoding_restrictions_byte = (data_byte & 0b00100000) >> 5;
                    let text_field_size_restrictions_byte = (data_byte & 0b00011000) >> 3;
                    let image_encoding_restrictions_byte = (data_byte & 0b00000100) >> 2;
                    let image_size_restrictions_byte = data_byte & 0b00000011;
                    ExtendedHeaderFlagDataType::TagRestrictions(TagRestrictions {
                        tag_size_restrictions: {
                            match tag_size_restrictions_byte {
                                0b00 => TagSizeRestrictions::Max128Frames1MB,
                                0b01 => TagSizeRestrictions::Max64Frames128KB,
                                0b10 => TagSizeRestrictions::Max32Frames40KB,
                                0b11 => TagSizeRestrictions::Max32Frames4KB,
                                _ => unreachable!(),
                            }
                        },
                        text_encoding_restrictions: {
                            match text_encoding_restrictions_byte {
                                0b00 => TextEncodingRestrictions::NoRestrictions,
                                0b01 => TextEncodingRestrictions::ISO88591OrUTF8,
                                _ => unreachable!(),
                            }
                        },
                        text_field_size_restrictions: {
                            match text_field_size_restrictions_byte {
                                0b00 => TextFieldSizeRestrictions::NoRestrictions,
                                0b01 => TextFieldSizeRestrictions::Max1024Chars,
                                0b10 => TextFieldSizeRestrictions::Max128Chars,
                                0b11 => TextFieldSizeRestrictions::Max30Chars,
                                _ => unreachable!(),
                            }
                        },
                        image_encoding_restrictions: {
                            match image_encoding_restrictions_byte {
                                0b00 => ImageEncodingRestrictions::NoRestrictions,
                                0b01 => ImageEncodingRestrictions::PngOrJpeg,
                                _ => unreachable!(),
                            }
                        },
                        image_size_restrictions: {
                            match image_size_restrictions_byte {
                                0b00 => ImageSizeRestrictions::NoRestrictions,
                                0b01 => ImageSizeRestrictions::Max256x256Pixels,
                                0b10 => ImageSizeRestrictions::Max64x64Pixels,
                                0b11 => ImageSizeRestrictions::Exactly64x64Pixels,
                                _ => unreachable!(),
                            }
                        },
                    })
                }
                _ => ExtendedHeaderFlagDataType::Unknown(byte_position, bit_position),
            };

            flags_data.push(ExtendedHeaderFlagData { size, data, typ });
        }

        Ok(flags_data)
    }
}

impl Default for ExtendedHeader {
    fn default() -> Self {
        Self {
            extended_header_size: 5,
            number_of_flag_bytes: 0,
            flag_data: vec![],
            flag_bytes: vec![0],
        }
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum TagSizeRestrictions {
    #[default]
    Max128Frames1MB,
    Max64Frames128KB,
    Max32Frames40KB,
    Max32Frames4KB,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum TextEncodingRestrictions {
    #[default]
    NoRestrictions,
    ISO88591OrUTF8,
}
#[derive(PartialEq, Clone, Debug, Default)]
pub enum TextFieldSizeRestrictions {
    #[default]
    NoRestrictions,
    Max1024Chars,
    Max128Chars,
    Max30Chars,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum ImageEncodingRestrictions {
    #[default]
    NoRestrictions,
    PngOrJpeg,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum ImageSizeRestrictions {
    #[default]
    NoRestrictions,
    Max256x256Pixels,
    Max64x64Pixels,
    Exactly64x64Pixels,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct TagRestrictions {
    tag_size_restrictions: TagSizeRestrictions,
    text_encoding_restrictions: TextEncodingRestrictions,
    text_field_size_restrictions: TextFieldSizeRestrictions,
    image_encoding_restrictions: ImageEncodingRestrictions,
    image_size_restrictions: ImageSizeRestrictions,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExtendedHeaderFlagDataType {
    TagIsAnUpdate,
    CrcDataPresent,
    TagRestrictions(TagRestrictions),
    Unknown(u8, BitPosition),
}

#[derive(PartialEq, Clone, Debug)]
pub struct ExtendedHeaderFlagData {
    /// in bytes
    size: u8,
    data: Vec<u8>,
    typ: ExtendedHeaderFlagDataType,
}
