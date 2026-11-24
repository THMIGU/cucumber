use bytemuck::{Pod, Zeroable};

pub struct Buffers {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
	pub pos: [f32; 2],
	pub uv: [f32; 2],
}
