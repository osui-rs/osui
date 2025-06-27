use osui::{
    element::input::{Input, InputUpdateEvent},
    events::{Close, EventManager},
    extensions::{keypress::KeyPressExtension, ExtensionManager},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let mut extensions = ExtensionManager::new();
    let mut events = EventManager::new();
    extensions.add(KeyPressExtension);

    screen.draw(Input::new()).component(Transform::center());

    events.on(|events, event: Box<InputUpdateEvent>| {
        if event.0 == "quit" {
            events.dispatch(Close);
        }
    });

    screen.run(&mut events, &mut extensions).unwrap();
}
