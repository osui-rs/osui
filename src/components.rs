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
                    this.set_child(closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Up,
                    ));
                }

                "down" => {
                    this.set_child(closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Down,
                    ));
                }

                "left" => {
                    this.set_child(closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Left,
                    ));
                }

                "right" => {
                    this.set_child(closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Right,
                    ));
                }

                _ => {
                    if let Some(child) = this.get_child() {
                        return (child.update)(child, k);
                    }
                }
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
        for (i, c) in &mut this.children.iter_mut().enumerate() {
            c.style.is_active = this.style.is_active && i == this.child;
            render_to_frame(&mut frame, c);
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

    component.style.clicked_fg = crate::style::Color::Red;
    component.style.hover_fg = crate::style::Color::Green;

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if k.kind == KeyKind::Enter {
            if this.toggle {
                this.clicked = !this.clicked;
                (this.on_click)(this);
            } else {
                return UpdateResponse::Tick(vec![1, 100]);
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
