mod chunk;
mod perlin_noise;
mod tile;

use crate::position::IntChunkCoordinates;
pub use chunk::Chunk;
use std::collections::HashMap;
use std::default::Default;
pub use tile::Tile;

/// Represents the simulation world.
#[derive(Default)]
pub struct World {
    pub chunks: HashMap<IntChunkCoordinates, Chunk>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        World {
            ..Default::default()
        }
    }

    pub fn generate_chunk(&mut self, chunk_pos: IntChunkCoordinates) {
        // Don't do anything if the chunk is already generated.
        if self.chunks.contains_key(&chunk_pos) {
            return;
        }

        let chunk = Chunk::generate(chunk_pos);
        self.chunks.insert(chunk_pos, chunk);
    }

    pub fn tick(&mut self) {}
}
