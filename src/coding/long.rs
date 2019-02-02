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

        let result: Long = temp as Long;
        Ok(result)
    }
}

impl Encodeable for Long {
    fn encode(&self) -> VecDeque<u8> {
        let mut result: VecDeque<u8> = VecDeque::with_capacity(8);
        // max long value: -9223372036854775808 / +9223372036854775807
        let mut value = *self as u64;

        for _ in 1..=7 {
            let byte = (value & 0b1111_1111) as u8;
            value >>= 8;
            result.push_front(byte);
        }

        // add remaining byte without shifting
        let byte = (value & 0b1111_1111) as u8;
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
            (-1, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
            (127, vec![0, 0, 0, 0, 0, 0, 0, 127]),
            (-127, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x81]),
            (-0x8000000000000000, vec![0x80, 0, 0, 0, 0, 0, 0, 0]), // lowest possible value
            (
                0x7FFFFFFFFFFFFFFF,
                vec![0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            ), // highest possible value
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
            (-1, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
            (127, vec![0, 0, 0, 0, 0, 0, 0, 127]),
            (-127, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x81]),
            (-0x8000000000000000, vec![0x80, 0, 0, 0, 0, 0, 0, 0]), // lowest possible value
            (
                0x7FFFFFFFFFFFFFFF,
                vec![0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            ), // highest possible value
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }

    use test::{black_box, Bencher};
    #[bench]
    fn bench_decoding(b: &mut Bencher) {
        b.iter(|| {
            let input: Vec<u8> = vec![128, 0, 0, 0, 0, 0, 0, 0];
            black_box::<Long>(VecDeque::from(input).decode().unwrap());
        });
    }
    #[bench]
    fn bench_encoding(b: &mut Bencher) {
        b.iter(|| {
            let input: Long = -0x7FFFFFFFFFFFFFFF;
            black_box::<VecDeque<u8>>(input.encode());
        });
    }
}
