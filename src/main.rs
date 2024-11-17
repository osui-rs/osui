use osui::{app::run, rsx, ui::*, Element};

fn main() {
    while run(&mut app()) {}
}

fn app() -> Box<dyn Element> {
    rsx! {
        text { "Hello, World!" }
    }
}
