use osui::prelude::*;

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    app().draw(&screen);

    screen.run()
}

pub fn app() -> Rsx {
    let items: State<Vec<i32>> = use_state(vec![1, 2]);

    std::thread::spawn({
        let items = items.clone();
        move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            items.get().push(3);
            std::thread::sleep(std::time::Duration::from_millis(1000));
            items.get().remove(0);
        }
    });

    Rsx(vec![RsxElement::Element(
        StaticWidget::new(Box::new(FlexRow::new())),
        Rsx(vec![RsxElement::Iter(
            Box::new({
                let items = items.clone();
                move || {
                    Rsx(items
                        .get_dl()
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            RsxElement::Element(
                                StaticWidget::new(Box::new(format!("Item {i}"))),
                                Rsx(vec![]),
                            )
                        })
                        .collect())
                }
            }),
            Box::new(items),
        )]),
    )])
}
