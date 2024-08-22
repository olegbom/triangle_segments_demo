use macroquad::prelude::*;
use rand::rand;

mod raw_miniquad;
mod segment_cell;

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

    let mut old_time = 0.;

    let mut r_num: u32 = 0;
    loop {
        clear_background(color_u8!(39, 40, 35, 255));
        let mut counter = 0;
        for i in -1..4 {
            for j in -0..4 {
                // cell.use_segments(masks[counter]);
                counter += 1;
                // cell.draw(
                //     k * SQRT_3 * 0.5 + k * (i as f32 + (j % 2) as f32 * 0.5) * SQRT_3,
                //     k * 0.5 + 1.5 * k * j as f32,
                // );
            }
        }

        // cell.draw(250.0, 250.0);
        if old_time + 0.3 < get_time() {
            old_time = get_time();

            if r_num != std::u32::MAX {
                r_num = (r_num << 1) | 1;
            } else {
                r_num = 0;
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
            for j in -4..4 {
                for i in -3..2 {
                    let t = t + i as f64 * 0.3;
                    let k = 0.3;
                    let dx = k * SQRT_3 * 0.5 + k * (i as f32 + (j % 2) as f32 * 0.5) * SQRT_3;
                    let dy = k * 0.5 + 1.5 * k * j as f32;

                    gl.quad_context
                        .apply_uniforms(miniquad::UniformsSource::table(
                            &raw_miniquad::shader::Uniforms {
                                offset: (dx, dy),
                                aspect: screen_width() / screen_height(),
                                bitfield: (
                                    (r_num & 0xFF) as i32,
                                    ((r_num >> 8) & 0xFF) as i32,
                                    ((r_num >> 16) & 0xFF) as i32,
                                    ((r_num >> 24) & 0xFF) as i32,
                                ),
                            },
                        ));
                    gl.quad_context.draw(
                        0,
                        (SegmentCell::HEX_NUMBER_OF_INDICES
                            + SegmentCell::TRIANGLE_NUMBER_OF_POINTS)
                            as i32,
                        1,
                    );
                }
            }

            gl.quad_context.end_render_pass();
        }

        next_frame().await
    }
}
