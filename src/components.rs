use std::collections::HashMap;

use crate::{
    create_frame,
    key::{self, KeyKind},
    utils::{closest_component, render_to_frame},
    Component, UpdateResponse,
};

pub fn div() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([
        (KeyKind::Up, String::from("up")),
        (KeyKind::Down, String::from("down")),
        (KeyKind::Left, String::from("left")),
        (KeyKind::Right, String::from("right")),
    ]);

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            match v.as_str() {
                "up" => {
                    this.child =
                        closest_component(&this.children, this.child, crate::utils::Direction::Up);
                }

                "down" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Down,
                    );
                }

                "left" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Left,
                    );
                }

                "right" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Right,
                    );
                }

                _ => {}
            }
        } else {
            if let Some(child) = this.get_child() {
                return (child.update)(child, k);
            }
        }

        UpdateResponse::None
    }

    fn render(this: &mut Component) -> String {
        let mut frame: Vec<String> = create_frame!(this.width, this.height);
        for (i, child) in &mut this.children.iter_mut().enumerate() {
            if child.width == 0 {
                child.width = this.width;
            }
            if child.height == 0 {
                child.height = this.height;
            }
            child.style.is_active = this.style.is_active && i == this.child;
            render_to_frame(&mut frame, child);
        }
        frame.join("\n")
    }

    fn tick(this: &mut Component, i: usize) {
        if let Some(child) = this.get_child() {
            return (child.tick)(child, i);
        }
    }

    component.update = update;
    component.render = render;
    component.tick = tick;
    component
}

pub fn text() -> Component {
    let mut component = Component::new();

    fn update(_: &mut Component, _: key::Key) -> UpdateResponse {
        UpdateResponse::None
    }

    fn render(s: &mut Component) -> String {
        s.expr.clone()
    }

    component.update = update;
    component.render = render;
    component
}

pub fn button() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([(key::KeyKind::Enter, String::from("click"))]);

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            if v == "click" {
                if this.toggle {
                    this.clicked = !this.clicked;
                    (this.on_click)(this);
                } else {
                    return UpdateResponse::Tick(vec![1, 100]);
                }
            }
        }
        UpdateResponse::None
    }

    fn tick(this: &mut Component, i: usize) {
        if i == 0 {
            this.clicked = true;
            (this.on_click)(this);
        } else if i == 1 {
            this.clicked = false;
            (this.on_click)(this);
        }
    }

    fn render(this: &mut Component) -> String {
        if this.clicked {
            return this.style.write_clicked(&this.expr);
        }
        this.style.write(&this.expr)
    }

    component.update = update;
    component.render = render;
    component.tick = tick;
    component
}

pub fn tab() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([
        (key::KeyKind::Tab, String::from("next")),
        (key::KeyKind::ShiftTab, String::from("previous")),
    ]);

    component.style.clicked_fg = crate::style::Color::Red;

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            match v.as_str() {
                "next" => {
                    if this.child + 1 < this.children.len() {
                        this.child += 1;
                    } else {
                        this.child = 0;
                    }
                }
                "previous" => {
                    if this.child == 0 {
                        this.child = this.children.len() - 1;
                    } else {
                        this.child -= 1;
                    }
                }
                _ => {}
            }
        } else {
            if let Some(child) = this.get_child() {
                return (child.update)(child, k);
            } else {
            }
        }
        UpdateResponse::None
    }

    fn render(this: &mut Component) -> String {
        let mut frame: Vec<String> = create_frame!(this.width, this.height - 1);
        let w = this.width;
        let h = this.height;
        if let Some(child) = this.get_child() {
            if child.width == 0 {
                child.width = w;
            }
            if child.height == 0 {
                child.height = h;
            }
            child.style.is_active = true;
            render_to_frame(&mut frame, child);
        }
        let mut v: String = " ".repeat((this.width / 2) - this.children.len());
        for (i, _) in (&this.children).into_iter().enumerate() {
            if i == this.child {
                v += this.style.write_clicked("*").as_str()
            } else {
                v += this.style.write("*").as_str()
            }
        }
        format!("{}\n{}", v, frame.join("\n"))
    }

    fn tick(this: &mut Component, i: usize) {
        if let Some(child) = this.get_child() {
            return (child.tick)(child, i);
        }
    }

    component.update = update;
    component.tick = tick;
    component.render = render;
    component
}
