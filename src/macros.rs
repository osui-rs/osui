#[macro_export]
macro_rules! __oml {
    () => { Vec::new() };

    // Props, With components (PC)
    ($tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*}) => {{
        vec![oml!($tag ($($k = $v),*) {$($inner)*})]
    }};

    ($tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($($k = $v),*) {$($inner)*}));
        comps
    }};

    // Components (C)
    ($tag:ident {$($inner:tt)*}) => {{
        vec![oml!($tag {$($inner)*})]
    }};

    ($tag:ident {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag {$($inner)*}));
        comps
    }};

    // Expression, With components (EC)
    ($tag:ident ($expr:expr; $($k:ident = $v:expr),*) {$($inner:tt)*}) => {{
        vec![oml!($tag ($expr; $($k = $v),*) {$($inner)*})]
    }};

    ($tag:ident ($expr:expr; $($k:ident = $v:expr),*) {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($expr; $($k = $v),*) {$($inner)*}));
        comps
    }};

    // Expression (E)
    ($tag:ident ($expr:expr; $($k:ident = $v:expr),*)) => {{
        vec![oml!($tag ($expr; $($k = $v),*))]
    }};

    ($tag:ident ($expr:expr; $($k:ident = $v:expr),*) $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($expr; $($k = $v),*)));
        comps
    }};

    // Props (P)
    ($tag:ident ($($k:ident = $v:expr),*)) => {{
        vec![oml!($tag ($($k = $v),*))]
    }};

    ($tag:ident ($($k:ident = $v:expr),*) $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($($k = $v),*)));
        comps
    }};

    // Pre-Defined (D)
    ($expr:expr;) => {{
        vec![oml!($expr;)]
    }};

    ($expr:expr; $($rest:tt)+) => {{
        let mut comps: Vec<osui::Component> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($expr));
        comps
    }};
}

#[macro_export]
/// Write OSUI Markup Language directly with rust.
macro_rules! oml {
    // Props (P)
    ($tag:ident ($($k:ident = $v:expr),*)) => {{
        let mut c = $tag(osui::Params { children: Vec::new(), expr: String::new() });
        $(
            c.$k = $v;
        )*
        c
    }};

    // Expression, With components (EC)
    ( $tag:ident ($expr:expr; $($k:ident = $v:expr),* ) {$($inner:tt)*} ) => {{
        let mut c = $tag(osui::Params { children: osui::__oml!($($inner)*), expr: String::from($expr) });
        $(
            c.$k = $v;
        )*
        c
    }};

    // Expression (E)
    ( $tag:ident ($expr:expr; $($k:ident = $v:expr),*) ) => {{
        let mut c = $tag(osui::Params { children: Vec::new(), expr: String::from($expr) });
        $(
            c.$k = $v;
        )*
        c
    }};

    // Components (C)
    ( $tag:ident {$($inner:tt)*} ) => {
        $tag(osui::Params { children: osui::__oml!($($inner)*), expr: String::new() })
    };

    // Props, With components (PC)
    ( $tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*} ) => {{
        let mut c = $tag(osui::Params { children: osui::__oml!($($inner)*), expr: String::new() });
        $(
            c.$k = $v;
        )*
        c
    }};

    // Pre-Defined (D)
    ($expr:expr;) => {{
        $expr
    }};
}

#[macro_export]
///```create_frame!(width, height)```
///
/// Create a frame for rendering multiple components
macro_rules! create_frame {
    ($width:expr, $height:expr) => {
        vec![" ".repeat($width); $height]
    };
}
