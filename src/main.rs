use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    rsx! {
        @transform! {
            width: Full,
            height: Full,
        };
        Div {
            @transform! {
                width: Full,
                height: Full,
            };
            RoundedOutline {}

            @Transform::center();
            "Welcome To OSUI!"
        } (0x000000)
    }
}
