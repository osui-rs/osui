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

        ($elem:expr, {$inner_elem:expr} $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push({$inner_elem});
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

#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {{
        let mut elem = $crate::ui::div();
        $crate::parse_rsx_param!(elem, $($inner)* );
        elem
    }};

}

#[macro_export]
macro_rules! val {
    ($a:expr) => {
        $crate::Value::new($a)
    };
}

#[macro_export]
macro_rules! run_handler {
    ($self:ident.$handler:ident ($($inner:tt)*)) => {{
        let a = std::sync::Arc::clone(&$self.$handler.0);
        let mut o = a.lock().unwrap();
        o($self, $($inner)*);
    }};
    ($self:ident.$handler:ident) => {{
        &$self.$handler;
    }};
}

#[macro_export]
macro_rules! __css {
    ($style:expr,)=>{};
    (
        $style:expr, ($name:ident): $value:expr
    ) => {{
        $style.other.insert(stringify!($name).to_string(), Box::new($value));
    }};
    (
        $style:expr, ($name:ident): $value:expr, $($other:tt)*
    ) => {{
        $style.other.insert(stringify!($name).to_string(), Box::new($value));
        $crate::__css!($($other)*);
    }};
    (
        $style:expr, $name:ident: $value:expr
    ) => {{
        $style.$name = $value;
    }};
}

#[macro_export]
macro_rules! _css {
    ($style:expr,) => {};
    (
        $hm:expr, .$name:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Class(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            $crate::__css!(style.0, $($inner)*);
        } else {
            let mut style = $crate::ui::Style::default();

            $crate::__css!(style.0, $($inner)*);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
    (
        $hm:expr, .$name:ident:$kind:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Class(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);
        } else {
            let mut style = $crate::ui::Style::default();

            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};


    (
        $hm:expr, #$name:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Id(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            $crate::__css!(style.0, $($inner)*);
        } else {
            let mut style = $crate::ui::Style::default();

            $crate::__css!(style.0, $($inner)*);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
    (
        $hm:expr, #$name:ident:$kind:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Id(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);
        } else {
            let mut style = $crate::ui::Style::default();

            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};


    (
        $hm:expr, $name:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Component(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            $crate::__css!(style.0, $($inner)*);
        } else {
            let mut style = $crate::ui::Style::default();

            $crate::__css!(style.0, $($inner)*);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
    (
        $hm:expr, $name:ident:$kind:ident { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Component(stringify!($name).to_string());
        if let Some(style) = $hm.get_mut(&name) {
            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);
        } else {
            let mut style = $crate::ui::Style::default();

            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            style.1.insert(stringify!($kind).to_string(), style_elem);

            $hm.insert(
                name,
                style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
}

#[macro_export]
macro_rules! css {
    (
        $($inner:tt)*
    ) => {{
        let mut hm: $crate::ui::Css = std::collections::HashMap::new();
        $crate::_css!(hm, $($inner)*);
        hm
    }};
}
