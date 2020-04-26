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
    let points = vec![
        pt2(0.0, 0.0),
        pt2(scale * radian.cos(), scale * radian.sin()),
    ];
    draw.polyline().points(points).color(WHITE);

    let points = (0..360).map(|i| {
        let radian = deg_to_rad(t + i as f32);
        let y = radian.sin();
        let x = -1.0 + i as f32 / 180.0;
        let scale = 100.0;
        pt2(x * scale, y * scale)
    });
    draw.polyline().points(points).color(GOLD);

    draw.to_frame(app, &frame).unwrap();
}
