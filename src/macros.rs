#[macro_export]
macro_rules! launch {
    ($elem:path) => {
        let mut element = <$elem>::default().create_element();
        let mut document = $crate::Document::with_elem(&mut element);
        while document.run() {
            element = <$elem>::default().create_element();
        }
    };
    ($elem:path, $css:expr) => {
        let mut element = <$elem>::default().create_element();
        let mut document = $crate::Document::with_elem(&mut element);
        document.set_css($css);
        while document.run() {
            element = <$elem>::default().create_element();
        }
    };
}
