use std::sync::Arc;

use osui::{
    element::{
        input::{Input, InputKeyPress},
        rect::Rect,
    },
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
    extensions.add(KeyPressExtension);
    extensions.add(TickRate(255));

    let my_state = states.use_state(20);

    screen
        .draw(Rect(0xffffff))
        .component(Transform::center().dimensions(30, 3));

    screen
        .draw(format!("Type quit to quit {}", my_state.get()))
        .component(Transform::new().bottom().margin(0, -1));

    screen
        .draw(Input::new())
        .component(Transform::new().bottom());

    events.on(
        move |events, event: Box<InputKeyPress>| match event.1.code {
            crossterm::event::KeyCode::Enter => {
                my_state.set(my_state.get() + 10);
                // if event.0 == "quit" {
                // events.dispatch(Close);
                // }
            }
            _ => {}
        },
    );

    screen.run(&mut events, &mut extensions).unwrap();
}
