use glam::Vec2;

pub struct Transform {
	pub position: Vec2,
	pub origin: Vec2,
	pub scale: Vec2,
	pub rotation: f32,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			origin: Vec2::ZERO,
			scale: Vec2::ONE,
			rotation: 0.0,
		}
	}
}
