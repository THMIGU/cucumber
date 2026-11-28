use std::time::Duration;

use glam::Vec2;
use hecs::World;
use winit_input_map::InputMap;

use crate::{
	core::{input::Action, transform::Transform},
	error::AppResult,
	game::{
		components::{
			player::{Player, PlayerMovement, PlayerSpeed},
			texture::Texture,
			velocity::Velocity,
		},
		systems::{
			input::input_system, movement::movement_system, physics::physics_system,
			render::render_system,
		},
	},
	graphics::{backend::wgpu_backend::WgpuBackend, camera::Camera},
};

pub struct Scene {
	pub world: World,
	pub camera: Camera,
	pub accumulator: Duration,
}

impl Scene {
	pub fn new(wgpu: &mut WgpuBackend) -> AppResult<Self> {
		let potato_texture = wgpu.create_texture(Some("potato"), "potato.png")?;

		let mut world = World::new();

		world.spawn((
			Player,
			Texture(potato_texture),
			Transform {
				position: Vec2::ZERO,
				origin: Vec2::splat(0.5),
				scale: Vec2::splat(30.0),
				rotation: 0.0,
			},
			Velocity(Vec2::ZERO),
			PlayerMovement(Vec2::ZERO),
			PlayerSpeed(1200.0),
		));

		let camera = Camera::new(Some("camera"));

		Ok(Self {
			world,
			camera,
			accumulator: Duration::ZERO,
		})
	}

	pub fn render(&self, wgpu: &mut WgpuBackend) -> AppResult {
		render_system(wgpu, &self.camera, &self.world)
	}

	pub fn input(&mut self, input: &InputMap<Action>) {
		input_system(&mut self.world, input);
	}

	pub fn update(&mut self, dt: Duration) {
		self.accumulator += dt;

		movement_system(&mut self.world, dt);
		physics_system(&mut self.world, dt);
	}
}
