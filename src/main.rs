use osui::xml;

fn main() {
    let doc = xml!(
        <div> {
            <div> {
            }
        }
    );

    println!("{doc}");
}
