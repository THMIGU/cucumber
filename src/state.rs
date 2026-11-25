use std::sync::Arc;

use anyhow::Context;
use glam::Vec2;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
	error::AppResult,
	graphics::{backend::wgpu_backend::WgpuBackend, camera::Camera, fps::FPS, sprite::Sprite},
};

pub struct State {
	window: Arc<Window>,
	wgpu: WgpuBackend,
	camera: Camera,
	sprites: Vec<Sprite>,
	fps: FPS,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let mut wgpu = WgpuBackend::new(window.clone())
			.await
			.context("Failed to initialize wgpu")?;

		let size = window.inner_size();

		let mut camera = Camera::new(Some("camera"));
		camera.transform.scale = (size.width as f32, size.height as f32).into();

		let texture = wgpu
			.create_texture(Some("potato"), "potato.png")
			.context("Failed to create texture")?;

		let mut potato1 = Sprite::new(Some("potato1"), texture);
		potato1.transform.scale = Vec2::splat(300.0);

		let mut potato2 = Sprite::new(Some("potato2"), texture);
		potato2.transform.position = Vec2::splat(100.0);
		potato2.transform.scale = Vec2::splat(300.0);

		let mut potato3 = Sprite::new(Some("potato3"), texture);
		potato3.transform.position = Vec2::splat(200.0);
		potato3.transform.scale = Vec2::splat(300.0);

		Ok(Self {
			window,
			wgpu,
			camera,
			sprites: vec![potato1, potato2, potato3],
			fps: FPS::new(),
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.wgpu
			.resize(width, height);
	}

	pub fn render(&mut self) -> AppResult {
		self.window.request_redraw();

		let fps = self.fps.fps();
		self.window
			.set_title(&format!("cucumber | fps: {:.0}", fps));

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
