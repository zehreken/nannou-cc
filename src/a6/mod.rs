use nannou::prelude::*;
use rand;

const colors:[Srgb<u8>; 5] = [GOLD, BURLYWOOD, CORAL, CRIMSON, DARKORANGE];

pub fn start_a6() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
    quarters: Vec<Quarter>,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("a6")
        .view(view)
        .build()
        .unwrap();

    let mut quarters: Vec<Quarter> = Vec::new();
    for row in -8..8 {
        for column in -8..8 {
            let mut quarter = Quarter::new();
            quarter.draw(rand::random::<u32>());
            quarters.push(quarter);
        }
    }
    Model { window, quarters }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let mut max = (30.0 * t % 100.0) as usize;
    max = if max > 11 { 11 } else { max };
    for row in 0..16 {
        for column in 0..16 {
            let index = row * 16 + column;
            let slice = &model.quarters[index].points[0..max];
            let color = model.quarters[index].color;
            draw.polygon()
                .points(slice.to_vec())
                .x_y(
                    (column as i32 - 8) as f32 * 32.0,
                    (row as i32 - 8) as f32 * 32.0,
                )
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Quarter {
    position: Point2,
    points: Vec<Point2>,
    color: Srgb<u8>,
}

impl Quarter {
    pub fn new() -> Quarter {
        Quarter {
            position: pt2(0.0, 0.0),
            points: Vec::new(),
            color: colors[rand::random::<usize>() % 5],
        }
    }

    pub fn draw(&mut self, mut q: u32) {
        let position;
        let range;
        q = q % 4;
        match q {
            1 => {
                position = pt2(32.0, 0.0);
                range = (90, 180)
            }
            2 => {
                position = pt2(32.0, 32.0);
                range = (180, 270)
            }
            3 => {
                position = pt2(0.0, 32.0);
                range = (270, 360)
            }
            _ => {
                position = pt2(0.0, 0.0);
                range = (0, 90)
            }
        }
        let (min, max) = range;
        self.points = (min..=max)
            .step_by(10)
            .map(|i| {
                let radian = deg_to_rad(i as f32);
                pt2(
                    position.x + radian.cos() * 32.0,
                    position.y + radian.sin() * 32.0,
                )
            })
            .collect();
        self.points.insert(0, position);
    }
}
