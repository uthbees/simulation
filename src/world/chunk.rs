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
}

/// The position of a chunk in terms of how far it is from the origin chunk (the one with a chunk
/// position of 0, 0 that has the 0, 0 world coordinate in its bottom left corner).
///
/// Another way to think about this is that it's equal to the world coordinate of the bottom left
/// corner modulo 32.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}
