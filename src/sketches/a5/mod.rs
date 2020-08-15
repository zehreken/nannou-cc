use nannou::prelude::*;

const TITLE: &str = "a5";

pub fn start_a5() {
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

    Model { window_id }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => app
            .main_window()
            .capture_frame_threaded(format!("{}.png", TITLE)),
        _ => (),
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let colors = vec![GOLD, CRIMSON, WHITE, ORANGE, CORNFLOWERBLUE, PURPLE];

    let cycle = 1.0 + (t as i32) as f32;
    for mut c in 1..=6 {
        c = 7 - c;
        let wave: Vec<Point2> = (0..360)
            .map(|i| {
                let angle = deg_to_rad(50.0 * t * c as f32 + cycle * i as f32);
                let sine_y = angle.sin();
                let radian = deg_to_rad(i as f32);
                let val = radian.sin() * 20.0 + radian.sin() * sine_y * 2.0;
                let x = radian.cos() * 20.0 + radian.cos() * sine_y * 2.0; // * (-PI * cycle + cycle * i as f32 * 2.0 * PI as f32 / 360.0);
                let scale = 2.0 * c as f32;
                pt2(x * scale, val * scale)
            })
            .collect();

        draw.polygon().points(wave).color(colors[c - 1]);
    }

    draw.to_frame(app, &frame).unwrap();
}
