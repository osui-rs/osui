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

#[macro_export]
macro_rules! component {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name;

        impl $crate::Component for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };

    ($name:ident {$($inner:tt)*}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($inner)*
        }

        impl $crate::Component for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };

    ($name:ident ($($inner:tt)*)) => {
        #[derive(Debug, Clone)]
        pub struct $name ($($inner)*);

        impl $crate::Component for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}
