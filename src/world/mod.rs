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
        let pos_0_0 = ChunkPosition { x: 0, y: 0 };

        world.chunks.insert(
            pos_0_0.clone(),
            Chunk {
                ..Default::default()
            },
        );

        let chunk_0_0 = world
            .chunks
            .get_mut(&pos_0_0)
            .expect("chunk should have just been inserted");

        chunk_0_0.tiles[0][0] = Tile::Grass;
        chunk_0_0.tiles[2][3] = Tile::Red;
        chunk_0_0.tiles[3][2] = Tile::White;
        chunk_0_0.tiles[4][3] = Tile::Blue;

        world
    }

    pub fn tick(&mut self) {}
}
