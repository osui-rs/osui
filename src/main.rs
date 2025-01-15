use osui::*;

fn main() {
    let mut term = init().unwrap();
    loop {
        term.draw(app).unwrap();
    }
}

fn app(frame: &mut Frame) {
    frame.render("hi");
}
