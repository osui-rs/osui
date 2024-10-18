use osui::{components::*, oml, App, Component};

fn main() {
    let mut app = App::new();
    app.set_component(app_elem());
    app.run();
}

fn onclick(c: &mut Component) {
    if c.clicked {
        c.expr = "Clicked!".to_string();
    } else {
        c.expr = "click me".to_string()
    }
}

fn app_elem() -> Component {
    oml!(
        div {
            button("click me"; toggle = true, on_click = onclick)
            button("click me"; toggle = true, y = 2, on_click = onclick)
        }
    )
}
