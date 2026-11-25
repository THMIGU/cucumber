use std::sync::Arc;

use anyhow::Context;
use glam::Vec2;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
	error::AppResult,
	graphics::{backend::wgpu_backend::WgpuBackend, camera::Camera, sprite::Sprite},
};

pub struct State {
	window: Arc<Window>,
	wgpu: WgpuBackend,
	camera: Camera,
	sprites: Vec<Sprite>,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let mut wgpu = WgpuBackend::new(window.clone())
			.await
			.context("Failed to initialize wgpu")?;

		let size = window.inner_size();
		let aspect = size.width as f32 / size.height as f32;

		let mut camera = Camera::new(Some("camera"));
		camera.transform.scale = (180.0 * aspect, 180.0).into();

		let texture = wgpu.create_texture(Some("potato"), "potato.png")?;

		let mut potato1 = Sprite::new(Some("potato1"), texture);
		potato1.transform.scale = Vec2::splat(30.0);

		let mut potato2 = Sprite::new(Some("potato2"), texture);
		potato2.transform.position = Vec2::splat(10.0);
		potato2.transform.scale = Vec2::splat(30.0);

		let mut potato3 = Sprite::new(Some("potato3"), texture);
		potato3.transform.position = Vec2::splat(20.0);
		potato3.transform.scale = Vec2::splat(30.0);

		Ok(Self {
			window,
			wgpu,
			camera,
			sprites: vec![potato1, potato2, potato3],
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.wgpu
			.resize(width, height);
	}

	pub fn render(&mut self) -> AppResult {
		self.window.request_redraw();

		self.wgpu
			.render(&self.camera, &self.sprites)
			.context("Failed to render sprites")?;

		Ok(())
	}

	pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
		match (code, is_pressed) {
			(KeyCode::Escape, true) => event_loop.exit(),
			_ => (),
		}
	}

	pub fn update(&mut self) {
		()
	}
}
