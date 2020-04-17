use nannou::prelude::*;

pub fn start_a5() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a5")
        .view(view)
        .build()
        .unwrap();
    Model { window: window }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let cycle = 8.0;
    let wave: Vec<Point2> = (0..=360)
        .step_by(2)
        .map(|i| {
            let angle = deg_to_rad(500.0 * t + cycle * i as f32);
            let radian = deg_to_rad(i as f32);
            let val = radian.sin() * 20.0 + angle.sin();
            let x = radian.cos() * 20.0 + angle.cos();// * (-PI * cycle + cycle * i as f32 * 2.0 * PI as f32 / 360.0);
            let scale = 10.0;
            pt2(x * scale, val * scale)
        })
        .collect();

    draw.polyline().points(wave).color(YELLOW);

    draw.to_frame(app, &frame).unwrap();
}
