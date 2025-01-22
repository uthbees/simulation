mod chunk;
mod tile;

pub use chunk::{Chunk, ChunkPosition};
use std::collections::HashMap;
use std::default::Default;
pub use tile::Tile;

/// Represents the simulation world.
#[derive(Default)]
pub struct World {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        let mut world = World {
            ..Default::default()
        };

        // TODO: Generate terrain automatically as the camera comes near
        world.generate_chunk(ChunkPosition { x: 0, y: 0 });

        world
    }

    pub fn generate_chunk(&mut self, chunk_pos: ChunkPosition) {
        let mut chunk = Chunk {
            ..Default::default()
        };

        chunk.tiles[0][0] = Tile::Grass;
        chunk.tiles[2][3] = Tile::Red;
        chunk.tiles[3][2] = Tile::White;
        chunk.tiles[4][3] = Tile::Blue;

        self.chunks.insert(chunk_pos, chunk);
    }

    pub fn tick(&mut self) {}
}
