use crate::{
    create_frame, key::KeyKind, utils::render_to_frame, Component, Params, UpdateContext,
    UpdateRequest,
};

pub fn div(params: Params) -> Component {
    let mut c = Component::new(params);
    c.render = |s: &mut Component| {
        let mut frame: Vec<String> = create_frame!(s.width, s.height);
        for c in &mut s.children {
            render_to_frame(&mut frame, c);
        }
        frame.join("\n")
    };
    c
}

pub fn text(params: Params) -> Component {
    let mut c = Component::new(params);
    c.render = |s: &mut Component| s.expr.clone();
    c
}

pub fn button(params: Params) -> Component {
    let mut c = Component::new(params);
    c.update = |s: &mut Component, ctx: &mut UpdateContext| {
        s.expr = "nah".to_string();
        match ctx.request.clone() {
            UpdateRequest::Key(k) => {
                if k.kind == KeyKind::Enter {
                    s.clicked = !s.clicked;
                    let mut cctx = UpdateContext {
                        request: UpdateRequest::Key(k),
                        response: crate::UpdateResponse::None,
                    };
                    (s.on_click)(&mut cctx);
                    ctx.response = cctx.response;
                }
            }
        }
    };
    c.render = |s: &mut Component| s.expr.clone();
    c
}
