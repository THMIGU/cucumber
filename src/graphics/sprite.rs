use crate::{core::transform::Transform, graphics::texture::TextureHandle};

pub struct Sprite {
	pub label: Option<&'static str>,
	pub texture: TextureHandle,
	pub transform: Transform,
}
