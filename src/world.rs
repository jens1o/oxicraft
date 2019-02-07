use crate::coding::{gamemode::Gamemode, level_type::LevelType};
use crate::difficulty::Difficulty;
use crate::dimension::Dimension;

pub struct World {
    pub gamemode: Gamemode,
    pub dimension: Dimension,
    pub difficulty: Difficulty,
    pub level_type: LevelType,
}

impl Default for World {
    fn default() -> World {
        World {
            gamemode: Gamemode::Creative,
            dimension: Dimension::Overworld,
            difficulty: Difficulty::Peaceful,
            level_type: LevelType::Flat,
        }
    }
}
