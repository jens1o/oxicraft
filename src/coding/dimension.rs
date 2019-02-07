use super::int::MinecraftInt;
use super::Encodeable;
use crate::dimension::Dimension;
use std::collections::VecDeque;

impl Encodeable for Dimension {
    fn encode(&self) -> VecDeque<u8> {
        let numeric_value: MinecraftInt = match self {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1,
        };

        numeric_value.encode()
    }

    fn byte_length(&self) -> u8 {
        4
    }
}
