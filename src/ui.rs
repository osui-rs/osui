use crate::Component;

pub fn text(text: &str) -> Component<String> {
    Component::new(text.to_string())
}
