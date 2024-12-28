use crate::display::get_linear_rgb;
use crate::world::World;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileInstance {
    /// The position of the center of the tile (x/y) in pixel coordinates.
    position: [f32; 2],
    /// The color of the tile in linear rgb.
    color: [f32; 3],
}

impl TileInstance {
    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<TileInstance>() as BufferAddress,
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

    pub fn vec_from_world(world: &World) -> Vec<TileInstance> {
        vec![
            TileInstance {
                position: [0.0, 0.0],
                color: get_linear_rgb([255, 0, 0]),
            },
            TileInstance {
                position: [-0.5, 0.0],
                color: get_linear_rgb([0, 255, 0]),
            },
            TileInstance {
                position: [0.5, 0.5],
                color: get_linear_rgb([0, 0, 0]),
            },
        ]
    }
}
