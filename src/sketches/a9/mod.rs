use nannou::prelude::*;

const TITLE: &str = "a9";

pub fn start_a9() {
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

    app.set_loop_mode(LoopMode::loop_ntimes(60));
    Model { window_id }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => capture_frame(app, 0),
        _ => (),
    }
}

fn capture_frame(app: &App, frame: u64) {
    app.main_window()
        .capture_frame(format!("{:02}.png", frame));
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();
    capture_frame(app, app.elapsed_frames());

    draw.background().color(BLACK);

    let colors = vec![CRIMSON, WHITE];

    const RADIUS: f32 = 20.0;
    let cycle = 2.0; //+ (t as i32) as f32;
    for mut c in 1..3 {
        c = 3 - c;
        let wave: Vec<Point2> = (0..360)
            .map(|i| {
                let angle = deg_to_rad(cycle * i as f32 + 6.0 * app.elapsed_frames() as f32 * c as f32); // Part after + is for animation
                let sine_y = angle.sin();
                let radian = deg_to_rad(i as f32);
                let y = radian.sin() * RADIUS + radian.sin() * sine_y * 2.0;
                let x = radian.cos() * RADIUS + radian.cos() * sine_y * 2.0;
                let scale = 5.0 * c as f32;
                pt2(x * scale, y * scale)
            })
            .collect();

        draw.polygon().points(wave).color(colors[c - 1]);
    }

    draw.to_frame(app, &frame).unwrap();
}
