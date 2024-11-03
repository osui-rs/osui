use osui::{rsx, ui::*, App};

fn main() {
    let mut app_screen = App::new();
    app_screen.set_component(rsx! {
        button { "test" }
    });
    app_screen.run();
}
