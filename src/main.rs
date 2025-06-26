use osui::{
    events::EventManager,
    extensions::{keypress::KeyPressExtension, ExtensionManager},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let mut extensions = ExtensionManager::new();
    let mut events = EventManager::new();
    extensions.add(KeyPressExtension);

    screen
        .draw("Hello, World".to_string())
        .component(Transform::center());

    screen.run(&mut events, &mut extensions).unwrap();
}
