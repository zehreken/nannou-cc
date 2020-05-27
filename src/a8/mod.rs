use nannou::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn start_a8() {
    nannou::app(model).update(update).run();
}

struct Model {
    handles: Vec<JoinHandle<()>>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .title("a8")
        .view(view)
        .build()
        .unwrap();

    Model { handles: vec![] }
}

fn update(app: &App, model: &mut Model, update: Update) {
    for _ in 0..8 {
        model.handles.push(thread::spawn(|| {
            for i in 0..8 {
                toggle(i, i);
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    thread::sleep(Duration::from_millis(1000));
}

fn toggle(i: u8, j: u8) {
    println!("{}, {}", i, j);
}

fn view(app: &App, model: &Model, frame: Frame) {}
