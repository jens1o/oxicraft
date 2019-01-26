use super::Encodeable;
use std::collections::VecDeque;

pub enum Gamemode {
    Survival = 0x00,
    Creative = 0x01,
    Adventure = 0x02,
    Hardcore = 0x08,
}

impl Encodeable for Gamemode {
    fn encode(&self) -> VecDeque<u8> {
        VecDeque::from(vec![match self {
            Gamemode::Survival => 0x00,
            Gamemode::Creative => 0x01,
            Gamemode::Adventure => 0x02,
            Gamemode::Hardcore => 0x08,
        }])
    }

    fn byte_length(&self) -> u8 {
        1
    }
}
