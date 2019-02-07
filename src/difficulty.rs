pub enum Difficulty {
    Peaceful = 0x00,
    Easy = 0x01,
    Normal = 0x02,
    Hard = 0x03,
}

impl Default for Difficulty {
    fn default() -> Difficulty {
        Difficulty::Peaceful
    }
}