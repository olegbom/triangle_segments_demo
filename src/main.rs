use macroquad::prelude::*;
use std::f32::consts::{SQRT_2, PI};

const SQRT_3: f32 = 1.73205080757;

fn conf() -> Conf {
    Conf {
        window_title: "Triangle Segments Demo".into(),
        window_width: 1024*2,
        window_height: 768*2,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    }
}

pub fn draw_segment(x: f32, y: f32, a: f32, b: f32, k: f32, color: Color) {
    let gl = unsafe { get_internal_gl().quad_gl };


    let push_vec2 = |list: &mut Vec<Vertex>, v: Vec2|
    {
        list.push(Vertex::new(v.x, v.y, 0.,0.,0., color));
    };

    let mut vertices_seg1 = Vec::<Vertex>::with_capacity(6);
    let mut indices = Vec::<u16>::with_capacity(12);
    
    push_vec2(&mut vertices_seg1, Vec2::new(0., b));
    push_vec2(&mut vertices_seg1, Vec2::new(a * 0.5, a * SQRT_3 * 0.5 + b));
    push_vec2(&mut vertices_seg1, Vec2::new(a * 0.5, k - b * SQRT_2 / (SQRT_3 - 1.) -  a * 0.5 / (2. - SQRT_3)));
    push_vec2(&mut vertices_seg1, Vec2::new(0., k - b * SQRT_2 / (SQRT_3 - 1.)));
    push_vec2(&mut vertices_seg1, Vec2::new(-a * 0.5, k - b * SQRT_2 / (SQRT_3 - 1.) - a * 0.5 / (2. - SQRT_3)));
    push_vec2(&mut vertices_seg1, Vec2::new(-a * 0.5, a * SQRT_3 * 0.5 + b));
    indices.extend_from_slice(&[0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5]);

    let mut vertices_seg2 = Vec::<Vertex>::with_capacity(6);
    vertices_seg2.push(Vertex::new(0., b, 0., 0., 0., color));
    vertices_seg2.push(Vertex::new(a * 0.5, a * SQRT_3 * 0.5 + b, 0., 0., 0., color));
    vertices_seg2.push(Vertex::new(a * 0.5, k * 0.5 - b * SQRT_2 * 0.5 - a * 0.5, 0., 0., 0., color));
    vertices_seg2.push(Vertex::new(0., k * 0.5 - b * SQRT_2 * 0.5, 0., 0., 0., color));
    vertices_seg2.push(Vertex::new(-a * 0.5, k * 0.5 - b * SQRT_2 * 0.5 - a * 0.5, 0., 0., 0., color));
    vertices_seg2.push(Vertex::new(-a * 0.5, a * SQRT_3 * 0.5 + b, 0., 0., 0., color));
        
    

    gl.push_model_matrix(Mat4::from_translation(Vec3::new(x, y, 0.0)));
    for i in 0..3 {
        gl.push_model_matrix(Mat4::from_rotation_z(i as f32 * 2. * PI / 3.));
        gl.texture(None);
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(&vertices_seg1, &indices);
        gl.pop_model_matrix();

        gl.push_model_matrix(Mat4::from_rotation_z(i as f32 * 2. * PI / 3. + PI / 3.));
        gl.texture(None);
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(&vertices_seg2, &indices);
        gl.pop_model_matrix();
    }

    gl.pop_model_matrix();
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        clear_background(GRAY);    
        draw_segment(350.0, 350.0, 30., 2., 250., BEIGE);          
        
 
        next_frame().await
    }
}
