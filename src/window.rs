use std::ffi::c_int;

use cef_sys::cef_window_info_t;

use crate::{string::CefString, Rect};

/// Representing window information.
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// The initial title of the window, to be set when the window is created.
    /// Some layout managers (e.g., Compiz) can look at the window title
    /// in order to decide where to place the window when it is
    /// created. When this attribute is not empty, the window title will
    /// be set before the window is mapped to the dispay. Otherwise the
    /// title will be initially empty.
    pub window_name: CefString,

    /// Initial window bounds.
    pub bounds: Rect,

    // TODO: raw_window_handle
    /// Pointer for the parent window.
    pub parent_window: u64,

    /// Set to true (1) to create the browser using windowless (off-screen)
    /// rendering. No window will be created for the browser and all rendering
    /// will occur via the CefRenderHandler interface. The |parent_window| value
    /// will be used to identify monitor info and to act as the parent window for
    /// dialogs, context menus, etc. If |parent_window| is not provided then the
    /// main screen monitor will be used and some functionality that requires a
    /// parent window may not function correctly. In order to create windowless
    /// browsers the CefSettings.windowless_rendering_enabled value must be set to
    /// true. Transparent painting is enabled by default but can be disabled by
    /// setting CefBrowserSettings.background_color to an opaque value.
    pub windowless_rendering_enabled: bool,

    /// Set to true (1) to enable shared textures for windowless rendering. Only
    /// valid if windowless_rendering_enabled above is also set to true. Currently
    /// only supported on Windows (D3D11).
    pub shared_texture_enabled: bool,

    /// Set to true (1) to enable the ability to issue BeginFrame requests from
    /// the client application by calling CefBrowserHost::SendExternalBeginFrame.
    pub external_begin_frame_enabled: bool,

    /// Pointer for the new browser window. Only used with windowed rendering.
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
