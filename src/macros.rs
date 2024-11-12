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
        $(#[$meta_style:meta])*
        style $style:ident {
            $($sn:ident: $st:ty),*$(,)?
        }
        $(#[$meta:meta])*
        $name:ident {
            $( $inner:tt )*
        }
        defaults {$( $defaults:tt )*}
        $( $functions:tt )*
    ) => {
        $(#[$meta_style])*
        #[derive(Clone)]
        pub struct $style {
            pub color: Color,
            pub background: Color,
            pub font: Font,
            pub hover_color: Color,
            pub hover_background: Color,
            pub hover_font: Font,
            $(pub $sn: $st),*
        }

        impl Default for $style {
            fn default() -> $style {
                $style {
                    color: Color::default(),
                    background: Color::default(),
                    font: Font::default(),
                    hover_color: Color::default(),
                    hover_background: Color::default(),
                    hover_font: Font::default(),
                    $($sn: <$st>::default()),*
                }
            }
        }

        element! {
            $(#[$meta])*
            $name {
                pub style: $style,
                $( $inner )*
            }
            defaults {
                style: $style::default(),
                $($defaults)*
            }
            $( $functions )*
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $inner:tt )*
        }
        defaults {$( $defaults:tt )*}
        $( $functions:tt )*
    ) => {
        $(#[$meta])*
        pub struct $name<'a> {
            pub x: std::sync::Mutex<$crate::Value<usize>>,
            pub y: std::sync::Mutex<usize>,
            pub width: std::sync::Mutex<$crate::Value<usize>>,
            pub height: std::sync::Mutex<$crate::Value<usize>>,
            pub children: std::sync::Mutex<Vec<std::sync::Arc<dyn $crate::Element>>>,
            pub child: std::sync::Mutex<usize>,
            pub text: std::sync::Mutex<String>,
            pub id: std::sync::Mutex<&'a str>,
            $( $inner )*
        }

        impl $crate::Element for $name<'_> {
            fn get_data(&self) -> ($crate::Value<usize>, usize, String) {
                (self.x.lock().unwrap().clone(), self.y.lock().unwrap().clone(), self.id.lock().unwrap().to_string())
            }

            fn update_data(&self, width: usize, height: usize) {
                self.width.lock().unwrap().try_set_value(width);
                self.height.lock().unwrap().try_set_value(height);
                let children = self.children.lock().unwrap();
                for child in children.iter() {
                    child.update_data(width, height);
                }
            }

            fn get_element_by_id(&self, id: &str) -> Option<std::sync::Arc<(dyn $crate::Element + 'static)>> {
                for elem in &mut self.children.lock().unwrap().iter() {
                    if elem.get_data().2 == id {
                        return Some(std::sync::Arc::clone(elem));
                    }
                }
                None
            }

            $( $functions )*
        }

        impl<'a> $name<'_> {
            /// Creates a new instance of the element with default values.
            pub fn new() -> $name<'a> {
                $crate::def_!($name {
                    children: Vec::new(),
                    x: $crate::Value::Default(0),
                    y: 0,
                    width: $crate::Value::Default(0),
                    height: $crate::Value::Default(0),
                    child: 0,
                    text: String::new(),
                    id: "",
                    $( $defaults )*
                })
            }

            pub fn get_child(&self) -> Option<std::sync::Arc<dyn $crate::Element>> {
                if let Ok(elements) = self.children.lock() {
                    if let Some(e) = elements.get(*self.child.lock().unwrap()) {
                        return Some(std::sync::Arc::clone(e));
                    }
                }
                None
            }
        }
    };
}

