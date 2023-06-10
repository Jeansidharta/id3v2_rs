use thiserror::Error;

use crate::utils::latin1_to_string;

#[derive(PartialEq, Clone, Debug)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Encoding {
    Latin1,
    UTF16(ByteOrder),
    UTF16BE,
    UTF8,
}

#[derive(PartialEq, Clone, Debug, Error)]
pub enum EncodingError {
    #[error("The given encoding byte {0} is unknown. It should either be 00, 01, 02 or 03")]
    UnknownEncoding(u8),
    #[error("Could not read enough bytes to parse the encoding")]
    MissingEncoding,
    #[error("Could not read enough bytes to parse the encoding Byte Order Mark")]
    MissingBOM,
    #[error("Invalid Byte Order Mark")]
    InvalidBOM(u8, u8),
}

fn find_substring(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    let pattern_match_index = 0;
    if pattern.len() == 0 {
        
    }
    for byte in bytes.iter() {}
    todo!()
}

impl Encoding {
    pub fn extract_from_vec(vector: &Vec<u8>) -> Result<Self, EncodingError> {
        let Some(encoding_byte) = vector.get(0) else {
            return Err(EncodingError::MissingEncoding);
        };
        match encoding_byte {
            0 => Ok(Encoding::Latin1),
            1 => {
                let (Some(a), Some(b)) = (vector.get(1), vector.get(2)) else {
                    return Err(EncodingError::MissingBOM);
                };
                match (a, b) {
                    (0xFE, 0xFF) => Ok(Encoding::UTF16(ByteOrder::BigEndian)),
                    (0xFF, 0xFE) => Ok(Encoding::UTF16(ByteOrder::LittleEndian)),
                    _ => Err(EncodingError::InvalidBOM(*a, *b)),
                }
            }
            2 => Ok(Encoding::UTF16BE),
            3 => Ok(Encoding::UTF8),
            _ => Err(EncodingError::UnknownEncoding(*encoding_byte)),
        }
    }

    pub fn string_separator(&self) -> &[u8] {
        match self {
            Encoding::Latin1 => &[0],
            Encoding::UTF16(_) => &[0, 0],
            Encoding::UTF16BE => &[0, 0],
            Encoding::UTF8 => &[0],
        }
    }

    pub fn split_bytes_by_string_separator<'a>(&self, bytes: &'a [u8]) -> Vec<&'a [u8]> {
        let mut bytes = bytes;
        let separator = self.string_separator();
        let mut vector = vec![];
        while let Some(index) = bytes.find_substring(separator) {
            vector.push(&bytes[..index]);
            bytes = &bytes[index + separator.len()..];
        }
        vector
    }

    pub fn make_string(&self, bytes: &[u8]) -> String {
        match self {
            Encoding::Latin1 => latin1_to_string(bytes),
            Encoding::UTF16(byte_order) => String::from_utf16_lossy(
                &bytes
                    .windows(2)
                    .map(|bytes| {
                        let b1 = bytes[0];
                        let b2 = bytes[1];
                        match byte_order {
                            ByteOrder::BigEndian => ((b1 as u16) << 8) | (b2 as u16),
                            ByteOrder::LittleEndian => ((b2 as u16) << 8) | (b1 as u16),
                        }
                    })
                    .collect::<Vec<u16>>()[..],
            ),
            Encoding::UTF16BE => String::from_utf16_lossy(
                &bytes
                    .windows(2)
                    .map(|bytes| {
                        let b1 = bytes[0];
                        let b2 = bytes[1];
                        ((b1 as u16) << 8) | (b2 as u16)
                    })
                    .collect::<Vec<u16>>()[..],
            ),
            Encoding::UTF8 => String::from_utf8_lossy(bytes).to_string(),
        }
    }

    fn bytes_length(&self) -> usize {
        match self {
            Encoding::Latin1 => 1,
            Encoding::UTF16(_) => 2,
            Encoding::UTF16BE => 1,
            Encoding::UTF8 => 1,
        }
    }
}
