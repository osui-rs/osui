use osui::{
    event_manager::EventManager, extensions::keypress::KeyPressExtension, ExtensionManager, Screen,
};

fn main() {
    let mut screen = Screen::new();
    let mut extensions = ExtensionManager::new();
    let mut events = EventManager::new();

    extensions.add(KeyPressExtension);

    screen.draw("Hello, World".to_string());

    screen.run(&mut events, &mut extensions).unwrap();
}
