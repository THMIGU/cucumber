use bytemuck::{Pod, Zeroable};

pub struct GpuCamera {
	pub buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

impl GpuCamera {
	pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
		let buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Camera Buffer"),
			size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
			label: Some("Camera Bind Group"),
		});

		Self {
			buffer,
			bind_group,
		}
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
	pub view_proj: [[f32; 4]; 4],
}
