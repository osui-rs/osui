use crate::prelude::*;

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

pub struct Button {
    pub on_click: Box<dyn FnMut()>,
    children: String,
    width: u16,
    height: u16,
    last_elem: (u16, u16),
}

pub fn btn() -> Button {
    Button {
        on_click: Box::new(|| {}),
        children: String::new(),
        width: 0,
        height: 0,
        last_elem: (0, 0),
    }
}

impl Widget for Button {
    fn render(&self) -> String {
        self.children.clone()
    }
    fn event(&mut self, _: crate::console::Event) {
        (self.on_click)()
    }
}

impl Button {
    pub fn draw<W>(&mut self, w: &W, props: crate::Props) -> crate::Result<()>
    where
        W: Widget,
    {
        let written = w.render();

        let (ww, wh) = utils::str_size(&written);

        let (width, height) = (
            props.width.get(ww, self.width),
            props.height.get(wh, self.height),
        );

        let (x, y) = (
            props.x.get(self.last_elem.0, width, self.width),
            props.y.get(self.last_elem.1, height, self.height),
        );

        self.children += written
            .lines()
            .enumerate()
            .filter(|(i, _)| -> bool { height > (*i as u16) })
            .map(|(_, s)| s.chars().take(width as usize).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
            .as_str();

        self.last_elem.0 = x + width;
        self.last_elem.1 = y + height;
        Ok(())
    }
}
