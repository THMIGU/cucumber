use anyhow::Context;
use image::GenericImageView;

use crate::{assets::texture, error::AppResult, graphics::texture::TextureHandle};

pub struct GpuTexture {
	#[allow(dead_code)]
	pub texture: wgpu::Texture,
	#[allow(dead_code)]
	pub view: wgpu::TextureView,
	#[allow(dead_code)]
	pub sampler: wgpu::Sampler,
	pub bind_group: wgpu::BindGroup,
}

impl GpuTexture {
	fn from_image(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		layout: &wgpu::BindGroupLayout,
		img: &image::DynamicImage,
		label: Option<&str>,
	) -> AppResult<Self> {
		let rgba = img.to_rgba8();
		let dimensions = img.dimensions();

		let size = wgpu::Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});

		queue.write_texture(
			wgpu::TexelCopyTextureInfo {
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
				aspect: wgpu::TextureAspect::All,
			},
			&rgba,
			wgpu::TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			size,
		);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::MipmapFilterMode::Nearest,
			..Default::default()
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&sampler),
				},
			],
			label: Some("Texture Bind Group"),
		});

		Ok(Self {
			texture,
			view,
			sampler,
			bind_group,
		})
	}

	fn from_bytes(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		layout: &wgpu::BindGroupLayout,
		bytes: &[u8],
		label: Option<&str>,
	) -> AppResult<Self> {
		let img = image::load_from_memory(bytes).context("Failed to load image from bytes")?;
		Self::from_image(device, queue, layout, &img, label)
	}

	pub fn from_file(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		layout: &wgpu::BindGroupLayout,
		filename: &str,
		label: Option<&str>,
	) -> AppResult<Self> {
		Self::from_bytes(device, queue, layout, &texture::bytes(filename)?, label)
	}
}

pub struct TextureManager {
	textures: Vec<GpuTexture>,
}

impl TextureManager {
	pub fn new() -> Self {
		Self {
			textures: vec![],
		}
	}

	pub fn add_texture(
		&mut self,
		label: Option<&'static str>,
		texture: GpuTexture,
	) -> TextureHandle {
		let id = self.textures.len();
		self.textures.push(texture);

		TextureHandle {
			label,
			id,
		}
	}

	pub fn get(&self, handle: TextureHandle) -> Option<&GpuTexture> {
		self.textures.get(handle.id)
	}
}
