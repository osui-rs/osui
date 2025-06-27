use osui::{
    element::{
        input::{Input, InputKeyPress},
        rect::Rect,
    },
    events::{Close, EventManager},
    extensions::{keypress::KeyPressExtension, tick_rate::TickRate, ExtensionManager},
    style::Transform,
    Screen,
};

fn main() {
    let mut screen = Screen::new();
    let mut extensions = ExtensionManager::new();
    let mut events = EventManager::new();
    extensions.add(KeyPressExtension);
    extensions.add(TickRate(255));

    screen
        .draw(Rect(0xffffff))
        .component(Transform::center().dimensions(30, 3));

    screen
        .draw(format!("Type quit to quit"))
        .component(Transform::new().bottom());

    screen
        .draw(Input::new())
        .component(Transform::new().bottom());

    events.on(|events, event: Box<InputKeyPress>| match event.1.code {
        crossterm::event::KeyCode::Enter => {
            if event.0 == "quit" {
                events.dispatch(Close);
            }
        }
        _ => {}
    });

    screen.run(&mut events, &mut extensions).unwrap();
}
