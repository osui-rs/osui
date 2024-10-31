use osui::{key::KeyKind, oml, ui::*, App};

fn main() {
    let mut app_screen = App::new();
    app_screen.set_component(oml!(

        tab {
            div {
                // Some welcome text
                text("Welcome! This is the OSUI Tutorial! Press tab to go to the next Page or press Shift+Tab to go to a previous page";)
            }

            // A button example With a div

            div {
                text("On the bottom there is a button. To hover it click the down key, Then press enter to select it";)

                // Button
                button("Click me!"; y = 3, style = Style {
                    hover_fg: Color::Magenta,
                    ..Default::default()
                }, on_click = |btn| {
                    btn.text = "Press tab to go to the next page".to_string();
                })
                
            }

            div {
                text("This is a menu, ";)

                // Button
                menu(y = 2, items = vec![
                    String::from("Item 1"),
                    String::from("Item 2"),
                    String::from("Item 3"),
                    String::from("Item 4"),
                    String::from("Item 5"),
                    String::from("Item 6"),
                ], on_click = |m, k| {
                    if k.kind == KeyKind::Escape {
                        return ClickResponse::Exit;
                    } else if k.kind == KeyKind::Enter {
                        m.items[m.selected] = "Press Tab to go to the next page".to_string();
                    }
                    ClickResponse::None
                })
                
            }
        }

    ));
    app_screen.run();
}
