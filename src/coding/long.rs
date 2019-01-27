use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::i64;
use std::io;

pub type Long = i64;

impl Decodeable<Long, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<Long, io::Error> {
        let mut temp: u64 = 0;

        #[inline(always)]
        fn get_byte_or_fail(vector: &mut VecDeque<u8>) -> Result<u8, io::Error> {
            let value = vector.pop_front();

            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not enough bytes to decode a long(i64)!",
                ))
            }
        };

        for _ in 1..=7 {
            let byte = get_byte_or_fail(self)? as u64;
            temp += byte;
            temp <<= 8;
        }

        // add remaining byte without the shift
        let byte = get_byte_or_fail(self)? as u64;
        temp += byte;

        let msb = temp >> 63;

        let mut result: Long = temp as Long;
        if msb == 0b1 {
            result = -(!temp as Long);
        }

        Ok(result)
    }
}

impl Encodeable for Long {
    fn encode(&self) -> VecDeque<u8> {
        let mut result: VecDeque<u8> = VecDeque::with_capacity(8);
        //max long value: +/- 9223372036854775808
        let mut value = (i64::abs(*self)) as u64;

        if i64::is_negative(*self) {
            value = !value | (0b1 << 63)
        }

        for _ in 1..=7 {
            let byte = (value & 0b11111111) as u8;
            value >>= 8;
            result.push_front(byte);
        }

        // add remaining byte without shifting
        let byte = (value & 0b11111111) as u8;
        result.push_front(byte);

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
            (0, vec![0, 0, 0, 0, 0, 0, 0, 0]),
            (1, vec![0, 0, 0, 0, 0, 0, 0, 1]),
            (-1, vec![255, 255, 255, 255, 255, 255, 255, 254]),
            (255, vec![0, 0, 0, 0, 0, 0, 0, 255]),
            (-255, vec![255, 255, 255, 255, 255, 255, 255, 0]),
            (
                0x7FFFFFFFFFFFFFFF,
                vec![127, 255, 255, 255, 255, 255, 255, 255],
            ),
            (-0x7FFFFFFFFFFFFFFF, vec![128, 0, 0, 0, 0, 0, 0, 0]),
        ];

        for mapping in mappings {
            let actual: Long = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }
    #[test]
    fn test_write_long_to_vec() {
        let mappings: Vec<(Long, Vec<u8>)> = vec![
            (0, vec![0, 0, 0, 0, 0, 0, 0, 0]),
            (1, vec![0, 0, 0, 0, 0, 0, 0, 1]),
            (-1, vec![255, 255, 255, 255, 255, 255, 255, 254]),
            (255, vec![0, 0, 0, 0, 0, 0, 0, 255]),
            (-255, vec![255, 255, 255, 255, 255, 255, 255, 0]),
            (
                0x7FFFFFFFFFFFFFFF,
                vec![127, 255, 255, 255, 255, 255, 255, 255],
            ),
            (-0x7FFFFFFFFFFFFFFF, vec![128, 0, 0, 0, 0, 0, 0, 0]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
