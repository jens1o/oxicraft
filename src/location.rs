pub struct Location {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Default for Location {
    fn default() -> Location {
        Location { x: 0, y: 0, z: 0 }
    }
}
