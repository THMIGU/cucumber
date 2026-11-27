use std::sync::Arc;

use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Fullscreen, WindowAttributes, WindowId},
};

use crate::state::State;

#[derive(Default)]
pub struct App {
	state: Option<State>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if self.state.is_some() {
			return;
		}

		let window_attributes = WindowAttributes::default()
			.with_title("cucumber")
			.with_fullscreen(Some(Fullscreen::Borderless(None)));

		let window = Arc::new(
			event_loop
				.create_window(window_attributes)
				.unwrap(),
		);

		self.state = Some(pollster::block_on(State::new(window)).unwrap());
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		let Some(state) = &mut self.state else {
			return;
		};

		state.keyboard_input(&event);

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(size) => state.resize(size.width, size.height),
			WindowEvent::RedrawRequested => {
				state.input();
				state.update();
				match state.render() {
					Ok(_) => (),
					Err(e) => {
						log::error!("{e}");
						event_loop.exit();
					}
				}
			}
			_ => (),
		}
	}
}
