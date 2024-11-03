#[macro_export]
macro_rules! __rsx {
    () => {
        (String::new(), Vec::new())
    };

    ($p:expr;) => {{
        (String::new(), vec![$p as Box<dyn osui::Element>])
    }};
    ($p:expr; $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__rsx!($($rest)+).1;
        comps.insert(0, $p as Box<dyn osui::Element>);
        (String::new(), comps)
    }};
    // Props, With components (PC)
    ($tag:path { $($inner:tt)* }) => {{
        (String::new(), vec![rsx!($tag {$($inner)*})])
    }};

    ($tag:path { $($inner:tt)* } $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__rsx!($($rest)+).1;
        comps.insert(0, rsx!($tag {$($inner)*}) );
        (String::new(), comps)
    }};

    ($tag:path { $($inner:tt)* }) => {{
        (String::new(), vec![rsx!($tag {$($inner)*})])
    }};

    ($tag:path { $($inner:tt)* } $($rest:tt)+) => {{
        let mut comps: Vec<Box<dyn osui::Element>> = osui::__rsx!($($rest)+).1;
        comps.insert(0, rsx!($tag {$($inner)*}) );
        (String::new(), comps)
    }};

    ($text:expr) => {{
        (format!($text), Vec::new())
    }};
}

#[macro_export]
/// Write OSUI Markup Language directly with rust.
macro_rules! rsx {
    ( $tag:path { $($k:ident: $v:expr),*; $($inner:tt)* } ) => {{
        let mut c = $tag();
        let a = osui::__rsx!($($inner)*);
        c.text = a.0;
        c.children = a.1;
        $(
            c.$k = $v;
        )*
        c as Box<dyn osui::Element>
    }};

    ( $tag:path { $($k:ident: $v:expr),* } ) => {{
        let mut c = $tag();
        $(
            c.$k = $v;
        )*
        c as Box<dyn osui::Element>
    }};

    ( $tag:path { $($inner:tt)* } ) => {{
        let mut c = $tag();
        let a = osui::__rsx!($($inner)*);
        c.text = a.0;
        c.children = a.1;
        c as Box<dyn osui::Element>
    }};

    ( $p:expr; ) => {{
        $p as Box<dyn osui::Element>
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
            pub tick_line: std::collections::HashMap<usize, String>,
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

            fn clear_ticks(&mut self) {
                self.tick_line.clear();
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
                    tick_line: std::collections::HashMap::new(),
                    $( $defaults )*
                }
            }

            pub fn get_action(&self, tick: usize) -> String {
                match self.tick_line.get(&tick) {
                    Some(action) => action.clone(),
                    None => String::new(),
                }
            }

            pub fn add_action(&mut self, tick: usize, action: &str) {
                if tick > 99 {
                    self.tick_line.insert(tick-100, action.to_string());
                } else {
                    self.tick_line.insert(tick, action.to_string());
                }
            }
        }
    };
}
