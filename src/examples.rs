use crate::prelude::*;

pub fn examples() -> Element {
    rsx! {
        styling: Some(styling()),
        button { class: "btn", on_click: Handler::new(|btn: &mut Button, _, document| {
            let data = document.get_element_by_id::<DataHolder<'_, String>>("my_data").unwrap();
            data.data += "!";
            btn.children.set_text(&data.data);
        }), "Hello, World!" }

        {
            let mut data = dataholder::<String>();
            data.id = "my_data";
            data
        }
    }
}

pub fn styling() -> Css {
    css! {
        .btn {
            x: 30%,
            y: Auto,
            color: Red,
        }

        .btn: clicked {
            color: Blue,
        }
    }
}
