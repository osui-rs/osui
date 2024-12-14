#[macro_export]
macro_rules! call {
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
macro_rules! launch {
    ($expr:expr) => {
        let mut element = $expr;
        let mut document = $crate::Document::with_elem(&mut element);
        while document.run() {
            element = $expr;
        }
    };
    ($expr:expr, $css:expr) => {
        let mut element = $expr;
        let mut document = $crate::Document::with_elem(&mut element);
        document.set_css($css);
        while document.run() {
            element = $expr;
        }
    };
}
