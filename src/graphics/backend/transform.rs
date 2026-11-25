use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::core::transform::Transform;

pub struct GpuTransform {
	pub buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
	pub stride: u64,
}

impl GpuTransform {
	pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
		let alignment = device
			.limits()
			.min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
		let size = std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress;

		let stride = size.div_ceil(alignment) * alignment;

		let buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Transform Buffer"),
			size: stride * 1024,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
					buffer: &buffer,
					offset: 0,
					size: Some(std::num::NonZeroU64::new(size).unwrap()),
				}),
			}],
			label: Some("Transform Bind Group"),
		});

		Self {
			buffer,
			bind_group,
			stride,
		}
	}

	pub fn write_transform(&self, queue: &wgpu::Queue, transform: &Transform, index: usize) {
		let uniform = TransformUniform::from_transform(transform);
		queue.write_buffer(&self.buffer, self.stride * index as u64, bytemuck::bytes_of(&uniform));
	}

	pub fn offset(&self, index: usize) -> wgpu::DynamicOffset {
		(self.stride * index as u64) as wgpu::DynamicOffset
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TransformUniform {
	mat: [[f32; 4]; 4],
}

impl TransformUniform {
	pub fn from_transform(transform: &Transform) -> Self {
		let translation = Mat4::from_translation(transform.position.extend(0.0));
		let rotation = Mat4::from_rotation_z(transform.rotation);
		let scale = Mat4::from_scale(transform.scale.extend(1.0));
		let origin = Mat4::from_translation(-transform.origin.extend(0.0));

		let mat = (translation * rotation * scale * origin).to_cols_array_2d();

		Self {
			mat,
		}
	}
}
