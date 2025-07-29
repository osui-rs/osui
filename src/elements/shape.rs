use crate::widget::Element;

pub struct RoundedOutline;

impl Element for RoundedOutline {
    fn render(&mut self, scope: &mut crate::prelude::RenderScope) {
        let (w, h) = scope.get_size_or_parent();
        if w > 2 && h > 2 {
            let d = "─".repeat(w as usize - 2);
            scope.draw_text(&format!(
                "╭{d}╮{}\n╰{d}╯",
                format!("\n│{}│", " ".repeat(w as usize - 2)).repeat(h as usize - 2),
            ));
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl RoundedOutline {
    pub fn new() -> Self {
        Self
    }
}

pub struct Outline;

impl Element for Outline {
    fn render(&mut self, scope: &mut crate::prelude::RenderScope) {
        let (w, h) = scope.get_size_or_parent();
        if w > 2 && h > 2 {
            let d = "─".repeat(w as usize - 2);
            scope.draw_text(&format!(
                "┌{d}┐{}\n└{d}┘",
                format!("\n│{}│", " ".repeat(w as usize - 2)).repeat(h as usize - 2),
            ));
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Outline {
    pub fn new() -> Self {
        Self
    }
}
