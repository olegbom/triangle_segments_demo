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
    let mut scale = 0.08;
    let mut segments_bits = vec![Vec2::new(u16::MAX as f32, u16::MAX as f32); 288];
    for i in 0..segments_bits.len() {
        segments_bits[i] = vec2((0xFFFF) as f32, (0xFFFF) as f32);
    }

    loop {
        fps_sum += get_fps() as f32;
        fps_counter += 1;
        clear_background(color_u8!(39, 40, 35, 255));
        if old_time + 1.0 < get_time() {
            old_time = get_time();
            fact_fps = fps_sum / fps_counter as f32;
            fps_sum = 0.;
            fps_counter = 0;
            if r_num == std::u32::MAX {
                r_num = 0;
            } else {
                r_num = (r_num << 1) | 1;
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

            if touch.phase == TouchPhase::Ended {
                stage.sg.modify_segments_bit(&mut segments_bits, touch.position.x, touch.position.y, scale, true);
                break;
            }
        }
        draw_text(format!("FPS: {}", fact_fps).as_str(), 0., 32., 64., RED);

        if is_mouse_button_down(MouseButton::Left) {
            stage.sg.modify_segments_bit(&mut segments_bits, mouse_position().0, mouse_position().1, scale, true);
        }

        if is_mouse_button_down(MouseButton::Right) {
            stage.sg.modify_segments_bit(&mut segments_bits, mouse_position().0, mouse_position().1, scale, false);
        }

        if is_key_pressed(KeyCode::C) && is_key_down(KeyCode::LeftControl) {
            for i in 0..segments_bits.len() {
                segments_bits[i] = vec2((0xFFFF) as f32, (0xFFFF) as f32);
            }
        }

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            gl.quad_context.buffer_update(
                stage.bindings.vertex_buffers[2],
                miniquad::BufferSource::slice(&segments_bits),
                
            );
            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            gl.quad_context
                .apply_uniforms(miniquad::UniformsSource::table(
                    &raw_miniquad::shader::Uniforms {
                        scale: (scale, scale * screen_width() / screen_height()),
                    },
                ));
            gl.quad_context.draw(
                0,
                (SegmentCell::HEX_NUMBER_OF_INDICES + SegmentCell::TRIANGLE_NUMBER_OF_POINTS)
                    as i32,
                288,
            );

            gl.quad_context.end_render_pass();
        }

        next_frame().await
    }
}
