use std::sync::Arc;

use crossterm::event::KeyEvent;
use osui::{
    element::rect::Rect,
    events::{Close, EventManager},
    extensions::{keypress::KeyPressExtension, tick_rate::TickRate, ExtensionManager},
    state::StateManager,
    style::Transform,
    Screen,
};

fn main() {
    let states = StateManager::new(app);
    app(states);
}

fn app(states: Arc<StateManager>) {
    let mut screen = Screen::new();
    let mut extensions = ExtensionManager::new();
    let mut events = EventManager::new();

    extensions.add(TickRate(50));
    extensions.add(KeyPressExtension);

    events.set_state_manager(states.clone());

    let count = states.use_state(0);

    screen
        .draw(Rect(0xffffff))
        .component(Transform::center().dimensions(30, 3));

    screen
        .draw(format!("Count: {}", count.get()))
        .component(Transform::center());

    events.on(move |events, event: Box<KeyEvent>| match event.code {
        crossterm::event::KeyCode::Enter => {
            count.set(count.get() + 1);
        }
        _ => {
            events.dispatch(Close);
        }
    });

    screen.run(&mut events, &mut extensions).unwrap();
}
