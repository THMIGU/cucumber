use std::borrow::Cow;

use anyhow::Context;
use rust_embed::Embed;

use crate::error::AppResult;

#[derive(Embed)]
#[folder = "assets/textures"]
pub struct TextureFiles;

pub fn bytes(filename: &str) -> AppResult<Cow<'static, [u8]>> {
	let asset = TextureFiles::get(filename).context("Failed to load texture")?;
	Ok(asset.data)
}
