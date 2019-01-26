use super::Encodeable;
use std::collections::VecDeque;

pub enum Difficulty {
    Peaceful = 0x00,
    Easy = 0x01,
    Normal = 0x02,
    Hard = 0x03,
}

impl Encodeable for Difficulty {
    fn encode(&self) -> VecDeque<u8> {
        VecDeque::from(vec![match self {
            Difficulty::Peaceful => 0x00,
            Difficulty::Easy => 0x01,
            Difficulty::Normal => 0x02,
            Difficulty::Hard => 0x03,
        }])
    }

    fn byte_length(&self) -> u8 {
        1
    }
}
