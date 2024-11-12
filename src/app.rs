use std::sync::{mpsc, Arc};
use std::thread;

use crate::{
    clear, create_frame, flush, get_term_size, hide_cursor, render_to_frame, Element, State, Value,
};

/// Renders an element with a given state to the terminal.
///
/// This macro handles the process of creating a frame, rendering the element to it,
/// clearing the terminal, and then printing the rendered frame.
///
/// # Parameters
///
/// * `$element` - The element to be rendered.
/// * `$state` - The current state of the element.
///
/// # Returns
///
/// This macro does not return a value. It performs side effects by printing to the terminal.
macro_rules! render {
    ($element:expr, $state:expr) => {
        let (width, height) = get_term_size();
        let mut frame: Vec<String> = create_frame(Value::new(width), Value::new(height));
        render_to_frame($state, width, &mut frame, $element);
        clear();
        print!("{}", frame.join(""));
        flush();
    };
}

pub fn run(element: Arc<dyn Element>) -> bool {
    hide_cursor();
    crossterm::terminal::enable_raw_mode().unwrap();
    clear();

    let (tx, rx) = mpsc::channel();

    let (exit_tx, exit_) = mpsc::channel::<()>();

    thread::spawn({
        let element_thread = Arc::clone(&element);
        move || loop {
            if exit_.try_recv().is_ok() {
                break;
            }
            let event = crossterm::event::read().unwrap();
            let e = Arc::clone(&element_thread);
            let command_handler = crate::CommandHandler(tx.clone());

            thread::spawn(move || {
                e.event(&command_handler, event);
            });
        }
    });

    let mut current_state = State::Hover;
    loop {
        if let Ok(command) = rx.try_recv() {
            match command {
                crate::Command::SetState(state) => {
                    current_state = state;
                }
                crate::Command::Exit => {
                    exit_tx.send(()).unwrap();
                    break;
                }
                crate::Command::Rebuild => {
                    exit_tx.send(()).unwrap();
                    return true;
                }
            }
        } else {
            render!(&element, current_state.clone());
        }
        thread::sleep(std::time::Duration::from_millis(20));
    }
    false
}
