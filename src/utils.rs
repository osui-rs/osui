use std::io::{stdout, Write};

pub struct Frame<'a> {
    used: Vec<u16>,
    writer: &'a mut crate::Writer,
}

impl<'a> Frame<'a> {
    pub fn new(writer: &'a mut crate::Writer) -> Self {
        Frame {
            used: vec![0; writer.size.1 as usize],
            writer,
        }
    }
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let style = element.get_style();
        let mut style_element = style.clone().get(focused);
        let css = unsafe { &*self.writer.css };

        for class in element.get_class().split(" ") {
            if let Some(upper) = css.get(&crate::ui::StyleName::Class(class.to_string())) {
                style_element.merge(upper);
            }

            if let Some(upper) = css.get(&crate::ui::StyleName::ClassState(
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

        if !style_element.visible.1 {
            return;
        }

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
            let x = style_element
                .x
                .1
                .as_position(style_element.outline.1, y_, self.writer.size.0);

            let mut writer = crate::Writer {
                focused,
                style: style_element.clone(),
                pos: (x, y),
                size: self.writer.size,
                css,
                parent: Some(self.writer),
                written: (0, 0),
            };
            element.render(&mut writer);
            let (width, height) = writer.get_size_root();
            {
                let (mut x, mut y) = (x, y);
                if style_element.outline.1 {
                    x -= 1;
                    y -= 1;
                }
                let height = if style_element.outline.1 {
                    height + 1
                } else {
                    height
                };

                for i in y..y + height {
                    if let Some(o) = self.used.get_mut(i as usize) {
                        if style_element.x.1 != crate::ui::Number::Auto {
                            *o += x;
                        }
                        *o += width;
                    }
                }
            }
            writer.after_render(width, height);
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

pub trait VisibleLength {
    fn visible_len(&self) -> usize;
}

impl VisibleLength for String {
    fn visible_len(&self) -> usize {
        let mut length = 0;
        let mut in_escape = false;

        for c in self.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if c.is_ascii_alphabetic() {
                    in_escape = false;
                }
            } else {
                length += 1;
            }
        }

        length
    }
}
