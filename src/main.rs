use osui::{
    element::input::{Input, InputUpdateEvent},
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

    let widget = screen.draw(Input::new()).component(Transform::center());

    let input_event = widget
        .0
        .as_any()
        .downcast_ref::<Input>()
        .unwrap()
        .get_event();

    input_event
        .lock()
        .unwrap()
        .on(|event: Box<InputUpdateEvent>| {
            if event.0 == "quit" {
                std::process::exit(0)
            }
        });

    screen.run(&mut events, &mut extensions).unwrap();
}
