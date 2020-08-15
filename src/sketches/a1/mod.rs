use nannou::prelude::*;
use rand;

pub fn start_a1() {
    nannou::app(model).run();
}

struct Model {
    random_seed: u32,
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a1")
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

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    const SIZE: f32 = 8.0;
    for row in 0..64 {
        for column in 0..64 {
            draw.rect()
                .w_h(SIZE, SIZE)
                .x_y(-252.0 + column as f32 * SIZE, -252.0 + row as f32 * SIZE)
                .color(rgb(
                    0.8 + rand::random::<f32>() * 0.2,
                    0.0,
                    0.7 + rand::random::<f32>() * 0.3,
                ));
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
