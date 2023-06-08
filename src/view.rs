use cef_sys::{cef_view_delegate_t, cef_view_t};

use crate::rc::{RcImpl, RefGuard};

/// A View is a rectangle within the views View hierarchy. It is the base
/// structure for all Views. All size and position values are in density
/// independent pixels (DIP) unless otherwise indicated. Methods must be called
/// on the browser process UI thread unless otherwise indicated.
#[derive(Debug, Clone)]
pub struct View(pub(crate) RefGuard<cef_view_t>);

/// Implement this structure to handle view events. All size and position values
/// are in density independent pixels (DIP) unless otherwise indicated. The
/// functions of this structure will be called on the browser process UI thread
/// unless otherwise indicated.
pub trait ViewDelegate: Clone + Send + Sync {
    /// Create cef raw types for internal usage. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide how to clone the value.
    fn to_raw(&self) -> *mut cef_view_delegate_t {
        let object: cef_view_delegate_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self.clone()) as *mut _
    }
}
