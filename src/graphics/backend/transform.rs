use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::core::transform::Transform;

pub struct GpuTransform {
	pub buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

impl GpuTransform {
	pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
		let buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Transform Buffer"),
			size: std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
			label: Some("Transform Bind Group"),
		});

		Self {
			buffer,
			bind_group,
		}
	}
	pub fn write_transform(&self, queue: &wgpu::Queue, transform: &Transform) {
		let uniform = TransformUniform::from_transform(transform);
		queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TransformUniform {
	mat: [[f32; 4]; 4],
}

impl TransformUniform {
	pub fn from_transform(transform: &Transform) -> Self {
		let translation = Mat4::from_translation((transform.position, 0.0).into());
		let rotation = Mat4::from_rotation_z(transform.rotation);
		let scale = Mat4::from_scale((transform.scale, 1.0).into());

		let mat = (translation * rotation * scale).to_cols_array_2d();

		Self {
			mat,
		}
	}
}
