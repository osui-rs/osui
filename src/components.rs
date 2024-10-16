use crate::{utils::render_to_frame, Component, Params};

pub fn div(params: Params) -> Component {
    Component::new(params, |s| {
        let mut frame: Vec<String> = vec![" ".repeat(s.width); s.height];
        for c in &mut s.children {
            render_to_frame(s.width, &mut frame, c);
        }
        frame.join("\n")
    })
}

pub fn text(params: Params) -> Component {
    Component::new(params, |c| c.expr.clone())
}
