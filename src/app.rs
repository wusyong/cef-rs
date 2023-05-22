use cef_sys::cef_app_t;

use crate::rc::RcImpl;

pub trait App {
    fn foo();
}

pub fn into_raw(interface: impl App) -> *mut cef_app_t {
    let mut object: cef_app_t = unsafe { std::mem::zeroed() };

    // object.get_render_process_handler = Some(get_render_process_handler::<T>);
    // object.get_browser_process_handler = Some(get_browser_process_handler::<T>);
    // object.on_before_command_line_processing = Some(on_before_command_line_processing::<T>);

    let rc = RcImpl::new(object, interface);

    Box::into_raw(Box::new(rc)) as *mut _
}