#[macro_export]
macro_rules! def_ {
    ($name:ident { $($k:ident : $v:expr),* $(,)? }) => {
        $name {
            $(
                $k : std::sync::Mutex::new($v)
            ),*
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
/// - A list of commands to be included in the `EventResponse::CommandList`.
#[macro_export]
macro_rules! command {
    ($($cmd:expr),*) => {
        EventResponse::CommandList(vec![$($cmd),*])
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
    ($elem:expr, $a:ident $($k:ident).+: $v:expr) => {
        $elem.$a.lock().unwrap().$($k).+ = $v;
    };

    ($elem:expr, $a:ident $(.$($k:ident).+)?: $v:expr, $($rest:tt)*) => {
        *$elem.$a.lock().unwrap()$($($k).+)? = $v;
        $crate::parse_rsx_param!($elem, $($rest)*);
    };

    // for completion purposes
    // ($elem:expr, $p:path) => {
    //     $p;
    // };
    // ($elem:expr, $a:ident . $($k:ident).+, $($rest:tt)*) => {
    //     $elem.$a.lock().unwrap().$($k).+;
    //     $crate::parse_rsx_param!($elem, $($rest)*);
    // };
    // ($elem:expr, $a:ident . $($k:ident).+., $($rest:tt)*) => {
    //     $elem.$a.lock().unwrap().$($k).
    //     +.;
    //     $crate::parse_rsx_param!($elem, $($rest)*);
    // };

    // ($elem:expr, $a:ident . $($k:ident).+.: $v:expr) => {
    //     $elem.$a.lock().unwrap().$($k).+. = $v;
    // };
    // ($elem:expr, $a:ident . $($k:ident).*: $v:expr, $($rest:tt)*) => {
    //     $elem.$a.lock().unwrap().$($k).+. = $v;
    //     $crate::parse_rsx_param!($elem, $($rest)*);
    // };

    // For loop
    ($elem:expr, for ($($for:tt)*) { $($inner:tt)* } $($rest:tt)*) => {
        for $($for)* {
            $elem.children.lock().unwrap().push({$($inner)*})
        }
        $crate::parse_rsx_param!($elem, $($rest)*);
    };

    // Expression
    ($elem:expr, {$ielem:expr} $($rest:tt)*) => {
        $elem.children.lock().unwrap().push($ielem);
        $crate::parse_rsx_param!($elem, $($rest)*);
    };

    // Element
    ($elem:expr, $pelem:path { $($inner:tt)* } $($rest:tt)*) => {
        $elem.children.lock().unwrap().push($crate::rsx_elem!($pelem { $($inner)* }));
        $crate::parse_rsx_param!($elem, $($rest)*)
    };

    // Text
    ($elem:expr, $text:expr) => {
        *$elem.text.lock().unwrap() = format!($text);
    };
    ($elem:expr, $text:expr, $($inner:tt)*) => {
        *$elem.text.lock().unwrap() = format!($text, $($inner)*);
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
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem as std::sync::Arc<dyn $crate::Element>
    }};
}

#[macro_export]
macro_rules! rsx_elem_raw {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        $crate::parse_rsx_param!(elem, $($inner)*);
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
        $crate::rsx_elem_raw!( $crate::ui::div { $($inner)* } )
    }};
}

/// Macro for defining CSS styles in OSUI.
///
/// # Example
/// ```
/// css!(Style; color: Color::Red; background: "white");
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

/// Macro for defining a `Arc<Mutex< T >>`
#[macro_export]
macro_rules! mux {
    ($a:expr) => {
        std::sync::Mutex::new($a)
    };
}

#[macro_export]
macro_rules! arc {
    ($a:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($a))
    };
}

/// Macro for defining a `Value< T >`
#[macro_export]
macro_rules! val {
    ($a:expr) => {
        $crate::Value::new($a)
    };
}

/// Macro for writing when requested to render
///
/// # Example
/// ```
/// write!(self, clicked, "Hello, World!") // For custom styles, You need (color, background, font)
///
/// write!((self, state), "Hello, World!") // When you don't need a custom style, you use () and put self and the state of the render
/// ```
///
/// This macro returns a `String` depending on the state or the kind of style
/// When on "state" mode it returns the default style if the state is 0
#[macro_export]
macro_rules! write {
    // Default state styling
    (($self:ident, $state:expr), $expr:expr) => {{
        if $state == 0 {
            return format!(
                "{}{}{}{}\x1b[0m",
                $self.style.color.ansi(),
                $self.style.background.ansi_bg(),
                $self.style.font.ansi(),
                $expr
            );
        }
        format!(
            "{}{}{}{}\x1b[0m",
            $self.style.hover_color.ansi(),
            $self.style.hover_background.ansi_bg(),
            $self.style.hover_font.ansi(),
            $expr
        )
    }};

    // Custom styles
    ($self:ident, $kind:ident, $expr:expr) => {{
        use paste::paste;
        format!(
            "{}{}{}{}\x1b[0m",
            paste! {
                $self.style.[<$kind _color>].ansi()
            },
            paste! {
                $self.style.[<$kind _background>].ansi_bg()
            },
            paste! {
                $self.style.[<$kind _font>].ansi()
            },
            $expr
        )
    }};
}

#[macro_export]
macro_rules! execute_response {
    ($self:expr, $res:expr) => {
        for response in $res.execute() {
            match response {
                EventResponse::UpdateElementById(id, elem) => {
                    for old in &mut $self.children {
                        if old.get_id() == id {
                            *old = elem.clone();
                        }
                    }
                }
                EventResponse::UpdateSelf(elem) => {
                    if let Some(child) = $self.get_child() {
                        *child = elem;
                    }
                }
                _ => {
                    return response;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! run_handler {
    ($self:ident.$handler:ident ($renderer:expr, $event:expr)) => {
        let mut o = $self.$handler.0.lock().unwrap();
        o($self, $renderer, $event);
    };
}
