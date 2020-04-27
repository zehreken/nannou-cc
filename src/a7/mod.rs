use nannou::prelude::*;
use std::collections::VecDeque;

pub fn start_a7() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
    points: VecDeque<Point2>,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a7")
        .view(view)
        .build()
        .unwrap();

    let mut points = VecDeque::new();
    points.extend((0..360).map(|i| {
        let x = -1.0 + i as f32 / 180.0;
        let y = 0.0;
        pt2(100.0 * x, y)
    }));
    Model { window, points }
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
    let (x, y) = (radian.cos(), radian.sin());
    let points = vec![pt2(0.0, 0.0), pt2(scale * x, scale * y)];
    draw.polyline().points(points).color(WHITE);

    let radian = deg_to_rad(4.0 * t);
    let (_x, _y) = (x + radian.cos() * 0.2, y + radian.sin() * 0.2);
    let points = vec![pt2(scale * x, scale * y), pt2(scale * _x, scale * _y)];
    draw.polyline().points(points).color(WHITE);

    let radian = deg_to_rad(16.0 * t);
    let (__x, __y) = (_x + radian.cos() * 0.2, _y + radian.sin() * 0.2);
    let points = vec![pt2(scale * _x, scale * _y), pt2(scale * __x, scale * __y)];
    draw.polyline().points(points).color(WHITE);

    // let points = (0..360).map(|i| {
    //     let mut radian = deg_to_rad(t + i as f32);
    //     let y = radian.sin();
    //     radian = deg_to_rad(4.0 * t + i as f32);
    //     let _y = y + radian.sin() * 0.2;
    //     radian = deg_to_rad(16.0 * t + i as f32);
    //     let __y = _y + radian.sin() * 0.2;
    //     let x = -1.0 + i as f32 / 180.0;
    //     let scale = 100.0;
    //     pt2(x * scale, __y * scale)
    // });
    // draw.polyline().points(points).color(GOLD);

    let points: Vec<Point2> = (0..360)
        .map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = -1.0 + i as f32 / 180.0;
            let y = radian.sin();
            pt2(x, y)
        })
        .collect();
    let mut factor = 3.0;
    let points: Vec<Point2> = (0..360)
        .map(|i| {
            let radian = deg_to_rad(factor * i as f32);
            let mut x = -1.0 + i as f32 / 180.0;
            let mut y = radian.sin() / factor;
            x += points[i].x;
            y += points[i].y;
            pt2(x, y)
        })
        .collect();
    factor += 2.0;
    let points: Vec<Point2> = (0..360)
        .map(|i| {
            let radian = deg_to_rad(factor * i as f32);
            let mut x = -1.0 + i as f32 / 180.0;
            let mut y = radian.sin() / factor;
            x += points[i].x;
            y += points[i].y;
            pt2(x, y)
        })
        .collect();
    factor += 2.0;
    let points: Vec<Point2> = (0..360)
        .map(|i| {
            let radian = deg_to_rad(factor * i as f32);
            let mut x = -1.0 + i as f32 / 180.0;
            let mut y = radian.sin() / factor;
            x += points[i].x;
            y += points[i].y;
            pt2(x, y)
        })
        .collect();
    factor += 2.0;
    let points = (0..360).map(|i| {
        let radian = deg_to_rad(factor * i as f32);
        let mut x = -1.0 + i as f32 / 180.0;
        let mut y = radian.sin() / factor;
        x += points[i].x;
        y += points[i].y;
        x /= 5.0;
        pt2(x * scale, y * scale)
    });

    draw.polyline().points(points).color(GOLD);

    // let mut points: Vec<Point2> = Vec::with_capacity(360);
    // for p in &model.points {
    //     points.push(*p);
    // }
    // draw.polyline().points(points).color(GOLD);

    draw.to_frame(app, &frame).unwrap();
}
