use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::io;

pub type MinecraftByte = u8;

impl Decodeable<MinecraftByte, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MinecraftByte, io::Error> {
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

impl Encodeable for MinecraftByte {
    fn encode(&self) -> VecDeque<u8> {
        return VecDeque::from(vec![*self]);
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, Encodeable, MinecraftByte};
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(MinecraftByte, Vec<u8>)> = vec![(0x01, vec![0x01]), (0x00, vec![0x00])];

        for mapping in mappings {
            let actual: MinecraftByte = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let actual: Result<MinecraftByte, io::Error> = VecDeque::from(vec![]).decode();

        assert!(actual.is_err());
    }

    #[test]
    fn test_encoding() {
        let mappings: Vec<(MinecraftByte, Vec<u8>)> = vec![(0x01, vec![0x01]), (0x00, vec![0x00])];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
