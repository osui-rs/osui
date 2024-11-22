use crate::prelude::*;

pub fn event_logger() -> Element {
    rsx_elem! {
        text { on_event: Handler::new(|txt: &mut Text, e, _| {
            txt.children.set_text(&format!("current event: {e:?}"));
        }), "current event: None" }
    }
}