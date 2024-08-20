use macroquad::prelude::*;
use rand::rand;

mod segment_cell;
mod raw_miniquad;

use segment_cell::{SegmentCell, SQRT_3};

fn conf() -> Conf {
    Conf {
        window_title: "Triangle Segments Demo".into(),
        window_width: 1024 * 2,
        window_height: 768 * 2,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        raw_miniquad::Stage::new(ctx)
    };

    
    let k: f32 = 150.;
    let mut cell = SegmentCell::new(k * 2. / 15.0, (k / 75.0).max(0.5), k, BEIGE);
    let mut old_time = 0.;
    let mut masks = Vec::<u32>::with_capacity((12 + 5) * (12 + 1));

    loop {
        clear_background(color_u8!(39, 40, 35, 255));
        let mut counter = 0;
        for i in -1..4 {
            for j in -0..4 {
                if masks.len() <= counter {
                    masks.push(rand() & rand());
                }
                // cell.use_segments(masks[counter]);
                counter += 1;
                cell.draw(
                    k * SQRT_3 * 0.5 + k * (i as f32 + (j % 2) as f32 * 0.5) * SQRT_3,
                    k * 0.5 + 1.5 * k * j as f32,
                );
            }
        }

        // cell.draw(250.0, 250.0);
        if old_time + 0.3 < get_time() {
            old_time = get_time();
            for i in 0..masks.len() {
                masks[i] = rand() & rand();
            }
        }

        for touch in touches() {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
        }
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 32., 64., RED);

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            let t = get_time();

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            for i in 0..10 {
                let t = t + i as f64 * 0.3;

                gl.quad_context
                    .apply_uniforms(miniquad::UniformsSource::table(
                        &raw_miniquad::shader::Uniforms {
                            offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                            aspect: screen_width() / screen_height(),
                        },
                    ));
                gl.quad_context.draw(0, 6, 1);
            }
            gl.quad_context.end_render_pass();
        }


        next_frame().await
    }
}
