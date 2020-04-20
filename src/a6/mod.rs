use nannou::prelude::*;
use rand;

const COLORS: [Srgb<u8>; 5] = [GOLD, BURLYWOOD, CORAL, CRIMSON, DARKORANGE];

pub fn start_a6() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: WindowId,
    duration: f32,
    max: usize,
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
    Model {
        window,
        duration: 0.0,
        max: 0,
        quarters,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.duration = model.duration + app.duration.since_prev_update.as_secs_f32();
    model.max = (model.duration * 30.0) as usize;
    if model.max > 11 {
        model.max = 11;
    }
    if model.duration > 3.0 {
        for quarter in model.quarters.iter_mut() {
            quarter.draw(rand::random::<u32>());
        }
        model.duration = 0.0;
        model.max = 0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for row in 0..16 {
        for column in 0..16 {
            let index = row * 16 + column;
            let slice = &model.quarters[index].points[0..model.max];
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
    points: Vec<Point2>,
    color: Srgb<u8>,
    state: u32,
}

impl Quarter {
    pub fn new() -> Quarter {
        Quarter {
            points: Vec::new(),
            color: COLORS[rand::random::<usize>() % 5],
            state: 0,
        }
    }

    pub fn draw(&mut self, mut state: u32) {
        let position;
        let range;
        let mut sign = 1.0;
        state = state % 8;
        match state {
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
            4 => {
                position = pt2(0.0, 32.0);
                range = (0, 90);
                sign = -1.0;
            }
            5 => {
                position = pt2(32.0, 32.0);
                range = (90, 180);
                sign = -1.0;
            }
            6 => {
                position = pt2(32.0, 0.0);
                range = (180, 270);
                sign = -1.0;
            }
            7 => {
                position = pt2(0.0, 0.0);
                range = (270, 360);
                sign = -1.0;
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
                let radian = deg_to_rad(sign * i as f32);
                pt2(
                    position.x + radian.cos() * 32.0,
                    position.y + radian.sin() * 32.0,
                )
            })
            .collect();
        self.points.insert(0, position);
    }
}
