use crate::{core::transform::Transform, graphics::texture::TextureHandle};

pub struct Sprite {
	pub texture: TextureHandle,
	pub transform: Transform,
}
