use crate::position::IntChunkCoordinates;
use crate::world::tile::Tile;

/// A 32x32 chunk of the map.
#[derive(Default)]
pub struct Chunk {
    /// The tiles in the chunk, indexed as x/y from the bottom left corner.
    pub tiles: [[Tile; Chunk::side_size() as usize]; Chunk::side_size() as usize],
}

impl Chunk {
    /// The size of a chunk on one side.
    pub const fn side_size() -> i32 {
        32
    }

    pub fn generate(position: IntChunkCoordinates) -> Self {
        let mut chunk = Self {
            ..Default::default()
        };

        chunk.tiles[0][0] = Tile::Grass;
        chunk.tiles[2][3] = Tile::Red;
        chunk.tiles[3][2] = Tile::White;
        chunk.tiles[4][3] = Tile::Blue;

        chunk
    }
}
