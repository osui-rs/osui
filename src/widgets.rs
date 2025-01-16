use crate::Widget;

impl Widget for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}
