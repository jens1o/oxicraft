use super::Encodeable;
use std::collections::VecDeque;

pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}

impl Encodeable for Dimension {
    fn encode(&self) -> VecDeque<u8> {
        let numeric_value = match self {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1,
        };

        // an int is 4 bytes long
        let mut result: VecDeque<u8> = VecDeque::with_capacity(4);

        let mut value = numeric_value;

        for _ in 1..=4 {
            let temp = value & 0b1111;

            value >>= 4;

            result.push_back(temp as u8);
        }

        result
    }

    fn byte_length(&self) -> u8 {
        4
    }
}
