use osui::prelude::*;
use osui::examples;

fn main() {
    while osui::run(&mut examples::event_logger()) {}
}
