use super::{Decodeable, Encodeable};
use std::collections::VecDeque;
use std::f32;
use std::io;

pub type Float = f32;

impl Decodeable<Float, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<Float, io::Error> {
        let mut temp: u32 = 0;

        #[inline(always)]
        fn get_byte_or_fail(vector: &mut VecDeque<u8>) -> Result<u8, io::Error> {
            let value = vector.pop_front();

            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not enough bytes to decode a float(f32)!",
                ))
            }
        };

        for _ in 1..=3 {
            let byte = get_byte_or_fail(self)? as u32;
            temp += byte;
            temp <<= 8;
        }

        // add remaining byte without the shift
        let byte = get_byte_or_fail(self)? as u32;
        temp += byte;

        let mut result: Float = f32::from_bits(temp);
        Ok(result)
    }
}

impl Encodeable for Float {
    fn encode(&self) -> VecDeque<u8> {
        let mut result: VecDeque<u8> = VecDeque::with_capacity(4);
        let mut value = f32::to_bits(*self);
        
        for _ in 1..=3 {
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
        4
    }
}

#[cfg(test)]
mod tests {
    use super::Float;
    use super::{Decodeable, Encodeable};
    use std::collections::VecDeque;

    #[test]
    fn test_read_float_on_vec() {
        let mappings: Vec<(Float, Vec<u8>)> = vec![
            (12.5f32, vec![0x41, 0x48, 0, 0]),
            (-12.5f32, vec![0xC1, 0x48, 0, 0]),
            (1f32, vec![0x3F, 0x80, 0, 0]),
            (-1f32, vec![0xBF, 0x80, 0, 0]),
        ];

        for mapping in mappings {
            let actual: Float = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }
    #[test]
    fn test_write_float_to_vec() {
        let mappings: Vec<(Float, Vec<u8>)> = vec![
            (12.5f32, vec![0x41, 0x48, 0, 0]),
            (-12.5f32, vec![0xC1, 0x48, 0, 0]),
            (1f32, vec![0x3F, 0x80, 0, 0]),
            (-1f32, vec![0xBF, 0x80, 0, 0]),
        ];

        for mapping in mappings {
            assert_eq!(VecDeque::from(mapping.1), mapping.0.encode());
        }
    }
}
