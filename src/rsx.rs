#[macro_export]
macro_rules! check_expr {
    (|| $code:block) => {
        $crate::Handler::new(move |_, _, _| $code)
    };
    (|$($inner:tt)*| $code:block) => {
        $crate::Handler::new(move |$($inner)*| $code)
    };
    ($expr:expr) => {
        $expr
    };
}
#[macro_export]
macro_rules! parse_rsx_param {
    ($elem:expr, async $code:block $($rest:tt)*) => {
        let elem_len = $elem.children.len();
        $elem.instructions.push($crate::Instruction::Load(Handler::new(
            move |d: &mut Div, _, _| {
                d.children.insert(elem_len, $code);
            }
        )));
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, %$elem_path:path { $($inner:tt)* } $($rest:tt)*) => {
        $elem.ghosts.push($elem.ghosts.len());
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push($crate::ersx!($elem_path { $($inner)* }));
        } else {
            $elem.children = $crate::Children::Children(vec![$crate::ersx!($elem_path { $($inner)* })], 0)
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, @ $($i:ident)::+($($inner:tt)*) $($rest:tt)*) => {
        $elem.instructions.push($($i)::+($($inner)*));
        osui::parse_rsx_param!($elem, $($rest)*);
    };
    ($elem:expr, @ $($i:ident)::+ $($rest:tt)*) => {
        $elem.instructions.push($($i)::+);
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, static $text:literal $($other:tt)*) => {
        $elem.children = $crate::Children::StaticText(format!($text $($other)*));
    };

    ($elem:expr, for ($($for:tt)*) $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            for $($for)* {
                children.push($code)
            }
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    //// FUNCTIONS

    ($elem:expr, $($k:ident).+: fn($($params:tt)*) $(@$($v:ident),+)? $code:block $(, $($rest:tt)*)?) => {
        $elem.$($k).+ = $crate::Handler::new({
            $($(
                #[allow(unused_mut)]
                let mut $v = $v.clone();
            )+)?
            move |$($params)*| {
                $code
            }
        });
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };

    //// FUNCTIONS END

    ($elem:expr, $($k:ident).+: $v:expr $(, $($rest:tt)*)?) => {
        $elem.$($k).+ = $crate::check_expr!($v);
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };
    ($elem:expr, $p:path) => {
        $p;
    };
    ($elem:expr, $($k:ident).+, $($rest:tt)*) => {
        $elem.$($k).+ = true;
        osui::parse_rsx_param!($elem, $($rest)*);
    };
    ($elem:expr, $($k:ident).+., $($rest:tt)*) => {
        $elem.$($k).+.;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $($k:ident).+.: $v:expr) => {
        $elem.$($k).+. = $crate::check_expr!($v);
    };
    ($elem:expr, $($k:ident).+.: $v:expr, $(, $($rest:tt)*)?) => {
        $elem.$($k).+. = $crate::check_expr!($v);
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };

    ($elem:expr, $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push($code);
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $elem_path:path { $($inner:tt)* } $($rest:tt)*) => {
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push($crate::ersx!($elem_path { $($inner)* }));
        } else {
            $elem.children = $crate::Children::Children(vec![$crate::ersx!($elem_path { $($inner)* })], 0)
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, $text:literal $($other:tt)*) => {
        $elem.children = $crate::Children::Text(
            std::sync::Arc::new(move || format!($text $($other)*))
        );
    };

    ($elem:expr,) => {};
}

#[macro_export]
macro_rules! ersx {
    ($elem:path { $($inner:tt)* }) => {{
        let mut elem = <$elem>::default();
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem.create_element()
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
        $crate::ersx!{ $crate::ui::div { $($inner)* } }
    };

}
