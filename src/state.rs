use std::{sync::Arc, time::Instant};

use anyhow::Context;
use gilrs::Gilrs;
use winit::{event::WindowEvent, window::Window};
use winit_input_map::InputMap;

use crate::{
	core::input::{Action, bindings},
	error::AppResult,
	game::scene::Scene,
	graphics::backend::wgpu_backend::WgpuBackend,
};

pub struct State {
	window: Arc<Window>,
	wgpu: WgpuBackend,
	scene: Scene,
	input: InputMap<Action>,
	gilrs: Gilrs,
	last_frame: Instant,
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

		let input = InputMap::new(&bindings());
		let gilrs = Gilrs::new().map_err(|e| anyhow::anyhow!("Failed to initialize Gilrs: {e}"))?;

		Ok(Self {
			window,
			wgpu,
			scene,
			input,
			gilrs,
			last_frame: Instant::now(),
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

	pub fn keyboard_input(&mut self, event: &WindowEvent) {
		self.input
			.update_with_window_event(event);
	}

	fn controller_input(&mut self) {
		self.input
			.update_with_gilrs(&mut self.gilrs);
	}

	pub fn input(&mut self) {
		self.controller_input();
		self.scene.input(&self.input);
	}

	pub fn update(&mut self) {
		let dt = self.last_frame.elapsed();
		self.last_frame = Instant::now();

		self.scene.update(dt);
	}
}
