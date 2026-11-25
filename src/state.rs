use std::sync::Arc;

use anyhow::Context;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{error::AppResult, graphics::backend::wgpu_backend::WgpuBackend};

pub struct State {
	window: Arc<Window>,
	wgpu: WgpuBackend,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let wgpu = WgpuBackend::new(window.clone())
			.await
			.context("Failed to initialize WGPU")?;

		Ok(Self {
			window,
			wgpu,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.wgpu
			.resize(width, height);
	}

	pub fn render(&mut self) -> AppResult {
		self.window.request_redraw();

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
