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
        World {
            ..Default::default()
        }
    }

    pub fn generate_chunk(&mut self, chunk_pos: ChunkPosition) {
        // Don't do anything if the chunk is already generated.
        if self.chunks.contains_key(&chunk_pos) {
            return;
        }

        let chunk = Chunk::generate(&chunk_pos);
        self.chunks.insert(chunk_pos, chunk);
    }

    pub fn tick(&mut self) {}
}
