//! **OSUI** – A Rust Terminal User Interface Library
//!
//! OSUI is a library for building interactive and customizable terminal user interfaces in Rust.
//! It provides a component system, real-time keyboard input handling, and a `rsx!` macro
//! for defining UI elements in a declarative way, however that is optional.
//!
//! ✅ **Features**
//! - 🧱 RSX-like syntax with `rsx!` macro
//! - 🖥️ Virtual screen abstraction
//! - 🎹 Keyboard input handling
//! - 🎯 Component-based design
//! - ⚡ Real-time rendering
//!
//! 🚀 **Quick Example**
//! ```rust
//! use osui::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     let screen = Screen::new();
//!     rsx! {
//!         "👋 Hello, World!"
//!     }.draw(&screen);
//!     screen.run()
//! }
//! ```
//!
//! ---
//! 🧰 For full documentation, visit: [osui.netlify.app/docs](https://osui.netlify.app/docs)  
//!
//! 🧪 Examples and demos: [github.com/osui-rs/osui/demos](https://github.com/osui-rs/osui/tree/master/src/demos)

use std::sync::{Arc, Mutex};

use crate::{
    extensions::{Extension, Handler},
    prelude::Context,
    render_scope::RenderScope,
    widget::{BoxedElement, DynWidget, Element, StaticWidget, Widget, WidgetLoad},
};

pub mod elements;
pub mod extensions;
pub mod frontend;
pub mod macros;
pub mod render_scope;
pub mod state;
pub mod style;
pub mod utils;
pub mod widget;

pub mod prelude {
    pub use crate::{
        elements::*, extensions::*, frontend::*, render_scope::*, state::*, style::*, utils::*,
        widget::*, *,
    };

    pub use crate::style::{
        Dimension::{Content, Full},
        Position::{Center, End},
    };
}

/// The main screen abstraction for rendering and managing widgets and extensions.
///
/// `Screen` holds the root widget list and registered extensions. It provides methods
/// for drawing elements, adding extensions, and running the main rendering loop.
///
/// # Examples
/// ```rust
/// let screen = Screen::new();
/// rsx! { "Hello" }.draw(&screen);
/// screen.run()?;
/// ```
pub struct Screen {
    /// The list of widgets currently managed by the screen.
    pub widgets: Mutex<Vec<Arc<Widget>>>,
    /// Registered extensions for the screen.
    extensions: Mutex<Vec<Arc<Mutex<Box<dyn Extension + Send + Sync>>>>>,
    /// If it's still running
    running: Mutex<bool>,
}

event!(RenderWrapperEvent(*mut RenderScope));
component!(NoRender);
component!(NoRenderRoot);

impl RenderWrapperEvent {
    /// Returns a mutable reference to the underlying `RenderScope`.
    ///
    /// # Safety
    /// The caller must ensure the pointer is valid for the lifetime of the event.
    pub fn get_scope(&self) -> &mut RenderScope {
        unsafe { &mut *self.0 }
    }
}

unsafe impl Send for RenderWrapperEvent {}
unsafe impl Sync for RenderWrapperEvent {}

impl Screen {
    /// Creates a new screen instance wrapped in an `Arc`.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            widgets: Mutex::new(Vec::new()),
            extensions: Mutex::new(Vec::new()),
            running: Mutex::new(true),
        })
    }

    /// Draws a static element and returns its widget handle.
    pub fn draw<E: Element + 'static + Send + Sync>(self: &Arc<Self>, element: E) -> Arc<Widget> {
        self.draw_box(Box::new(element))
    }

    /// Draws a boxed element and returns its widget handle.
    pub fn draw_box(self: &Arc<Self>, element: BoxedElement) -> Arc<Widget> {
        let w = Arc::new(Widget::Static(StaticWidget::new(element)));
        self.draw_widget(w.clone());
        w
    }

    /// Draws a dynamic element using a closure and returns its widget handle.
    pub fn draw_dyn<F: FnMut() -> WidgetLoad + 'static + Send + Sync>(
        self: &Arc<Self>,
        element: F,
    ) -> Arc<Widget> {
        self.draw_box_dyn(Box::new(element))
    }

    /// Draws a dynamic element from a boxed closure and returns its widget handle.
    pub fn draw_box_dyn(
        self: &Arc<Self>,
        element: Box<dyn FnMut() -> WidgetLoad + Send + Sync>,
    ) -> Arc<Widget> {
        let w = Arc::new(Widget::Dynamic(DynWidget::new(element)));
        self.draw_widget(w.clone());
        w
    }

    /// Adds an existing widget to the screen.
    pub fn draw_widget(self: &Arc<Self>, widget: Arc<Widget>) {
        if self.widgets.lock().unwrap().len() == 0 {
            widget.set_focused(true); // first widget is focused
            self.widgets.lock().unwrap().push(widget);
        } else {
            self.widgets.lock().unwrap().push(widget);
        }
    }

    /// Registers an extension with the screen.
    pub fn extension<E: Extension + Send + Sync + 'static>(self: &Arc<Self>, ext: E) {
        self.extensions
            .lock()
            .unwrap()
            .push(Arc::new(Mutex::new(Box::new(ext))));
    }

    /// Runs the main rendering loop, calling extensions and rendering widgets.
    ///
    /// This method blocks and repeatedly renders the screen at a fixed interval.
    pub fn run(self: &Arc<Self>) -> std::io::Result<()> {
        let ctx = Context::new(self.clone());

        for ext in self.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().init(&ctx);
        }

        utils::hide_cursor()?;

        while *self.running.lock().unwrap() {
            self.render(&ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(28));
        }

        Ok(())
    }

    /// Renders all widgets and applies extensions.
    ///
    /// This method is called internally by `run`.
    pub fn render(self: &Arc<Self>, ctx: &Context) -> std::io::Result<()> {
        let mut scope = RenderScope::new();
        let (w, h) = crossterm::terminal::size().unwrap();
        scope.set_parent_size(w, h);

        for ext in self.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().render(ctx, &mut scope);
        }

        utils::clear()?;
        for elem in self.widgets.lock().unwrap().iter() {
            if elem.get::<NoRender>().is_some() || elem.get::<NoRenderRoot>().is_some() {
                elem.auto_refresh();
                continue;
            }

            if let Some(wrapper) = elem.get::<Handler<RenderWrapperEvent>>() {
                wrapper.call(elem, &RenderWrapperEvent(&mut scope));
            } else {
                scope.clear();

                if let Some(style) = elem.get() {
                    scope.set_style(style);
                }
                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }

                for ext in self.extensions.lock().unwrap().iter() {
                    ext.lock().unwrap().render_widget(ctx, &mut scope, elem);
                }

                elem.get_elem().render(&mut scope, ctx);

                if let Some(t) = elem.get() {
                    scope.set_transform(&t);
                }
                scope.draw();

                elem.get_elem().after_render(&mut scope, ctx);
            }

            elem.auto_refresh();
        }
        Ok(())
    }

    /// Closes the loop and calls `on_close` in the extensions
    pub fn close(self: &Arc<Self>) {
        *self.running.lock().unwrap() = false;

        utils::show_cursor().unwrap();

        utils::clear().unwrap();

        for ext in self.extensions.lock().unwrap().iter() {
            ext.lock().unwrap().on_close();
        }
    }
}
