mod tile;

use tile::Tile;

/// Represents the simulation world.
pub struct World {
    pub tiles: Vec<TileState>,
}

// temporary
pub struct TileState {
    pub tile: Tile,
    pub x: i32,
    pub y: i32,
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
            tiles: vec![
                TileState {
                    tile: Tile::Grass,
                    x: 0,
                    y: 0,
                },
                TileState {
                    tile: Tile::Red,
                    x: -500,
                    y: 0,
                },
                TileState {
                    tile: Tile::White,
                    x: 500,
                    y: 400,
                },
            ],
        }
    }

    pub fn tick(&mut self) {}
}
