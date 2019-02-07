use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::io;

impl Encodeable for VecDeque<u8> {
    fn encode(&self) -> VecDeque<u8> {
        // TODO: Get rid of this memory clone
        (*self).clone()
    }

    fn byte_length(&self) -> u8 {
        self.len() as u8
    }
}

impl Decodeable<VecDeque<u8>, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<VecDeque<u8>, io::Error> {
        Ok(self.iter().cloned().collect::<VecDeque<u8>>())
    }
}
