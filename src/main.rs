use osui::{self, ui};

fn main() {
    osui::clear();
    let mut txt = ui::text("hello, world!");
    txt.render();
}
