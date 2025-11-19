use osui::Node;

fn app() -> Node {
    Box::new(|renderer| {
        renderer.draw_text("Hello, World!", 0, 0).unwrap();
    })
}

fn main() {
    let mut osui = osui::Osui::new(Box::new(app));
    loop {
        osui.render().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
