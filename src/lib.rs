pub mod components;
pub mod key;
pub mod macros;
pub mod utils;

use crossterm;
use key::Key;
use std::collections::HashMap;
use utils::{clear, show_cursor, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateResponse {
    Exit,
    Done,
    None,
    SetComponent(Component),
}

#[derive(Debug, Clone)]
pub enum UpdateRequest {
    Key(Key),
}

#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub request: UpdateRequest,
    pub response: UpdateResponse,
}

#[derive(Debug)]
pub struct Params {
    pub expr: String,
    pub children: Vec<Component>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub render: fn(&mut Component) -> String,
    pub update: fn(&mut Component, &mut UpdateContext),
    pub on_click: fn(&mut UpdateContext),
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub expr: String,
    pub children: Vec<Component>,
    pub clicked: bool,
    pub toggle: bool,
    pub data: HashMap<String, Value>,
}

impl Component {
    /// Creates a new component
    fn new(params: Params) -> Component {
        Component {
            render: |_| String::new(),
            update: |_, _| {},
            on_click: |_| {},
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            expr: params.expr,
            children: params.children,
            clicked: false,
            toggle: false,
            data: HashMap::new(),
        }
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
        self.render();
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
            let mut ctx = UpdateContext {
                response: UpdateResponse::None,
                request: UpdateRequest::Key(key::read_key()),
            };
            (self.component.update)(&mut self.component, &mut ctx);
            match ctx.response {
                UpdateResponse::Exit => {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    clear();
                    show_cursor();
                    return;
                }
                UpdateResponse::SetComponent(c) => self.component = c,
                _ => {}
            }
        }
    }
}
