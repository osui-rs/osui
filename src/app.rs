use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::{
    utils::{clear, create_frame, flush, get_term_size, hide_cursor, render_to_frame, show_cursor},
    Element, State, StateChanger,
};
use std::ffi::c_void;

struct ElementWrapper<'a>(&'a mut Element);

impl<'a> ElementWrapper<'a> {
    pub unsafe fn from_raw(ptr: *mut c_void) -> &'a mut Element {
        assert!(!ptr.is_null(), "Pointer to ElementWrapper is null");
        let wrapper = ptr as *mut ElementWrapper;
        (*wrapper).0
    }
}

extern "C" {
    fn c_run(element: *mut c_void) -> bool;
}

#[no_mangle]
pub extern "C" fn render(ptr: *mut c_void, state: u32) {
    let element = unsafe { ElementWrapper::from_raw(ptr) };
    let (width, height) = get_term_size();
    let mut frame: Vec<String> = create_frame(width, height);
    render_to_frame(State::from_u32(state), width, &mut frame, element);
    clear();
    print!("{}", frame.join(""));
    flush();
}

#[no_mangle]
pub extern "C" fn event(ptr: *mut c_void, state: *mut u32) {
    let event = crossterm::event::read().unwrap();
    let element = unsafe { ElementWrapper::from_raw(ptr) };
    if let crossterm::event::Event::Resize(width, height) = event {
        element.update_data(width as usize, height as usize);
    }
    element.event(event, &StateChanger(state));
}

#[no_mangle]
pub extern "C" fn init_event(ptr: *mut c_void, state: *mut u32) {
    let event = crossterm::event::Event::FocusGained;
    let element = unsafe { ElementWrapper::from_raw(ptr) };
    let (width, height) = get_term_size();
    element.update_data(width, height);
    element.event(event, &StateChanger(state));
}

pub fn run(element: &mut Element) -> bool {
    hide_cursor();
    enable_raw_mode().unwrap();
    clear();
    unsafe {
        let c = c_run(&mut ElementWrapper(element) as *mut ElementWrapper as *mut c_void);
        show_cursor();
        disable_raw_mode().unwrap();
        clear();
        return c;
    }
}
