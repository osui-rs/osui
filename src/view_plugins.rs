//! # `View` Plugins Module
//!
//! Provides the essential plugins for modifying a `View`

use crate::{
    render::{DrawContext, DrawInstruction},
    View,
};

/// Horizontally centers the allocated area within the available draw area.
///
/// # Behavior
/// - Modifies `ctx.allocated.x`
/// - Does **not** modify height or width
/// - Assumes `ctx.allocated.width <= ctx.area.width`
///
/// # Notes
/// This function only affects positioning, not sizing.
pub fn x_center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

/// Vertically centers the allocated area within the available draw area.
///
/// # Behavior
/// - Modifies `ctx.allocated.y`
/// - Does **not** modify width or height
/// - Assumes `ctx.allocated.height <= ctx.area.height`
///
/// # Typical usage
/// Called after size has been resolved (e.g. after `size_auto`,
/// `height_auto`, or a fixed height has been set).
pub fn y_center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

/// Centers the allocated area both horizontally and vertically.
///
/// # Behavior
/// - Modifies `ctx.allocated.x` and `ctx.allocated.y`
/// - Does **not** modify width or height
///
/// # Order
/// Should generally be called **after** size resolution
/// (e.g. `size_auto`, `width_auto`, `height_auto`).
pub fn center(ctx: &mut DrawContext, _view: &View) {
    ctx.allocated.x = (ctx.area.width - ctx.allocated.width) / 2;
    ctx.allocated.y = (ctx.area.height - ctx.allocated.height) / 2;
}

/// Automatically computes both width and height based on drawn content.
///
/// # Sizing rules
/// - `Text`:
///   - Width = longest line length
///   - Height = number of lines
/// - `View`:
///   - Recursively executes the view in a fresh `DrawContext`
///   - Uses the child view's allocated size
/// - `Child`: ignored
///
/// # Notes
/// - This function performs a layout pass only.
/// - It does not draw anything.
/// - Later instructions may overwrite the computed size.
pub fn size_auto(ctx: &mut DrawContext, _view: &View) {
    for i in &ctx.drawing {
        match i {
            DrawInstruction::Text(_, text) => {
                let mut height = 0;

                for line in text.lines() {
                    ctx.allocated.width = ctx.allocated.width.max(line.len() as u16);
                    height += 1;
                }

                ctx.allocated.height = height;
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

/// Automatically computes width based on drawn content.
///
/// # Sizing rules
/// - `Text`: width is the longest line
/// - `View`: width is taken from the child view's auto-sized result
/// - Height is left unchanged
///
/// # When to use
/// Useful when height is fixed or externally constrained.
pub fn width_auto(ctx: &mut DrawContext, _view: &View) {
    for i in &ctx.drawing {
        match i {
            DrawInstruction::Text(_, text) => {
                for line in text.lines() {
                    ctx.allocated.width = ctx.allocated.width.max(line.len() as u16);
                }
            }
            DrawInstruction::View(area, view) => {
                let mut c = DrawContext::new(area.clone());
                view(&mut c);
                size_auto(&mut c, view);
                ctx.allocated.width = c.allocated.width;
            }
            DrawInstruction::Child(_, _) => {}
        }
    }
}

/// Automatically computes height based on drawn content.
///
/// # Sizing rules
/// - `Text`: height is the number of lines
/// - `View`: height is taken from the child view's auto-sized result
/// - Width is left unchanged
///
/// # Notes
/// This pass ignores line width and wrapping concerns.
pub fn height_auto(ctx: &mut DrawContext, _view: &View) {
    for i in &ctx.drawing {
        match i {
            DrawInstruction::Text(_, text) => {
                let mut height = 0;

                for _ in text.lines() {
                    height += 1;
                }

                ctx.allocated.height = height;
            }
            DrawInstruction::View(area, view) => {
                let mut c = DrawContext::new(area.clone());
                view(&mut c);
                size_auto(&mut c, view);
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
