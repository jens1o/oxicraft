use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::io;

pub type MinecraftBoolean = bool;

impl Decodeable<MinecraftBoolean, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MinecraftBoolean, io::Error> {
        let value = self.pop_front();

        if value.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "A minecraft boolean consists of one byte, but there is none!",
            ));
        }

        let value = value.unwrap();

        if value == 0x01 {
            Ok(true)
        } else if value == 0x00 {
            Ok(false)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid boolean value {}!", value),
            ))
        }
    }
}

impl Encodeable for MinecraftBoolean {
    fn encode(&self) -> VecDeque<u8> {
        return VecDeque::from(vec![if *self { 0x01 } else { 0x00 }]);
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, Encodeable, MinecraftBoolean};
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(MinecraftBoolean, Vec<u8>)> =
            vec![(true, vec![0x01]), (false, vec![0x00])];

        for mapping in mappings {
            let actual: MinecraftBoolean = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let mappings: Vec<Vec<u8>> = vec![vec![0x42], vec![0x21], vec![]];

        for mapping in mappings {
            let actual: Result<MinecraftBoolean, io::Error> = VecDeque::from(mapping).decode();

            assert!(actual.is_err());
        }
    }

    #[test]
    fn test_encoding() {
        let mappings: Vec<(MinecraftBoolean, Vec<u8>)> =
            vec![(true, vec![0x01]), (false, vec![0x00])];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
