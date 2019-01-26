use super::Decodeable;
use std::collections::VecDeque;
use std::io;

pub type Short = i16;
pub type UnsignedShort = u16;

impl Decodeable<UnsignedShort, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<UnsignedShort, io::Error> {
        let error_message =
            "VecDeque needs to have at least 2 bytes for reading an unsigned-short.";

        let result: UnsignedShort = (u16::from(self.pop_front().expect(error_message)) << 8)
            .checked_add(u16::from(self.pop_front().expect(error_message)))
            .expect("Unsigned short is too big!");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Decodeable, UnsignedShort};
    use std::collections::VecDeque;

    #[test]
    fn test_read_unsigned_short_on_vec() {
        let mappings: Vec<(UnsignedShort, Vec<u8>)> = vec![
            (25565, vec![99, 221]),
            (25555, vec![99, 211]),
            (24555, vec![95, 235]),
        ];

        for mapping in mappings {
            let actual: UnsignedShort = VecDeque::from(mapping.1).decode().unwrap();

            assert_eq!(mapping.0, actual);
        }
    }
}
