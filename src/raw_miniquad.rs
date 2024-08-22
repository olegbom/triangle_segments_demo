use macroquad::miniquad;
use macroquad::miniquad::*;
use glam::{vec2, Vec2};

use crate::segment_cell::SegmentCell;

#[repr(C)]
pub struct VertexQ {
    pub pos: Vec2,
    pub index: f32,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut dyn RenderingBackend) -> Stage {

        let k = 0.3;
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

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
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
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("index", VertexFormat::Float1)
            ],
            shader,
            Default::default(),
        );

        Stage { pipeline, bindings }
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
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("aspect", UniformType::Float1),
                    UniformDesc::new("bitfield", UniformType::Int4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
        pub aspect: f32,
        pub bitfield: (i32, i32, i32, i32),
    }
}
