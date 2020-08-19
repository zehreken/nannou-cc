use nannou::prelude::*;

pub fn capture_frame(app: &App, frame: u64) {
    app.main_window().capture_frame(format!("{:05}.png", frame));
}
