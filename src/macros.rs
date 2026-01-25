#[macro_export]
macro_rules! rsx {
    () => {
        $crate::frontend::Rsx::new()
    };
}

#[macro_export]
macro_rules! rsx_scope {
    () => {};
}
