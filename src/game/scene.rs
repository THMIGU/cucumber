use std::time::Duration;

use glam::Vec2;
use hecs::World;

use crate::{
	core::transform::Transform,
	error::AppResult,
	game::{
		components::{potato::Potato, texture::Texture},
		systems::{potato::update_potato, render::render_system},
	},
	graphics::{backend::wgpu_backend::WgpuBackend, camera::Camera},
};

pub struct Scene {
	pub world: World,
	pub camera: Camera,
}

impl Scene {
	pub fn new(wgpu: &mut WgpuBackend) -> AppResult<Self> {
		let potato_texture = wgpu.create_texture(Some("potato"), "potato.png")?;

		let mut world = World::new();

		world.spawn((
			Potato,
			Texture {
				handle: potato_texture,
			},
			Transform {
				position: Vec2::ZERO,
				origin: Vec2::splat(0.5),
				scale: Vec2::splat(30.0),
				rotation: 0.0,
			},
		));

		let camera = Camera::new(Some("camera"));

		Ok(Self {
			world,
			camera,
		})
	}

	pub fn render(&self, wgpu: &mut WgpuBackend) -> AppResult {
		render_system(wgpu, &self.camera, &self.world)
	}

	pub fn update(&mut self, dt: Duration) {
		update_potato(&mut self.world, dt);
	}
}
