use crate::graphics::{
	backend::{
		buffer::{Buffers, Vertex},
		camera::GpuCamera,
		pipeline::Pipeline,
		texture::TextureManager,
		transform::GpuTransform,
	},
	sprite::Sprite,
};

pub struct Renderer {
	buffers: Buffers,
	pipeline: Pipeline,
	camera: GpuCamera,
	transform: GpuTransform,
}

impl Renderer {
	pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
		let buffers = Buffers::new(device);
		let pipeline = Pipeline::new(device, format);
		let camera = GpuCamera::new(device, &pipeline.camera_layout);
		let transform = GpuTransform::new(device, &pipeline.transform_layout);

		Self {
			buffers,
			pipeline,
			camera,
			transform,
		}
	}

	pub fn render(
		&mut self,
		queue: &wgpu::Queue,
		pass: &mut wgpu::RenderPass,
		sprites: &[Sprite],
		textures: &TextureManager,
	) {
		let mut vertices: Vec<Vertex> = vec![];
		for sprite in sprites {
			let pos = &sprite.transform.position;
			let scl = &sprite.transform.scale;

			let sprite_vertices = [
				Vertex {
					pos: (pos.x, scl.y + pos.y).into(),
					uv: (0.0, 1.0).into(),
				},
				Vertex {
					pos: (pos.x, pos.y).into(),
					uv: (0.0, 0.0).into(),
				},
				Vertex {
					pos: (scl.x + pos.x, pos.y).into(),
					uv: (1.0, 0.0).into(),
				},
				Vertex {
					pos: (scl.x + pos.x, scl.y + pos.y).into(),
					uv: (1.0, 1.0).into(),
				},
			];
			vertices.extend(sprite_vertices);
		}

		self.buffers
			.write_vertex(queue, &vertices);

		pass.set_pipeline(&self.pipeline.pipeline);

		pass.set_bind_group(0, &self.camera.bind_group, &[]);
		pass.set_bind_group(1, &self.transform.bind_group, &[]);

		pass.set_vertex_buffer(
			0,
			self.buffers
				.vertex_buffer
				.slice(..),
		);
		pass.set_index_buffer(
			self.buffers
				.index_buffer
				.slice(..),
			wgpu::IndexFormat::Uint16,
		);

		for (index, sprite) in sprites.iter().enumerate() {
			let Some(texture) = textures.get(sprite.texture) else {
				log::warn!(
					"Missing texture for sprite '{}'",
					sprite
						.label
						.unwrap_or("unknown")
				);
				continue;
			};

			self.transform
				.write_transform(queue, &sprite.transform);

			pass.set_bind_group(2, &texture.bind_group, &[]);
			pass.draw_indexed(0..6, (index * 4) as i32, 0..1);
		}
	}
}
