use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: WindowId,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(512, 512)
        .with_title("nannou")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
    Model { _window }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("event: {:?}", event);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(PLUM);

    for i in 0..9 {
        let radius = 450.0 - i as f32 * 50.0;
        let points = (0..=360).step_by(45).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            pt2(x, y)
        });
        if i % 2 == 0 {
            draw.polygon()
                .color(STEELBLUE)
                .points(points)
                .rotate(-t)
                .x_y(-25.0, -25.0);
        } else {
            draw.polygon().color(PLUM).points(points).rotate(t);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
