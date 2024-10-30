use osui::{oml, ui::*, App};

fn main() {
    let mut app_screen = App::new();
    app_screen.set_component(app());
    app_screen.run();
}

fn app() -> Box<Text> {
    oml!(
        text("Hello, World!";)
    )
}
