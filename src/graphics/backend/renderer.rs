use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::{core::transform::Transform, graphics::{
	backend::{
		buffer::{Buffers, Vertex},
		camera::GpuCamera,
		pipeline::Pipeline,
		texture::TextureManager,
	},
	sprite::Sprite,
}};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TransformUniform {
	mat: [[f32; 4]; 4]
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

pub struct Renderer {
	buffers: Buffers,
	pipeline: Pipeline,
	camera: GpuCamera,
	transform: wgpu::BindGroup
}

impl Renderer {
	pub fn render(
		&mut self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		pass: &mut wgpu::RenderPass,
		sprites: &[Sprite],
		textures: &TextureManager,
	) {
		let mut vertices: Vec<Vertex> = vec![];

		for sprite in sprites {
			let pos = &sprite.transform.position;
			let scl = &sprite.transform.scale;

			let index_offset = vertices.len();

			vertices.push(Vertex {
				pos: (sprite., 0.0).into(),
				uv: (0.0, 0.0).into(),
			});

			self.buffers
		}
	}
}
