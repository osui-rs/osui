#[macro_export]
macro_rules! event {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name;

        impl $crate::extensions::Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    ($name:ident {$($inner:tt)*}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($inner)*
        }

        impl $crate::extensions::Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    ($name:ident ($($inner:tt)*)) => {
        #[derive(Debug, Clone)]
        pub struct $name ($($inner)*);

        impl $crate::extensions::Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
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

        impl $crate::widget::Component for $name {
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

        impl $crate::widget::Component for $name {
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

        impl $crate::widget::Component for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

/// Macro to create an event handler closure that calls a method on `self`
///
/// This macro automates the pattern of capturing `self` as a raw pointer,
/// then passing an event to a method on `self` inside an unsafe closure.
///
/// # Parameters
/// - `$self_ty`: The self type.
/// - `$self`: The instance variable (usually `self`) to call the method on.
/// - `$events`: The event source which has an `.on` method to register the handler.
/// - `$method`: The method name on `self` to call when an event occurs.
///
/// # Usage
/// ```rust
/// event_handler!(self, events, on_keypress);
/// ```
///
/// This expands roughly to:
/// ```rust
/// let self_ref = self as *mut Self;
/// events.on(move |event| unsafe { (*self_ref).on_keypress(event) });
/// ```
///
/// # Safety
/// This macro uses `unsafe` code because it dereferences a raw pointer inside the closure.
/// Ensure that the `self` reference lives at least as long as the closure to avoid undefined behavior.
///
/// # Why use raw pointers here?
/// Often, event handlers require `'static` closures, but `self` is a stack reference.
/// Capturing `self` directly is not possible, so this workaround uses a raw pointer.
#[macro_export]
macro_rules! event_handler {
    ($self_ty:ty, $self:ident, $events:ident, $method:ident) => {{
        let self_ref = $self as *mut $self_ty;
        $events.on(move |es, e| unsafe { (*self_ref).$method(es, e) });
    }};
}

#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {{
        let mut r = Rsx(Vec::new());

        $crate::rsx_inner! { r, $($inner)* };

        r
    }};
}

#[macro_export]
macro_rules! rsx_inner {
    ($r:expr, $(%$dep:ident)* $s:literal $($rest:tt)*) => {
        $r.0.push((Box::new({ $(let $dep = $dep.clone();)* move || Box::new(format!($s)) }), vec![$(Arc::new($dep.clone()),)*]));
        $crate::rsx_inner!{ $r, $($rest)* };
    };
    ($r:expr,) => {};
}
