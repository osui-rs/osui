use osui::{
    components::{button, div},
    oml, App, Component, Params,
};

fn main() {
    let mut app = App::new();
    app.set_component(oml!(app_elem()));
    app.run();
}

fn onclick(c: &mut Component) {
    if c.clicked {
        c.expr = "Clicked!".to_string();
    } else {
        c.expr = "click me".to_string()
    }
}

fn app_elem(_: Params) -> Component {
    oml!(
        div {
            button("click me"; on_click = onclick)
            button("click me"; y = 2, on_click = onclick)
        }
    )
}
