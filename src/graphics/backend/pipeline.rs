pub struct Pipeline {
	pub pipeline: wgpu::RenderPipeline,
	pub texture_layout: wgpu::BindGroupLayout,
	pub camera_layout: wgpu::BindGroupLayout,
	pub transform_layout: wgpu::BindGroupLayout,
}
