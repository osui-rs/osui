#[macro_export]
macro_rules! rsx {
    ($($e:literal)*) => {
        std::sync::Arc::new(
            move |frame: &$crate::Frame| -> $crate::Result<()> {
                $(
                    frame.draw(&format!($e), Area::center())?;
                )*
                Ok(())
            }
        )
    };
}
