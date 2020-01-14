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

    // for i in 0..vertices.len() {
    //     vertices[i] = rotate_vector(vertices[i], 1.0);
    // }
    // for vertex in vertices {
    //     let p = get_projected_position(camera_position, vertex, 1.0);
    //     println!("{:?}", p);

    //     draw.rect()
    //         .w_h(4.0, 4.0)
    //         .x_y(p.x * 100.0, p.y * 100.0)
    //         .color(WHITE);
    // }

    let n_points = vertices.len();
    let points: Vec<Point2> = (0..n_points)
        .map(|i| {
            let vertex = vertices[i];
            let p = get_projected_position(camera_position, vertex, 1.0);
            pt2(p.x * 100.0, p.y * 100.0)
        })
        .collect();

    draw.line()
        .color(PALEGOLDENROD)
        .points(points[0], points[1]);

    draw.background().color(BLACK);

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

fn rotate_vector(v: Vector3, angle: f32) -> Vector3 {
    Vector3::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
        v.z,
    )
}
