use crossterm::{
    event::Event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::prelude::*;
use crate::utils::{
    clear, create_frame, flush, get_term_size, hide_cursor, render_to_frame, show_cursor,
};
use std::{ffi::c_void, ptr::null, sync::mpsc};

pub struct LArgs<'a> {
    pub element: &'a mut Element,

    // for sending commands
    pub stx: mpsc::Sender<Command>,
    pub srx: mpsc::Receiver<Command>,

    // for returning command output
    pub rtx: mpsc::Sender<Option<*mut Element>>,
    pub rrx: mpsc::Receiver<Option<*mut Element>>,
    pub running: bool,
}

extern "C" {
    fn c_run(args: *mut c_void) -> bool;
}

fn event(element: &mut Element, event: Event, document: &Document) {
    if let crossterm::event::Event::Resize(width, height) = event {
        element.update_data(width as usize, height as usize);
    }
    element.event(event, document);
}

pub fn run(element: &mut Element) -> bool {
    hide_cursor();
    enable_raw_mode().unwrap();
    clear();
    unsafe {
        let (stx, srx) = mpsc::channel();
        let (rtx, rrx) = mpsc::channel();
        let c = c_run(&mut LArgs {
            running: true,
            element,
            stx,
            srx,
            rtx,
            rrx,
        } as *mut LArgs as *mut c_void);
        if !c {
            show_cursor();
            disable_raw_mode().unwrap();
            clear();
        }
        return c;
    }
}

#[no_mangle]
extern "C" fn render_loop(ptr: *mut c_void) -> bool {
    if ptr.is_null() {
        return false;
    }
    let args = unsafe { &mut *(ptr as *mut LArgs) };
    while args.running {
        let (width, height) = get_term_size();
        let mut frame: Vec<String> = create_frame(width, height);
        render_to_frame(true, width, &mut frame, args.element);
        clear();
        print!("{}", frame.join(""));
        flush();
    }
    false
}

#[no_mangle]
extern "C" fn event_loop(ptr: *mut c_void) -> *const c_void {
    if ptr.is_null() {
        return null() as *const c_void;
    }
    let args = unsafe { &mut *(ptr as *mut LArgs) };
    let document = Document {
        cmd_sender: args.stx.clone(),
        cmd_recv: &args.rrx as *const mpsc::Receiver<Option<*mut Element>> as *const c_void,
    };

    let (width, height) = get_term_size();
    args.element.update_data(width, height);
    event(
        args.element,
        crossterm::event::Event::FocusGained,
        &document,
    );

    while args.running {
        if ptr.is_null() {
            break;
        }
        event(args.element, crossterm::event::read().unwrap(), &document);
    }
    null() as *const c_void
}

#[no_mangle]
extern "C" fn cmd_loop(ptr: *mut c_void) -> *const c_void {
    if ptr.is_null() {
        return null() as *const c_void;
    }
    let args = unsafe { &mut *(ptr as *mut LArgs) };

    loop {
        match args.srx.recv().unwrap() {
            Command::Exit => {
                args.running = false;
                break;
            }
            Command::GetElementById(id) => {
                args.rtx
                    .send(if let Some(e) = args.element.get_element_by_id(&id) {
                        Some(e as *mut Element)
                    } else {
                        None
                    })
                    .unwrap();
            }
        }
    }
    null()
}
