use nannou::draw::Draw;
use nannou::prelude::*;
use rand;

// const COLORS: [Srgb<u8>; 5] = [GOLD, BURLYWOOD, CORAL, CRIMSON, DARKORANGE];
const COLORS: [Srgb<u8>; 5] = [
    CRIMSON,
    CORNFLOWERBLUE,
    DODGERBLUE,
    DEEPSKYBLUE,
    MEDIUMSPRINGGREEN,
];
const MOORE_DIRECTIONS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];

#[derive(Copy, Clone)]
enum Quarter {
    TopRightCCW,
    TopLeftCCW,
    BottomLeftCCW,
    BottomRightCCW,
    BottomRightCW,
    BottomLeftCW,
    TopLeftCW,
    TopRightCW,
    Empty,
}

impl Quarter {
    fn int_to_enum(i: u32) -> Self {
        match i {
            0 => Quarter::TopRightCCW,
            1 => Quarter::TopLeftCCW,
            2 => Quarter::BottomLeftCCW,
            3 => Quarter::BottomRightCCW,
            4 => Quarter::BottomRightCW,
            5 => Quarter::BottomLeftCW,
            6 => Quarter::TopLeftCW,
            7 => Quarter::TopRightCW,
            8 => Quarter::Empty,
            _ => Quarter::int_to_enum(i % 9),
        }
    }

    fn value(&self) -> (Point2, Point, i8) {
        match *self {
            Quarter::TopRightCCW => (pt2(0.0, 0.0), Point { x: 0, y: 90 }, 1),
            Quarter::TopLeftCCW => (pt2(32.0, 0.0), Point { x: 90, y: 180 }, 1),
            Quarter::BottomLeftCCW => (pt2(32.0, 32.0), Point { x: 180, y: 270 }, 1),
            Quarter::BottomRightCCW => (pt2(0.0, 32.0), Point { x: 270, y: 360 }, 1),
            Quarter::BottomRightCW => (pt2(0.0, 32.0), Point { x: 0, y: 90 }, -1),
            Quarter::BottomLeftCW => (pt2(32.0, 32.0), Point { x: 90, y: 180 }, -1),
            Quarter::TopLeftCW => (pt2(32.0, 0.0), Point { x: 180, y: 270 }, -1),
            Quarter::TopRightCW => (pt2(0.0, 0.0), Point { x: 270, y: 360 }, -1),
            Quarter::Empty => (pt2(0.0, 0.0), Point { x: 0, y: 0 }, 1),
        }
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Cell {
    pub points: Vec<Point2>,
    pub color: Srgb<u8>,
    pub neighbours: [Point; 8],
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            points: Vec::new(),
            color: COLORS[rand::random::<usize>() % 5],
            neighbours: calculate_neighbours(0, 0),
        }
    }

    pub fn tick(&mut self, state: u32) {
        let quarter = Quarter::int_to_enum(state);
        let position = quarter.value().0;
        let range = quarter.value().1;
        let sign = quarter.value().2;
        let (min, max) = (range.x, range.y);
        self.points = (min..=max)
            .step_by(10)
            .map(|i| {
                let radian = deg_to_rad(sign as f32 * i as f32);
                pt2(
                    position.x + radian.cos() * 32.0,
                    position.y + radian.sin() * 32.0,
                )
            })
            .collect();
        self.points.insert(0, position);
    }

    pub fn draw(&self, draw: &Draw, x: f32, y: f32, max: usize) {
        if self.points.len() > 10 {
            let slice = &self.points[0..max];
            draw.polygon()
                .points(slice.to_vec())
                .x_y((x - 8.0) * 32.0, (y - 8.0) * 32.0)
                .color(self.color);
        }
    }
}

pub fn calculate_neighbours(x: i32, y: i32) -> [Point; 8] {
    let neighbours: [Point; 8] = [
        Point {
            x: MOORE_DIRECTIONS[0].x + x,
            y: MOORE_DIRECTIONS[0].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[1].x + x,
            y: MOORE_DIRECTIONS[1].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[2].x + x,
            y: MOORE_DIRECTIONS[2].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[3].x + x,
            y: MOORE_DIRECTIONS[3].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[4].x + x,
            y: MOORE_DIRECTIONS[4].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[5].x + x,
            y: MOORE_DIRECTIONS[5].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[6].x + x,
            y: MOORE_DIRECTIONS[6].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[7].x + x,
            y: MOORE_DIRECTIONS[7].y + y,
        },
    ];

    neighbours
}
