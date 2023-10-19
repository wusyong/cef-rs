use std::ffi::c_int;

use cef_sys::{
    cef_window_create_top_level, cef_window_delegate_t, cef_window_info_t, cef_window_t,
};

use crate::{
    panel::{Panel, PanelDelegate},
    rc::{RcImpl, RefGuard},
    string::CefString,
    Rect,
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

    pub fn into_raw(self) -> cef_window_info_t {
        cef_window_info_t {
            window_name: self.window_name.into_raw(),
            bounds: self.bounds,
            parent_window: self.parent_window,
            windowless_rendering_enabled: self.windowless_rendering_enabled as c_int,
            shared_texture_enabled: self.shared_texture_enabled as c_int,
            external_begin_frame_enabled: self.external_begin_frame_enabled as c_int,
            window: self.window,
        }
    }
}

/// See [cef_window_t] for more documentation.
#[derive(Debug, Clone)]
pub struct Window(RefGuard<cef_window_t>);

impl Window {
    pub fn as_panel(&self) -> Panel {
        unsafe { Panel(self.0.clone().convert()) }
    }

    pub fn show(&self) {
        unsafe {
            self.0.show.map(|f| f(self.0.get_raw()));
        }
    }

    pub fn from_raw(raw: *mut cef_window_t) -> Option<Window> {
        if raw.is_null() {
            None
        } else {
            Some(Window(RefGuard::from_raw(raw)))
        }
    }
}

/// See [cef_window_delegate_t] for more documentation.
pub trait WindowDelegate: PanelDelegate + Clone + Send + Sync {
    fn on_window_created(&mut self, _window: &Window) {}

    /// Create cef raw types for internal usage. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide how to clone the value.
    fn to_raw(&self) -> *mut cef_window_delegate_t {
        let mut object: cef_window_delegate_t = unsafe { std::mem::zeroed() };

        object.on_window_created = Some(on_window_created::<Self>);

        RcImpl::new(object, self.clone()) as *mut _
    }
}

extern "C" fn on_window_created<I: WindowDelegate>(
    this: *mut cef_window_delegate_t,
    window: *mut cef_window_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let window = Window(RefGuard::from_raw(window));
    obj.interface.on_window_created(&window);
    window.0.into_raw();
}

pub fn create_top_level_window(delegate: impl WindowDelegate) -> Window {
    let window = unsafe { cef_window_create_top_level(WindowDelegate::to_raw(&delegate)) };
    Window::from_raw(window).expect("Failed to create window.")
}
