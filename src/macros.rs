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
        while $crate::run(&mut $expr) {}
    };
}
