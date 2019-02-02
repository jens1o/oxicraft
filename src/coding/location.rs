use super::{Decodeable, Encodeable};
use crate::coding::long::Long;
use crate::location::Location;
use std::collections::VecDeque;
use std::io;

impl Decodeable<Location, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<Location, io::Error> {
        let raw_value: Long = self.decode()?;

        let x = raw_value >> 38;

        if x < -33_554_432 || x > 33_554_431 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "x-coordinate not in valid range!",
            ));
        }

        let y = (raw_value >> 26) & 0xFFF;

        if y < -2048 || y > 2047 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "y-coordinate not in valid range!",
            ));
        }

        let z = raw_value << 38 >> 38;

        if z < -33_554_432 || z > 33_554_431 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "z-coordinate not in valid range!",
            ));
        }

        Ok(Location {
            x: x as i32,
            y: y as i16,
            z: z as i32,
        })
    }
}

impl Encodeable for Location {
    fn encode(&self) -> VecDeque<u8> {
        let value: Long = (((self.x & 0x03FF_FFFF) as i64) << 38)
            | (((self.y & 0xFFF) as i64) << 26)
            | (self.z & 0x03FF_FFFF) as i64;

        value.encode()
    }

    fn byte_length(&self) -> u8 {
        8
    }
}

// TODO: Add tests with values provided by the client
