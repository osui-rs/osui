use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    rsx! {
        @Transform::new();
        @Style { background: Background::RoundedOutline(0xff0000), foreground: None };
        Div { @Transform::center(); "Hello, World!" }
    }
}
