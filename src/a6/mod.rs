use nannou::prelude::*;

pub fn start_a6() {
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
    Model { window }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.rect().w_h(32.0, 32.0).color(GOLD);

    draw.to_frame(app, &frame).unwrap();
}
