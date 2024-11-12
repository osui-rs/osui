use std::sync::{Arc, Mutex};

use crate::{rsx_elem, ui::*, Element, Handler};

pub(crate) fn app(i: Arc<Mutex<i32>>) -> std::sync::Arc<dyn Element> {
    let i1 = Arc::clone(&i);
    rsx_elem! { text { on_click: Handler::new(move |t: &Text, ch, _| {
        *i1.lock().unwrap() += 1;
        ch.rebuild();
        // {
        //     let mut i = i1.lock().unwrap();
        //     *i += 1;
        //     *t.text.lock().unwrap() = i.to_string();
        // }
    }), "{}", i.lock().unwrap() } }
}
