use crossterm::{
    event::Event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::prelude::*;
use crate::utils::{
    clear, create_frame, flush, get_term_size, hide_cursor, render_to_frame, show_cursor,
};
use std::{ffi::c_void, ptr::null, sync::mpsc};

pub struct LArgs {
    pub element: *mut c_void,
    pub sender: *const c_void,
    pub receiver: *const c_void,
    pub running: bool,
}

extern "C" {
    fn c_run(element: *mut c_void, sender: *const c_void, receiver: *const c_void) -> bool;
}

fn event(element: &mut Element, event: Event) {
    if let crossterm::event::Event::Resize(width, height) = event {
        element.update_data(width as usize, height as usize);
    }
    element.event(event);
}

pub fn run(element: &mut Element) -> bool {
    hide_cursor();
    enable_raw_mode().unwrap();
    clear();
    unsafe {
        let (tx, rx) = mpsc::channel::<Command>();
        let c = c_run(
            element as *mut Element as *mut c_void,
            &tx as *const mpsc::Sender<Command> as *const c_void,
            &rx as *const mpsc::Receiver<Command> as *const c_void,
        );
        if !c {
            show_cursor();
            disable_raw_mode().unwrap();
            clear();
        }
        return c;
    }
}

#[no_mangle]
extern "C" fn render(ptr: *mut c_void) -> bool {
    if ptr.is_null() {
        return false;
    }
    let element = unsafe { &mut *(ptr as *mut Element) };
    let (width, height) = get_term_size();
    let mut frame: Vec<String> = create_frame(width, height);
    render_to_frame(true, width, &mut frame, element);
    clear();
    print!("{}", frame.join(""));
    flush();
    true
}

#[no_mangle]
extern "C" fn event_loop(ptr: *mut c_void) -> *const c_void {
    if ptr.is_null() {
        return null() as *const c_void;
    }
    let args = unsafe { &mut *(ptr as *mut LArgs) };
    let element = unsafe { &mut *(args.element as *mut Element) };
    let tx = unsafe { &*(args.sender as *const mpsc::Sender<Command>) };

    let (width, height) = get_term_size();
    element.update_data(width, height);
    event(element, crossterm::event::Event::FocusGained);

    while args.running {
        if ptr.is_null() {
            break;
        }
        event(element, crossterm::event::read().unwrap());
        tx.send(Command::Exit).unwrap();
    }
    null() as *const c_void
}

#[no_mangle]
extern "C" fn cmd_loop(ptr: *mut c_void) -> *const c_void {
    if ptr.is_null() {
        return null() as *const c_void;
    }
    let args = unsafe { &mut *(ptr as *mut LArgs) };
    let _element = unsafe { &mut *(args.element as *mut Element) };
    let rx = unsafe { &*(args.receiver as *const mpsc::Receiver<Command>) };
    loop {
        match rx.recv().unwrap() {
            Command::Exit => {
                args.running = false;
                break;
            }
        }
    }
    null()
}
