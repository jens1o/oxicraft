use std::collections::VecDeque;

pub mod boolean;
pub mod byte;
pub mod int;
pub mod long;
pub mod short;
pub mod string;
pub mod varint;

pub trait Decodeable<T, E> {
    /// Decodes from the Minecraft format into the type T, optionally returning
    /// an error
    fn decode(&mut self) -> Result<T, E>;
}

pub trait Encodeable {
    /// Encodes from the Rust type into the Minecraft format(an array of bytes)
    fn encode(&self) -> VecDeque<u8>;

    /// Returns a guess on how much space is needed to encode it to improve allocations.
    fn byte_length(&self) -> u8;
}
