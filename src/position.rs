use crate::world::{Chunk, Tile};

/// A position in the game world. Supports pixels, tiles, and chunks (a pixel being a screen pixel
/// at 1x zoom).
///
/// Notes:
/// - Both tiles and chunks are identified by their lower left corners
/// - 0, 0 is the same in all three coordinate systems
#[derive(Default, Copy, Clone)]
pub struct Position {
    // Note that fixed-point decimal numbers would be more efficient (they would just take more effort).
    // Also note that x and y are stored as pixels.
    x: f64,
    y: f64,
}

impl Position {
    /// Mode here refers to the format of the input x and y arguments.
    pub fn new(x: f64, y: f64, mode: PositionMode) -> Self {
        match mode {
            PositionMode::Pixels => Self { x, y },
            PositionMode::Tiles => Self {
                x: Self::tiles_to_pixels(x),
                y: Self::tiles_to_pixels(y),
            },
            PositionMode::Chunks => Self {
                x: Self::chunks_to_pixels(x),
                y: Self::chunks_to_pixels(y),
            },
        }
    }

    pub fn x(&self, mode: PositionMode) -> f64 {
        match mode {
            PositionMode::Pixels => self.x,
            PositionMode::Tiles => Self::pixels_to_tiles(self.x),
            PositionMode::Chunks => Self::pixels_to_chunks(self.x),
        }
    }

    pub fn y(&self, mode: PositionMode) -> f64 {
        match mode {
            PositionMode::Pixels => self.y,
            PositionMode::Tiles => Self::pixels_to_tiles(self.y),
            PositionMode::Chunks => Self::pixels_to_chunks(self.y),
        }
    }

    pub fn set_x(&mut self, value: f64, mode: PositionMode) {
        self.x = match mode {
            PositionMode::Pixels => value,
            PositionMode::Tiles => Self::tiles_to_pixels(value),
            PositionMode::Chunks => Self::chunks_to_pixels(value),
        }
    }

    pub fn set_y(&mut self, value: f64, mode: PositionMode) {
        self.y = match mode {
            PositionMode::Pixels => value,
            PositionMode::Tiles => Self::tiles_to_pixels(value),
            PositionMode::Chunks => Self::chunks_to_pixels(value),
        }
    }

    pub fn into_int_chunk_coords(self) -> IntChunkCoordinates {
        let int_x = Self::pixels_to_chunks(self.x) as i32;
        let int_y = Self::pixels_to_chunks(self.y) as i32;

        // We have to do a little bit of logic here because `as i32` always rounds towards zero,
        // whereas chunks are identified by their lower left corners.
        IntChunkCoordinates {
            x: if self.x >= 0.0 { int_x } else { int_x - 1 },
            y: if self.y >= 0.0 { int_y } else { int_y - 1 },
        }
    }

    fn pixels_to_tiles(pixel_coordinate: f64) -> f64 {
        pixel_coordinate / f64::from(Tile::width_px())
    }

    fn tiles_to_pixels(tile_coordinate: f64) -> f64 {
        tile_coordinate * f64::from(Tile::width_px())
    }

    fn pixels_to_chunks(pixel_coordinate: f64) -> f64 {
        Self::pixels_to_tiles(pixel_coordinate) / f64::from(Chunk::side_size())
    }

    fn chunks_to_pixels(chunk_coordinate: f64) -> f64 {
        Self::tiles_to_pixels(chunk_coordinate * f64::from(Chunk::side_size()))
    }
}

#[derive(Copy, Clone)]
pub enum PositionMode {
    Pixels,
    Tiles,
    Chunks,
}

/// A struct with the integer coordinates of a chunk, for when you care about which chunk
/// but not where in the chunk.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct IntChunkCoordinates {
    pub x: i32,
    pub y: i32,
}
