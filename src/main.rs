use osui::{examples, prelude::*};
fn main() {
    launch!(App);
}

#[component]
pub fn App() {
    rsx! {
        @SetStyle(app_style())

        text { static "Select an example" }

        button { class: "example", on_click: fn(_, _, document) {
            document.clear_css();
            document.draw(examples::counter_example::App::default().create_element());
        }, static "Counter" }

        button { class: "example", on_click: fn(_, _, document) {
            document.clear_css();
            document.draw(examples::todo_example::App::default().create_element());
        }, static "Todo" }

        button { class: "example", on_click: fn(_, _, document) {
            document.clear_css();
            document.draw(examples::login_example::App::default().create_element());
        }, static "Login" }
    }
}

pub fn app_style() -> Css {
    css! {
        "example" {
            outline: true,
            y: Auto,
            x: 0 px,
        }
        "example": "hover" {
            color: Blue,
            outline_color: Red,
        }
    }
}
