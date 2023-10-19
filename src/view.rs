use cef_sys::{cef_view_delegate_t, cef_view_t};

use crate::rc::{RcImpl, RefGuard};

/// See [cef_view_t] for more documentation.
#[derive(Debug, Clone)]
pub struct View(pub(crate) RefGuard<cef_view_t>);

/// See [cef_view_delegate_t] for more documentation.
pub trait ViewDelegate: Clone + Send + Sync {
    /// Create cef raw types for internal usage. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide how to clone the value.
    fn to_raw(&self) -> *mut cef_view_delegate_t {
        let object: cef_view_delegate_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self.clone()) as *mut _
    }
}
