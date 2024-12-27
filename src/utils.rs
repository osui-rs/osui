use std::io::{stdout, Write};

pub struct Frame<'a> {
    used: Vec<u16>,
    css: &'a crate::ui::Css,
    writer: &'a mut crate::Writer,
}

impl<'a> Frame<'a> {
    pub fn new(writer: &'a mut crate::Writer, css: &'a crate::ui::Css) -> Self {
        Frame {
            used: vec![0; writer.size.1 as usize],
            css,
            writer,
        }
    }
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let style = element.get_style();
        let mut style_element = style.clone().get(focused);

        for class in element.get_class().split(" ") {
            if let Some(upper) = self
                .css
                .get(&crate::ui::StyleName::Class(class.to_string()))
            {
                style_element.merge(upper);
            }

            if let Some(upper) = self.css.get(&crate::ui::StyleName::ClassState(
                class.to_string(),
                if style.2 == "" && focused {
                    "hover".to_string()
                } else {
                    style.2.clone()
                },
            )) {
                style_element.merge(upper);
            }
        }

        if style_element.visible.1 {
            let y = style_element.y.1.as_position(
                style_element.outline.1,
                &{
                    let mut y = 0;
                    while y < self.used.len() && self.used[y] != 0 {
                        y += 1;
                    }
                    y as u16
                },
                self.writer.size.1,
            );
            if let Some(y_) = self.used.get(y as usize) {
                let x =
                    style_element
                        .x
                        .1
                        .as_position(style_element.outline.1, y_, self.writer.size.0);
                let mut writer = crate::Writer::new(
                    focused,
                    style_element.clone(),
                    (x, y),
                    self.writer.size,
                    self.css,
                );
                element.render(&mut writer);
                let (mut width, mut height) = writer.get_size_root();
                if style_element.outline.1 {
                    width += 2;
                    height += 2;
                }
                for i in y..y + height {
                    if let Some(o) = self.used.get_mut(i as usize) {
                        *o += x + width;
                    }
                }
            }
        }
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
    stdout().flush().unwrap();
}

pub fn show_cursor() {
    print!("\x1B[?25h");
    stdout().flush().unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}
