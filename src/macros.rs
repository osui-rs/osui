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
        #[derive(Clone)]
        pub struct $name<'a> {
            pub x: usize,
            pub y: usize,
            pub width: Value<usize>,
            pub height: Value<usize>,
            pub children: Vec<Box<dyn Element>>,
            pub child: usize,
            pub text: String,
            pub id: &'a str,
            $( $inner )*
        }

        impl Element for $name<'_> {
            fn get_data(&self) -> ElementData {
                ElementData {
                    x: self.x,
                    y: self.y,
                    width: self.width.clone(),
                    height: self.height.clone(),
                    children: self.children.clone(),
                    child: self.child.clone(),
                    text: self.text.clone(),
                    id: self.id.to_string(),
                }
            }

            fn update_data(&mut self, width: usize, height: usize) {
                self.width.try_set_value(width);
                self.height.try_set_value(height);
                for child in &mut self.children {
                    child.update_data(width, height);
                }
            }

            fn get_id(&mut self) -> String {
                self.id.to_string()
            }

            fn get_element_by_id(&mut self, id: &str) -> Option<&mut Box<dyn Element>> {
                for elem in &mut self.children {
                    if elem.get_id() == id {
                        return Some(elem);
                    }
                }
                None
            }

            fn set_data(&mut self, data: ElementData) {
                self.x = data.x;
                self.y = data.y;
                self.width = data.width;
                self.height = data.height;
                self.children = data.children;
                self.child = data.child;
                self.text = data.text;
            }

            $( $functions )*
        }

        impl<'a> $name<'_> {
            /// Creates a new instance of the element with default values.
            pub fn new() -> $name<'a> {
                $name {
                    children: Vec::new(),
                    x: 0,
                    y: 0,
                    width: Value::Default(0),
                    height: Value::Default(0),
                    child: 0,
                    text: String::new(),
                    id: "",
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
/// - Various key-value pairs, child elements, and text content. even for loops
#[macro_export]
macro_rules! parse_rsx_param {
    // Properties
    ($elem:expr, $($k:ident).+: $v:expr) => {
        $elem.$($k).+ = $v;
    };

    ($elem:expr, $($k:ident).+: $v:expr, $($rest:tt)*) => {
        $elem.$($k).+ = $v;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // for completion purposes
    ($elem:expr, $p:path) => {
        $p;
    };
    ($elem:expr, $($k:ident).+., $($rest:tt)*) => {
        $elem.$($k)+.;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $($k:ident).+.: $v:expr) => {
        $elem.$($k).+. = $v;
    };
    ($elem:expr, $($k:ident).+.: $v:expr, $($rest:tt)*) => {
        $elem.$($k).+. = $v;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // For loop
    ($elem:expr, for ($($for:tt)*) { $($inner:tt)* } $($rest:tt)*) => {
        for $($for)* {
            $elem.children.push({$($inner)*})
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // Expression
    ($elem:expr, {$ielem:expr} $($rest:tt)*) => {
        $elem.children.push($ielem);
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // Element
    ($elem:expr, $pelem:path { $($inner:tt)* } $($rest:tt)*) => {
        $elem.children.push(osui::rsx_elem!($pelem { $($inner)* }));
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    // Text
    ($elem:expr, $text:expr) => {
        $elem.text = format!($text);
    };

    // Empty
    ($elem:expr, ) => {};
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
/// # For loops
/// ```
/// rsx! {
///     text { "Welcome!" }
///     for (i in 1..5) { // must be contained with ()
///         rsx_elem!{text { y: i, "Text: {i}" }}
///     }
/// }
/// ```
///
/// This macro provides a clean way to express OSUI elements, functioning like a div that can
/// contain multiple components.
#[macro_export]
macro_rules! rsx_elem {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        osui::parse_rsx_param!(elem, $($inner)*);
        elem as Box<dyn osui::Element>
    }};
}

#[macro_export]
macro_rules! rsx_elem_raw {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        osui::parse_rsx_param!(elem, $($inner)*);
        elem
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
        osui::rsx_elem_raw!( osui::ui::div { $($inner)* } )
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

#[macro_export]
macro_rules! arc {
    ($a:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($a))
    };
}

#[macro_export]
macro_rules! val {
    ($a:expr) => {
        osui::Value::new($a)
    };
}
