use super::sketch_utils::*;
use nannou::prelude::*;

const TIME_FACTOR: f32 = 10.0;
pub fn start_a11() {
    nannou::app(model).update(update).run();
}

struct Segment {
    pub start: Point2,
    pub end: Point2,
}

impl Segment {
    fn new() -> Self {
        Self {
            start: Point2::zero(),
            end: Point2::zero(),
        }
    }
}

struct Model {
    points: Vec<Point2>,
    segments: Vec<Segment>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .key_pressed(key_pressed)
        .title("a11")
        .view(view)
        .build()
        .unwrap();

    const SEGMENT_COUNT: usize = 10;
    let segments = (0..SEGMENT_COUNT).map(|_| Segment::new()).collect();

    Model {
        points: vec![],
        segments,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => capture_frame(app, 0),
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = TIME_FACTOR * app.duration.since_start.as_secs_f32() + 0.0;
    const SCALE: f32 = 200.0;
    let mut center = pt2(0.0, 0.0);
    let mut point_on_circle = pt2(0.0, 0.0);

    for i in 0..model.segments.len() {
        let scale = SCALE / (model.segments.len() - i) as f32;
        let radian = PI / 2.0 + deg_to_rad(2.0.pow(i as f32) as f32 * t);
        let (x, y) = (
            point_on_circle.x + radian.cos() * scale,
            point_on_circle.y + radian.sin() * scale,
        );

        point_on_circle = pt2(x, y);
        model.segments[i].start = center;
        model.segments[i].end = point_on_circle;
        center = point_on_circle;
    }

    model.points.push(point_on_circle);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    draw = draw.blend(BLEND_NORMAL);
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    /*
    // The circle
    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.cos();
        let y = radian.sin();
        let scale = 100.0;
        pt2(x * scale, y * scale)
    });
    draw.polyline().points(points).color(CRIMSON);
    */
    let colors = vec![
        nannou::color::rgba(1.0, 0.0, 0.21, 0.2),
        nannou::color::rgba(1.0, 0.2, 0.21, 0.2),
        nannou::color::rgba(1.0, 0.4, 0.21, 0.2),
        nannou::color::rgba(0.0, 0.0, 0.21, 0.2),
        nannou::color::rgba(1.0, 1.0, 0.21, 0.2),
        nannou::color::rgba(0.0, 0.8, 0.4, 0.2),
        nannou::color::rgba(0.5, 0.0, 0.21, 0.2),
        nannou::color::rgba(0.9, 0.3, 1.0, 0.2),
        nannou::color::rgba(0.3, 0.3, 0.1, 0.2),
        nannou::color::rgba(1.0, 0.8, 0.0, 0.2),
    ];
    for (i, segment) in model.segments.iter().enumerate() {
        // let points = vec![segment.start, segment.end];
        // draw.polyline()
        //     .weight(1.0)
        //     .points(points)
        //     .color(WHITE);

        // if i == model.segments.len() - 1 {
        draw.ellipse()
            .w_h(30.0 - 3.0 * i as f32, 30.0 - 3.0 * i as f32)
            .x_y(segment.end.x, segment.end.y)
            .color(colors[i % colors.len()]);
        // }
    }

    // draw.polyline()
    //     .weight(5.0)
    //     .points(model.points.clone())
    //     .color(CRIMSON)
    //     .rotate(PI / 2.0);

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
