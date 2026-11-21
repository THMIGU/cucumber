struct CameraUniform {
	mat: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
	@location(0) pos: vec3<f32>,
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

	out.uv = model.uv;
	out.pos = camera.mat * vec4<f32>(model.pos, 1.0);

	return out;
}
