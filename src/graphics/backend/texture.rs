use std::collections::HashMap;

use crate::graphics::texture::TextureHandle;

pub struct GpuTexture {
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub bind_group: wgpu::BindGroup,
}

pub struct TextureManager {
	textures: HashMap<u32, GpuTexture>,
}

impl TextureManager {
	pub fn new() -> Self {
		Self {
			textures: HashMap::new(),
		}
	}

	pub fn get(&self, handle: TextureHandle) -> Option<&GpuTexture> {
		self.textures.get(&handle.id)
	}
}
