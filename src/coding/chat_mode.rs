use super::varint::Varint;
use super::Decodeable;
use crate::client_settings::ChatMode;
use std::collections::VecDeque;
use std::io;

impl Decodeable<ChatMode, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<ChatMode, io::Error> {
        let raw_value: Varint = self.decode()?;

        if raw_value == 0x00 {
            Ok(ChatMode::Enabled)
        } else if raw_value == 0x01 {
            Ok(ChatMode::CommandsOnly)
        } else if raw_value == 0x02 {
            Ok(ChatMode::Hidden)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The raw value for decoding the chat mode needs to be one, two or zero!",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Decodeable;
    use crate::client_settings::ChatMode;
    use std::collections::VecDeque;
    use std::io;

    #[test]
    fn test_decoding() {
        let mappings: Vec<(ChatMode, Vec<u8>)> = vec![
            (ChatMode::Enabled, vec![0x00]),
            (ChatMode::CommandsOnly, vec![0x01, 0x02]),
            (ChatMode::Hidden, vec![0x02]),
        ];

        for mapping in mappings {
            let actual: ChatMode = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }

    #[test]
    fn test_decoding_err() {
        let mappings: Vec<Vec<u8>> = vec![vec![0x42], vec![0x07, 0x03]];

        for mapping in mappings {
            let actual: Result<ChatMode, io::Error> = dbg!(VecDeque::from(mapping).decode());

            assert!(actual.is_err());

            let err = actual.unwrap_err();

            assert_eq!(err.kind(), io::ErrorKind::InvalidData);
            assert_eq!(
                err.to_string(),
                "The raw value for decoding the chat mode needs to be one, two or zero!"
            );
        }
    }

    #[test]
    fn test_decoding_err_empty() {
        let actual: Result<ChatMode, io::Error> = VecDeque::from(vec![]).decode();

        assert!(actual.is_err());
        let err = actual.unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "Not enough bytes to decode a Varint!");
    }
}
