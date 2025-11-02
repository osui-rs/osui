use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    rsx! {
        @Style { foreground: None, background: Background::Outline(0xff0000) };
        @Transform::new().padding(1, 1);
        VelocityHandler {
            @Velocity(200, 100);
            "test"
        }
    }
}
