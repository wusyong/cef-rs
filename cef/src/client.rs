use cef_sys::cef_client_t;

use crate::rc::RcImpl;

/// See [cef_client_t] for more documentation.
pub trait Client: Sized {
    // TODO draw the rest owl
    fn into_raw(self) -> *mut cef_client_t {
        let object: cef_client_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self) as *mut _
    }
}
