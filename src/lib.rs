#[macro_export]
macro_rules! __xml {
    ($(<$tag:ident> {$($inner:tt)*})+) => {{
        let mut comps = String::new();
        $(
            let inner = osui::__xml!($($inner)*);
            comps.push_str(&format!("{} {{{}}}", stringify!($tag), inner));
        )*
        comps
    }};
    () => { String::new() };
}

#[macro_export]
macro_rules! xml {
    (<$tag:ident> {$($inner:tt)*}) => {
        osui::__xml!(<$tag> {$($inner)*})
    };
}
