use osui::{self, render_frame, ui};

fn main() {
    let txt = ui::text("Hello World!");
    render_frame(vec![txt.clone()]);
}
