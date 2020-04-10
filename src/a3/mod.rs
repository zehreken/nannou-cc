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

    for n in 1..11 {
        let points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos() * 25.0 * n as f32;
            let y = radian.sin() * 25.0 * n as f32;
            (pt2(x, y), WHITE)
        });
        draw.polyline().weight(1.0).colored_points(points);
    }

    for i in 1..11 {
        draw.ellipse()
            .w_h(10.0, 10.0)
            .x_y((t * 0.2 * i as f32).cos() * 25.0 * i as f32, (t * 0.2 * i as f32).sin() * 25.0 * i as f32)
            .color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
