use nannou::math::Matrix4;
use nannou::prelude::*;
use rand;
use std::cmp::min;

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

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("event: {:?}", event);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let angle_of_view = 90.0;
    let near = 0.1;
    let far = 100.0;
    let m_proj = set_projection_matrix(angle_of_view, near, far);
    let mut world_to_camera: Matrix4<f32> = Matrix4::zero();
    world_to_camera[3][1] = -10.0;
    world_to_camera[3][2] = -20.0;

    let v0: Vector3<f32> = Vector3::new(0.0, 0.0, -10.0);
    let v1: Vector3<f32> = Vector3::new(-0.5, 0.0, 0.0);
    let vertices = vec![v0];

    for vertex in vertices {
        let vert_camera = mul_point_matrix(vertex, world_to_camera);
        let vert_projected = mul_point_matrix(vert_camera, m_proj);

        let x: i32 = min(512 - 1, ((vert_projected.x + 1.0) * 0.5 * 512.0) as i32);
        let y: i32 = min(
            512 - 1,
            ((1.0 - (vert_projected.y + 1.0) * 0.5) * 512.0) as i32,
        );

        println!("{:?}, {:?}, {:?}, {}, {}", vertex, vert_camera, vert_projected, x, y);

        draw.rect().w_h(4.0, 4.0).x_y(x as f32, y as f32).color(WHITE);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn set_projection_matrix(angle_of_view: f32, near: f32, far: f32) -> Matrix4<f32> {
    let scale: f32 = 1.0 / (angle_of_view * 0.5 * 3.141592 / 180.0).tan();
    let mut m: Matrix4<f32> = Matrix4::zero();
    m[0][0] = scale;
    m[1][1] = scale;
    m[2][2] = -far / (far - near);
    m[3][2] = -far * near / (far - near);
    m[2][3] = -1.0;
    m[3][3] = 0.0;

    m
}

fn mul_point_matrix(vec: Vector3<f32>, m: Matrix4<f32>) -> Vector3<f32> {
    let mut out: Vector3<f32> = Vector3::zero();
    out.x = vec.x * m[0][0] + vec.y * m[1][0] + vec.z * m[2][0] + m[3][0];
    out.y = vec.x * m[0][1] + vec.y * m[1][1] + vec.z * m[2][1] + m[3][1];
    out.z = vec.x * m[0][2] + vec.y * m[1][2] + vec.z * m[2][2] + m[3][2];
    let w: f32 = vec.x * m[0][3] + vec.y * m[1][3] + vec.z * m[2][3] + m[3][3];

    if w != 1.0 {
        out.x /= w;
        out.y /= w;
        out.z /= w;
    }

    out
}
