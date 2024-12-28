use wgpu::{BufferDescriptor, BufferSize};

pub struct InstanceBuffer {
    pub buffer: wgpu::Buffer,
    size: u64,
}

impl InstanceBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        InstanceBuffer {
            buffer: Self::create_buffer(device, INITIAL_SIZE_BYTES),
            size: INITIAL_SIZE_BYTES,
        }
    }

    pub fn write_data(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, data: &[u8]) {
        // Return early if no data was provided - there's nothing to do, and write_buffer_with doesn't like sizes of zero.
        if data.is_empty() {
            return;
        }

        // Expand the buffer exponentially if it's not big enough.
        let mut new_size = self.size;
        while new_size < (data.len() as u64) {
            new_size *= 2;
            assert!(
                new_size <= MAX_SIZE_BYTES,
                "attempted to allocate more bytes to the instance buffer than allowed ({new_size} > {MAX_SIZE_BYTES})",
            );
        }
        if new_size > self.size {
            self.buffer = Self::create_buffer(device, new_size);
        }

        let mut staging_buffer = queue
            .write_buffer_with(
                &self.buffer,
                0,
                BufferSize::try_from(data.len() as u64)
                    .expect("execution should not reach this point if data.len() is 0"),
            )
            .expect("buffer should be expanded enough to make room for the data");
        staging_buffer[..data.len()].copy_from_slice(data);
    }

    fn create_buffer(device: &wgpu::Device, size: u64) -> wgpu::Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
            size,
        })
    }
}

/// The size to initially create the buffer with.
const INITIAL_SIZE_BYTES: u64 = 1024;
/// The size to cap the buffer to. The program will panic if it tries to make the buffer larger than this.
const MAX_SIZE_BYTES: u64 = 1024 * 1024 * 1024; // 1 GB
