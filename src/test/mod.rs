use crate::{self as osui, arc, rsx, rsx_elem, ui::*, Element, EventResponse};

pub fn app() -> Box<dyn Element> {
    let mut something = String::from("!");
    rsx! {
        button { on_click: arc!(move |_btn: &mut Button, response: &mut EventResponse| {
            something += "!";
            response.update_element_by_id("my_id", rsx_elem! {
                text { id: "my_id", y: 2, "{something}" }
            });
        }), style.clicked_background: Color::Green, style.hover_background: Color::Magenta, "click me" }
        text { id: "my_id", y: 2, "!" }
    }
}
