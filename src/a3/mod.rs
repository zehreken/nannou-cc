use nannou::prelude::*;
use rand;

pub fn start_a3() {
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

    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.cos() * 100.0;
        let y = radian.sin() * 100.0;
        (pt2(x, y), WHITE)
    });
    draw.polyline().weight(1.0).colored_points(points);

    draw.ellipse()
        .w_h(10.0, 10.0)
        .x_y(t.cos() * 100.0, t.sin() * 100.0)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
