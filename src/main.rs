use osui::{rsx, ui::*};

fn main() {
    osui::app::run(&mut app());
}

fn app() -> Box<dyn osui::Element> {
    rsx! {
        text { "Hello, World!" }
    }
}
