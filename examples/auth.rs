use std::sync::Arc;

use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app(screen.clone()).draw(&screen);

    screen.run()
}

pub fn app(_screen: Arc<Screen>) -> Rsx {
    rsx! {
        FlexRow, gap: 1, {
            @Style { foreground: Some(0xffffff), background: Background::Solid(0x00ff00) };
            FlexRow {
                @Transform::new().padding(1, 1).dimensions(40, 1);
                @Style { foreground: Some(0xffffff), background: Background::RoundedOutline(0xff0000) };
                @Focused;
                Input { }
            }

            // @Style { foreground: Some(0xffffff), background: Background::Solid(0x00ff00) };
            // FlexRow {
            //     @Transform::new().padding(1, 1).dimensions(40, 1);
            //     @Style { foreground: Some(0xffffff), background: Background::RoundedOutline(0xffff00) };
            //     Input { }
            // }
        }
    }
}
