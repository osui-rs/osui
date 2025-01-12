use osui::{examples, prelude::*};

fn main() {
    launch!(App);
}

#[component]
fn App() {
    rsx! {
        @SetStyle(css! {
            example: "hover" {
                color: Blue,
            }
        })

        %text { "Choose an example to run" }
        button { class: "example", on_click: |_, _, doc| {
            doc.draw(examples::counter_example::App::default().create_element());
        }, "Counter" }
        button { class: "example", on_click: |_, _, doc| {
            doc.draw(examples::login_example::App::default().create_element());
        }, "Login" }
        button { class: "example", on_click: |_, _, doc| {
            doc.draw(examples::todo_example::App::default().create_element());
        }, "Todo" }
    }
}
