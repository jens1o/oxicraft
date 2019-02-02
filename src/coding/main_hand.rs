use super::varint::Varint;
use super::Decodeable;
use crate::client_settings::MainHand;
use std::collections::VecDeque;
use std::io;

impl Decodeable<MainHand, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MainHand, io::Error> {
        let raw_value: Varint = self.decode()?;

        if raw_value == 0x00 {
            Ok(MainHand::Left)
        } else if raw_value == 0x01 {
            Ok(MainHand::Right)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "The raw value for decoding the main hand needs to be one or zero, {:?} given!",
                    raw_value
                ),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Decodeable;
    use crate::client_settings::MainHand;
    use crate::coding::varint::Varint;
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(MainHand, Vec<u8>)> =
            vec![(MainHand::Left, vec![0x00]), (MainHand::Right, vec![0x01])];

        for mapping in mappings {
            let actual: MainHand = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let mappings: Vec<Vec<u8>> = vec![vec![0x42], vec![0x05]];

        for mapping in mappings {
            let error_message = format!(
                "The raw value for decoding the main hand needs to be one or zero, {:?} given!",
                Varint(*mapping.first().unwrap() as i32)
            );

            let actual: Result<MainHand, io::Error> = VecDeque::from(mapping).decode();

            assert!(actual.is_err());

            let err = actual.unwrap_err();

            assert_eq!(err.kind(), io::ErrorKind::InvalidData);
            assert_eq!(err.to_string(), error_message);
        }
    }

    #[test]
    fn test_decoding_err_empty() {
        let actual: Result<MainHand, io::Error> = VecDeque::from(vec![]).decode();

        assert!(actual.is_err());

        let err = actual.unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "Not enough bytes to decode a Varint!");
    }
}
