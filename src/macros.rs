#[macro_export]
macro_rules! __oml {
    () => { Vec::new() };

    // Props, With components (PC)
    ($tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*}) => {{
        vec![oml!($tag ($($k = $v),*) {$($inner)*})]
    }};

    ($tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($($k = $v),*) {$($inner)*}));
        comps
    }};

    // Components (C)
    ($tag:ident {$($inner:tt)*}) => {{
        vec![oml!($tag {$($inner)*})]
    }};

    ($tag:ident {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag {$($inner)*}));
        comps
    }};

    // Expression, With components (EC)
    ($tag:ident ($text:expr; $($k:ident = $v:expr),*) {$($inner:tt)*}) => {{
        vec![oml!($tag ($text; $($k = $v),*) {$($inner)*})]
    }};

    ($tag:ident ($text:expr; $($k:ident = $v:expr),*) {$($inner:tt)*} $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($text; $($k = $v),*) {$($inner)*}));
        comps
    }};

    // Expression (E)
    ($tag:ident ($text:expr; $($k:ident = $v:expr),*)) => {{
        vec![oml!($tag ($text; $($k = $v),*))]
    }};

    ($tag:ident ($text:expr; $($k:ident = $v:expr),*) $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($text; $($k = $v),*)));
        comps
    }};

    // Props (P)
    ($tag:ident ($($k:ident = $v:expr),*)) => {{
        vec![oml!($tag ($($k = $v),*))]
    }};

    ($tag:ident ($($k:ident = $v:expr),*) $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($tag ($($k = $v),*)));
        comps
    }};

    // Pre-Defined (D)
    ($text:expr;) => {{
        vec![oml!($text;)]
    }};

    ($text:expr; $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__oml!($($rest)+);
        comps.insert(0, oml!($text));
        comps
    }};
}

#[macro_export]
/// Write OSUI Markup Language directly with rust.
macro_rules! oml {
    // Props (P)
    ($tag:ident ($($k:ident = $v:expr),*)) => {{
        let mut c = $tag();
        $(
            c.$k = $v;
        )*
        c
    }};

    // Expression, With components (EC)
    ( $tag:ident ($text:expr; $($k:ident = $v:expr),* ) {$($inner:tt)*} ) => {{
        let mut c = $tag();
        c.children = osui::__oml!($($inner)*);
        c.text = String::from($text);
        $(
            c.$k = $v;
        )*
        c
    }};

    // Expression (E)
    ( $tag:ident ($text:expr; $($k:ident = $v:expr),*) ) => {{
        let mut c = $tag();
        c.text = String::from($text);
        $(
            c.$k = $v;
        )*
        c
    }};

    // Components (C)
    ( $tag:ident {$($inner:tt)*} ) => {{
        let mut c = $tag();
        c.children = osui::__oml!($($inner)*);
        c
    }};

    // Props, With components (PC)
    ( $tag:ident ($($k:ident = $v:expr),*) {$($inner:tt)*} ) => {{
        let mut c = $tag();
        c.children = osui::__oml!($($inner)*);
        $(
            c.$k = $v;
        )*
        c
    }};

    // Pre-Defined (D)
    ($text:expr;) => {{
        $text
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

#[macro_export]
///```component!{
///     MyElem {}
///     defaults {}
/// }```
///
/// Create a frame for rendering multiple components
macro_rules! element {
    (
        $name:ident {
            $( $inner:tt )*
        }
        defaults {$( $defaults:tt )*}
        $( $functions:tt )*
    ) => {
        #[derive(Debug)]
        pub struct $name {
            pub x: usize,
            pub y: usize,
            pub width: usize,
            pub height: usize,
            pub children: Vec<Box<dyn Element>>,
            pub child: usize,
            pub text: String,
            pub style: Style,
            $( $inner )*
        }

        impl Element for $name {
            fn get_child(&mut self) -> Option<&mut Box<dyn Element>> {
                self.children.get_mut(self.child)
            }
            fn get_data(&self) -> ElementData {
                ElementData {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                    style: self.style.clone(),
                }
            }

            fn set_data(&mut self, data: ElementData) {
                self.x = data.x;
                self.y = data.y;
                self.width = data.width;
                self.height = data.height;
                self.style = data.style;
            }
            $( $functions )*
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                    children: Vec::new(),
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    child: 0,
                    text: String::new(),
                    style: Style::default(),
                    $( $defaults )*
                }
            }
        }
    };
}
