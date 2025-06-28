use std::sync::Arc;

use crossterm::event::KeyEvent;
use osui::{
    element::rect::Rect,
    events::EventManager,
    extensions::{keypress::KeyPressExtension, ExtensionManager},
    state::StateManager,
    style::Transform,
    Screen,
};

fn main() {
    init(&StateManager::new(init));
}

fn init(states: &Arc<StateManager>) {
    let mut screen = Screen::new();
    screen.events.set_state_manager(states.clone());

    let mut extensions = ExtensionManager::new();
    extensions.add(KeyPressExtension);

    app(states, &mut screen);

    screen.run(&mut extensions).unwrap();
}

fn app(states: &Arc<StateManager>, screen: &mut Screen) {
    screen.events.set_state_manager(states.clone());

    let mut count = states.use_state(0);

    screen
        .draw(Rect(0xffffff))
        .component(Transform::center().dimensions(30, 3));

    screen
        .draw(format!("Count: {count}"))
        .component(Transform::center());

    screen.events.on(
        move |events: &Arc<EventManager>, event: &KeyEvent| match event.code {
            crossterm::event::KeyCode::Enter => {
                count += 1;
            }
            _ => {
                events.close();
            }
        },
    );
}
