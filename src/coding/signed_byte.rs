use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::io;

pub type MinecraftSignedByte = i8;

impl Decodeable<MinecraftSignedByte, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MinecraftSignedByte, io::Error> {
        let value = self.pop_front();

        if value.is_some() {
            let result = value.unwrap() as MinecraftSignedByte;
            Ok(result)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "A minecraft byte consists of one byte, but there is none!",
            ))
        }
    }
}

impl Encodeable for MinecraftSignedByte {
    fn encode(&self) -> VecDeque<u8> {
        VecDeque::from(vec![*self as u8])
    }

    fn byte_length(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, Encodeable, MinecraftSignedByte};
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(MinecraftSignedByte, Vec<u8>)> = vec![
            (0x01, vec![0x01]),
            (0x00, vec![0x00]),
            (-128, vec![0x80]),
            (127, vec![0x7F]),
        ];

        for mapping in mappings {
            let actual: MinecraftSignedByte = VecDeque::from(mapping.1).decode().unwrap();
            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let actual: Result<MinecraftSignedByte, io::Error> = VecDeque::from(vec![]).decode();

        assert!(actual.is_err());
    }

    #[test]
    fn test_encoding() {
        let mappings: Vec<(MinecraftSignedByte, Vec<u8>)> = vec![
            (0x01, vec![0x01]),
            (0x00, vec![0x00]),
            (-128, vec![0x80]),
            (127, vec![0x7F]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
