use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use std::{
    io::{self, Read, Write},
    str::from_utf8,
};
pub mod key;
pub mod ui;

#[derive(Clone)]
pub struct ComponentWrapper<T> {
    pub component: T,
    pub x: usize,
    pub y: usize,
}

pub trait Component {
    fn render(&self) -> String {
        String::new()
    }
    fn run(&mut self, x: usize, y: usize) -> String {
        let _ = x;
        let _ = y;
        String::new()
    }
}

impl<T: Component> ComponentWrapper<T> {
    pub fn new(component: T) -> ComponentWrapper<T> {
        ComponentWrapper {
            component,
            x: 0,
            y: 0,
        }
    }
    pub fn run(&mut self) -> String {
        self.component.run(self.x, self.y)
    }
}

pub fn clear() {
    print!("\x1b[H\x1b[2J\x1b[3J");
    flush();
}

pub fn flush() {
    io::stdout().flush().unwrap();
}

pub fn render_frame<T: Component>(components: Vec<ComponentWrapper<T>>) {
    clear();
    let (w, h) = terminal::size().unwrap();
    let mut frame: Vec<String> = vec![" ".repeat(w as usize); h as usize - 1];
    for c in components {
        for (i, line) in c.component.render().split("\n").enumerate() {
            frame[c.y + i] = render_line(frame[c.y + i].clone(), line.to_string(), c.x);
        }
    }
    print!("{}", frame.join("\n"));
    flush();
}

fn render_line(frame_line: String, line: String, x: usize) -> String {
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
