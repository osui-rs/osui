use osui::prelude::*;

fn main() {
    launch!(App);
}

#[component]
pub fn App() {
    rsx! {
        async {
            sleep(3000);
            ersx!(text {
                "OK"
            })
        }
        text { "Hello world!" }
    }
}
