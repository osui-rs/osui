use std::{thread::sleep, time::Duration};

use crossterm::terminal;
use utils::{clear, hide_cursor, render_to_frame};

pub mod components;
pub mod macros;
pub mod utils;

#[derive(Debug)]
pub struct Params {
    pub expr: String,
    pub children: Vec<Component>,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub render: fn(&mut Component) -> String,
    pub update: fn(&mut Component),
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub expr: String,
    pub children: Vec<Component>,
}

impl Component {
    fn new(params: Params, render: fn(&mut Component) -> String) -> Component {
        Component {
            render,
            update: |_| {},
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            expr: params.expr,
            children: params.children,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Screen {
    component: Component,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            component: Component::new(
                Params {
                    expr: "".to_string(),
                    children: vec![],
                },
                |_| String::new(),
            ),
        }
    }
    pub fn set_component(&mut self, component: Component) {
        let (width, height) = terminal::size().unwrap();
        self.component = component;
        if self.component.width == 0 {
            self.component.width = width as usize;
        }
        if self.component.width == 0 {
            self.component.height = height as usize;
        }
        self.render();
    }

    pub fn render(&mut self) {
        let (width, height) = terminal::size().unwrap();
        let mut frame: Vec<String> = vec![" ".repeat(width as usize); height as usize];
        render_to_frame(width as usize, &mut frame, &mut self.component);
        clear();
        print!("{}", frame.join("\n"));
    }

    pub fn run(&mut self) {
        hide_cursor();
        // enable_raw_mode().unwrap();
        loop {
            self.render();
            sleep(Duration::from_millis(500));
        }
    }
}
