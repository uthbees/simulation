pub enum Tile {
    Grass,
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl Tile {
    pub fn color(&self) -> [u8; 3] {
        match self {
            Tile::Grass => [0, 154, 23],
            Tile::Red => [255, 0, 0],
            Tile::Green => [0, 255, 0],
            Tile::Blue => [0, 0, 255],
            Tile::Black => [0, 0, 0],
            Tile::White => [255, 255, 255],
        }
    }

    /// The size of a side of a tile in pixels at 1x zoom.
    pub const fn width_px() -> i32 {
        128
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Black
    }
}
