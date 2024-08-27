use glam::{vec2, Vec2};
use macroquad::miniquad;
use macroquad::miniquad::*;

use crate::segment_cell::{SegmentCell, SQRT_3};

#[repr(C)]
pub struct VertexQ {
    pub pos: Vec2,
    pub index: f32,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
    pub sg: SegmentCell,
}

impl Stage {
    pub fn new(ctx: &mut dyn RenderingBackend) -> Stage {
        let k = 1.0;
        let sg: SegmentCell = SegmentCell::new(k * 2. / 15.0, k / 75.0, k);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&sg.vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&sg.indices[..]),
        );
        let mut coords = vec![Vec2::new(SQRT_3 * 0.5, 0.5); 10000];
        let mut counter = 0;
        for j in -50..50 {
            for i in -50..50 {
                let dx = SQRT_3 * 0.5 + (i as f32 + (j % 2) as f32 * 0.5) * SQRT_3;
                let dy = 0.5 + 1.5 * j as f32;
                coords[counter] = vec2(dx, dy);
                counter += 1;
            }
        }
        let instance_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&coords[..]),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer, instance_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("pos", VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("index", VertexFormat::Float1, 0),
                VertexAttribute::with_buffer("offset", VertexFormat::Float2, 1),
            ],
            shader,
            Default::default(),
        );

        Stage {
            pipeline,
            bindings,
            sg,
        }
    }
}

pub mod shader {
    use macroquad::miniquad::*;

    pub const VERTEX: &str = include_str!("shaders/vertex.glsl");
    pub const FRAGMENT: &str = include_str!("shaders/fragment.glsl");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("scale", UniformType::Float2),
                    UniformDesc::new("bitfield", UniformType::Int4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub scale: (f32, f32),
        pub bitfield: (i32, i32, i32, i32),
    }
}
