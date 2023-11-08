use std::{mem::size_of, ptr::null_mut};

use crate::keyboard::get_keyboard_handler;
use cef_sys::{_cef_base_ref_counted_t, cef_client_t, cef_load_handler_t};

use crate::rc::RcImpl;

/// See [cef_client_t] for more documentation.
pub trait ClientTrait: Sized {
    // TODO draw the rest owl
    fn into_raw(self) -> *mut cef_client_t {
        let mut object: cef_client_t = unsafe { std::mem::zeroed() };
        object.get_keyboard_handler = Some(get_keyboard_handler);
        RcImpl::new(object, self) as *mut _
    }
}

#[doc = "See [cef_client_t] for more documentation."]
#[derive(Debug, Clone)]
pub struct CefClient {}

impl ClientTrait for CefClient {}
impl CefClient {
    pub fn new() -> Self {
        CefClient {}
    }
}
