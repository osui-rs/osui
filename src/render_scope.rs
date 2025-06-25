pub struct RenderScope {}

impl RenderScope {
    pub fn draw_text(&mut self, text: &str) {
        println!("{text}");
    }
}
