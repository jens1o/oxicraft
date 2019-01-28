use super::Encodeable;
use std::collections::VecDeque;

impl Encodeable for VecDeque<u8> {
    fn encode(&self) -> VecDeque<u8> {
        // TODO: Get rid of this memory clone
        (*self).clone()
    }

    fn byte_length(&self) -> u8 {
        self.len() as u8
    }
}
