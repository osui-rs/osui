use osui::{
    components::{div, text, Div},
    oml, ComponentParams,
};

fn main() {
    let some_text = app(ComponentParams {
        children: vec![],
        expr: "".to_string(),
    });
    let doc = oml!(div { some_text; });

    println!("{:?}", doc);
}

fn app(_: ComponentParams) -> Box<Div> {
    oml!(
        // Components (C)
        div {
            // Props, With components (PC)
            div () {

                // Props (P)
                text(/* Properties go here, Example: x = 20 y = 5 */)
            }

            // Expression (E)
            text("Hello";)

            // Expression, With components (EC)
            text("World!";) {
                // Components in here
            }
        }
    )
}
