use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
	pub mat: [[f32; 4]; 4],
}

impl CameraUniform {
	pub fn new() -> Self {
		Self {
			mat: Mat4::IDENTITY.to_cols_array_2d(),
		}
	}

	// pub fn update(&mut self, camera: &Camera) {
	// 	self.mat = camera
	// 		.matrix()
	// 		.to_cols_array_2d();
	// }
}

pub struct CameraBinding {
	pub uniform: CameraUniform,
	pub buffer: wgpu::Buffer,
	pub binding: wgpu::BindGroup,
}

impl CameraBinding {
	pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
		let uniform = CameraUniform::new();

		let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Camera Buffer"),
			contents: bytemuck::cast_slice(&[uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let binding = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
			label: Some("Camera Bind Group"),
		});

		Self {
			uniform,
			buffer,
			binding,
		}
	}
}
