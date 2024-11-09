use crate::{self as osui, event::new_handler, rsx, ui::*, Element};

pub fn app() -> Box<dyn Element> {
    rsx! {
        button { on_click: new_handler(move |btn: &mut Button, _response| {
            btn.text = "Clicked".to_string();
        }), "click me" }
    }
}
