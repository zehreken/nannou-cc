use nannou::prelude::*;
use rand;

pub fn start_a2() {
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
        .with_title("a2")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .event(event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
    Model {
        random_seed: 1,
        window: window,
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let t = app.time;
    let draw = app.draw();

    let camera_position = Vector3::new(0.0, 0.0, 0.0);
    let v0 = Vector3::new(-1.0, 1.0, 2.0);
    let v1 = Vector3::new(1.0, 1.0, 2.0);
    let v2 = Vector3::new(1.0, -1.0, 2.0);
    let v3 = Vector3::new(-1.0, -1.0, 2.0);
    let v4 = Vector3::new(-1.0, 1.0, 4.0);
    let v5 = Vector3::new(1.0, 1.0, 4.0);
    let v6 = Vector3::new(1.0, -1.0, 4.0);
    let v7 = Vector3::new(-1.0, -1.0, 4.0);
    let mut vertices = vec![v0, v1, v2, v3, v4, v5, v6, v7];

    for i in 0..vertices.len() {
        vertices[i] = rotate_around_y(vertices[i], 0.4 * t, Vector3::new(0.0, 0.0, 3.0));
    }

    // for i in 0..vertices.len() {
    //     vertices[i] = translate_vector(vertices[i], Vector3::new(1.5 * t.cos(), 0.0, 0.0));
    // }

    // for i in 0..vertices.len() {
    //     vertices[i] = rotate_vector(vertices[i], 0.1 * t);
    // }

    let indices = vec![
        0, 1, 2, 0, 2, 3, // front
        4, 5, 7, 5, 6, 7, // back
        4, 5, 1, 4, 1, 0, // top
        3, 2, 6, 3, 6, 7, // bottom
        0, 4, 7, 0, 7, 3, // left
        1, 5, 2, 5, 6, 2, // right
    ];

    let n_points = vertices.len();
    let points: Vec<Point2> = (0..n_points)
        .map(|i| {
            let vertex = vertices[i];
            let p = get_projected_position(camera_position, vertex, 1.0);
            pt2(p.x * 256.0, p.y * 256.0)
        })
        .collect();

    for i in (0..indices.len()).step_by(3) {
        let _points = vec![
            points[indices[i]],
            points[indices[i + 1]],
            points[indices[i + 2]],
            points[indices[i]],
        ];
        draw.polygon()
            .color(Rgba::new(1.0 - i as f32 * 0.1, 0.0, i as f32 * 0.05, 0.5))
            .points(_points.clone());
        draw.polyline().color(WHITE).points(_points);
    }

    draw.background().color(WHITE);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn get_projected_position(
    camera_position: Vector3,
    vertex: Vector3,
    view_plane_distance: f32,
) -> Vector3 {
    let mut distance = vertex - camera_position;
    distance = distance * view_plane_distance / distance.z;

    distance
}

fn translate_vector(v: Vector3, delta: Vector3) -> Vector3 {
    Vector3::new(v.x + delta.x, v.y + delta.y, v.z + delta.z)
}

fn rotate_around_x(v: Vector3, angle: f32, axis: Vector3) -> Vector3 {
    let diff = v - axis;
    let rotation = Vector3::new(
        diff.x,
        diff.y * angle.cos() - diff.z * angle.sin(),
        diff.y * angle.sin() + diff.z * angle.cos(),
    );

    rotation + axis
}

fn rotate_around_y(v: Vector3, angle: f32, axis: Vector3) -> Vector3 {
    let diff = v - axis;
    let rotation = Vector3::new(
        diff.x * angle.cos() + diff.z * angle.sin(),
        diff.y,
        -diff.x * angle.sin() + diff.z * angle.cos(),
    );

    rotation + axis
}

fn rotate_around_z(v: Vector3, angle: f32, axis: Vector3) -> Vector3 {
    let diff = v - axis;
    let rotation = Vector3::new(
        diff.x * angle.cos() - diff.y * angle.sin(),
        diff.x * angle.sin() + diff.y * angle.cos(),
        diff.x,
    );

    rotation + axis
}
