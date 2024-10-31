use std::collections::HashMap;

use crate::{
    create_frame, element,
    key::{Key, KeyKind},
    ui::Style,
    Element, ElementData, UpdateResponse,
};

macro_rules! update_child_data {
    ($elem:ident, $child:ident, $i:expr) => {
        let mut child_data = $child.get_data();
        if child_data.width == 0 {
            child_data.width = $elem.width;
        }
        if child_data.height == 0 {
            child_data.height = $elem.height;
        }
        child_data.style.is_active = $elem.style.is_active && $i == $elem.child;
        $child.set_data(child_data);
    };

    ($elem:expr, $child:ident) => {
        let mut child_data = $child.get_data();
        if child_data.width == 0 {
            child_data.width = $elem.width;
        }
        if child_data.height == 0 {
            child_data.height = $elem.height;
        }
        child_data.style.is_active = true;
        $child.set_data(child_data);
    };
}

element! {
    Text {}
    defaults {}
    fn render(&mut self) -> String {
        self.style.write(&self.text)
    }
}

element! {
    Div {
        pub binds: HashMap<KeyKind, String>
    }
    defaults {
        binds: HashMap::from([
            (KeyKind::Up, String::from("up")),
            (KeyKind::Down, String::from("down")),
            (KeyKind::Left, String::from("left")),
            (KeyKind::Right, String::from("right")),
        ]),
    }
    fn update(&mut self, k: Key) -> UpdateResponse {
        if let Some(v) = self.binds.get(&k.kind) {
            match v.as_str() {
                "up" => {
                    self.child =
                        crate::utils::closest_component(&self.children, self.child, crate::utils::Direction::Up);
                }

                "down" => {
                    self.child = crate::utils::closest_component(
                        &self.children,
                        self.child,
                        crate::utils::Direction::Down,
                    );
                }

                "left" => {
                    self.child = crate::utils::closest_component(
                        &self.children,
                        self.child,
                        crate::utils::Direction::Left,
                    );
                }

                "right" => {
                    self.child = crate::utils::closest_component(
                        &self.children,
                        self.child,
                        crate::utils::Direction::Right,
                    );
                }

                _ => {}
            }
        } else {
            if let Some(child) = self.get_child() {
                return child.update(k);
            }
        }

        UpdateResponse::None
    }

    fn render(&mut self) -> String {
        let mut frame: Vec<String> = create_frame!(self.width, self.height);
        for (i, child) in &mut self.children.iter_mut().enumerate() {
            update_child_data!(self, child, i);
            crate::utils::render_to_frame(&mut frame, child);
        }
        frame.join("\n")
    }

    fn tick(&mut self, i: usize) {
        if let Some(child) = self.get_child() {
            return child.tick(i);
        }
    }

}

element! {
    Button {
        pub binds: HashMap<KeyKind, String>,
        pub toggle: bool,
        pub on_click: fn(&mut Button),
        clicked: bool,
    }
    defaults {
        binds: HashMap::from([(KeyKind::Enter, String::from("click"))]),
        toggle: false,
        clicked: false,
        on_click: |_|{},
    }
    fn update(&mut self, k: Key) -> UpdateResponse {
        if let Some(v) = self.binds.get(&k.kind) {
            if v == "click" {
                if self.toggle {
                    self.clicked = !self.clicked;
                    (self.on_click)(self);
                } else {
                    return UpdateResponse::Tick(vec![1, 100]);
                }
            }
        }
        UpdateResponse::None
    }

    fn tick(&mut self, i: usize) {
        if i == 0 {
            self.clicked = true;
            (self.on_click)(self);
        } else if i == 1 {
            self.clicked = false;
            (self.on_click)(self);
        }
    }

    fn render(&mut self) -> String {
        if self.clicked {
            return self.style.write_clicked(&self.text);
        }
        self.style.write(&self.text)
    }

}

element! {
    Tab {
        pub binds: HashMap<KeyKind, String>,
    }
    defaults {
        binds: HashMap::from([
            (KeyKind::Tab, String::from("next")),
            (KeyKind::ShiftTab, String::from("previous")),
        ]),
    }
    fn update(&mut self, k: Key) -> UpdateResponse {
        if let Some(v) = self.binds.get(&k.kind) {
            match v.as_str() {
                "next" => {
                    if self.child + 1 < self.children.len() {
                        self.child += 1;
                    } else {
                        self.child = 0;
                    }
                }
                "previous" => {
                    if self.child == 0 {
                        self.child = self.children.len() - 1;
                    } else {
                        self.child -= 1;
                    }
                }
                _ => {}
            }
        } else {
            if let Some(child) = self.get_child() {
                return child.update(k);
            } else {
            }
        }
        UpdateResponse::None
    }

    fn render(&mut self) -> String {
        let mut frame: Vec<String> = create_frame!(self.width, self.height - 1);
        let d = self.get_data();
        if let Some(child) = self.get_child() {
            update_child_data!(d, child);
            crate::utils::render_to_frame(&mut frame, child);
        }
        let mut v: String = " ".repeat((self.width / 2) - self.children.len());
        for (i, _) in (&self.children).into_iter().enumerate() {
            if i == self.child {
                v += self.style.write_selected("*").as_str()
            } else {
                v += self.style.write("*").as_str()
            }
        }
        format!("{}\n{}", v, frame.join("\n"))
    }

    fn tick(&mut self, i: usize) {
        if let Some(child) = self.get_child() {
            return child.tick(i);
        }
    }
}

/// The response of a on_click function in a element
pub enum ClickResponse {
    None,
    Exit,
}

element! {
    Menu {
        pub items: Vec<String>,
        pub selected: usize,
        pub on_click: fn(&mut Menu, _: Key) -> ClickResponse,
    }
    defaults {
        items: Vec::new(),
        selected: 0,
        on_click: |_, _|ClickResponse::None
    }
    fn render(&mut self) -> String {
        let mut res: Vec<String> = Vec::new();

        for (i, item) in self.items.iter_mut().enumerate() {
            if i == self.selected {
                res.push(format!(
                    "{}{}",
                    self.style.write_cursor("> "),
                    self.style.write_selected(&item)
                ));
            } else {
                res.push(format!("  {}", item));
            }
        }
        res.join(&self.style.write("\n"))
    }

    fn update(&mut self, k: Key) -> UpdateResponse {
        if k.kind == KeyKind::Down {
            if self.items.len() > self.selected + 1 {
                self.selected += 1;
            } else {
                self.selected = 0;
            }
        } else if k.kind == KeyKind::Up {
            if self.child > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.items.len() - 1;
            }
        } else {
            return match (self.on_click)(self, k) {
                ClickResponse::Exit => UpdateResponse::Exit,
                ClickResponse::None => UpdateResponse::None,
            }
        }
        UpdateResponse::None
    }
}
