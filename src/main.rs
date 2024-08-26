use macroquad::prelude::*;

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

    let mut r_num: u32 = std::u32::MAX;
    let mut fps_sum = 0.0;
    let mut fps_counter = 0;
    let mut fact_fps = 0.0;
    loop {
        fps_sum += get_fps() as f32;
        fps_counter += 1;
        clear_background(color_u8!(39, 40, 35, 255));
        if old_time + 1.0 < get_time() {
            old_time = get_time();
            fact_fps = fps_sum / fps_counter as f32;
            fps_sum = 0.;
            fps_counter = 0;
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

            if touch.phase == TouchPhase::Ended {
                let (mut mx, mut my) = (touch.position.x, touch.position.y);
                mx = mx / screen_width() * 2.0 - 1.0;
                mx = mx / 0.3 - SQRT_3 * 0.5;
                my = my / screen_height() * 2.0 - 1.0;
                my = my / (0.3 * screen_width() / screen_height()) + 0.5;
                my = -my;
                let index = stage.sg.get_segment_index(vec2(mx, my));
                if index >= 0 {
                    r_num ^= 1 << index;
                }
            }
        }
        draw_text(format!("FPS: {}", fact_fps).as_str(), 0., 32., 64., RED);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mut mx, mut my) = mouse_position();
            mx = mx / screen_width() * 2.0 - 1.0;
            mx = mx / 0.3 - SQRT_3 * 0.5;
            my = my / screen_height() * 2.0 - 1.0;
            my = my / (0.3 * screen_width() / screen_height()) + 0.5;
            my = -my;
            let index = stage.sg.get_segment_index(vec2(mx, my));
            if index >= 0 {
                r_num ^= 1 << index;
            }
        }

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            for j in -0..1 {
                for i in -0..1 {
                    let dx = SQRT_3 * 0.5 + (i as f32 + (j % 2) as f32 * 0.5) * SQRT_3;
                    let dy = 0.5 + 1.5 * j as f32;

                    gl.quad_context
                        .apply_uniforms(miniquad::UniformsSource::table(
                            &raw_miniquad::shader::Uniforms {
                                scale: (0.3, 0.3 * screen_width() / screen_height()),
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
                        64,
                    );
                }
            }

            gl.quad_context.end_render_pass();
        }

        next_frame().await
    }
}
