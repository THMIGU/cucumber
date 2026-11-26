use std::sync::Arc;

use anyhow::Context;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{error::AppResult, game::scene::Scene, graphics::backend::wgpu_backend::WgpuBackend};

pub struct State {
	window: Arc<Window>,
	wgpu: WgpuBackend,
	scene: Scene,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let mut wgpu = WgpuBackend::new(window.clone())
			.await
			.context("Failed to initialize wgpu")?;

		let size = window.inner_size();
		let aspect = size.width as f32 / size.height as f32;

		let mut scene = Scene::new(&mut wgpu).context("Failed to create scene")?;
		scene.camera.transform.scale = (180.0 * aspect, 180.0).into();

		Ok(Self {
			window,
			wgpu,
			scene,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.wgpu
			.resize(width, height);
	}

	pub fn render(&mut self) -> AppResult {
		self.window.request_redraw();

		self.scene
			.render(&mut self.wgpu)?;

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
