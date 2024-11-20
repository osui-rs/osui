use crossterm::{
    event::Event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{
    utils::{clear, create_frame, flush, get_term_size, hide_cursor, render_to_frame, show_cursor},
    Element,
};
use std::{ffi::c_void, ptr::null};

extern "C" {
    fn c_run(element: *mut c_void) -> bool;
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
    let element = unsafe { &mut *(ptr as *mut Element) };
    let (width, height) = get_term_size();
    element.update_data(width, height);
    event(element, crossterm::event::Event::FocusGained);
    loop {
        if ptr.is_null() {
            break;
        }
        event(element, crossterm::event::read().unwrap())
    }
    null() as *const c_void
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
        let c = c_run(element as *mut Element as *mut c_void);
        if !c {
            show_cursor();
            disable_raw_mode().unwrap();
            clear();
        }
        return c;
    }
}
