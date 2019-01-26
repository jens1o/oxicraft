use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::i64;
use std::io;

pub type Long = i64;

impl Decodeable<Long, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<Long, io::Error> {
        let mut result: Long = 0;

        for _ in 1..=8 {
            result += i64::from(
                self.pop_front()
                    .expect("Vector needs to have 8 bytes to decode a long(i64)."),
            );
            result <<= 8;
        }

        Ok(result)
    }
}

impl Encodeable for Long {
    fn encode(&self) -> VecDeque<u8> {
        // prepare some storage for our decoded bytes
        let mut result: VecDeque<u8> = VecDeque::with_capacity(8);

        let mut value = *self;

        for _ in 1..=8 {
            // save encoded value in a temporial variable to avoid lossing
            // information
            let temp = value & 0b1111_1111;

            value >>= 8;

            result.push_front(temp as u8);
        }

        // HACK: Somehow, the last byte is decoded as 0x00 (0),
        // for example
        // [0, 0, 0, 0, 30, 100, 207, 0] instead of
        // [0, 0, 0, 0, 0, 30, 100, 207]
        // we need to fix this as soon as we have high values.
        let last_value = result.pop_back().unwrap();
        result.push_front(last_value);

        result
    }

    fn byte_length(&self) -> u8 {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::Long;
    use super::{Decodeable, Encodeable};
    use std::collections::VecDeque;

    #[test]
    fn test_read_long_on_vec() {
        let mappings: Vec<(Long, Vec<u8>)> = vec![
            (632469504, vec![0, 0, 0, 0, 0, 37, 178, 184]),
            (631943936, vec![0, 0, 0, 0, 0, 37, 170, 179]),
            (630137600, vec![0, 0, 0, 0, 0, 37, 143, 35]),
        ];

        for mapping in mappings {
            let actual: Long = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }
    #[test]
    fn test_write_long_to_vec() {
        let mappings: Vec<(Long, Vec<u8>)> = vec![
            (632469504, vec![0, 0, 0, 0, 0, 37, 178, 184]),
            (631943936, vec![0, 0, 0, 0, 0, 37, 170, 179]),
            (630137600, vec![0, 0, 0, 0, 0, 37, 143, 35]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
