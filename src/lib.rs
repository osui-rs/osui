pub mod components;
pub mod key;
pub mod macros;
pub mod utils;

use crossterm;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateResponse {
    Exit,
    Done,
    None,
}

#[derive(Debug)]
pub struct Params {
    pub expr: String,
    pub children: Vec<Component>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub render: fn(&mut Component) -> String,
    pub update: fn(&mut Component, key::Key) -> UpdateResponse,
    pub on_click: fn(&mut Component),
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub expr: String,
    pub children: Vec<Component>,
    pub active_child: usize,
    pub clicked: bool,
    pub toggle: bool,
    pub binds: HashMap<key::KeyKind, String>,
}

impl Component {
    /// Creates a new component
    fn new(params: Params) -> Component {
        Component {
            render: |_| String::new(),
            update: |_, _| UpdateResponse::None,
            on_click: |_| {},
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            expr: params.expr,
            children: params.children,
            active_child: 0,
            clicked: false,
            toggle: false,
            binds: HashMap::new(),
        }
    }

    pub fn get_active_child(&mut self) -> Option<&mut Component> {
        if self.active_child < self.children.len() {
            return self.children.get_mut(self.active_child);
        }
        None
    }

    fn manhattan_distance(&self, other: &Component) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
pub struct App {
    component: Component,
}

impl App {
    /// Creates a new screen to render components
    pub fn new() -> App {
        App {
            component: Component::new(Params {
                expr: "".to_string(),
                children: vec![],
            }),
        }
    }

    /// Sets a component
    pub fn set_component(&mut self, component: Component) {
        let (width, height) = crossterm::terminal::size().unwrap();
        self.component = component;
        if self.component.width == 0 {
            self.component.width = width as usize;
        }
        if self.component.height == 0 {
            self.component.height = height as usize;
        }
    }

    /// Render to the screen
    pub fn render(&mut self) {
        let (width, height) = crossterm::terminal::size().unwrap();
        let mut frame: Vec<String> = create_frame!(width as usize, height as usize);
        utils::render_to_frame(&mut frame, &mut self.component);
        utils::clear();
        print!("{}", frame.join(""));
        utils::flush();
    }

    /// Run the screen
    pub fn run(&mut self) {
        utils::hide_cursor();
        utils::clear();
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            self.render();
            self.component.expr = "Hello".to_string();
            match (self.component.update)(&mut self.component, key::read_key()) {
                UpdateResponse::Exit => {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    utils::clear();
                    utils::show_cursor();
                    return;
                }
                _ => {}
            }
        }
    }
}
