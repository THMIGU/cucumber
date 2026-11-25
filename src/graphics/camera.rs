use crate::core::transform::Transform;

pub struct Camera {
	pub label: Option<&'static str>,
	pub transform: Transform,
}

impl Camera {
	pub fn new(label: Option<&'static str>) -> Self {
		Self {
			label,
			transform: Default::default(),
		}
	}
}
