use std::collections::VecDeque;
use std::io::{self, Read};
use std::net::TcpStream;

pub type Varint = i32;

pub trait ReadVarint<E> {
    fn read_varint(&mut self) -> Result<Varint, E>;
}

pub trait ToVarint {
    fn to_varint(&self) -> Vec<Varint>;
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

            let value = byte & 0b01111111;
            result |= (value as i32) << (7 * num_of_reads);

            num_of_reads += 1;
            if num_of_reads > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarInt is too big",
                ));
            }

            if (byte & 0b10000000) == 0 {
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
            let value = byte & 0b01111111;
            result |= (value as i32) << (7 * num_of_reads);

            num_of_reads += 1;
            if num_of_reads > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "VarInt is too big",
                ));
            }

            if (byte & 0b10000000) == 0 {
                break;
            }
        }

        Ok(result)
    }
}

impl ToVarint for i32 {
    fn to_varint(&self) -> Vec<Varint> {
        let mut remaining = self.clone();
        let mut result = Vec::with_capacity(7);

        loop {
            let part = remaining & 0x7F; // First 7 bits
            remaining >>= 7;
            if remaining == 0 {
                result.push(part);
                break;
            } else {
                result.push(part | 0x80);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Varint;
    use crate::varint::ReadVarint;
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
}
