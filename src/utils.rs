pub fn read_syncsafe_integer(bytes: [u8; 4]) -> u32 {
    (bytes[3] as u32)
        | ((bytes[2] as u32) << 7)
        | ((bytes[1] as u32) << 14)
        | ((bytes[0] as u32) << 21)
}

pub fn read_syncsafe_integer_5bytes(bytes: [u8; 5]) -> u64 {
    (bytes[4] as u64)
        | ((bytes[3] as u64) << 7)
        | ((bytes[2] as u64) << 14)
        | ((bytes[1] as u64) << 21)
        | ((bytes[0] as u64) << 28)
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum BitPosition {
    LSB,
    LSBPlus1,
    LSBPlus2,
    LSBPlus3,
    LSBPlus4,
    LSBPlus5,
    LSBPlus6,
    LSBPlus7,
}

impl BitPosition {
    pub fn iter_right() -> std::array::IntoIter<BitPosition, 8> {
        [
            BitPosition::LSB,
            BitPosition::LSBPlus1,
            BitPosition::LSBPlus2,
            BitPosition::LSBPlus3,
            BitPosition::LSBPlus4,
            BitPosition::LSBPlus5,
            BitPosition::LSBPlus6,
            BitPosition::LSBPlus7,
        ]
        .into_iter()
    }

    pub fn is_set_on(&self, num: u8) -> bool {
        num & self.binary_representation() != 0
    }

    pub fn binary_representation(&self) -> u8 {
        match self {
            BitPosition::LSB => 0b00000001,
            BitPosition::LSBPlus1 => 0b00000010,
            BitPosition::LSBPlus2 => 0b00000100,
            BitPosition::LSBPlus3 => 0b00001000,
            BitPosition::LSBPlus4 => 0b00010000,
            BitPosition::LSBPlus5 => 0b00100000,
            BitPosition::LSBPlus6 => 0b01000000,
            BitPosition::LSBPlus7 => 0b10000000,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            BitPosition::LSB => BitPosition::LSBPlus7,
            BitPosition::LSBPlus1 => BitPosition::LSB,
            BitPosition::LSBPlus2 => BitPosition::LSBPlus1,
            BitPosition::LSBPlus3 => BitPosition::LSBPlus2,
            BitPosition::LSBPlus4 => BitPosition::LSBPlus3,
            BitPosition::LSBPlus5 => BitPosition::LSBPlus4,
            BitPosition::LSBPlus6 => BitPosition::LSBPlus5,
            BitPosition::LSBPlus7 => BitPosition::LSBPlus6,
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            BitPosition::LSB => BitPosition::LSBPlus1,
            BitPosition::LSBPlus1 => BitPosition::LSBPlus2,
            BitPosition::LSBPlus2 => BitPosition::LSBPlus3,
            BitPosition::LSBPlus3 => BitPosition::LSBPlus4,
            BitPosition::LSBPlus4 => BitPosition::LSBPlus5,
            BitPosition::LSBPlus5 => BitPosition::LSBPlus6,
            BitPosition::LSBPlus6 => BitPosition::LSBPlus7,
            BitPosition::LSBPlus7 => BitPosition::LSB,
        }
    }
}

pub fn latin1_to_string(latin1: &[u8]) -> String {
    latin1.iter().map(|byte| *byte as char).collect()
}

pub fn string_to_latin1(string: &str) -> Vec<u8> {
    string.chars().map(|char| char as u8).collect()
}
