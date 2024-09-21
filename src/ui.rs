use crate::{Component, ComponentWrapper};
use k_board::{keyboard::Keyboard, keys::Keys};

impl Component for String {
    fn render(&mut self) -> String {
        self.clone()
    }
}

pub fn text(text: &str) -> ComponentWrapper<String> {
    ComponentWrapper::new(text.to_string())
}

pub struct Input {
    pub max_length: usize,
}

impl Component for Input {
    fn render(&mut self) -> String {
        format!(
            " {}\n|{}|\n {}\x1b[1A\x1b[{}D",
            "_".repeat(self.max_length),
            " ".repeat(self.max_length),
            "‾".repeat(self.max_length),
            self.max_length
        )
    }
}

impl Input {
    pub fn get_input(&mut self) -> String {
        let mut input = String::new();
        loop {
            let key = Keyboard::new().read_key();
            if key == Keys::Enter {
                return input;
            }
            if input.len() < self.max_length {
                match key {
                    Keys::Char(c) => {
                        input += c.to_string().as_str();
                        print!("{c}");
                    }
                    Keys::Space => {
                        input += " ";
                        print!(" ");
                    }
                    Keys::Delete => {
                        input.pop();
                        print!("\x1b[1D \x1b[1D");
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn input(max_length: usize) -> ComponentWrapper<Input> {
    ComponentWrapper::new(Input { max_length })
}
