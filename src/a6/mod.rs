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
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let mut quarters: Vec<Quarter> = Vec::new();
    for row in -8..8 {
        for column in -8..8 {
            let mut quarter = Quarter::new();
            quarter.draw();
            quarters.push(quarter);
        }
    }

    let max = (t % 12.0) as usize;
    for row in -8..8 {
        for column in -8..8 {
            let slice = &quarters[0].points[0..max];
            draw.polygon()
                .points(slice.to_vec())
                .x_y(column as f32 * 32.0, row as f32 * 32.0)
                .color(GOLD);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Quarter {
    position: Point2,
    points: Vec<Point2>,
}

impl Quarter {
    pub fn new() -> Quarter {
        Quarter {
            position: pt2(0.0, 0.0),
            points: Vec::new(),
        }
    }

    pub fn draw(&mut self) {
        self.points = (0..=90)
            .step_by(10)
            .map(|i| {
                let radian = deg_to_rad(i as f32);
                pt2(radian.cos() * 32.0, radian.sin() * 32.0)
            })
            .collect();
        self.points.insert(0, self.position);
    }
}
