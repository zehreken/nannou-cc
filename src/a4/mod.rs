use nannou::prelude::*;
use rand;

pub fn start_a4() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a3")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
    Model { window: window }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let points = (0..=14400).step_by(10).map(|i| {
        let radian = deg_to_rad(t.cos() * 0.5 * i as f32);
        let x = radian.cos() * i as f32 * 0.03;
        let y = radian.sin() * i as f32 * 0.03;
        (pt2(x, y), rgba(0.0, 0.0, 1.0, 0.8))
    });
    draw.polyline().weight(8.0).colored_points(points);

    let points = (0..=14400).step_by(10).map(|i| {
        let radian = deg_to_rad(t.cos() * 0.5 * i as f32 + 120.0);
        let x = radian.cos() * i as f32 * 0.03;
        let y = radian.sin() * i as f32 * 0.03;
        (pt2(x, y), rgba(0.0, 1.0, 0.0, 0.8))
    });
    draw.polyline().weight(8.0).colored_points(points);

    let points = (0..=14400).step_by(10).map(|i| {
        let radian = deg_to_rad(t.cos() * 0.5 * i as f32 + 240.0);
        let x = radian.cos() * i as f32 * 0.03;
        let y = radian.sin() * i as f32 * 0.03;
        (pt2(x, y), rgba(1.0, 0.0, 0.0, 0.8))
    });
    draw.polyline().weight(8.0).colored_points(points);

    draw.to_frame(app, &frame).unwrap();
}
