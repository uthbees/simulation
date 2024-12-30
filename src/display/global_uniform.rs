use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BufferSize, Device};

pub struct GlobalUniform {
    data: GlobalUniformData,
    buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl GlobalUniform {
    pub fn new(device: &Device, data: GlobalUniformData) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Global uniform buffer"),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Global uniform bind group layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Global uniform bind group"),
        });

        GlobalUniform {
            data,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn write_data(&mut self, queue: &wgpu::Queue, data: GlobalUniformData) {
        self.data = data;

        let data = [data];
        let raw_data: &[u8] = bytemuck::cast_slice(&data);

        let mut staging_buffer = queue
            .write_buffer_with(
                &self.buffer,
                0,
                BufferSize::try_from(raw_data.len() as u64)
                    .expect("the global uniform should contain data"),
            )
            .expect("the global uniform buffer should have enough space for its data");
        staging_buffer.copy_from_slice(raw_data);
    }

    pub fn data(&self) -> &GlobalUniformData {
        &self.data
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalUniformData {
    pub window_size_px: [f32; 2],
    pub camera_pos: [f32; 2],
    pub camera_zoom: f32,
    // WASM requires buffer bindings to be 16-bit aligned, so we need to add some padding.
    pub padding: [f32; 3],
}

impl Default for GlobalUniformData {
    fn default() -> Self {
        GlobalUniformData {
            window_size_px: [1.0, 1.0],
            camera_pos: [0.0, 0.0],
            camera_zoom: 1.0,
            padding: [0.0, 0.0, 0.0],
        }
    }
}
