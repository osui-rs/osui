use std::sync::Arc;

use crate::{
    state::DependencyHandler,
    widget::{StaticWidget, Widget, WidgetLoad},
    Screen,
};

pub enum RsxElement {
    Element(Arc<StaticWidget>, Rsx),

    DynElement(
        Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
        Vec<Box<dyn DependencyHandler>>,
        Rsx,
    ),
}

pub struct Rsx(pub Vec<RsxElement>);

impl Rsx {
    pub fn draw(self, screen: &Arc<Screen>) {
        self.draw_parent(screen, None);
    }

    pub fn draw_parent(self, screen: &Arc<Screen>, parent: Option<Arc<Widget>>) {
        for rsx_elem in self.0 {
            match rsx_elem {
                RsxElement::DynElement(f, dep, child) => {
                    let w = if let Some(parent) = &parent {
                        let w = screen.draw_box_dyn(f);
                        parent.get_elem().draw_child(&w);
                        w
                    } else {
                        screen.draw_box_dyn(f)
                    };

                    for d in dep {
                        w.dependency_box(d);
                    }
                    child.draw_parent(screen, Some(w.clone()));
                }

                RsxElement::Element(ws, child) => {
                    let w = Arc::new(Widget::Static(ws));
                    screen.draw_widget(w.clone());

                    if let Some(parent) = &parent {
                        parent.get_elem().draw_child(&w);
                    }

                    child.draw_parent(screen, Some(w.clone()));
                }
            }
        }
    }

    pub fn create_element<F: FnMut() -> WidgetLoad + Send + Sync + 'static>(
        &mut self,
        load: F,
        dependencies: Vec<Box<dyn DependencyHandler>>,
        children: Rsx,
    ) {
        self.0.push(RsxElement::DynElement(
            Box::new(load),
            dependencies,
            children,
        ));
    }

    pub fn create_element_static(&mut self, element: Arc<StaticWidget>, children: Rsx) {
        self.0.push(RsxElement::Element(element, children));
    }

    pub fn expand(&mut self, other: &mut Rsx) {
        self.0.append(&mut other.0);
    }
}
