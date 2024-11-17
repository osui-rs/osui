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
    ($elem:expr, $($k:ident).+, $($rest:tt)*) => {
        $elem.$($k).+;
        osui::parse_rsx_param!($elem, $($rest)*);
    };
    ($elem:expr, $($k:ident).+., $($rest:tt)*) => {
        $elem.$($k).
        +.;
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
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            for $($for)* {
                children.push({$($inner)*})
            }
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // Expression
    ($elem:expr, {$ielem:expr} $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push({$ielem});
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    // Element
    ($elem:expr, $pelem:path { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push(osui::rsx_elem!($pelem { $($inner)* }));
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    // Text
    ($elem:expr, $text:expr) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Text(format!($text))}
    };
    ($elem:expr, $text:expr, $($inner:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Text(format!($text, $($inner)*))}
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
        elem as Box<dyn $crate::Element>
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
        $crate::rsx_elem!( $crate::ui::div { $($inner)* } )
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
