#[derive(Default)]
pub enum Tile {
    #[default]
    Black,
    Grass,
    Beach,
    Water,
    Mountain,
}

/// A world tile. Tiles are rendered centered on their position.
impl Tile {
    pub fn color(&self) -> [u8; 3] {
        match self {
            Tile::Grass => [51, 127, 51],
            Tile::Beach => [255, 204, 0],
            Tile::Water => [0, 0, 255],
            Tile::Mountain => [127, 127, 127],
            Tile::Black => [0, 0, 0],
        }
    }

    /// The size of a side of a tile in pixels at 1x zoom.
    pub const fn width_px() -> i32 {
        128
    }
}
