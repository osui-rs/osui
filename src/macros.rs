#[macro_export]
macro_rules! rsx {
    () => {
        $crate::frontend::Rsx::new()
    };

    ($($rsx:tt)+) => {{
        let mut r = $crate::frontend::Rsx::new();
        {
            $crate::rsx_scope!(r, $($rsx)+);
        }
        r
    }};
}

#[macro_export]
macro_rules! rsx_scope {
    ($rsx:expr, for $(%$($dep:ident $(as $dp:pat,)?),+)? ($p:pat in $v:expr) {$($inner:tt)*} $($rest:tt)*) => {
        {
            $rsx.dynamic_scope({
                $($($crate::rsx_dep!($dep $(as $dp)?);)+)?
                move |scope| {
                    for $p in $v {
                        $crate::rsx_child!(scope, $($inner)*);
                    }
                }
            }, vec![$($(Box::new($dep)),+)?]);
        }

        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, if $(%$($dep:ident $(as $dp:pat,)?),+)? ($st:expr) {$($inner:tt)*} $($rest:tt)*) => {
        {
            $rsx.dynamic_scope({
                $($($crate::rsx_dep!($dep $(as $dp)?);)+)?
                move |scope| {
                    if $st {
                        if scope.children.lock().unwrap().len() == 0 {
                            $crate::rsx_child!(scope, $($inner)*);
                        }
                    } else {
                        scope.children.lock().unwrap().clear();
                    }
                }
            }, vec![$($(Box::new($dep)),+)?]);
        }

        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, $(%$($dep:ident $(as $dp:pat,)?),+)? $text:literal $($rest:tt)*) => {
        {
            let scope = $crate::component::Scope::new();

            $($($crate::rsx_dep!($dep $(as $dp)?);)+)?

            $crate::rsx_child!(scope, $text);

            $rsx.static_scope(scope);
        }

        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, $component:ident ($ctx:ident, $view:ident) $body:block $($rest:tt)*) => {
        {
            let scope = $crate::component::Scope::new();

            $crate::rsx_child!(scope, $component ($ctx, $view) $body);

            $rsx.static_scope(scope);
        }

        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr, $component:ident $($rest:tt)*) => {
        {
            let scope = $crate::component::Scope::new();

            $crate::rsx_child!(scope, $component);

            $rsx.static_scope(scope);
        }

        $crate::rsx_scope!($rsx, $($rest)*);
    };

    ($rsx:expr,) => {};
}

#[macro_export]
macro_rules! rsx_child {
    ($scope:expr, $text:literal $($rest:tt)*) => {
        {
            $scope.view(Arc::new(move |ctx| ctx.draw_text(Point { x: 0, y: 0 }, &format!($text))));
        }

        $crate::rsx_child!($scope, $($rest)*);
    };

    ($scope:expr, $component:ident ($ctx:ident, $view:ident) $body:block $($rest:tt)*) => {
        {
            $scope.child($component, Some(Arc::new(|$ctx, $view| $body)));
        }

        $crate::rsx_child!($scope, $($rest)*);
    };

    ($scope:expr, $component:ident $($rest:tt)*) => {
        {
            $scope.child($component, None);
        }
        $crate::rsx_child!($scope, $($rest)*);
    };

    ($rsx:expr,) => {};
}

#[macro_export]
macro_rules! rsx_dep {
    ($dep:ident as $dp:pat) => {
        let $dp = $dep.clone();
    };

    ($dep:ident) => {
        let $dep = $dep.clone();
    };
}
