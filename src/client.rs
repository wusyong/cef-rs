use cef_sys::cef_client_t;

use crate::rc::RcImpl;

/// Implement this structure to provide handler implementations.
pub trait Client: Clone {
    // TODO draw the rest owl
    /// Create cef raw types for internal usafe. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide to wrap your own type with `Arc` or
    /// perform deep clone.
    fn to_raw(&self) -> *mut cef_client_t {
        let object: cef_client_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self.clone()) as *mut _
    }
}
