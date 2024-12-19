#[macro_export]
macro_rules! launch {
    ($expr:expr) => {
        let mut element = $expr;
        let mut document = $crate::Document::with_elem(&mut element);
        while document.run() {
            element = $expr;
        }
    };
    ($expr:expr, $css:expr) => {
        let mut element = $expr;
        let mut document = $crate::Document::with_elem(&mut element);
        document.set_css($css);
        while document.run() {
            element = $expr;
        }
    };
}
