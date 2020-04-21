use nannou::prelude::*;
use rand;

const COLORS: [Srgb<u8>; 5] = [GOLD, BURLYWOOD, CORAL, CRIMSON, DARKORANGE];

pub struct Cell {
    pub points: Vec<Point2>,
    pub color: Srgb<u8>,
    state: u32,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
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
