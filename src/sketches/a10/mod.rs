use super::sketch_utils::*;
use nannou::prelude::*;

const TITLE: &str = "a10";

pub fn start_a10() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
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
    Model { window_id }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => capture_frame(app, 0),
        _ => (),
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // let t = app.time;
    let t = app.elapsed_frames() as f32 / 10.0;
    let draw = app.draw();
    // capture_frame(app, app.elapsed_frames());

    draw.background().color(CRIMSON);

    const RADIUS: f32 = 10.0;
    let cycle = 10.0; //+ (t as i32) as f32;
    for mut c in 1..2 {
        c = 5 - c;
        let wave: Vec<Point2> = (0..=360)
            .map(|i| {
                let angle = deg_to_rad(cycle * i as f32 + 6.0 * t * c as f32); // Part after + is for animation
                let sine_y = angle.sin();
                let radian = deg_to_rad(i as f32);
                let y = radian.sin() * RADIUS + radian.sin() * sine_y * 2.0;
                let x = radian.cos() * RADIUS + radian.cos() * sine_y * 2.0;
                let scale = 5.0 * c as f32;
                pt2(x * scale, y * scale)
            })
            .collect();

        draw.polyline().weight(10.0).points(wave).color(WHITE);
        // draw.polygon().points(wave).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
