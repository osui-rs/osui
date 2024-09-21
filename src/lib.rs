use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use std::{
    io::{self, Read, Write},
    str::from_utf8,
};
pub mod key;
pub mod ui;

#[derive(Clone, Copy)]
pub struct ComponentWrapper<T> {
    pub contents: T,
    pub x: usize,
    pub y: usize,
    update_fn: fn(&mut T) -> bool,
}

pub trait Component {
    fn render(&self) -> String {
        String::new()
    }

    fn run(&mut self, x: usize, y: usize, update_fn: fn(&mut Self) -> bool) -> String {
        let _ = x;
        let _ = y;
        let _ = update_fn;
        String::new()
    }
}

impl<T: Component> ComponentWrapper<T> {
    pub fn new(component: T) -> ComponentWrapper<T> {
        ComponentWrapper {
            contents: component,
            x: 0,
            y: 0,
            update_fn: (|_| false),
        }
    }
    pub fn run(&mut self) -> String {
        self.contents.run(self.x, self.y, self.update_fn)
    }
    pub fn render(&self, frame: &mut Vec<String>) {
        for (i, line) in self.contents.render().split("\n").enumerate() {
            frame[self.y + i] = _render_line(frame[self.y + i].clone(), line.to_string(), self.x);
        }
    }
    pub fn update(&mut self, update_fn: fn(&mut T) -> bool) {
        self.update_fn = update_fn;
    }
}

pub fn clear() {
    print!("\x1b[H\x1b[2J\x1b[3J");
    flush();
}

pub fn flush() {
    io::stdout().flush().unwrap();
}

pub fn create_frame() -> Vec<String> {
    clear();
    let (w, h) = terminal::size().unwrap();
    vec![" ".repeat(w as usize); h as usize - 1]
}

fn _render_line(frame_line: String, line: String, x: usize) -> String {
    let mut result = String::new();
    if x >= frame_line.len() {
        return result;
    }
    let lchars: Vec<char> = line.chars().collect();
    for (i, c) in frame_line.chars().enumerate() {
        if i >= x && lchars.len() > (i - x) {
            result += lchars[i - x].to_string().as_str();
        } else {
            result += c.to_string().as_str();
        }
    }
    result
}

fn read_key_() -> String {
    let mut buf = [0u8; 2];
    io::stdin().read(&mut buf).unwrap();
    let mut res = from_utf8(&buf).unwrap().to_string();
    if !res.ends_with("\0") {
        res += read_key_().as_str()
    }
    return res.strip_suffix("\0").unwrap_or(&res).to_string();
}

pub fn read_key() -> String {
    enable_raw_mode().unwrap();
    let res = read_key_();
    disable_raw_mode().unwrap();
    return res;
}

#[macro_export]
macro_rules! render_frame {
    ($first:expr) => {{
        let mut frame = osui::create_frame();
        $first.render(&mut frame);
        print!("{}", frame.join("\n"));
        frame
    }};
    ($first:expr, $($rest:expr),+) => {{
        let mut frame = osui::create_frame();
        $first.render(&mut frame);
        $( $rest.render(&mut frame); )+
        print!("{}", frame.join("\n"));
        frame
    }};
}

#[macro_export]
macro_rules! srender_frame {
    ($first:expr) => {{
        let mut frame = create_frame();
        $first.render(&mut frame);
        frame
    }};
    ($first:expr, $($rest:expr),+) => {{
        let mut frame = create_frame();
        $first.render(&mut frame);
        $( $rest.render(&mut frame); )+
        frame
    }};
}
