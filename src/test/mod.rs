use crate::{self as osui, arc, rsx, rsx_elem, ui::*, Element, EventResponse};

pub fn app() -> Box<dyn Element> {
    rsx! {
        button { on_click: arc!(move |btn: &mut Button| {
            btn.event_response = EventResponse::UpdateElementById("my_id".to_string(), rsx_elem! { button { y: 2, id: "my_id", "updated!" } } );
        }), "click me" }

        button { id: "my_id", y: 2, "ok" }
    }
}
