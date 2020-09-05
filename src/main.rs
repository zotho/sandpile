use macroquad::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Triangles".to_owned(),
        fullscreen: true,
        window_width: 1920,
        window_height: 1080,
        sample_count: 64,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (w, h) = (screen_width(), screen_height());

    let n: usize = 10;
    let mut points = Vec::with_capacity((n - 1).pow(2));
    for i in 1..n {
        let pw = i as f32 * w / n as f32;
        for j in 1..n {
            let ph = j as f32 * h / n as f32;
            points.push(vec2(pw, ph));
        }
    }

    let line_w = 4.0;

    loop {
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

        for (i, p) in points.iter_mut().enumerate() {
            *p.x_mut() += (i % 2) as f32;
            *p.x_mut() = *p.x_mut() % w;
            *p.y_mut() += ((i+1) % 2) as f32;
            *p.y_mut() = *p.y_mut() % h;
        }


        clear_background(RED);

        for p in points.iter() {
            draw_poly(p.x(), p.y(), 10, line_w / 2.0, 0.0, WHITE);
            let mut other_points = points.iter()
                .map(|other_p| ((*p - *other_p).length_squared(), *other_p))
                .collect::<Vec<(f32, Vec2)>>();
            
            other_points[..].sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
            
            if let [_, (_, a), (_, b), (_, c)] = other_points[..4] {
                draw_line(p.x(), p.y(), a.x(), a.y(), line_w, WHITE);
                draw_line(p.x(), p.y(), b.x(), b.y(), line_w, WHITE);
                draw_line(p.x(), p.y(), c.x(), c.y(), line_w, WHITE);
            }
        }

        set_default_camera();
        next_frame().await
    }
}


