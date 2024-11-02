use std::collections::HashMap;

use crate::{
    element,
    key::KeyKind,
    ui::Style,
    Element, ElementData, UpdateContext,
};

element! {
    Text {}
    defaults {}
    fn render(&mut self, _: usize) -> String {
        self.style.write(&self.text)
    }
}

element! {
    Button {
        pub binds: HashMap<KeyKind, String>,
        pub on_click: fn(&mut Button),
        clicked: bool
    }
    defaults {
        binds: HashMap::from([(KeyKind::Enter, String::from("click"))]),
        on_click: |_|{},
        clicked: false,
    }

    fn update(&mut self, ctx: &mut UpdateContext) {
        if let Some(v) = self.binds.get(&ctx.key.kind) {
            if v == "click" {
                self.clicked = true;
                self.add_action(ctx.tick+5, "un_click");
                (self.on_click)(self);
            }
        }
    }

    fn render(&mut self, tick: usize) -> String {

        if self.get_action(tick)=="un_click" {
            self.clicked = false;
        }

        if self.clicked {
            return self.style.write_clicked(&self.text);
        }
        self.style.write(&self.text)
    }

}
