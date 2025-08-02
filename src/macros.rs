/// Declares a struct that implements the `Event` trait.
///
/// This macro simplifies the creation of event types used within OSUI's reactive system.
///
/// # Variants
///
/// - `event!(Name)`
///   Defines a unit struct named `Name`.
///
/// - `event!(Name { ... })`
///   Defines a named struct with fields.
///
/// - `event!(Name (...))`
///   Defines a tuple struct.
///
/// # Examples
/// ```rust
/// event!(Clicked);
/// event!(Resized { width: u32, height: u32 });
/// event!(Moved(u32, u32));
/// ```
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

/// Declares a struct that implements the `Component` trait.
///
/// Components allow widgets to extend their behavior or contain additional data.
/// This macro helps avoid boilerplate when defining new components.
///
/// # Variants
///
/// - `component!(Name)`
///   Defines a unit struct.
///
/// - `component!(Name { ... })`
///   Defines a named struct with fields.
///
/// - `component!(Name (...))`
///   Defines a tuple struct.
///
/// # Examples
/// ```rust
/// component!(Focusable);
/// component!(Tooltip { text: String });
/// component!(Size(u32, u32));
/// ```
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

/// Creates an event handler closure that calls a method on `self`.
///
/// Useful when you need to register `'static` event handlers that interact with the current instance.
///
/// # Arguments
/// - `$self_ty`: The type of `self`.
/// - `$self`: The instance variable (usually `self`) being used.
/// - `$events`: The event source object with an `.on` method.
/// - `$method`: The method to call when an event is received.
///
/// # Example
/// ```rust
/// event_handler!(Self, self, events, on_event);
/// ```
/// Expands roughly to:
/// ```rust
/// let self_ref = self as *mut Self;
/// events.on(move |es, e| unsafe { (*self_ref).on_event(es, e) });
/// ```
///
/// # Safety
/// This macro uses `unsafe` code to cast `self` to a raw pointer and dereference it.
/// Make sure the reference is valid for the closure’s lifetime.
#[macro_export]
macro_rules! event_handler {
    ($self_ty:ty, $self:ident, $events:ident, $method:ident) => {{
        let self_ref = $self as *mut $self_ty;
        $events.on(move |es, e| unsafe { (*self_ref).$method(es, e) });
    }};
}

/// Creates a `Transform` with property overrides.
///
/// Each key-value pair sets a field on a `Transform` struct.
///
/// # Example
/// ```rust
/// let t = transform!(
///     x: 10,
///     y: 20,
///     scale: 1.5,
/// );
/// ```
///
/// This expands to:
/// ```rust
/// let mut t = Transform::new();
/// t.x = 10.into();
/// t.y = 20.into();
/// t.scale = 1.5.into();
/// ```
#[macro_export]
macro_rules! transform {
    ($($f:ident: $v:expr),* $(,)?) => {{
        let mut t = Transform::new();
        $(t.$f = $v.into();)*
        t
    }};
}

/// Constructs an `Rsx` tree using declarative syntax.
///
/// This macro is similar to JSX in UI frameworks, allowing you to nest widgets and assign components or dependencies.
/// It expands into a tree of `RsxElement` objects.
///
/// Internally calls the recursive `rsx_inner!` macro.
///
/// # Example
/// ```rust
/// rsx! {
///     "Hello"
///     static Label { } ("Text")
///     %state Label { }
///     @Velocity(20, 0); Transform::new(); "Moving Text"
/// }
/// ```
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {{
        let mut r = $crate::frontend::Rsx(Vec::new());

        $crate::rsx_inner! { r, $($inner)* };

        r
    }};
}

/// Internal macro used by `rsx!` to recursively build `Rsx` trees.
///
/// This macro handles various syntactic forms used in the declarative layout system:
///
/// - Static text or components
/// - Dynamic widgets with dependencies (`%dep`)
/// - Component annotations (`@comp`)
/// - Argument passing to constructors
/// - Nesting and expansion from other `Rsx` blocks
///
/// **This macro is not intended for direct use**; use `rsx!` instead.
///
/// # Example expansion
/// ```rust
/// rsx! {
///     static Label { } ("Title")
///     %state Label { inner }
///     "Text"
/// }
/// ```
///
/// Would produce a nested `Rsx` tree of static and dynamic widgets.
#[macro_export]
macro_rules! rsx_inner {
    // static
    ($r:expr, $(@$comp:expr;)* static $s:literal $($rest:tt)*) => {
        let w = $crate::widget::StaticWidget::new(Box::new(format!($s)));
        $(w.component($comp);)*
        $r.create_element_static(w, $crate::frontend::Rsx(Vec::new()));
        $crate::rsx_inner! { $r, $($rest)* };
    };

    ($r:expr, $(@$comp:expr;)* $(%$dep:ident)* $s:literal $($rest:tt)*) => {
        $r.create_element({ $(let $dep = $dep.clone();)* move || {
            $crate::widget::WidgetLoad::new(format!($s)) $(.component($comp))*
        } }, vec![$(Box::new($dep.clone())),*], $crate::frontend::Rsx(Vec::new()));
        $crate::rsx_inner! { $r, $($rest)* };
    };

    // static
    ($r:expr, $(@$comp:expr;)* static $name:path { $($inner:tt)* } ($($e:expr),*) $($rest:tt)*) => {
        let w = $crate::widget::StaticWidget::new(Box::new(<$name>::new($($e),*)));
        $(w.component($comp);)*
        $r.create_element_static(w, $crate::frontend::Rsx(Vec::new()));
        $crate::rsx_inner! { $r, $($rest)* };
    };

    // static
    ($r:expr, $(@$comp:expr;)* static $name:path { $($inner:tt)* } $($rest:tt)*) => {
        let w = $crate::widget::StaticWidget::new(Box::new(<$name>::new()));
        $(w.component($comp);)*
        $r.create_element_static(w, $crate::frontend::Rsx(Vec::new()));
        $crate::rsx_inner! { $r, $($rest)* };
    };

    ($r:expr, $(@$comp:expr;)* $(%$dep:ident)* $name:path { $($inner:tt)* } ($($e:expr),*) $($rest:tt)*) => {
        $r.create_element({ $(let $dep = $dep.clone();)* move || {
            $crate::widget::WidgetLoad::new(<$name>::new($($e),*)) $(.component($comp))*
        } }, vec![$(Box::new($dep.clone())),*], $crate::rsx!{ $($inner)* });
        $crate::rsx_inner! { $r, $($rest)* };
    };

    ($r:expr, $(@$comp:expr;)* $(%$dep:ident)* $name:path { $($inner:tt)* } $($rest:tt)*) => {
        $r.create_element({ $(let $dep = $dep.clone();)* move || {
            $crate::widget::WidgetLoad::new(<$name>::new()) $(.component($comp))*
        } }, vec![$(Box::new($dep.clone())),*], $crate::rsx!{ $($inner)* });
        $crate::rsx_inner! { $r, $($rest)* };
    };

    ($r:expr, $expand:ident => ($($inner:tt)*) $($rest:tt)*) => {
        $r.expand(&mut $expand($($inner)*));
        $crate::rsx_inner! { $r, $($rest)* };
    };

    ($r:expr,) => {};
}
