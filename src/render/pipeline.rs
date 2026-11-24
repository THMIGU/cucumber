use crate::render::{bindings::layouts::BindLayouts, shaders, vertex::Vertex};

pub fn create(
	device: &wgpu::Device,
	layouts: &BindLayouts,
	format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
	let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("Render Pipeline Layout"),
		bind_group_layouts: &[Some(&layouts.texture_layout), Some(&layouts.camera_layout)],
		immediate_size: 0,
	});

	device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
		label: Some("Render Pipeline"),
		layout: Some(&render_pipeline_layout),
		vertex: wgpu::VertexState {
			module: &shaders::create_vertex_shader(device),
			entry_point: Some("vs_main"),
			buffers: &[Some(Vertex::desc())],
			compilation_options: wgpu::PipelineCompilationOptions::default(),
		},
		fragment: Some(wgpu::FragmentState {
			module: &shaders::create_fragment_shader(device),
			entry_point: Some("fs_main"),
			targets: &[Some(wgpu::ColorTargetState {
				format,
				blend: Some(wgpu::BlendState::REPLACE),
				write_mask: wgpu::ColorWrites::ALL,
			})],
			compilation_options: wgpu::PipelineCompilationOptions::default(),
		}),
		primitive: wgpu::PrimitiveState {
			topology: wgpu::PrimitiveTopology::TriangleList,
			strip_index_format: None,
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			polygon_mode: wgpu::PolygonMode::Fill,
			unclipped_depth: false,
			conservative: false,
		},
		depth_stencil: None,
		multisample: wgpu::MultisampleState {
			count: 1,
			mask: !0,
			alpha_to_coverage_enabled: false,
		},
		multiview_mask: None,
		cache: None,
	})
}
