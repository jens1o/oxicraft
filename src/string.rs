use crate::connection::ensure_data_size;
use crate::varint::{ReadVarint, Varint};
use std::char;
use std::collections::VecDeque;
use std::io;

pub type MinecraftString = String;

pub trait ReadString<E> {
    /// reads a string from the buffer, throwing an error if the length is over `max_size`
    fn read_string(&mut self, max_size: u16) -> Result<MinecraftString, E>;
}

pub trait ToString {
    fn to_string(&self) -> Vec<u8>;
}

impl ReadString<io::Error> for VecDeque<u8> {
    fn read_string(&mut self, max_size: u16) -> Result<MinecraftString, io::Error> {
        let mut length = self.read_varint()?;
        ensure_data_size(length)?;

        if length > max_size as Varint {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "String is too large.",
            ));
        }

        let mut result = String::with_capacity(length as usize);

        while length > 0 {
            result.push(
                char::from_u32(self.pop_front().expect(
                    "Vec of bytes is too short to read the length that the string should be.",
                ) as u32)
                .expect("Invalid character in String"),
            );
            length -= 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::ReadString;
    use std::collections::VecDeque;

    #[test]
    fn read_string() {
        let vector = vec![9, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert_eq!("localhost", VecDeque::from(vector).read_string(9).unwrap());
    }

    #[test]
    fn read_string_should_err_if_too_long() {
        let vector = vec![111, 108, 111, 99, 97, 108, 104, 111, 115, 116];
        assert!(VecDeque::from(vector).read_string(9).is_err());
    }
}
