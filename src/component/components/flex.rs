use crate::component_prelude::*;

#[component]
pub fn FlexRow(cx: &Arc<Context>, children: Rsx) -> View {
    children.generate_children(&cx);

    Arc::new({
        let cx = cx.clone();
        move |ctx| {
            for (child, view_wrapper) in cx.get_children() {
                let view = child.get_view();

                if let Some(view_wrapper) = view_wrapper {
                    let mut ctx2 = ctx.clone();

                    view_wrapper(&mut ctx2, view);

                    ctx.drawing.append(&mut ctx2.drawing);
                    ctx.area.y += ctx2.allocated.height;
                } else {
                    ctx.draw_view(ctx.area.clone(), view);
                }
            }
        }
    })
}

#[component]
pub fn FlexColumn(cx: &Arc<Context>, children: Rsx) -> View {
    children.generate_children(&cx);

    Arc::new({
        let cx = cx.clone();
        move |ctx| {
            for (child, view_wrapper) in cx.get_children() {
                let view = child.get_view();

                if let Some(view_wrapper) = view_wrapper {
                    let mut ctx2 = ctx.clone();

                    view_wrapper(&mut ctx2, view);

                    ctx.drawing.append(&mut ctx2.drawing);
                    ctx.area.x += ctx2.allocated.width;
                } else {
                    ctx.draw_view(ctx.area.clone(), view);
                }
            }
        }
    })
}
