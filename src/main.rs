use osui::{style::Transform, Screen};

fn main() {
    let mut screen = Screen::new();

    screen
        .draw(format!("Hello, World!"))
        .component(Transform::center());

    screen.run().unwrap();
}
