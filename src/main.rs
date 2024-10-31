use osui::{oml, ui::*, App};

fn main() {
    let mut app_screen = App::new();
    app_screen.set_component(app());
    app_screen.run();
}

fn app() -> Box<Tab> {
    oml!(
        tab {
            text("Hello!"; style = Style {
                hover_fg: Color::Red,
                ..Default::default()
            })
            button("Hello 2!"; y = 1, style = Style {
                hover_fg: Color::Green,
                ..Default::default()
            })
            menu(items = vec![String::from("Hello, World!"), String::from("Item 2!")])
        }
    )
}
