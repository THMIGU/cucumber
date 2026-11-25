use std::time::{Duration, Instant};

pub struct FPS {
	last_frame: Instant,
	fps_timer: Duration,
	fps_frames: u32,
	display_fps: f64,
}

impl FPS {
	pub fn new() -> Self {
		Self {
			last_frame: Instant::now(),
			fps_timer: Duration::ZERO,
			fps_frames: 0,
			display_fps: 0_f64,
		}
	}

	pub fn fps(&mut self) -> f64 {
		self.fps_timer += self.last_frame.elapsed();
		self.last_frame = Instant::now();

		self.fps_frames += 1;

		if self.fps_timer > Duration::from_secs(1) {
			self.display_fps = self.fps_frames as f64 / self.fps_timer.as_secs_f64();

			self.fps_timer = Duration::ZERO;
			self.fps_frames = 0;
		}

		self.display_fps
	}
}
