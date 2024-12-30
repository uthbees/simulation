use crate::display::get_linear_rgb;
use crate::world::{Chunk, Tile, World};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileRenderInstance {
    /// The position of the center of the tile (x/y) in pixel coordinates.
    position: [f32; 2],
    /// The color of the tile in linear rgb.
    color: [f32; 3],
}

impl TileRenderInstance {
    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<TileRenderInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }

    pub fn vec_from_world(world: &World) -> Vec<TileRenderInstance> {
        let mut tile_render_instances = vec![];

        // TODO: Only render chunks/tiles that are visible

        for (chunk_pos, chunk) in &world.chunks {
            for (x_within_chunk, column) in chunk.tiles.iter().enumerate() {
                for (y_within_chunk, tile) in column.iter().enumerate() {
                    tile_render_instances.push(TileRenderInstance {
                        position: [
                            ((x_within_chunk as i32 + chunk_pos.x * Chunk::side_size())
                                * Tile::width_px()) as f32,
                            ((y_within_chunk as i32 + chunk_pos.y * Chunk::side_size())
                                * Tile::width_px()) as f32,
                        ],
                        color: get_linear_rgb(tile.color()),
                    });
                }
            }
        }

        tile_render_instances
    }
}
