use crate::display::get_linear_rgb;
use crate::world::{Chunk, ChunkPosition, Tile, World};
use crate::Position;
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

    pub fn vec_from_world(
        world: &World,
        screen_center_pos: &Position,
        screen_width: i32,
        screen_height: i32,
        camera_zoom: f32,
    ) -> Vec<TileRenderInstance> {
        let mut tile_render_instances = vec![];

        let screen_tile_width =
            f64::from(screen_width) / f64::from(camera_zoom) / f64::from(Tile::width_px());
        let screen_tile_height =
            f64::from(screen_height) / f64::from(camera_zoom) / f64::from(Tile::width_px());

        let screen_left_edge = screen_center_pos.x - (screen_tile_width / 2.0);
        let screen_right_edge = screen_center_pos.x + (screen_tile_width / 2.0);
        let screen_bottom_edge = screen_center_pos.y - (screen_tile_height / 2.0);
        let screen_top_edge = screen_center_pos.y + (screen_tile_height / 2.0);
        println!(
            "Center: {screen_center_pos:?}, left: {screen_left_edge}, right: {screen_right_edge}"
        );

        let bottom_left_chunk_pos = ChunkPosition::from_world_coords(&Position {
            x: screen_left_edge,
            y: screen_bottom_edge,
        });
        let top_right_chunk_pos = ChunkPosition::from_world_coords(&Position {
            x: screen_right_edge,
            y: screen_top_edge,
        });

        println!(
            "Rendering x: {}-{}, y: {}-{}",
            bottom_left_chunk_pos.x,
            top_right_chunk_pos.x,
            bottom_left_chunk_pos.y,
            top_right_chunk_pos.y
        );

        let tile_width_px = f64::from(Tile::width_px());

        // TODO: Fix the tile rendering.
        //  It seems to be rendering correctly at all zooms now when at 0,0, but never at any other position.
        //  - Is it sometimes grabbing the wrong chunks?
        //  - Is the zoom being applied correctly at every step?

        for chunk_x in bottom_left_chunk_pos.x..=top_right_chunk_pos.x {
            for chunk_y in bottom_left_chunk_pos.y..=top_right_chunk_pos.y {
                if let Some(chunk) = &world.chunks.get(&ChunkPosition {
                    x: chunk_x,
                    y: chunk_y,
                }) {
                    let chunk_screen_x =
                        f64::from(chunk_x * Chunk::side_size()) + screen_center_pos.x;
                    let chunk_screen_y =
                        f64::from(chunk_y * Chunk::side_size()) + screen_center_pos.y;

                    for (x_within_chunk, column) in chunk.tiles.iter().enumerate() {
                        for (y_within_chunk, tile) in column.iter().enumerate() {
                            tile_render_instances.push(TileRenderInstance {
                                position: [
                                    ((x_within_chunk as f64 + chunk_screen_x) * tile_width_px)
                                        as f32,
                                    ((y_within_chunk as f64 + chunk_screen_y) * tile_width_px)
                                        as f32,
                                ],
                                color: get_linear_rgb(tile.color()),
                            });
                        }
                    }
                } else {
                    println!("Tried to render un-generated chunk!");
                }
            }
        }

        tile_render_instances
    }
}
