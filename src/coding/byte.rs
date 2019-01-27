use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::io;

pub type MinecraftUnsignedByte = u8;

impl Decodeable<MinecraftUnsignedByte, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MinecraftUnsignedByte, io::Error> {
        let value = self.pop_front();

        if value.is_some() {
            Ok(value.unwrap())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "A minecraft byte consists of one byte, but there is none!",
            ))
        }
    }
}

impl Encodeable for MinecraftUnsignedByte {
    fn encode(&self) -> VecDeque<u8> {
        VecDeque::from(vec![*self])
    }

    fn byte_length(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, Encodeable, MinecraftUnsignedByte};
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(MinecraftUnsignedByte, Vec<u8>)> =
            vec![(0x01, vec![0x01]), (0x00, vec![0x00])];

        for mapping in mappings {
            let actual: MinecraftUnsignedByte = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let actual: Result<MinecraftUnsignedByte, io::Error> = VecDeque::from(vec![]).decode();

        assert!(actual.is_err());
    }

    #[test]
    fn test_encoding() {
        let mappings: Vec<(MinecraftUnsignedByte, Vec<u8>)> =
            vec![(0x01, vec![0x01]), (0x00, vec![0x00])];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
