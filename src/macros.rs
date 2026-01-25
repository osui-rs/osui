#[macro_export]
macro_rules! rsx {
    () => {
        $crate::frontend::Rsx::new()
    };

    ($($rsx:tt)+) => {{
        let mut r = $crate::frontend::Rsx::new();
        $crate::rsx_scope!(r, $($rsx)+);
        r
    }};
}

#[macro_export]
macro_rules! rsx_scope {
    ($rsx:expr, $text:literal $($rest:tt)*) => {
        let scope = $crate::component::Scope::new();

        scope.view(Arc::new(move |ctx| ctx.draw_text(Point { x: 0, y: 0 }, &format!($text))));

        $rsx.static_scope(scope);
        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, $component:ident ($ctx:ident, $view:ident) $body:block $($rest:tt)*) => {
        let scope = $crate::component::Scope::new();

        scope.child($component, Some(Arc::new(|$ctx, $view| $body)));

        $rsx.static_scope(scope);
        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, $component:ident $($rest:tt)*) => {
        let scope = $crate::component::Scope::new();

        scope.child($component, None);

        $rsx.static_scope(scope);
        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr,) => {};
}
