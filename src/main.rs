use std::time;

use macroquad::prelude::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn round(value: f32, factor: f32) -> f32 {
    (value / factor).round() * factor
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Cellular".to_owned(),
        fullscreen: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        sample_count: 64,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h, size, fill, delay) = match args.as_slice() {
        [_, w, h, size, fill, delay] => (
            w.parse().unwrap(),
            h.parse().unwrap(),
            size.parse().unwrap(),
            fill.parse().unwrap(),
            delay.parse().unwrap(),
        ),
        _ => (100, 100, 10.0, 0.5, 100),
    };

    let index = |x, y| y * w + x;

    let mut field = vec![false; w * h];
    let mut old_field = field.clone();

    field.iter_mut().for_each(|cell| {
        *cell = rand::gen_range::<f32>(0.0, 1.0) < fill;
    });

    let delay = time::Duration::from_millis(delay);
    let mut paused = false;
    let mut last_time = time::Instant::now();

    // For more smooth fps
    let fps_n_last = 20.0;
    let mut fps = 1.0 / get_frame_time();

    loop {
        let new_time = time::Instant::now();
        if !paused && new_time - last_time > delay {
            std::mem::swap(&mut field, &mut old_field);
            for y in 0..h {
                for x in 0..w {
                    let start_x = x.saturating_sub(1);
                    let start_y = y.saturating_sub(1);
                    let end_x = (x + 1).min(w - 1);
                    let end_y = (y + 1).min(h - 1);
                    let mut sum = 0;
                    for inner_x in start_x..=end_x {
                        for inner_y in start_y..=end_y {
                            if old_field[index(inner_x, inner_y)] && !(x == inner_x && y == inner_y)
                            {
                                sum += 1;
                            }
                        }
                    }
                    let current_index = index(x, y);
                    let current_cell = old_field[current_index];

                    // N3, S23
                    field[current_index] = matches!((sum, current_cell),(2, true) | (3, _));
                }
            }
            last_time = new_time;
        }

        let (sw, sh) = (screen_width(), screen_height());
        let x_offset = sw / 2.0 - w as f32 * size / 2.0;
        let y_offset = sh / 2.0 - h as f32 * size / 2.0;
        let centered = |x, y| (x * size + x_offset, y * size + y_offset);
        let from_centered = |x, y| ((x - x_offset) / size, (y - y_offset) / size);

        let (mx, my) = mouse_position();

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = from_centered(mx, my);
            if (0.0..w as f32).contains(&x) && (0.0..h as f32).contains(&y) {
                field[index(x as usize, y as usize)] = true;
            }
        }

        clear_background(GRAY);

        let (start_x, start_y) = centered(0.0, 0.0);
        draw_rectangle(start_x, start_y, w as f32 * size, h as f32 * size, BLACK);

        for y in 0..h {
            for x in 0..w {
                let (cx, cy) = centered(x as f32, y as f32);
                if field[index(x, y)] {
                    draw_rectangle(cx, cy, size, size, WHITE);
                }
            }
        }

        draw_circle(mx, my, 10.0, DARKGRAY);
        
        fps = (fps * (fps_n_last - 1.0) + 1.0 / get_frame_time()) / fps_n_last;
        draw_text(
            &format!("{:3.0}", round(fps, 5.0)),
            15.0,
            10.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}