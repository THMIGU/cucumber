use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::{core::transform::Transform, graphics::camera::Camera};

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

	pub fn write_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
		let uniform = CameraUniform::from_transform(&camera.transform);
		queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
	pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
	pub fn from_transform(transform: &Transform) -> Self {
		let proj = glam::camera::rh::proj::directx::orthographic(
			0.0,
			transform.scale.x,
			transform.scale.y,
			0.0,
			-1.0,
			1.0,
		);

		let translation = Mat4::from_translation((-transform.position, 0.0).into());
		let rotation = Mat4::from_rotation_z(transform.rotation);

		let view = translation * rotation;

		let view_proj = (proj * view).to_cols_array_2d();

		Self {
			view_proj,
		}
	}
}
