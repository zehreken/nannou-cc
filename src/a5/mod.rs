use nannou::prelude::*;

pub fn start_a5() {
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
    Model { window: window }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let cycle = 8.0;
    for y in 0..10 {
        let wave: Vec<Point2> = (0..360)
            .map(|i| {
                let sign = if y % 2 == 0 {-1.0} else {1.0};
                let angle = deg_to_rad(500.0 * t * sign + cycle * i as f32);
                let val = -22.0 + y as f32 * 5.0 + angle.sin();
                let x = -PI * cycle + cycle * i as f32 * 2.0 * PI as f32 / 360.0;
                let scale = 10.0;
                pt2(x * scale, val * scale)
            })
            .collect();

        draw.polyline().weight(4.0).points(wave).color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}
