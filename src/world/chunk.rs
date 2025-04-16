use crate::position::{IntChunkCoordinates, PositionMode};
use crate::world::perlin_noise;
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

        let bottom_left_pos = position.into_bottom_left_tile_pos();

        for x_in_chunk in 0..Chunk::side_size() {
            let world_x = x_in_chunk + bottom_left_pos.x(PositionMode::Tiles) as i32;
            for y_in_chunk in 0..Chunk::side_size() {
                let world_y = y_in_chunk + bottom_left_pos.y(PositionMode::Tiles) as i32;

                let noise = perlin_noise::octaved_noise(
                    f64::from(world_x),
                    f64::from(world_y),
                    0.0,
                    Some(30.0),
                    None,
                );

                // Adjust the noise up a bit to reduce the amount of water
                let tile_height = noise + 0.05;

                chunk.tiles[x_in_chunk as usize][y_in_chunk as usize] = match tile_height {
                    h if h < 0.0 => Tile::Water,
                    h if h < 0.04 => Tile::Beach,
                    h if h < 0.3 => Tile::Grass,
                    _ => Tile::Mountain,
                }
            }
        }

        chunk
    }
}
