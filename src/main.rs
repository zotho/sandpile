use macroquad::*;


const LINE_WIDTH: bool = false;

fn window_conf() -> Conf {
    Conf {
        window_title: "Triangles".to_owned(),
        fullscreen: true,
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = Vec::new();

    let (w, h) = (screen_width(), screen_height());
    let n = 10;
    for i in 1..n {
        let pw = i as f32 * w / n as f32;
        for j in 1..n {
            let ph = j as f32 * h / n as f32;
            points.push(vec2(pw, ph));
        }
    }

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
            draw_poly(p.x(), p.y(), 3, 2.0, 0.0, YELLOW);
            let mut other_points = points.iter().map(|other_p| ((*p - *other_p).length_squared(), *other_p)).collect::<Vec<(f32, Vec2)>>();
            other_points[..].sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
            if let [_, (al, a), (bl, b), (cl, c)] = other_points[..4] {
                let (al, bl, cl) = (al.sqrt(), bl.sqrt(), cl.sqrt());
                let avgl = (al + bl + cl) / 3.0;
                let coeffw = 4.0;
                let max_w = 10.0;
                let aw = coeffw * avgl / al;
                let bw = coeffw * avgl / bl;
                let cw = coeffw * avgl / cl;

                if LINE_WIDTH {
                    draw_poly(p.x(), p.y(), 40, aw.min(max_w) / 2.0, 0.0, WHITE);
                    draw_line(p.x(), p.y(), a.x(), a.y(), aw.min(max_w), WHITE);
                    draw_line(p.x(), p.y(), b.x(), b.y(), bw.min(max_w), WHITE);
                    draw_line(p.x(), p.y(), c.x(), c.y(), cw.min(max_w), WHITE);
                } else {
                    draw_line(p.x(), p.y(), a.x(), a.y(), coeffw, WHITE);
                    draw_line(p.x(), p.y(), b.x(), b.y(), coeffw, WHITE);
                    draw_line(p.x(), p.y(), c.x(), c.y(), coeffw, WHITE);
                }
            }
        }


        set_default_camera();
        next_frame().await
    }
}


