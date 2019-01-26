use super::varint::Varint;
use super::{Decodeable, Encodeable};
use crate::connection::ensure_data_size;
use std::char;
use std::collections::VecDeque;
use std::io;

pub type MinecraftString = String;

pub trait ReadString<E> {
    /// reads a string from the buffer, throwing an error if the length is over `max_size`
    fn read_string(&mut self, max_size: u16) -> Result<MinecraftString, E>;
}

impl ReadString<io::Error> for VecDeque<u8> {
    fn read_string(&mut self, max_size: u16) -> Result<MinecraftString, io::Error> {
        let length: Varint = self.decode()?;
        let mut length = length.0;
        ensure_data_size(length)?;

        if length > i32::from(max_size) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "String is too large.",
            ));
        }

        let mut result = String::with_capacity(length as usize);

        while length > 0 {
            result.push(
                char::from_u32(u32::from(self.pop_front().expect(
                    "Vec of bytes is too short to read the length that the string should be.",
                )))
                .expect("Invalid character in String"),
            );
            length -= 1;
        }

        Ok(result)
    }
}

impl Encodeable for String {
    fn encode(&self) -> VecDeque<u8> {
        let length_varint = Varint(self.len() as i32).encode();

        let mut result = VecDeque::with_capacity(self.len() + length_varint.len());

        result.extend(length_varint);

        self.chars().for_each(|x| result.push_back(x as u8));

        result
    }

    fn byte_length(&self) -> u8 {
        self.len() as u8 + 7 // we need to prefix the length with a varint, which is encoded in seven bytes
    }
}

impl Encodeable for str {
    fn encode(&self) -> VecDeque<u8> {
        let length_varint = Varint(self.len() as i32).encode();

        let mut result = VecDeque::with_capacity(self.len() + length_varint.len());

        result.extend(length_varint);

        self.chars().for_each(|x| result.push_back(x as u8));

        result
    }

    fn byte_length(&self) -> u8 {
        self.len() as u8 + 7 // we need to prefix the length with a varint, which is encoded in seven bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{Encodeable, ReadString};
    use std::collections::VecDeque;

    #[test]
    fn read_string() {
        let vector = vec![9, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert_eq!("localhost", VecDeque::from(vector).read_string(9).unwrap());
    }

    #[test]
    fn write_str() {
        let expected = vec![9, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert_eq!(VecDeque::from(expected), "localhost".encode());
    }

    #[test]
    fn write_string() {
        let expected = vec![9, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert_eq!(VecDeque::from(expected), "localhost".to_owned().encode());
    }

    #[test]
    fn read_string_should_err_if_too_long() {
        let vector = vec![111, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert!(VecDeque::from(vector).read_string(9).is_err());
    }
}
