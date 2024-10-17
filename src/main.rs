use osui::{
    components::{button, div},
    oml, App, Component, Params, UpdateResponse,
};

fn main() {
    let mut app = App::new();
    app.set_component(oml!(app_elem()));
    app.run();
}

fn app_elem(_: Params) -> Component {
    oml!(
        div {
            button("click me"; on_click = |c| {
                c.response = UpdateResponse::SetComponent(
                    oml!(button("Clicked!";))
                );
            })
        }
    )
}
