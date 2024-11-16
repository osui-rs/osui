use osui::{app::run, rsx_elem, ui::text};

fn main() {
    run(&mut rsx_elem! { text { "Hello, World!" } });
}
