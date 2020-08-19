use super::sketch_utils::*;
use nannou::prelude::*;
use rand::{rngs::SmallRng, Rng};

const TITLE: &str = "a10";

pub fn start_a10() {
    nannou::app(model).run();
}

struct Model {
    rands: Vec<f32>,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(512, 512)
        .key_pressed(key_pressed)
        .title(TITLE)
        .view(view)
        .build()
        .unwrap();

    // app.set_loop_mode(LoopMode::loop_ntimes(60));
    let mut rng: SmallRng = rand::SeedableRng::seed_from_u64(1);
    Model {
        rands: (1..20).map(|_| 10.0 + rng.gen::<f32>() * 10.0).collect(),
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => capture_frame(app, 0),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // let t = app.time;
    let t = app.elapsed_frames() as f32;
    let draw = app.draw();
    // capture_frame(app, app.elapsed_frames());

    draw.background().color(BLACK);

    const RADIUS: f32 = 30.0;
    let cycle = 1.0; //+ (t as i32) as f32;
    let n = 12;
    for mut c in 1..n {
        c = n - c;
        let wave: Vec<Point2> = (0..=360)
            .map(|i| {
                let factor = if i < 60 {
                    i as f32 / 60.0
                } else if i > 300 {
                    (360.0 - i as f32) / 60.0
                } else {
                    1.0
                };
                let secondary = deg_to_rad(model.rands[c - 1] * i as f32 + 6.0 * 2.0);
                let primary = deg_to_rad(cycle * i as f32 + 6.0 * -t as f32); // Part after + is for animation
                let sine_y = primary.sin();
                let sine_y = sine_y - secondary.sin() * factor;
                let radian = deg_to_rad(i as f32);
                let y = radian.sin() * RADIUS + radian.sin() * sine_y * 2.0;
                let x = radian.cos() * RADIUS + radian.cos() * sine_y * 2.0;
                let scale = 1.0 * c as f32;
                pt2(x * scale, y * scale)
            })
            .collect();

        draw.polyline().weight(1.0).points(wave.clone()).color(WHITE);
        // draw.polyline().weight(5.0).points(wave).color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
