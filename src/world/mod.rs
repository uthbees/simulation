use crate::world::tile::Tile;

mod tile;

/// Represents the simulation world.
pub struct World {
    pub tile: Tile,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tile: Tile {
                color: [65, 152, 10],
            },
        }
    }

    pub fn tick(&mut self) {}
}
