use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::f64;
use std::io;

pub type MinecraftDouble = f64;

impl Decodeable<MinecraftDouble, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<MinecraftDouble, io::Error> {
        let mut temp: u64 = 0;

        #[inline(always)]
        fn get_byte_or_fail(vector: &mut VecDeque<u8>) -> Result<u8, io::Error> {
            let value = vector.pop_front();

            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not enough bytes to decode a MinecraftDouble(f64)!",
                ))
            }
        };

        for _ in 1..=3 {
            let byte = u64::from(get_byte_or_fail(self)?);
            temp += byte;
            temp <<= 8;
        }

        // add remaining byte without the shift
        let byte = u64::from(get_byte_or_fail(self)?);
        temp += byte;

        let result: MinecraftDouble = f64::from_bits(temp);
        Ok(result)
    }
}

impl Encodeable for MinecraftDouble {
    fn encode(&self) -> VecDeque<u8> {
        let mut result: VecDeque<u8> = VecDeque::with_capacity(8);
        let mut value = f64::to_bits(*self);

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

// TODO: Add tests with valid values.
