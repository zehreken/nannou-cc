use nannou::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn start_a8() {
    nannou::app(model).run();
}

struct Model {
    colors: Vec<u64>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .title("a8")
        .view(view)
        .build()
        .unwrap();

    let (tx, rx) = mpsc::channel();
    for _ in 0..8 {
        let tx_t = tx.clone();
        thread::spawn(move || {
            for i in 0..8 {
                tx_t.send(i).unwrap();
                thread::sleep(Duration::from_millis(i));
            }
        });
    }
    drop(tx);

    let mut colors = vec![];
    let handle = thread::spawn(move || {
        for received in rx {
            colors.push(received);
        }
        colors
    });
    colors = handle.join().unwrap();

    Model { colors }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let colors = [
        RED, BLUE, GREEN, ORANGE, AQUAMARINE, GOLD, DEEPPINK, HONEYDEW,
    ];
    let draw = app.draw();

    let mut index = 0;
    for i in &model.colors {
        draw.rect()
            .x_y(
                (index % 8) as f32 * 64.0 - 224.0,
                (index / 8) as f32 * 64.0 - 224.0,
            )
            .w_h(64.0, 64.0)
            .color(colors[*i as usize]);

        index += 1;
    }

    draw.to_frame(app, &frame).unwrap();
}
