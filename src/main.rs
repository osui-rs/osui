use osui::{elements::div::Div, frontend::Rsx, rsx, style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    rsx! {
        @Transform::new();
        Div {
            @Transform::center().dimensions(40, 3);
            Div {
                @Transform::center().dimensions(8, 1);
                Div {
                    "hello"
                } (0x00ff00)
            } (0xff0000)
        } (0x0000ff)
    }
}
