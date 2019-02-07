use super::Encodeable;
use crate::difficulty::Difficulty;
use std::collections::VecDeque;

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
