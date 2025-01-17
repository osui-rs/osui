use crate::Widget;

impl Widget for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Widget for String {
    fn render(&self) -> String {
        self.clone()
    }
}
