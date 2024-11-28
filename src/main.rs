use osui::prelude::*;

fn main() {
    launch!(app());
}

// A simple counter
fn app() -> Element {
    rsx! {
        // Create a button with a on_click handler
        button { on_click: Handler::new(move |btn: &mut Button, _, document| {
            if let Some(count) = document.get_element_by_id::<DataHolder<u32>>("count") {
                count.data += 1;
                btn.children.set_text(&count.data.to_string());
            }
        }), "0" }

        // Store the data (in this case count)
        data_holder as u32 { id: "count" }
    }
}
