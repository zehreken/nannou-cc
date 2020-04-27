use nannou::prelude::*;
use std::collections::VecDeque;

pub fn start_a7() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a7")
        .view(view)
        .build()
        .unwrap();

    Model { window }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = 44.0 * app.duration.since_start.as_secs_f32();
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = (0..360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.cos();
        let y = radian.sin();
        let scale = 100.0;
        pt2(x * scale, y * scale)
    });
    draw.polyline().points(points).color(GREEN);

    let radian = deg_to_rad(1.0 * t);
    let scale = 100.0;
    let (x, y) = (radian.cos(), radian.sin());
    let points = vec![pt2(0.0, 0.0), pt2(scale * x, scale * y)];
    draw.polyline().points(points).color(WHITE);

    let radian = deg_to_rad(4.0 * t);
    let (_x, _y) = (x + radian.cos() * 0.2, y + radian.sin() * 0.2);
    let points = vec![pt2(scale * x, scale * y), pt2(scale * _x, scale * _y)];
    draw.polyline().points(points).color(WHITE);

    let radian = deg_to_rad(16.0 * t);
    let (__x, __y) = (_x + radian.cos() * 0.2, _y + radian.sin() * 0.2);
    let points = vec![pt2(scale * _x, scale * _y), pt2(scale * __x, scale * __y)];
    draw.polyline().points(points).color(WHITE);

    let mut points: [f32; 360] = [0.0; 360];
    let t = app.duration.since_start.as_secs() as usize;
    for s in (1..=t).step_by(2) {
        for i in 0..360 {
            let radian = deg_to_rad((s * i) as f32);
            let y = radian.sin() / s as f32;
            points[i] += y;
        }
    }
    let points = (0..360).map(|i| {
        let x = -1.0 + i as f32 / 180.0;
        let y = points[i];
        pt2(x * scale, y * scale)
    });

    draw.polyline().points(points).color(GOLD);

    draw.to_frame(app, &frame).unwrap();
}
