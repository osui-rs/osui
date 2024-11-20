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

/// Creates a styling map `Css`
///
/// # Examples
/// ```
/// .btn: clicked {
///     color: Blue
/// }
/// ```
/// 
/// # Returns
/// `Css`
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
