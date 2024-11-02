use osui::{oml, ui::*, App};

fn main() {
    let mut app_screen = App::new();
    app_screen.set_component(oml!(
        button("okay";)
    ));
    app_screen.run();
}
