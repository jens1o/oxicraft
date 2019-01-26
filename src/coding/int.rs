use super::Encodeable;
use std::collections::VecDeque;

pub type Int = i32;

impl Encodeable for Int {
    fn encode(&self) -> VecDeque<u8> {
        // an int is 4 bytes long
        let mut result: VecDeque<u8> = VecDeque::with_capacity(4);

        let mut value = *self;

        for _ in 1..=4 {
            let temp = value & 0b1111;

            value = value >> 4;

            result.push_back(temp as u8);
        }

        result
    }
}
