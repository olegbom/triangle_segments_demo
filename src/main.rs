use macroquad::prelude::*;
use rand::rand;

mod segment_cell;

use segment_cell::{SQRT_3, SegmentCell};


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

#[macroquad::main(conf)]
async fn main() {
    let k: f32 = 50.;
    let mut cell = SegmentCell::new(k*2./25.0, 0.5, k, BEIGE);
    let mut old_time = 0.;
    let mut masks = Vec::<u32>::with_capacity((12 + 5) * (12 + 1));

    loop {
        clear_background(color_u8!(39, 40, 35, 255));  
        let mut counter = 0;
        for i in -6..12 {
            for j in -2..12 {
                if masks.len() <= counter {
                    masks.push(rand() & rand());
                }
                cell.use_segments(masks[counter]);
                counter += 1;
                cell.draw(150.0 + k * (i as f32 + j as f32 * 0.5) * SQRT_3 , 150.0 + 1.5 * k * j as f32);  
            }
        }  

        if old_time + 0.3 < get_time() {
            old_time = get_time();
            for i in 0..masks.len() {
                masks[i] = rand() & rand();
            }
        }

        next_frame().await
    }
}
