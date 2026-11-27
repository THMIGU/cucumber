#[cfg(not(debug_assertions))]
use std::borrow::Cow;

use anyhow::Context;
#[cfg(not(debug_assertions))]
use rust_embed::Embed;

use crate::error::AppResult;

#[cfg(not(debug_assertions))]
#[derive(Embed)]
#[folder = "assets/textures"]
pub struct TextureFiles;

#[cfg(not(debug_assertions))]
pub fn bytes(filename: &str) -> AppResult<Cow<'static, [u8]>> {
	let asset = TextureFiles::get(filename).context("Failed to read texture")?;
	Ok(asset.data)
}

#[cfg(debug_assertions)]
pub fn bytes(filename: &str) -> AppResult<Vec<u8>> {
	let bytes =
		std::fs::read(format!("assets/textures/{}", filename)).context("Failed to read texture")?;
	Ok(bytes)
}
