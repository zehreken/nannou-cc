use super::sketch_utils::*;
use nannou::prelude::*;

pub fn start_a12() {
    nannou::app(model).update(update).run();
}

struct Model {
    position: f32,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(800, 80)
        .title("a12")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
        app.set_loop_mode(LoopMode::loop_ntimes(400));

    Model {
        position: 0.0,
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("event: {:?}", event);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.position += 1.0;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let speed = app.elapsed_frames() as f32;
    // let speed = speed.sin();
    let draw = app.draw();
    // capture_frame(app, app.elapsed_frames());

    draw.background().color(BLUE);

    const SIZE: f32 = 8.0;

    for row in (0..11).step_by(2) {
        for column in (0..101).step_by(2) {
            draw.rect()
                .w_h(5.0 + SIZE + speed.sin() * 2.0, 5.0 + SIZE + speed.sin() * 2.0)
                .x_y(-400.0 + column as f32 * SIZE, -40.0 + row as f32 * SIZE)
                .rotate(speed.sin())
                .color(RED);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
