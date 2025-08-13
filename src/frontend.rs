//! The frontend binder for OSUI.
//!
//! This module defines the internal representation of RSX elements — the tree-like structure used
//! by the `rsx!` macro in OSUI to declaratively build terminal user interfaces.
//!
//! RSX elements can be either static widgets or dynamically loaded ones with dependencies.
//! This structure enables recursive rendering, parent-child composition, and runtime dependency handling.

use std::sync::Arc;

use crate::{
    state::DependencyHandler,
    widget::{StaticWidget, Widget, WidgetLoad},
    Screen,
};

/// Represents a single element in the RSX tree.
pub enum RsxElement {
    /// A static widget with children.
    Element(StaticWidget, Rsx),

    /// A dynamically generated widget (e.g., with state) with associated dependencies and children.
    DynElement(
        Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
        Vec<Box<dyn DependencyHandler>>,
        Rsx,
    ),

    /// A dynamic iteration of widgets, keeping track of their state.
    Iter(
        Box<dyn FnMut() -> Rsx + Send + Sync>,
        Box<dyn DependencyHandler>,
    ),
}

/// A container representing a group of RSX elements.
/// This is typically created via the `rsx!` macro and rendered using a `Screen`.
pub struct Rsx(pub Vec<RsxElement>);

impl Rsx {
    /// Draws the RSX tree onto the given screen without a parent widget.
    ///
    /// This is the entry point for rendering the UI.
    pub fn draw(self, screen: &Arc<Screen>) {
        self.draw_parent(screen, None);
    }

    /// Recursively draws the RSX tree with an optional parent widget.
    ///
    /// Used internally to establish parent-child widget relationships.
    pub fn draw_parent(
        self,
        screen: &Arc<Screen>,
        parent: Option<Arc<Widget>>,
    ) -> Option<Arc<Widget>> {
        let mut first_widget = None;

        for rsx_elem in self.0 {
            match rsx_elem {
                RsxElement::DynElement(f, dep, child) => {
                    let w = screen.draw_box_dyn(f);

                    if let Some(parent) = &parent {
                        parent.get_elem().draw_child(&w);
                    }

                    for d in dep {
                        w.dependency_box(d);
                    }

                    child.draw_parent(screen, Some(w.clone()));
                    if first_widget.is_none() {
                        first_widget = Some(w)
                    }
                }

                RsxElement::Element(ws, child) => {
                    let w = Arc::new(Widget::Static(ws));
                    screen.draw_widget(w.clone());

                    if let Some(parent) = &parent {
                        parent.get_elem().draw_child(&w);
                    }

                    child.draw_parent(screen, Some(w.clone()));
                    if first_widget.is_none() {
                        first_widget = Some(w)
                    }
                }

                RsxElement::Iter(mut rsx, dep) => {
                    #[allow(unused)]
                    let mut rsx_len = 0;
                    let mut fw = {
                        let r = rsx();
                        rsx_len = r.0.len();
                        r.draw_parent(screen, parent.clone())
                    };
                    let parent = parent.clone();
                    let s = screen.clone();

                    screen.add_dependency(vec![dep], move || {
                        if let Some(fww) = &fw {
                            {
                                let mut widgets = s.widgets.lock().unwrap();
                                if let Some(i) = widgets.iter().position(|v| Arc::ptr_eq(v, fww)) {
                                    for v in (i..(i + rsx_len)).rev() {
                                        if let Some(w) = widgets.get(v) {
                                            if let Some(p) = &parent {
                                                p.get_elem().undraw_child(w);
                                            }
                                            widgets.remove(v);
                                        }
                                    }
                                }
                            }
                            fw = {
                                let r = rsx();
                                rsx_len = r.0.len();
                                r.draw_parent(&s, parent.clone())
                            };
                        }
                    });
                }
            }
        }

        first_widget
    }

    /// Adds a dynamically constructed element to the RSX tree.
    ///
    /// - `load`: A closure returning a `WidgetLoad` used to generate the widget.
    /// - `dependencies`: A list of dependency handlers for state or event updates.
    /// - `children`: Child RSX elements.
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

    /// Adds a statically defined element to the RSX tree with its children.
    pub fn create_element_static(&mut self, element: StaticWidget, children: Rsx) {
        self.0.push(RsxElement::Element(element, children));
    }

    /// Adds a statically defined element to the RSX tree with its children.
    pub fn iter<F: FnMut() -> Rsx + Send + Sync + 'static>(
        &mut self,
        rsx: F,
        dependency: Box<dyn DependencyHandler>,
    ) {
        self.0.push(RsxElement::Iter(Box::new(rsx), dependency));
    }

    /// Appends the elements from another `Rsx` tree into this one.
    pub fn expand(&mut self, other: &mut Rsx) {
        self.0.append(&mut other.0);
    }
}
