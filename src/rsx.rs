/// A brief summary of what the macro does.
///
/// A detailed explanation about the macro's purpose,
/// how it expands, and when to use it.
///
/// # Examples
///
/// Simple
/// ```rust
/// rsx! {
///     "Hello, World!"
/// }
/// ```
/// 
/// Button example
/// ```rust
/// rsx! {
///     button { "{count}" }
/// }
/// ```
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {
        std::sync::Arc::new(
            move |frame: &mut $crate::Frame, event: Option<$crate::console::Event>| -> $crate::Result<()> {
                $crate::rsx_inner!(frame, event; $($inner)*);
                Ok(())
            }
        )
    };
}

/// # Warning
///
/// **Don't use this macro manually** use `rsx!` instead
#[macro_export]
macro_rules! rsx_inner {
    // For loop
    ($frame:expr, $event:expr;
        for ($i:pat in $e:expr) $(%$s:ident),* {$($inner:tt)*}
    $($rest:tt)*) => {
        for $i in $e {
            $(let $s = $s.copy_state();)*
            $crate::rsx_inner!($frame, $event; $($inner)*);
        }
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };

    // If statement
    ($frame:expr, $event:expr;
        if ($e:expr) $(%$s:ident),* {$($inner:tt)*}
        $(else if ($ee:expr) {$($elif_inner:tt)*})*
        else {$($else_inner:tt)*}
    $($rest:tt)*) => {
        $(let $s = $s.copy_state();)*
        if $e {
            $crate::rsx_inner!($frame, $event; $($inner)*);
        } $(else if $ee {
            $crate::rsx_inner!($frame, $event; $($elif_inner)*);
        })* else {
            $crate::rsx_inner!($frame, $event; $($else_inner)*);
        }
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };

    ($frame:expr, $event:expr;
        if ($e:expr) $(%$s:ident),* {$($inner:tt)*}
    $($rest:tt)*) => {
        $(let $s = $s.copy_state();)*
        if $e {
            $crate::rsx_inner!($frame, $event; $($inner)*);
        }
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };

    // Literal
    ($frame:expr, $event:expr;
        $elem:literal ($($inner:tt)*)
    $($rest:tt)*) => {
        let mut area = $crate::Area::new();
        $crate::tw_area!(area, $($inner)*);
        $frame.draw(&format!($elem), area)?;
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };
    ($frame:expr, $event:expr;
        $elem:literal
    $($rest:tt)*) => {
        $frame.draw(&format!($elem), Area::new())?;
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };

    // name { rsx }
    ($frame:expr, $event:expr;
        $elem:path {
            $($einner:tt)*
        } ($($inner:tt)*)
    $($rest:tt)*) => {
        let mut area = $crate::Area::new();
        $crate::tw_area!(area, $($inner)*);
        $elem()($frame, $event)?;
        $crate::rsx_inner!($frame, $event; $($rest)*);
    };

    ($frame:expr, $event:expr;) => {};
}

/// # Warning
///
/// **Don't use this macro manually** use `rsx!` instead
#[macro_export]
macro_rules! tw_area {
    ($a:expr, $n:ident) => {
        $a.$n;
    };

    // X
    ($a:expr, x-$v:ident $($rest:tt)*) => {
        $a.x = $crate::Pos::$v;
        $crate::tw_area!($a, $($rest)*);
    };

    ($a:expr, x-$v:literal $($rest:tt)*) => {
        $a.x = $crate::Pos::Num($v);
        $crate::tw_area!($a, $($rest)*);
    };

    // Y
    ($a:expr, y-$v:ident $($rest:tt)*) => {
        $a.y = $crate::Pos::$v;
        $crate::tw_area!($a, $($rest)*);
    };

    ($a:expr, y-$v:literal $($rest:tt)*) => {
        $a.y = $crate::Pos::Num($v);
        $crate::tw_area!($a, $($rest)*);
    };

    // Width
    ($a:expr, width-$v:ident $($rest:tt)*) => {
        $a.width = $crate::Size::$v;
        $crate::tw_area!($a, $($rest)*);
    };

    ($a:expr, width-$v:literal $($rest:tt)*) => {
        $a.width = $crate::Size::Num($v);
        $crate::tw_area!($a, $($rest)*);
    };

    // Height
    ($a:expr, height-$v:ident $($rest:tt)*) => {
        $a.height = $crate::Size::$v;
        $crate::tw_area!($a, $($rest)*);
    };

    ($a:expr, height-$v:literal $($rest:tt)*) => {
        $a.height = $crate::Size::Num($v);
        $crate::tw_area!($a, $($rest)*);
    };

    ($a:expr,) => {};
}
