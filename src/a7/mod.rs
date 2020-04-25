use nannou::prelude::*;

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
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = (0..360).map(|i| {
        let radian = deg_to_rad(20.0 * i as f32);
        let y = radian.sin();
        let x = -1.0 + i as f32 / 180.0;
        let scale = 100.0;
        pt2(x * scale, y * scale)
    });

    draw.polyline().points(points).color(GOLD);

    draw.to_frame(app, &frame).unwrap();
}
