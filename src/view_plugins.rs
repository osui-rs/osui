use crate::{
    render::{DrawContext, DrawInstruction},
    View,
};

pub fn x_center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

pub fn y_center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

pub fn center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.x = (ctx.area.width - ctx.allocated.width) / 2;
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

pub fn size_auto(ctx: &mut DrawContext, _view: &View) {
    for i in &ctx.drawing {
        match i {
            DrawInstruction::Text(_, text) => {
                ctx.allocated.width = text.len() as u16;
                ctx.allocated.height = 1;
            }
            DrawInstruction::View(area, view) => {
                let mut c = DrawContext::new(area.clone());
                view(&mut c);
                size_auto(&mut c, view);
                ctx.allocated.width = c.allocated.width;
                ctx.allocated.height = c.allocated.height;
            }
            DrawInstruction::Child(_, _) => {}
        }
    }
}

pub fn redraw(ctx: &mut DrawContext, view: &View) {
    ctx.clear();
    ctx.draw_view(ctx.allocated.clone(), view.clone());
}
