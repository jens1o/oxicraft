use std::collections::VecDeque;
use std::io;

pub type Long = i64;

pub trait ReadLong<E> {
    fn read_long(&mut self) -> Result<Long, E>;
}

pub trait WriteLong {
    fn write_long(&self) -> Vec<u8>;
}

impl ReadLong<io::Error> for VecDeque<u8> {
    fn read_long(&mut self) -> Result<Long, io::Error> {
        // TODO: Implement this

        Ok(0)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::UnsignedShort;
//     use crate::short::ReadUnsignedShort;
//     use std::collections::VecDeque;

//     #[test]
//     fn test_read_unsigned_short_on_vec() {
//         let mappings: Vec<(UnsignedShort, Vec<u8>)> = vec![
//             (25565, vec![99, 221]),
//             (25555, vec![99, 211]),
//             (24555, vec![95, 235]),
//         ];

//         for mapping in mappings {
//             assert_eq!(
//                 mapping.0,
//                 VecDeque::from(mapping.1).read_unsigned_short().unwrap()
//             );
//         }
//     }
// }
