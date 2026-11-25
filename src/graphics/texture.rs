#[derive(Clone, Copy)]
pub struct TextureHandle {
	pub label: Option<&'static str>,
	pub id: usize,
}
