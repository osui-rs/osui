#[macro_export]
macro_rules! event {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name;

        impl $crate::events::Event for $name {
            fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
        }
    };

    ($name:ident {$($inner:tt)*}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($inner)*
        }

        impl $crate::events::Event for $name {
            fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
        }
    };

    ($name:ident ($($inner:tt)*)) => {
        #[derive(Debug, Clone)]
        pub struct $name ($($inner)*);

        impl $crate::events::Event for $name {
            fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
        }
    };
}
