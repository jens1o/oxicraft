use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Read};
use std::net::TcpStream;
use std::{i32, u8};

#[derive(PartialEq)]
pub struct Varint(pub i32);

impl fmt::Debug for Varint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the output in big hexadecimal notation
        write!(f, "Varint({:#X})", self.0)
    }
}

impl fmt::Display for Varint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<Varint> for i32 {
    fn eq(&self, other: &Varint) -> bool {
        *self == other.0
    }
}
impl PartialEq<i32> for Varint {
    fn eq(&self, other: &i32) -> bool {
        (*self).0 == *other
    }
}

impl Decodeable<Varint, io::Error> for TcpStream {
    fn decode(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_of_reads: u8 = 0;
        let mut result: i32 = 0;

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

        Ok(Varint(result))
    }
}

impl Decodeable<Varint, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_of_reads: u8 = 0;
        let mut result: i32 = 0;

        #[inline(always)]
        fn get_byte_or_fail(vector: &mut VecDeque<u8>) -> Result<u8, io::Error> {
            let value = vector.pop_front();

            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not enough bytes to decode a Varint!",
                ))
            }
        };

        loop {
            let byte = get_byte_or_fail(self)?;
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

        Ok(Varint(result))
    }
}

impl Encodeable for Varint {
    fn encode(&self) -> VecDeque<u8> {
        let mut result = VecDeque::with_capacity(7);

        let mut value = (*self).0 as u32;

        loop {
            let mut temp = value & 0b0111_1111;
            value >>= 7;
            if value != 0 {
                temp |= 0b1000_0000;
            }

            result.push_back(temp as u8);

            if value == 0 {
                break;
            }
        }

        result
    }

    fn byte_length(&self) -> u8 {
        7
    }
}

impl Encodeable for usize {
    fn encode(&self) -> VecDeque<u8> {
        let mut result = VecDeque::with_capacity(7);

        let mut value = *self;

        loop {
            let mut temp = value & 0b0111_1111;
            value >>= 7;
            if value != 0 {
                temp |= 0b1000_0000;
            }

            result.push_front(temp as u8);

            if value == 0 {
                break;
            }
        }

        result
    }

    fn byte_length(&self) -> u8 {
        7
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, Encodeable, Varint};
    use std::collections::VecDeque;
    use std::i32;
    use std::io;

    #[test]
    fn test_read_varint_from_vec() {
        let mappings: Vec<(i32, Vec<u8>)> = vec![
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
            let expected: Varint = VecDeque::from(mapping.1).decode().unwrap();
            assert_eq!(mapping.0, expected);
        }
    }

    #[test]
    fn test_read_err() {
        let mappings: Vec<Vec<u8>> = vec![vec![]];

        for mapping in mappings {
            let expected: Result<Varint, io::Error> = VecDeque::from(mapping).decode();
            assert!(expected.is_err());

            let err = expected.unwrap_err();

            assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
            assert_eq!(err.to_string(), "Not enough bytes to decode a Varint!");
        }
    }

    #[test]
    fn test_write_vec_from_varint_negative() {
        let mappings: Vec<(i32, Vec<u8>)> = vec![
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), Varint(mapping.0).encode());
        }
    }

    #[test]
    fn test_write_vec_from_varint_positive() {
        let mappings: Vec<(i32, Vec<u8>)> = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), Varint(mapping.0).encode());
        }
    }

    #[test]
    fn test_varint_and_i32_are_same() {
        for i in (i32::min_value()..=i32::max_value()).take(1000) {
            assert_eq!(i, Varint(i));
        }
    }
}
