use macroquad::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, PI, SQRT_2};

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

struct SegmentCell {
    vertices: Vec<Vertex>,
    indices: Vec<u16>
} 

impl SegmentCell {
    pub fn new(a: f32, b: f32, k: f32, color: Color) -> SegmentCell {
        let mut cell = SegmentCell { 
            vertices: Vec::<Vertex>::with_capacity(6 * 3 * 2 + 6 * 3 * 2),
            indices: Vec::<u16>::with_capacity(3 * 4 * 3 * 2 + 4 * 3 * 3 * 2)
        };
        cell.generate_vertices(a, b, k, color);
        cell.use_segments(std::u32::MAX);
        cell
    }

    pub fn generate_vertices(&mut self, a: f32, b: f32, k: f32, color: Color) {
        let vec2_to_vertex = |v: Vec2| Vertex::new(v.x, v.y, 0.,0.,0., color);
    
        let center_arrow = [
            Vec2::new(a * 0.5, a * SQRT_3 * 0.5 + b),
            Vec2::new(0., b), 
            Vec2::new(-a * 0.5, a * SQRT_3 * 0.5 + b)
        ];
        
        let vertex_arrow = [
            Vec2::new(a * 0.5, - b * SQRT_2 / (SQRT_3 - 1.) - a * 0.5 / (2. - SQRT_3)),
            Vec2::new(0., - b * SQRT_2 / (SQRT_3 - 1.)),
            Vec2::new(-a * 0.5, - b * SQRT_2 / (SQRT_3 - 1.) - a * 0.5 / (2. - SQRT_3)),
        ];
        
        let edge_arrow = [
            Vec2::new(a * 0.5, - b * SQRT_2 * 0.5 - a * 0.5),
            Vec2::new(0., - b * SQRT_2 * 0.5),
            Vec2::new(-a * 0.5, - b * SQRT_2 * 0.5 - a * 0.5)
        ];
    
        self.vertices.clear();
        
        for i in 0..3 {
            
            for j in [0.,1.] {
                let sign = -j * 2.0 + 1.;
                let dv = j * Vec2::from_angle(FRAC_PI_6) * k;
                let rot = Vec2::from_angle(i as f32 * 2.0 * PI / 3.);
                self.vertices.extend(center_arrow.into_iter()
                    .map(|v| v.rotate(rot))
                    .map(|v| v * sign + dv)
                    .map(vec2_to_vertex));
                self.vertices.extend(vertex_arrow.into_iter().rev()
                    .map(|v| v.with_y(v.y + k).rotate(rot))
                    .map(|v| v * sign + dv)
                    .map(vec2_to_vertex));
                
                let rot = Vec2::from_angle(i as f32 * 2.0 * PI / 3. + PI / 3.);
                self.vertices.extend(center_arrow.into_iter()
                    .map(|v| rot.rotate(v))
                    .map(|v| v * sign + dv)
                    .map(vec2_to_vertex));
                self.vertices.extend(edge_arrow.into_iter().rev()
                    .map(|v| v.with_y(v.y + k * 0.5).rotate(rot))
                    .map(|v| v * sign + dv)
                    .map(vec2_to_vertex));
            }
    
    
            let rot = i as f32 * 2.0 * PI / 3.;
            for sign in [-1., 1.] {
                self.vertices.extend(vertex_arrow.into_iter()
                        .map(|v| v.rotate(Vec2::from_angle(FRAC_PI_6 * sign)))
                        .map(|v| v.with_y(v.y + k).rotate(Vec2::from_angle(rot)))
                        .map(vec2_to_vertex));
                    self.vertices.extend(edge_arrow.into_iter()
                    .map(|v| v.rotate(Vec2::from_angle(-FRAC_PI_2 * sign)))
                    .map(|v| v.with_y(v.y + k * 0.5).rotate(Vec2::from_angle(-FRAC_PI_3 * sign + rot)))
                    .map(vec2_to_vertex));
            }
        }
    
        self.indices.clear(); 
        for j in (0..self.vertices.len() as u16).step_by(6) {
            for i in j+1..j+5 {
                self.indices.extend_from_slice(&[j, i, i + 1]);
            }
        } 
    }

    pub fn use_segments(&mut self, bits: u32)
    {
        self.indices.clear(); 
        for j in (0..self.vertices.len() as u16).step_by(6) {
            for i in j+1..j+5 {
                self.indices.extend_from_slice(&[j, i, i + 1]);
            }
        } 
    }

    pub fn draw(&self, x: f32, y: f32) {
        let gl = unsafe { get_internal_gl().quad_gl };
        gl.push_model_matrix(Mat4::from_translation(Vec3::new(x, y, 0.0)));

        gl.texture(None);
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(&self.vertices, &self.indices);
       
        gl.pop_model_matrix();
    }

}

#[macroquad::main(conf)]
async fn main() {
    let cell = SegmentCell::new(30.0, 3.0, 250., BEIGE);

    loop {
        clear_background(GRAY);    
        cell.draw(350.0, 350.0);
        // draw_segment(350.0, 350.0, 30., 10. + (get_time() as f32 * 5.0).sin() * 5.0, 250., BEIGE);          
        
 
        next_frame().await
    }
}
