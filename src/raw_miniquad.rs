use macroquad::miniquad;
use macroquad::miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut dyn RenderingBackend) -> Stage {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices[..]),
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
                    UniformDesc::new("aspect", UniformType::Float1)
                    ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
        pub aspect: f32,
    }
}
