use std::collections::VecDeque;
use std::io::{self, Read};
use std::net::TcpStream;
use std::{i32, u8};

pub type Varint = i32;

pub trait ReadVarint<E> {
    fn read_varint(&mut self) -> Result<Varint, E>;
}

pub trait WriteVarint {
    fn write_varint(&self) -> Vec<u8>;
}

impl ReadVarint<io::Error> for TcpStream {
    fn read_varint(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_of_reads: u8 = 0;
        let mut result: Varint = 0;

        loop {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer)?;
            let byte = buffer[0];

            let value = byte & 0b0111_1111;
            result |= i32::from(value) << (7 * num_of_reads);

            num_of_reads += 1;
            if num_of_reads > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarInt is too big",
                ));
            }

            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }

        Ok(result)
    }
}

impl ReadVarint<io::Error> for VecDeque<u8> {
    fn read_varint(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_of_reads: u8 = 0;
        let mut result: Varint = 0;

        loop {
            let byte = self.pop_front().unwrap();
            let value = byte & 0b0111_1111;
            result |= i32::from(value) << (7 * num_of_reads);

            num_of_reads += 1;
            if num_of_reads > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarInt is too big",
                ));
            }

            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }

        Ok(result)
    }
}

impl WriteVarint for i32 {
    fn write_varint(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(7);

        let mut value = *self as u32;

        loop {
            let mut temp = value & 0b01111111;
            value = value >> 7;
            if value != 0 {
                temp |= 0b10000000;
            }

            result.push(temp as u8);

            if value == 0 {
                break;
            }
        }

        result
    }
}

impl WriteVarint for usize {
    fn write_varint(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(7);

        let mut value = *self;

        let mut num_iterations = 0;

        loop {
            let mut temp = value & 0b01111111;
            value = value >> 7;
            if value != 0 {
                temp |= 0b10000000;
            }

            result.push(temp as u8);

            num_iterations += 1;
            if num_iterations > 7 {
                panic!("Too many iterations!");
            }

            if value == 0 {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Varint;
    use crate::varint::{ReadVarint, WriteVarint};
    use std::collections::VecDeque;

    #[test]
    fn test_read_varint_from_vec() {
        let mappings: Vec<(Varint, Vec<u8>)> = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for mapping in mappings {
            assert_eq!(mapping.0, VecDeque::from(mapping.1).read_varint().unwrap());
        }
    }

    #[test]
    fn test_write_vec_from_varint_negative() {
        let mappings: Vec<(Varint, Vec<u8>)> = vec![
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for mapping in mappings {
            assert_eq!(mapping.1, mapping.0.write_varint());
        }
    }

    #[test]
    fn test_write_vec_from_varint_positive() {
        let mappings: Vec<(Varint, Vec<u8>)> = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        ];

        for mapping in mappings {
            assert_eq!(mapping.1, mapping.0.write_varint());
        }
    }

    #[test]
    fn test_write_vec_from_usize() {
        let mappings: Vec<(usize, Vec<u8>)> = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        ];

        for mapping in mappings {
            assert_eq!(mapping.1, mapping.0.write_varint());
        }
    }
}
