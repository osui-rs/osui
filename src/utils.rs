use std::io::{stdout, Write};

use crossterm::terminal::disable_raw_mode;

use crate::ui;

pub struct Frame {
    used: Vec<u16>,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    css: *const ui::Css,
}

impl Frame {
    pub fn new((x, y): (u16, u16), (width, height): (u16, u16), css: &crate::ui::Css) -> Self {
        Frame {
            used: vec![0; height as usize],
            width,
            height,
            x,
            y,
            css,
        }
    }
    pub fn render(&mut self, focused: bool, element: &crate::Element) {
        let style = element.get_style();
        let mut style_element = style.clone().get(focused);

        if let Some(upper) = unsafe { (*self.css).get(&ui::StyleName::Class(element.get_class())) }
        {
            style_element.merge(upper);
        }

        if let Some(upper) = unsafe {
            (*self.css).get(&ui::StyleName::ClassState(
                element.get_class(),
                if style.2 == "" && focused {
                    "hover".to_string()
                } else {
                    style.2.clone()
                },
            ))
        } {
            style_element.merge(upper);
        }

        if style_element.visible.1 {
            let y = style_element.y.1.as_position(
                &{
                    let mut y = 0;
                    while y < self.used.len() && self.used[y] != 0 {
                        y += 1;
                    }
                    y as u16
                },
                self.height,
            );
            if let Some(y_) = self.used.get(y as usize) {
                let x = style_element.x.1.as_position(y_, self.width);
                let mut writer = crate::Writer::new(
                    focused,
                    style_element.clone(),
                    (x + self.x, y + self.y),
                    (self.width, self.height),
                    unsafe { &*self.css },
                );
                element.render(&mut writer);
                let (width, height) = writer.get_size_root();
                if style_element.outline.1 {
                    clear();
                    show_cursor();
                    disable_raw_mode().unwrap();
                    panic!("the 'outline' option isn't available right now. please set 'outline' to 'false'")
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
