mod tile;

use tile::Tile;

/// Represents the simulation world.
pub struct World {
    pub tiles: Vec<TileState>,
}

// temporary
pub struct TileState {
    pub tile: Tile,
    pub pos: Position,
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
                    pos: Position { x: 0.0, y: 0.0 },
                },
                TileState {
                    tile: Tile::Red,
                    pos: Position { x: -500.0, y: 0.0 },
                },
                TileState {
                    tile: Tile::White,
                    pos: Position { x: 500.0, y: 400.0 },
                },
            ],
        }
    }

    pub fn tick(&mut self) {}
}

pub struct Position {
    // Note that fixed-point decimal numbers would be more efficient
    pub x: f64,
    pub y: f64,
}
