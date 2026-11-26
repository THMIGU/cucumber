use anyhow::Context;
use hecs::World;

use crate::{
	core::transform::Transform,
	error::AppResult,
	game::components::texture::Texture,
	graphics::{backend::wgpu_backend::WgpuBackend, camera::Camera, sprite::Sprite},
};

pub fn render_system(wgpu: &mut WgpuBackend, camera: &Camera, world: &World) -> AppResult {
	let mut sprites: Vec<Sprite> = vec![];

	for (texture, transform) in world
		.query::<(&Texture, &Transform)>()
		.iter()
	{
		sprites.push(Sprite {
			texture: texture.handle,
			transform: transform.clone(),
		});
	}

	wgpu.render(camera, &sprites)
		.context("Failed to render scene")?;

	Ok(())
}
