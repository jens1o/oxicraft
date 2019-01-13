use crate::connection::ensure_data_size;
use crate::varint::{ReadVarint, Varint};
use std::char;
use std::collections::VecDeque;
use std::io;

pub type MinecraftString = String;

pub trait ReadString<E> {
    fn read_string(&mut self, max_size: u16) -> Result<MinecraftString, E>;
}

pub trait ToString {
    fn to_string(&self) -> Vec<MinecraftString>;
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
