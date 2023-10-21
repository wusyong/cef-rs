use std::ffi::c_int;

use cef_sys::{cef_view_delegate_t, cef_view_t};

use crate::rc::{RcImpl, RefGuard};

/// See [cef_view_t] for more documentation.
#[derive(Debug, Clone)]
pub struct View(pub(crate) RefGuard<cef_view_t>);

/// See [cef_view_delegate_t] for more documentation.
pub trait ViewDelegate: Sized {
    fn on_parent_view_changed(&mut self, _view: &View, _added: bool, _parent: &View) {}
    fn on_child_view_changed(&mut self, _view: &View, _added: bool, _child: &View) {}
    fn on_window_changed(&mut self, _view: &View, _added: bool) {}

    fn into_raw(self) -> *mut cef_view_delegate_t {
        let mut object: cef_view_delegate_t = unsafe { std::mem::zeroed() };

        object.on_parent_view_changed = Some(on_parent_view_changed::<Self>);
        object.on_child_view_changed = Some(on_child_view_changed::<Self>);
        object.on_window_changed = Some(on_window_changed::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

pub(crate) extern "C" fn on_parent_view_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
    parent: *mut cef_view_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    let parent = View(unsafe { RefGuard::from_raw(parent) });
    obj.interface.on_child_view_changed(&view, added, &parent);
}

pub(crate) extern "C" fn on_child_view_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
    child: *mut cef_view_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    let child = View(unsafe { RefGuard::from_raw(child) });
    obj.interface.on_child_view_changed(&view, added, &child);
}

pub(crate) extern "C" fn on_window_changed<I: ViewDelegate>(
    this: *mut cef_view_delegate_t,
    view: *mut cef_view_t,
    added: c_int,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let view = View(unsafe { RefGuard::from_raw(view) });
    let added = added != 0;
    obj.interface.on_window_changed(&view, added);
}
