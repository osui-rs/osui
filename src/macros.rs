#[macro_export]
macro_rules! __oml {
    ($( ($tag:ident $($k:ident = $v:expr)*) {$($inner:tt)*} )+) => {{
        let mut comps: Vec<Box<dyn Component>> = Vec::new();
        $(
            // comps.push(Box::new($tag(ComponentParams { children: osui::__oml!($($inner)*) })));
            comps.push(oml!(($tag $($k = $v)*) {$($inner)*}));
        )*
        comps
    }};
    () => { Vec::new() };
}

#[macro_export]
macro_rules! oml {
    ( ($tag:ident) {} ) => {
        Box::new($tag(ComponentParams { children: Vec::new() }))
    };

    ( ($tag:ident) {$($inner:tt)*} ) => {
        Box::new($tag(ComponentParams { children: osui::__oml!($($inner)*) }))
    };

    ( ($tag:ident $($k:ident = $v:expr)+ ) {$($inner:tt)*} ) => {{
        let mut c = $tag(ComponentParams { children: osui::__oml!($($inner)*) });
        $(
            c.$k = $v;
        )+
        Box::new(c)
    }};
}
