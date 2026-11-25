struct CameraUniform {
	view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TransformUniform {
	mat: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> transform: TransformUniform;

struct VertexInput {
	@location(0) pos: vec2<f32>,
	@location(1) uv: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) pos: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
	model: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;

	let mvp = camera.view_proj * transform.mat;
	out.pos = mvp * vec4<f32>(model.pos, 0.0, 1.0);

	out.uv = model.uv;

	return out;
}
