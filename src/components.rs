use std::collections::HashMap;

use crate::{
    create_frame,
    key::KeyKind,
    utils::{closest_component, render_to_frame},
    Component, UpdateRequest, UpdateResponse,
};

pub fn div() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([
        (KeyKind::Up, String::from("up")),
        (KeyKind::Down, String::from("down")),
        (KeyKind::Left, String::from("left")),
        (KeyKind::Right, String::from("right")),
    ]);

    fn update(this: &mut Component, req: UpdateRequest) -> UpdateResponse {
        if let Some(v) = this.binds.get(&req.key.kind) {
            match v.as_str() {
                "up" => {
                    this.active_child = closest_component(
                        &this.children,
                        this.active_child,
                        crate::utils::Direction::Up,
                    );
                }

                "down" => {
                    this.active_child = closest_component(
                        &this.children,
                        this.active_child,
                        crate::utils::Direction::Down,
                    );
                }

                "left" => {
                    this.active_child = closest_component(
                        &this.children,
                        this.active_child,
                        crate::utils::Direction::Left,
                    );
                }

                "right" => {
                    this.active_child = closest_component(
                        &this.children,
                        this.active_child,
                        crate::utils::Direction::Right,
                    );
                }

                _ => {
                    if let Some(child) = this.get_active_child() {
                        return (child.update)(child, req);
                    }
                }
            }
        } else {
            if let Some(child) = this.get_active_child() {
                return (child.update)(child, req);
            }
        }

        UpdateResponse::None
    }

    fn render(s: &mut Component) -> String {
        let mut frame: Vec<String> = create_frame!(s.width, s.height);
        for c in &mut s.children {
            render_to_frame(&mut frame, c);
        }
        frame.join("\n")
    }

    component.update = update;
    component.render = render;
    component
}

pub fn text() -> Component {
    let mut component = Component::new();

    fn update(_: &mut Component, _: UpdateRequest) -> UpdateResponse {
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

    fn update(this: &mut Component, req: UpdateRequest) -> UpdateResponse {
        if req.key.kind == KeyKind::Enter {
            if this.toggle {
                this.clicked = !this.clicked;
                (this.on_click)(this);
            } else {
                this.clicked = true;
                (this.on_click)(this);
                std::thread::sleep(std::time::Duration::from_millis(300));
                this.clicked = false;
                (this.on_click)(this);
            }
        }
        UpdateResponse::None
    }

    fn render(s: &mut Component) -> String {
        s.expr.clone()
    }

    component.update = update;
    component.render = render;
    component
}
