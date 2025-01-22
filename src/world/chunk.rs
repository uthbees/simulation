use crate::world::tile::Tile;
use crate::Position;

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

    pub fn generate(position: &ChunkPosition) -> Self {
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

/// The position of a chunk in terms of how far it is from the origin chunk (the one with a chunk
/// position of 0, 0 that has the 0, 0 world coordinate in its bottom left corner).
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

impl ChunkPosition {
    pub fn from_world_coords(pos: &Position) -> Self {
        let int_x = pos.x as i32;
        let int_y = pos.y as i32;

        Self {
            x: if int_x >= 0 {
                int_x / 32
            } else {
                (int_x / 32) - 1
            },
            y: if int_y >= 0 {
                int_y / 32
            } else {
                (int_y / 32) - 1
            },
        }
    }
}
