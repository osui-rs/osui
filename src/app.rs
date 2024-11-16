use crossterm::terminal::enable_raw_mode;

use crate::{clear, create_frame, flush, get_term_size, render_to_frame, Element, State, StateChanger, Value};
use std::ffi::c_void;

pub struct ElementWrapper<'a> {
    element: &'a mut Box<dyn Element>,
}

impl<'a> ElementWrapper<'a> {
    pub fn new(element: &'a mut Box<dyn Element>) -> *mut c_void {
        Box::into_raw(Box::new(ElementWrapper { element })) as *mut c_void
    }

    pub unsafe fn from_raw(ptr: *mut c_void) -> &'a mut Box<dyn Element> {
        assert!(!ptr.is_null(), "Pointer to ElementWrapper is null");
        let wrapper = ptr as *mut ElementWrapper;
        (*wrapper).element
    }
}

extern "C" {
    fn c_run(element: *mut c_void) -> bool;
}

#[no_mangle]
pub extern "C" fn render(ptr: *mut c_void, state: u32) {
    let element = unsafe { ElementWrapper::from_raw(ptr) };
    let (width, height) = get_term_size();
    let mut frame: Vec<String> = create_frame(Value::new(width), Value::new(height));
    render_to_frame(State::from_u32(state), width, &mut frame, element);
    clear();
    print!("{}", frame.join(""));
    flush();
}

#[no_mangle]
pub extern "C" fn event(ptr: *mut c_void, state: *mut u32) {
    let event = crossterm::event::read().unwrap();
    let element = unsafe { ElementWrapper::from_raw(ptr) };
    element.event(event, &StateChanger(state));
}

pub fn run(data: &mut Box<dyn Element>) -> bool {
    enable_raw_mode().unwrap();
    unsafe {
        let element_ptr = ElementWrapper::new(data);
        return c_run(element_ptr);
    }
}
