#[macro_export]
macro_rules! __css {
    ($_style:expr,)=>{};
    // percentage (n%)
    (
        $_style:expr, $name:ident: $value:literal% $(, $($rest:tt)*)?
    ) => {{
        $_style.$name = Number::Pe($value);
        $crate::__css!($_style, $($($rest)*)?);
    }};

    // Number
    (
        $_style:expr, $name:ident: $value:literal px $(, $($rest:tt)*)?
    ) => {{
        $_style.$name = Number::Px($value);
        $crate::__css!($_style, $($($rest)*)?);
    }};

    // Vector
    (
        $_style:expr, $name:ident: [$($inner:tt),*] $(, $($rest:tt)*)?
    ) => {{
        $_style.$name = vec![$($inner),*];
        $crate::__css!($_style, $($($rest)*)?);
    }};

    // Normal
    (
        $_style:expr, $name:ident: $value:expr $(, $($rest:tt)*)?
    ) => {{
        $_style.$name = $value;
        $crate::__css!($_style, $($($rest)*)?);
    }};
}

#[macro_export]
macro_rules! _css {
    ($_style:expr,) => {};
    (
        $hm:expr, $name:literal { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Class(String::from($name));
        if let Some(_style) = $hm.get_mut(&name) {
            $crate::__css!(_style.0, $($inner)*);
        } else {
            let mut _style = $crate::ui::Style::default();

            $crate::__css!(_style.0, $($inner)*);

            $hm.insert(
                name,
                _style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
    (
        $hm:expr, $name:literal: $kind:literal { $($inner:tt)* } $($rest:tt)*
    ) => {{
        let name = $crate::ui::StyleName::Class(String::from($name));
        if let Some(_style) = $hm.get_mut(&name) {
            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            _style.1.insert($kind.to_string(), style_elem);
        } else {
            let mut _style = $crate::ui::Style::default();

            let mut style_elem = $crate::ui::StyleElement::default();
            $crate::__css!(style_elem, $($inner)*);
            _style.1.insert($kind.to_string(), style_elem);

            $hm.insert(
                name,
                _style,
            );
        }
        $crate::_css!($hm, $($rest)*);
    }};
}

/// Creates a styling map `Css`
///
/// # Examples
/// ```
/// "btn" {
///     color: Red,
/// }
/// "btn": "clicked" {
///     color: Blue,
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
        #[allow(unused_mut)]
        let mut hm: $crate::ui::Css = std::collections::HashMap::new();
        $crate::_css!(hm, $($inner)*);
        hm
    }};
}

#[macro_export]
macro_rules! style {
    (
        $key:ident: $value:literal% $(, $($rest:tt)*)?
    ) => {{
        let mut _style = $crate::ui::Style::default();
        $crate::__css!(_style.0, $key: $crate::ui::Number::Pe($value));
        $crate::style!{ _style; $($($rest)*)? }
    }};
    (
        $key:ident: $value:literal px $(, $($rest:tt)*)?
    ) => {{
        let mut _style = $crate::ui::Style::default();
        $crate::__css!(_style.0, $key: $crate::ui::Number::Px($value));
        $crate::style!{ _style; $($($rest)*)? }
    }};
    (
        $style:expr; $key:ident: $value:literal% $(, $($rest:tt)*)?
    ) => {{
        $crate::__css!($style.0, $key: $crate::ui::Number::Pe($value));
        $crate::style!{ $style; $($($rest)*)? }
    }};
    (
        $style:expr; $key:ident: $value:literal px $(, $($rest:tt)*)?
    ) => {{
        $crate::__css!($style.0, $key: $crate::ui::Number::Px($value));
        $crate::style!{ $style; $($($rest)*)? }
    }};
    (
        $key:ident: $value:expr $(, $($rest:tt)*)?
    ) => {{
        let mut _style = $crate::ui::Style::default();
        $crate::__css!(_style.0, $key: $value);
        $crate::style!{ _style; $($($rest)*)? }
    }};
    (
        $style:expr; $key:ident: $value:expr $(, $($rest:tt)*)?
    ) => {{
        let mut style_elem = $crate::ui::StyleElement::default();
        $crate::__css!($style.0, $key: $value);
        $crate::style!{ $style; $($($rest)*)? }
    }};
    (
        ($key:ident): $value:expr $(, $($rest:tt)*)?
    ) => {{
        let mut _style = $crate::ui::Style::default();
        $crate::__css!(_style.0, ($key): $value);
        $crate::style!{ _style; $($($rest)*)? }
    }};
    (
        $style:expr; $key:literal: $value:expr $(, $($rest:tt)*)?
    ) => {{
        let mut style_elem = $crate::ui::StyleElement::default();
        $crate::__css!($style.0, $key: $value);
        $crate::style!{ $style; $($($rest)*)? }
    }};

    (
        $kind:ident {$($inner:tt)*} $(, $($rest:tt)*)?
    ) => {{
        let mut _style = $crate::ui::Style::default();
        let mut style_elem = $crate::ui::StyleElement::default();
        $crate::__css!(style_elem, $($inner)*);
        _style.1.insert(stringify!($kind).to_string(), style_elem);
        $crate::style!{ _style; $($($rest)*)? }
    }};
    (
        $style:expr; $kind:ident {$($inner:tt)*} $(, $($rest:tt)*)?
    ) => {{
        let mut style_elem = $crate::ui::StyleElement::default();
        $crate::__css!(style_elem, $($inner)*);
        $style.1.insert(stringify!($kind).to_string(), style_elem);
        $crate::style!{ $style; $($($rest)*)? }
    }};

    (
        $style:expr;
    ) => {{
        $style
    }};

    () => {{
        $crate::ui::Style::default()
    }};
}
