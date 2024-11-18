/// **parse_rsx_param!** - A macro for parsing and setting parameters on elements, allowing for dynamic handling of element properties and children.
/// 
/// This macro enables flexible configuration of UI elements. It handles setting fields, adding children, invoking paths, and processing complex structures.
///
/// ## Example
/// ```rust
/// parse_rsx_param!(elem, title: "Hello", for (i in 0..5) { <child> }, text: "World");
/// ```
/// 
/// ## Syntax
/// 
/// **Set property or field of an element:**
/// ```rust
/// parse_rsx_param!($elem, $($k:ident).+: $v:expr);
/// ```
///
/// **Process multiple parameters:**
/// ```rust
/// parse_rsx_param!($elem, $($k:ident).+: $v:expr, $($rest:tt)*);
/// ```
///
/// **Handle path expression:**
/// ```rust
/// parse_rsx_param!($elem, $p:path);
/// ```
///
/// **Add nested children:**
/// ```rust
/// parse_rsx_param!($elem, for ($($for:tt)*) { $($inner:tt)* } $($rest:tt)*);
/// ```
///
/// **Add child element directly:**
/// ```rust
/// parse_rsx_param!($elem, {$ielem:expr} $($rest:tt)*);
/// ```
/// 
/// **Set element text:**
/// ```rust
/// parse_rsx_param!($elem, $text:expr);
/// ```
///
/// **Handle empty case:**
/// ```rust
/// parse_rsx_param!($elem, );
/// ```
#[macro_export]
macro_rules! parse_rsx_param {
        ($elem:expr, $($k:ident).+: $v:expr) => {
        $elem.$($k).+ = $v;
    };

    ($elem:expr, $($k:ident).+: $v:expr, $($rest:tt)*) => {
        $elem.$($k).+ = $v;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

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

        ($elem:expr, for ($($for:tt)*) { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            for $($for)* {
                children.push({$($inner)*})
            }
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

        ($elem:expr, {$ielem:expr} $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push({$ielem});
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

        ($elem:expr, $pelem:path { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push(osui::rsx_elem!($pelem { $($inner)* }));
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

        ($elem:expr, $text:expr) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Text(format!($text))}
    };
    ($elem:expr, $text:expr, $($inner:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Text(format!($text, $($inner)*))}
    };

        ($elem:expr, ) => {};
}

/// **rsx_elem!** - A macro to instantiate a UI element and apply parameters to it.
/// 
/// This macro simplifies creating an element and applying a set of properties or children to it, enabling a declarative style of UI building.
///
/// ## Example
/// ```rust
/// let element = rsx_elem!(ui::Button { text: "Click me" });
/// ```
/// 
/// ## Syntax
/// ```rust
/// rsx_elem!($elem:path { $($inner:tt)* })
/// ```
#[macro_export]
macro_rules! rsx_elem {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem as $crate::Element
    }};
}

/// **rsx!** - A shorthand macro to create a `div` element and apply nested content to it.
///
/// This macro is a simple wrapper for `rsx_elem!`, making it easy to create a `div` element and add children to it.
///
/// ## Example
/// ```rust
/// rsx! {
///     text { "Hello, World" }
///     div { button {"Click me!"} }
/// } 
/// ```
/// 
/// ## Syntax
/// ```rust
/// rsx!($($inner:tt)*)
/// ```
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {{
        $crate::rsx_elem!( $crate::ui::div { $($inner)* } )
    }};
}

#[macro_export]
macro_rules! val {
    ($a:expr) => {
        $crate::Value::new($a)
    };
}

#[macro_export]
macro_rules! write {
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
macro_rules! run_handler {
    ($self:ident.$handler:ident ($renderer:expr, $event:expr)) => {
        let mut o = $self.$handler.0.lock().unwrap();
        o($self, $renderer, $event);
    };
}
