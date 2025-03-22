use crate::display::get_linear_rgb;
use crate::position::{IntChunkCoordinates, Position, PositionMode};
use crate::world::{Tile, World};
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

        let screen_left_edge = screen_center_pos.x(PositionMode::Tiles) - (screen_tile_width / 2.0);
        let screen_right_edge =
            screen_center_pos.x(PositionMode::Tiles) + (screen_tile_width / 2.0);
        let screen_bottom_edge =
            screen_center_pos.y(PositionMode::Tiles) - (screen_tile_height / 2.0);
        let screen_top_edge = screen_center_pos.y(PositionMode::Tiles) + (screen_tile_height / 2.0);

        let bottom_left_chunk_pos =
            Position::new(screen_left_edge, screen_bottom_edge, PositionMode::Tiles)
                .into_int_chunk_coords();
        let top_right_chunk_pos =
            Position::new(screen_right_edge, screen_top_edge, PositionMode::Tiles)
                .into_int_chunk_coords();

        let tile_width_px = f64::from(Tile::width_px());

        for chunk_x in bottom_left_chunk_pos.x..=top_right_chunk_pos.x {
            for chunk_y in bottom_left_chunk_pos.y..=top_right_chunk_pos.y {
                if let Some(chunk) = &world.chunks.get(&IntChunkCoordinates {
                    x: chunk_x,
                    y: chunk_y,
                }) {
                    let chunk_pos =
                        Position::new(f64::from(chunk_x), f64::from(chunk_y), PositionMode::Chunks);

                    for (x_within_chunk, column) in chunk.tiles.iter().enumerate() {
                        for (y_within_chunk, tile) in column.iter().enumerate() {
                            tile_render_instances.push(TileRenderInstance {
                                position: [
                                    ((x_within_chunk as f64 + chunk_pos.x(PositionMode::Tiles))
                                        * tile_width_px) as f32,
                                    ((y_within_chunk as f64 + chunk_pos.y(PositionMode::Tiles))
                                        * tile_width_px) as f32,
                                ],
                                color: get_linear_rgb(tile.color()),
                            });
                        }
                    }
                } else {
                    println!("Tried to render ungenerated chunk!");
                }
            }
        }

        tile_render_instances
    }
}
