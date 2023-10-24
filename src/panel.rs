use cef_sys::{cef_panel_delegate_t, cef_panel_t};

use crate::{
    add_view_delegate_methods, impl_rc,
    rc::{RcImpl, RefGuard},
    view::View,
    ViewDelegate, Window,
};

/// See [cef_panel_t] for more documentation.
#[derive(Debug, Clone)]
pub struct Panel(pub(crate) RefGuard<cef_panel_t>);

impl_rc!(Panel, cef_panel_t, base);

impl Panel {
    pub fn add_child_view(&self, view: View) {
        self.0
            .add_child_view
            .map(|f| unsafe { f(self.0.get_raw(), view.0.into_raw()) });
    }

    pub fn as_window(&self) -> Option<Window> {
        self.0
            .as_window
            .map(|f| {
                let p = unsafe { f(self.0.get_raw()) };
                if p.is_null() {
                    None
                } else {
                    Some(Window(unsafe { RefGuard::from_raw(p) }))
                }
            })
            .flatten()
    }
}

/// See [cef_panel_delegate_t] for more documentation.
pub trait PanelDelegate: ViewDelegate {
    fn into_raw(self) -> *mut cef_panel_delegate_t {
        let mut object: cef_panel_delegate_t = unsafe { std::mem::zeroed() };

        let view = &mut object.base;
        add_view_delegate_methods!(view);

        RcImpl::new(object, self) as *mut _
    }
}
