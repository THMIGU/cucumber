use bytemuck::{Pod, Zeroable};

pub struct GpuCamera {
	pub buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
	pub view_proj: [[f32; 4]; 4],
}
