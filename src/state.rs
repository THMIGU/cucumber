use std::sync::Arc;

use anyhow::Context;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{error::AppResult, render::Render};

pub struct State {
	window: Arc<Window>,
	render: Render,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let render = Render::new(window.clone())
			.await
			.context("Failed to initialize renderer")?;

		Ok(Self {
			window,
			render,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.render
			.resize(width, height);
	}

	pub fn render(&mut self) -> AppResult {
		self.window.request_redraw();

		self.render
			.render_temp()
			.context("Failed to render")?;

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
