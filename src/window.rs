use std::ffi::c_int;

use cef_sys::{
    cef_window_create_top_level, cef_window_delegate_t, cef_window_info_t, cef_window_t,
};

use crate::{
    add_view_delegate_methods,
    panel::{Panel, PanelDelegate},
    rc::{RcImpl, RefGuard},
    string::CefString,
    wrapper, Rect,
};

/// See [cef_window_info_t] for more documentation.
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub window_name: CefString,
    pub bounds: Rect,
    // TODO: raw_window_handle
    pub parent_window: u64,
    pub windowless_rendering_enabled: bool,
    pub shared_texture_enabled: bool,
    pub external_begin_frame_enabled: bool,
    pub window: u64,
}

impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            window_name: CefString::new("UwU"),
            bounds: Rect {
                x: 0,
                y: 0,
                width: 1280,
                height: 720,
            },
            parent_window: 0,
            windowless_rendering_enabled: false,
            shared_texture_enabled: false,
            external_begin_frame_enabled: false,
            window: 0,
        }
    }
}

impl WindowInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_raw(self) -> cef_window_info_t {
        cef_window_info_t {
            window_name: self.window_name.get_raw(),
            bounds: self.bounds,
            parent_window: self.parent_window,
            windowless_rendering_enabled: self.windowless_rendering_enabled as c_int,
            shared_texture_enabled: self.shared_texture_enabled as c_int,
            external_begin_frame_enabled: self.external_begin_frame_enabled as c_int,
            window: self.window,
        }
    }
}

wrapper!(
    #[doc = "See [cef_window_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Window(cef_window_t);
    // method to close window
    pub fn close(&self);

    pub fn show(&self);
    // method to hide window
    pub fn hide(&self);

    pub fn bring_to_top(&self);

    /*  TODO: not work, to fixed */
    //pub fn maximize(&self);

    pub fn minimize(&self);

);

impl Window {
    pub fn get_panel(&self) -> Panel {
        unsafe { Panel(self.0.convert()) }
    }

    // implement _cef_window_t::set_fullscreen
    pub fn set_fullscreen(&self) {
        unsafe {
            let cef_sys_win: RefGuard<cef_sys::_cef_window_t> = self.clone().0;
            match cef_sys_win.set_fullscreen {
                Some(cef_sys_fn)=>{
                    cef_sys_fn(self.clone().into_raw(),1);
                }
                None=>{}
            }
        }
    }
}

/// See [cef_window_delegate_t] for more documentation.
pub trait WindowDelegate: PanelDelegate {
    fn on_window_created(&self, _window: Window) {}
    fn on_window_closing(&self, _window: Window) {}
    fn on_window_destroyed(&self, _window: Window) {}
    fn can_close(&mut self, _window: Window) -> bool {
        true
    }

    fn into_raw(self) -> *mut cef_window_delegate_t {
        let mut object: cef_window_delegate_t = unsafe { std::mem::zeroed() };

        // Panal delegate doesn't have any methods. So we skip to view.
        let view = &mut object.base.base;
        add_view_delegate_methods!(view);

        object.on_window_created = Some(on_window_created::<Self>);
        object.on_window_closing = Some(on_window_closing::<Self>);
        object.on_window_destroyed = Some(on_window_destroyed::<Self>);
        object.can_close = Some(can_close::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

extern "C" fn on_window_created<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_created(window);
}

extern "C" fn on_window_closing<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_closing(window);
}

extern "C" fn on_window_destroyed<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    obj.interface.on_window_destroyed(window);
}

extern "C" fn can_close<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) -> i32 {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(unsafe { RefGuard::from_raw(window) });
    let result = obj.interface.can_close(window);
    result as i32
}

pub fn create_top_level_window(delegate: impl WindowDelegate) -> Window {
    let window = unsafe { cef_window_create_top_level(WindowDelegate::into_raw(delegate)) };
    Window(unsafe { RefGuard::from_raw(window) })
}
