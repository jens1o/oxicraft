use super::Encodeable;
use std::collections::VecDeque;
use std::fmt;

pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Default1_1,
}

impl fmt::Display for LevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_value = match self {
            LevelType::Default => "default",
            LevelType::Flat => "flat",
            LevelType::LargeBiomes => "largeBiomes",
            LevelType::Amplified => "amplified",
            LevelType::Default1_1 => "default_1_1",
        };

        write!(f, "{}", string_value)
    }
}

impl Encodeable for LevelType {
    fn encode(&self) -> VecDeque<u8> {
        self.to_string().encode()
    }

    fn byte_length(&self) -> u8 {
        11 + 7 // Max string size + Varint length of the string (prefixed when encoded)
    }
}
