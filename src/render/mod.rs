pub mod bindings;
pub mod context;
pub mod pipeline;
pub mod shaders;
pub mod surface;
pub mod texture;
pub mod vertex;

use std::sync::Arc;

use anyhow::{Context, bail};
use glam::Vec2;
use wgpu::{RenderPass, util::DeviceExt};
use winit::window::Window;

use crate::{
	error::AppResult,
	render::{
		bindings::{camera::CameraBinding, layouts::BindLayouts},
		context::GPUContext,
		surface::SurfaceState,
		texture::Texture,
		vertex::Vertex,
	},
};

pub struct Render {
	window: Arc<Window>,
	gpu_ctx: GPUContext,
	surface_state: SurfaceState,
	bind_layouts: BindLayouts,
	pipeline: wgpu::RenderPipeline,
	camera_binding: CameraBinding,
}

impl Render {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			flags: Default::default(),
			memory_budget_thresholds: Default::default(),
			backend_options: Default::default(),
			display: None,
		});

		let (surface_state, adapter) = SurfaceState::new(&instance, window.clone())
			.await
			.context("Failed to initialize surface")?;
		let gpu_ctx = GPUContext::new(&adapter)
			.await
			.context("Failed to initialize GPU context")?;

		let bind_layouts = BindLayouts::new(&gpu_ctx.device);
		let pipeline =
			pipeline::create(&gpu_ctx.device, &bind_layouts, surface_state.config.format);
		let camera_binding = CameraBinding::new(&gpu_ctx.device, &bind_layouts.camera_layout);

		Ok(Self {
			window,
			gpu_ctx,
			surface_state,
			bind_layouts,
			pipeline,
			camera_binding,
		})
	}

	pub fn draw_texture(&self, pass: &mut RenderPass, texture: &Texture, pos: Vec2, size: Vec2) {
		let vertices = [
			Vertex {
				pos: (pos.x, size.y + pos.y, 0.0).into(),
				uv: (0.0, 1.0).into(),
			},
			Vertex {
				pos: (pos.x, pos.y, 0.0).into(),
				uv: (0.0, 0.0).into(),
			},
			Vertex {
				pos: (size.x + pos.x, pos.y, 0.0).into(),
				uv: (1.0, 0.0).into(),
			},
			Vertex {
				pos: (size.x + pos.x, size.y + pos.y, 0.0).into(),
				uv: (1.0, 1.0).into(),
			},
		];

		#[rustfmt::skip]
		let indices: [u16; 6] = [
			0, 1, 2,
			0, 2, 3,
		];

		let device = &self.gpu_ctx.device;

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(&indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		pass.set_bind_group(0, &texture.binding, &[]);
		pass.set_bind_group(1, &self.camera_binding.binding, &[]);

		pass.set_vertex_buffer(0, vertex_buffer.slice(..));
		pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

		pass.draw_indexed(0..6, 0, 0..1);
	}

	pub fn render_temp(&mut self) -> AppResult {
		if !self.surface_state.configured {
			return Ok(());
		}

		let output = match self
			.surface_state
			.surface
			.get_current_texture()
		{
			wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
			wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
			wgpu::CurrentSurfaceTexture::Timeout
			| wgpu::CurrentSurfaceTexture::Occluded
			| wgpu::CurrentSurfaceTexture::Validation => {
				return Ok(());
			}
			wgpu::CurrentSurfaceTexture::Outdated => {
				self.surface_state
					.surface
					.configure(&self.gpu_ctx.device, &self.surface_state.config);
				return Ok(());
			}
			wgpu::CurrentSurfaceTexture::Lost => {
				bail!("Lost device");
			}
		};

		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self
			.gpu_ctx
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		let texture = Texture::from_file(
			&self.gpu_ctx,
			&self
				.bind_layouts
				.texture_layout,
			"potato.png",
			Some("Potato Texture"),
		)?;

		let width = self
			.surface_state
			.config
			.width;
		let height = self
			.surface_state
			.config
			.height;
		let ortho = glam::camera::rh::proj::directx::orthographic(
			0.0,
			width as f32,
			height as f32,
			0.0,
			-1.0,
			1.0,
		);

		self.camera_binding
			.uniform
			.mat = ortho.to_cols_array_2d();
		self.gpu_ctx
			.queue
			.write_buffer(
				&self.camera_binding.buffer,
				0,
				bytemuck::cast_slice(&[self.camera_binding.uniform]),
			);

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

			render_pass.set_pipeline(&self.pipeline);
			self.draw_texture(&mut render_pass, &texture, (50.0, 50.0).into(), Vec2::splat(100.0));
		}

		self.gpu_ctx
			.queue
			.submit(std::iter::once(encoder.finish()));
		self.gpu_ctx
			.queue
			.present(output);

		Ok(())
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.surface_state
				.config
				.width = width;
			self.surface_state
				.config
				.height = height;

			self.surface_state
				.surface
				.configure(&self.gpu_ctx.device, &self.surface_state.config);
			self.surface_state.configured = true;
		}
	}
}
