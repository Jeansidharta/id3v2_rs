#[derive(PartialEq, Clone, Default)]
pub struct FrameID([u8; 4]);

impl TryFrom<[u8; 4]> for FrameID {
    type Error = FrameID;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let id = FrameID(value);
        if id.is_valid() {
            Ok(id)
        } else {
            Err(id)
        }
    }
}

impl FrameID {
    pub fn bytes(&self) -> &[u8; 4] {
        &self.0
    }

    fn is_byte_valid_id(byte: u8) -> bool {
        (byte.is_ascii_alphabetic() && byte.is_ascii_uppercase()) || byte.is_ascii_digit()
    }

    pub fn is_valid(&self) -> bool {
        FrameID::is_byte_valid_id(self.0[0])
            && FrameID::is_byte_valid_id(self.0[1])
            && FrameID::is_byte_valid_id(self.0[2])
            && FrameID::is_byte_valid_id(self.0[3])
    }
}

impl std::fmt::Debug for FrameID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = String::from_utf8(self.0.to_vec()).unwrap();
        f.debug_tuple("FrameID").field(&string).finish()
    }
}

impl std::fmt::Display for FrameID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = String::from_utf8(self.0.to_vec()).unwrap();
        write!(f, "{}", string)
    }
}
