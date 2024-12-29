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
}
