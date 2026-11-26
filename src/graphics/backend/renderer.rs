use crate::graphics::{
	backend::{
		buffer::Buffers, camera::GpuCamera, pipeline::Pipeline, texture::TextureManager,
		transform::GpuTransform,
	},
	camera::Camera,
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

	pub fn texture_layout(&self) -> &wgpu::BindGroupLayout {
		&self.pipeline.texture_layout
	}

	pub fn render(
		&mut self,
		queue: &wgpu::Queue,
		pass: &mut wgpu::RenderPass,
		textures: &TextureManager,
		camera: &Camera,
		sprites: &[Sprite],
	) {
		self.camera
			.write_camera(queue, camera);

		pass.set_pipeline(&self.pipeline.pipeline);

		pass.set_bind_group(0, &self.camera.bind_group, &[]);

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
					"Missing texture '{}'",
					sprite
						.texture
						.label
						.unwrap_or("unknown")
				);
				continue;
			};

			self.transform
				.write_transform(queue, &sprite.transform, index);

			pass.set_bind_group(1, &self.transform.bind_group, &[self.transform.offset(index)]);
			pass.set_bind_group(2, &texture.bind_group, &[]);

			pass.draw_indexed(0..6, 0, 0..1);
		}
	}
}
