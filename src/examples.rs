use crate::prelude::*;

pub fn event_logger() -> Element {
    rsx! {
        id: "root",
        text { on_event: Handler::new(|_, _, document| {
            let root = document.get_element_by_id::<Div>("root").unwrap();
            root.children.add_child(rsx_elem!( text { "Ok" } ));
        }), id: "txt", "current event: None" }
    }
}
