#[macro_export]
macro_rules! parse_rsx_param {
    ($elem:expr, for ($($for:tt)*) $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            for $($for)* {
                children.push($code)
            }
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

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

    ($elem:expr, $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push($code);
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $elem_path:path { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push(osui::rsx_elem!($elem_path { $($inner)* }));
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

#[macro_export]
macro_rules! rsx_elem {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem as $crate::Element
    }};
}

/// Makes a div and puts elements into it
///
/// # Example
/// ```
/// rsx! {
///     button { class: "btn", "Click me!" }
/// }
/// ```
///
/// # Returns
/// A `osui::Element` - Which is a `Box<dyn osui::ElementWidget>`
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {
        $crate::rsx_elem!{ $crate::ui::div { $($inner)* } }
    };

}
