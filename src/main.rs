use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

fn app() -> Rsx {
    rsx! {
        @Transform::new().padding(3, 3);
        @Style { background: Background::RoundedOutline(0xff0000), foreground: None };
        Div {
            @Transform::new().padding(1, 1);
            @Style { background: Background::RoundedOutline(0x00ff00), foreground: None };
            FlexCol {
                @Transform::new().padding(1, 1);
                "Hello, World!"
                @Transform::new().padding(1, 1);
                "Hello, World!"
                @Transform::new().padding(1, 1);
                "Hello, World!"
                @Transform::new().padding(1, 1);
                "Hello, World!"
                @Transform::new().padding(1, 1);
                "Hello, World!"
                @Transform::new().padding(1, 1);
                "Hello, World!"
            } (2)
        }
    }
}
