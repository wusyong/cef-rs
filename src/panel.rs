use cef_sys::{cef_panel_delegate_t, cef_panel_t};

use crate::{
    rc::{RcImpl, RefGuard},
    view::View,
    ViewDelegate,
};

/// See [cef_panel_t] for more documentation.
#[derive(Debug, Clone)]
pub struct Panel(pub(crate) RefGuard<cef_panel_t>);

impl Panel {
    pub fn add_child_view(&self, view: &View) {
        self.0
            .add_child_view
            .map(|f| unsafe { f(self.0.get_raw(), view.0.get_raw()) });
    }
}

/// See [cef_panel_delegate_t] for more documentation.
pub trait PanelDelegate: ViewDelegate + Clone + Send + Sync {
    /// Create cef raw types for internal usage. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide how to clone the value.
    fn to_raw(&self) -> *mut cef_panel_delegate_t {
        let object: cef_panel_delegate_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self.clone()) as *mut _
    }
}
