use std::sync::Arc;

use anyhow::{Context, bail};
use winit::window::Window;

use crate::{
	error::AppResult,
	graphics::{
		backend::{
			context::WgpuContext,
			renderer::Renderer,
			texture::{GpuTexture, TextureManager},
		},
		camera::Camera,
		sprite::Sprite,
		texture::TextureHandle,
	},
};

pub struct WgpuBackend {
	context: WgpuContext,
	textures: TextureManager,
	renderer: Renderer,
}

impl WgpuBackend {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let context = WgpuContext::new(window)
			.await
			.context("Failed to create GPU context")?;
		let textures = TextureManager::new();
		let renderer = Renderer::new(&context.device, context.surface_config.format);

		Ok(Self {
			context,
			textures,
			renderer,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			let context = &mut self.context;

			let device = &context.device;
			let surface = &context.surface;
			let config = &mut context.surface_config;

			config.width = width;
			config.height = height;

			surface.configure(device, config);
		}
	}

	pub fn create_texture(
		&mut self,
		label: Option<&'static str>,
		filename: &'static str,
	) -> AppResult<TextureHandle> {
		let context = &self.context;
		let device = &context.device;
		let queue = &context.queue;

		let layout = self.renderer.texture_layout();

		let texture = GpuTexture::from_file(device, queue, layout, filename, label)
			.context("Failed to create GPU texture")?;

		Ok(self
			.textures
			.add_texture(label, texture))
	}

	pub fn render(&mut self, camera: &Camera, sprites: &[Sprite]) -> AppResult {
		let context = &self.context;

		let device = &context.device;
		let queue = &context.queue;
		let surface = &context.surface;
		let config = &context.surface_config;

		let Some(_) = surface.get_configuration() else {
			return Ok(());
		};

		let frame = match surface.get_current_texture() {
			wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
			wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
			wgpu::CurrentSurfaceTexture::Timeout
			| wgpu::CurrentSurfaceTexture::Occluded
			| wgpu::CurrentSurfaceTexture::Validation => {
				return Ok(());
			}
			wgpu::CurrentSurfaceTexture::Outdated => {
				surface.configure(device, config);
				return Ok(());
			}
			wgpu::CurrentSurfaceTexture::Lost => {
				bail!("Lost device");
			}
		};

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					depth_slice: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.0,
							g: 0.0,
							b: 0.0,
							a: 1.0,
						}),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				occlusion_query_set: None,
				timestamp_writes: None,
				multiview_mask: None,
			});

			self.renderer
				.render(queue, &mut render_pass, &self.textures, camera, sprites);
		}

		queue.submit(std::iter::once(encoder.finish()));
		queue.present(frame);

		Ok(())
	}
}
