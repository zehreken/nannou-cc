use nannou::prelude::*;
use rand;

pub fn start_a0() {
    nannou::app(model).run();
}

struct Model {
    random_seed: u32,
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .with_dimensions(512, 512)
        .with_title("nannou")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
    Model {
        random_seed: 1,
        window: window,
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("event: {:?}", event);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    for i in 0..16 {
        for j in 0..16 {
            draw.ellipse()
                .w_h(
                    30.0 + 12.0 * t.cos() * rand::random::<f32>(),
                    30.0 + 12.0 * t.cos() * rand::random::<f32>(),
                )
                .x_y(
                    -256.0 + i as f32 * 32.0 + 16.0 + rand::random::<f32>(),
                    -256.0 + j as f32 * 32.0 + 16.0 + rand::random::<f32>(),
                )
                .color(rgb(
                    0.9 + rand::random::<f32>() * 0.1,
                    0.7 + rand::random::<f32>() * 0.3,
                    0.0,
                ));
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
