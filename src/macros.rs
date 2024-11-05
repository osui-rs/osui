//! # Macros Module
//!
//! This module defines various macros for creating and manipulating OSUI elements.
//! The macros provide a clean and concise syntax for defining UI components and
//! handling their parameters, commands, and rendering.

/// Macro for defining an OSUI `Element`.
///
/// This macro generates a new element type with specified parameters and default values.
///
/// # Example
/// ```
/// element! {
///     MyElem {}
///     defaults {}
///     // functions (render, update)
/// }
/// ```
///
/// # Parameters
/// - `name`: The name of the element type being defined.
/// - `inner`: Additional fields or methods for the element.
/// - `defaults`: Default values for the element's properties.
/// - `functions`: Functions to implement for the element (e.g., rendering, updating).
#[macro_export]
macro_rules! element {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $inner:tt )*
        }
        defaults {$( $defaults:tt )*}
        $( $functions:tt )*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            pub x: usize,
            pub y: usize,
            pub width: ElementSize,
            pub height: ElementSize,
            pub children: Vec<Box<dyn Element>>,
            pub child: usize,
            pub text: String,
            pub style: Style,
            $( $inner )*
        }

        impl Element for $name {
            fn get_data(&self) -> ElementData {
                ElementData {
                    x: self.x,
                    y: self.y,
                    width: self.width.clone(),
                    height: self.height.clone(),
                }
            }

            fn update_data(&mut self, width: usize, height: usize) {
                self.width.try_set_size(width);
                self.height.try_set_size(height);
                for child in &mut self.children {
                    child.update_data(width, height);
                }
            }
            $( $functions )*
        }

        impl $name {
            /// Creates a new instance of the element with default values.
            pub fn new() -> $name {
                $name {
                    children: Vec::new(),
                    x: 0,
                    y: 0,
                    width: ElementSize::Default(0),
                    height: ElementSize::Default(0),
                    child: 0,
                    text: String::new(),
                    style: Style::default(),
                    $( $defaults )*
                }
            }

            /// Retrieves a mutable reference to the current child element, if any.
            ///
            /// # Returns
            /// An `Option` containing a mutable reference to the child `Element` or `None`.
            pub fn get_child(&mut self) -> Option<&mut Box<dyn Element>> {
                self.children.get_mut(self.child)
            }
        }
    };
}

/// Macro for creating a command response from a list of commands.
///
/// # Example
/// ```
/// command!(Update(1), Render(2));
/// ```
///
/// # Parameters
/// - A list of commands to be included in the `UpdateResponse::CommandList`.
#[macro_export]
macro_rules! command {
    ($($cmd:expr),*) => {
        UpdateResponse::CommandList(vec![$($cmd),*])
    };
}

/// Macro for parsing parameters and children for an OSUI element.
///
/// This macro updates the properties of an element based on provided key-value pairs,
/// and adds child elements to the parent element.
///
/// # Parameters
/// - `elem`: The element being updated.
/// - Various key-value pairs, child elements, and text content.
#[macro_export]
macro_rules! parse_rsx_param {
    ($elem:expr, ) => {};

    ($elem:expr, $($k:ident: $v:expr),*) => {
        $(
            $elem.$k = $v;
        )*
    };

    ($elem:expr, $($k:ident: $v:expr),*; $($rest:tt)*) => {
        $(
            $elem.$k = $v;
        )*
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, {$ielem:expr} $($rest:tt)*) => {
        $elem.children.push($ielem);
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $($k:ident: $v:expr),*, $text:expr) => {
        $(
            $elem.$k = $v;
        )*
        $elem.text = format!($text);
    };

    ($elem:expr, $pelem:path { $($inner:tt)* } $($rest:tt)*) => {
        $elem.children.push(osui::rsx_elem!($pelem { $($inner)* }));
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, $text:expr) => {
        $elem.text = format!($text);
    };
}

/// Macro for creating an OSUI element with parsed parameters.
///
/// # Example
/// ```
/// rsx! {
///     text { "Hello, World!" }
///     div {
///         button { y: 2, "click me" }
///     }
/// }
/// ```
///
/// This macro provides a clean way to express OSUI elements, functioning like a div that can
/// contain multiple components.
#[macro_export]
macro_rules! rsx_elem {
    ($elem:path { $($inner:tt)* }) => {{
        let mut elem = $elem();
        osui::parse_rsx_param!(elem, $($inner)*);
        elem as Box<dyn osui::Element>
    }};
}

/// Macro for defining a structured representation of UI elements in OSUI.
///
/// # Example
/// ```
/// rsx! {
///     text { "Hello, World!" }
///     div {
///         button { y: 2, "click me" }
///     }
/// }
/// ```
///
/// This macro allows the creation of a root element that can contain other elements.
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {{
        osui::rsx_elem!( osui::ui::div { $($inner)* } )
    }};
}

/// Macro for defining CSS styles in OSUI.
///
/// # Example
/// ```
/// css!(Style; color: "red"; background: "white");
/// ```
///
/// This macro creates a new style based on the provided properties, applying default values
/// for any unspecified fields.
#[macro_export]
macro_rules! css {
    (
        $style:path;
        $($inner:tt)*
    ) => {{
        $style {
            $($inner)*
            ..Default::default()
        }
    }};
}