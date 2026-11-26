use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

const VERTICES: [Vertex; 4] = [
	Vertex {
		pos: [0.0, 1.0],
		uv: [0.0, 1.0],
	},
	Vertex {
		pos: [0.0, 0.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		pos: [1.0, 0.0],
		uv: [1.0, 0.0],
	},
	Vertex {
		pos: [1.0, 1.0],
		uv: [1.0, 1.0],
	},
];
#[rustfmt::skip]
const INDICES: [u16; 6] = [
	0, 1, 2,
	0, 2, 3,
];

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
	pub pos: [f32; 2],
	pub uv: [f32; 2],
}

impl Vertex {
	pub fn desc() -> wgpu::VertexBufferLayout<'static> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2,
				},
				wgpu::VertexAttribute {
					offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x2,
				},
			],
		}
	}
}

pub struct Buffers {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
}

impl Buffers {
	pub fn new(device: &wgpu::Device) -> Self {
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(&VERTICES),
			usage: wgpu::BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(&INDICES),
			usage: wgpu::BufferUsages::INDEX,
		});

		Self {
			vertex_buffer,
			index_buffer,
		}
	}
}
