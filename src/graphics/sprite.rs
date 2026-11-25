use crate::{core::transform::Transform, graphics::texture::TextureHandle};

pub struct Sprite {
	pub label: Option<&'static str>,
	pub texture: TextureHandle,
	pub transform: Transform,
}

impl Sprite {
	pub fn new(label: Option<&'static str>, texture: TextureHandle) -> Self {
		Self {
			label,
			texture,
			transform: Default::default(),
		}
	}
}
