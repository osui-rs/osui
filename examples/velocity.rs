use std::sync::Arc;

use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();
    screen.extension(InputExtension);

    app(screen.clone()).draw(&screen);

    screen.run()
}

pub fn app(screen: Arc<Screen>) -> Rsx {
    rsx! {
        @Handler::new({
            let screen = screen.clone();
            move |_, e: &crossterm::event::Event| {
                if let crossterm::event::Event::Key(e) = e {
                    if e.code == crossterm::event::KeyCode::Esc {
                        screen.close();
                    }
                }
            }
        });
        @Style { foreground: None, background: Background::Outline(0xff0000) };
        VelocityHandler {
            @Velocity(700, 0);
            @Handler::new({
                move |w, e: &crossterm::event::Event| {
                    if let crossterm::event::Event::Key(e) = e {
                        if e.code == crossterm::event::KeyCode::Char(' ') {
                            if let Some(Velocity(v, _)) = w.get() {
                                w.set_component(Velocity(-v, 0));
                            } else {
                                w.component(Velocity(700, 0));
                            }
                        }
                    }
                }
            });
            "(•_•)"
        }

        @Transform::new().dimensions(Dimension::Full, Dimension::Full);
        VelocityHandler {
            @Velocity(0, -150);
            @Transform::center().bottom();
            "^"
        }
    }
}
