use nannou::prelude::*;

const TIME_FACTOR: f32 = 20.0;
pub fn start_a7() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .title("a7")
        .view(view)
        .build()
        .unwrap();

    Model { points: vec![] }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = TIME_FACTOR * app.duration.since_start.as_secs_f32() + 90.0;
    const SCALE: f32 = 100.0;
    let mut point_on_circle = pt2(0.0, 0.0);

    for i in 0..5 {
        let scale = SCALE / (5 - i) as f32;
        let radian = deg_to_rad(2.0.pow(i as f32) as f32 * t);
        let (x, y) = (
            point_on_circle.x + radian.cos() * scale,
            point_on_circle.y + radian.sin() * scale,
        );

        point_on_circle = pt2(x, y);
    }

    model.points.push(point_on_circle);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = TIME_FACTOR * app.duration.since_start.as_secs_f32() + 90.0;
    let draw = app.draw();
    draw.background().color(BLACK);

    /*
    // The circle
    let points = (0..360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.cos();
        let y = radian.sin();
        let scale = 100.0;
        pt2(x * scale, y * scale)
    });
    draw.polyline().points(points).color(GREEN);
    */

    const SCALE: f32 = 100.0;
    let mut center = pt2(0.0, 0.0);
    let mut point_on_circle = pt2(0.0, 0.0);
    for i in 0..5 {
        let scale = SCALE / (5 - i) as f32;
        let radian = deg_to_rad(2.0.pow(i as f32) as f32 * t);
        let (x, y) = (
            point_on_circle.x + radian.cos() * scale,
            point_on_circle.y + radian.sin() * scale,
        );

        point_on_circle = pt2(x, y);
        let points = vec![center, point_on_circle];
        draw.polyline().weight(2.0).points(points).color(WHITE);
        center = point_on_circle;
    }

    draw.polyline()
        .points(model.points.clone())
        .color(CHARTREUSE);

    /*
    let scale = 100.0;
    let mut points: [f32; 360] = [0.0; 360];
    let t = app.duration.since_start.as_secs() as usize;
    // This goes 1, 3, 5, 7... sin(x) + sin(3x)/3 + sin(5x)/5 + sin(7x)/7 + ...
    for s in (1..=t).step_by(2) {
        for i in 0..360 {
            let radian = deg_to_rad((s * i) as f32);
            let y = radian.sin() / s as f32;
            points[i] += y;
        }
    }
    let points = (0..360).map(|i| {
        let x = -1.0 + i as f32 / 180.0;
        let y = points[i];
        pt2(x * scale, y * scale)
    });

    draw.polyline().points(points).color(GOLD);
    */

    draw.to_frame(app, &frame).unwrap();
}
