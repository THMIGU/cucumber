use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};

pub struct Camera {
	pub pos: Vec2,
	pub aspect: f32,
	pub scale: f32,
}

impl Camera {
	pub fn matrix(&self) -> Mat4 {
		let aspect = Mat4::from_scale((1.0, self.aspect, 1.0).into());
		let scale = Mat4::from_scale((Vec2::splat(self.scale), 1.0).into());
		let translation = Mat4::from_translation((-self.pos, 0.0).into());

		aspect * scale * translation
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
	pub mat: [[f32; 4]; 4],
}

impl CameraUniform {
	pub fn new() -> Self {
		Self {
			mat: Mat4::ZERO.to_cols_array_2d(),
		}
	}

	pub fn update(&mut self, camera: &Camera) {
		self.mat = camera
			.matrix()
			.to_cols_array_2d();
	}
}
