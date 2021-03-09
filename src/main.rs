use std::fs::{read, write};
use std::path::Path;

use clap::Clap;
use macroquad::prelude::*;

mod field;
use field::Field;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

// const COLORS: [Color; 25] = [
//      YELLOW, GOLD, ORANGE, PINK, RED, MAROON, GREEN, LIME, DARKGREEN,
//     SKYBLUE, BLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, BLACK,
//     BLANK, MAGENTA,
// ];

const COLORS: [Color; 23] = [
    RED, ORANGE, YELLOW, GOLD, GREEN, BLUE, DARKBLUE, PURPLE, VIOLET, PINK, MAROON, LIME,
    DARKGREEN, SKYBLUE, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, LIGHTGRAY, GRAY, DARKGRAY,
    MAGENTA,
];

fn round(value: f32, factor: f32) -> f32 {
    (value / factor).round() * factor
}

fn draw_text_background(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    bg_color: Color,
) -> (f32, f32) {
    let (w, h) = measure_text(text, None, font_size as u16, 1.0);

    let padding = 5.0;
    draw_rectangle(
        x - padding,
        y + padding,
        w + 2.0 * padding,
        h + 2.0 * padding,
        bg_color,
    );

    draw_text(text, x, y, font_size, color);

    (w, h)
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Zotho>")]
struct Opts {
    #[clap(short, long)]
    infile: Option<String>,
    #[clap(short, long, default_value = "output.bin")]
    outfile: String,
    #[clap(short, long, default_value = "500")]
    width: usize,
    #[clap(short, long, default_value = "500")]
    height: usize,
    #[clap(short, long, default_value = "2.0")]
    size: f32,
    #[clap(short, long, default_value = "0.0")]
    delay: f64,
    #[clap(short, long, default_value = "10")]
    num_updates: usize,
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
    let opts: Opts = Opts::parse();
    let infile = opts.infile;
    let outfile = Path::new(&opts.outfile);
    let size = opts.size;
    let delay = opts.delay;
    let num_updates = opts.num_updates;

    let mut field = if let Some(infile) = infile {
        let data = read(Path::new(&infile)).unwrap();
        bincode::deserialize(data.as_slice()).unwrap()
    } else {
        let mut field = Field::new(opts.width, opts.height);
        let index = field.index(field.width / 2, field.height / 2);
        field.inner_field[index] = 1000000;
        field.fill_job_queue();
        field
    };
    // let w = field.width;
    // let h = field.height;

    // Update delay in millis
    let delay = delay / 1000.0;
    let mut paused = false;
    let mut last_time = get_time();

    // For more smooth fps
    let fps_n_last = 20.0;
    let mut fps = 1.0 / get_frame_time();

    let mut debug = true;

    let mut last_x = 0;
    let mut last_y = 0;

    let mut sw;
    let mut sh;
    // let (mut sw, mut sh) = (WIDTH as f32, HEIGHT as f32);

    let mut i = 0;
    let start = get_time();
    let mut elapsed = std::f64::INFINITY;

    loop {
        let new_time = get_time();
        if !paused && new_time - last_time > delay {
            // let (mut swap, mut clone, mut all) = (0.0, 0.0, 0.0);
            for _ in 0..num_updates {
                // let (dswap, dclone, dall) = field.update();
                field.update();
                // swap += dswap;
                // clone += dclone;
                // all += dall;
                
                // if field.job_queue.len() == 0 || i == 89107 {
                //     break;
                // }
                if field.job_queue.len() == 0 {
                    if elapsed.is_infinite() {
                        elapsed = get_time() - start;
                    }
                    break;
                } else {
                    i += 1;
                }
            }
            last_time = new_time;
        }

        let new_sw = screen_width();
        let new_sh = screen_height();

        // if new_sw != sw || new_sh != sh {
        //     field = Field::new((new_sw / size) as usize, (new_sh / size) as usize);

        //     // field.inner_field.iter_mut().for_each(|cell| {
        //     //     *cell = rand::gen_range::<f32>(0.0, 1.0) < fill;
        //     // });
        // }
        sw = new_sw;
        sh = new_sh;

        let x_offset = sw / 2.0 - field.width as f32 * size / 2.0;
        let y_offset = sh / 2.0 - field.height as f32 * size / 2.0;
        let centered = |x, y| (x * size + x_offset, y * size + y_offset);
        let from_centered = |x, y| ((x - x_offset) / size, (y - y_offset) / size);

        let (mx, my) = mouse_position();

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            let data = bincode::serialize(&field).unwrap();
            write(outfile, data).unwrap();
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::H) {
            debug = !debug;
        }

        if is_key_pressed(KeyCode::Enter) {
            field.update();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = from_centered(mx, my);
            let (x, y) = field.check_coords(x, y);
            last_x = x;
            last_y = y;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = from_centered(mx, my);
            let (x, y) = field.check_coords(x, y);
            field.put_line(last_x, last_y, x, y);

            last_x = x;
            last_y = y;
        }

        clear_background(GRAY);

        let (start_x, start_y) = centered(0.0, 0.0);
        draw_rectangle(
            start_x,
            start_y,
            field.width as f32 * size,
            field.height as f32 * size,
            BLACK,
        );

        for y in 0..field.height {
            for x in 0..field.width {
                let (cx, cy) = centered(x as f32, y as f32);
                let count = field.get(x, y);
                if count >= 1 {
                    let color = COLORS[(count as usize - 1) % COLORS.len()];
                    // let color = Color::from_hsl(count as f32 % 4.0 / 4.0, 1.0, 0.5);
                    draw_rectangle(cx, cy, size, size, color);
                }
            }
        }

        draw_circle(mx, my, 10.0, DARKGRAY);

        if debug {
            let mut height = 0.0;
            let mut draw_text_line = |text| {
                let (_w, h) =
                    draw_text_background(text, 15.0, 10.0 + height, 25.0, LIGHTGRAY, BLACK);
                height += h + 15.0;
            };

            fps = (fps * (fps_n_last - 1.0) + 1.0 / get_frame_time()) / fps_n_last;

            let fps_text = format!("FPS: {:3.0}", round(fps, 5.0));
            let elapsed_text = format!("Elapsed: {:3.3}", elapsed.min(get_time() - start));
            let iter_text = format!("Iteration: {}", i);
            let pause_text = format!(
                "{} (space bar to {})",
                if paused { "PAUSED" } else { "PLAYING" },
                if paused { "play" } else { "pause" }
            );

            draw_text_line(&fps_text);
            draw_text_line(&elapsed_text);
            draw_text_line(&iter_text);
            draw_text_line(&pause_text);
            draw_text_line(&"Mouse to draw");
            draw_text_line(&"H to hide this text");
        }

        next_frame().await
    }
}
