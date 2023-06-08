use std::{ffi::c_int, ptr::null_mut};

use cef_sys::{cef_browser_settings_t, cef_browser_view_t};

use crate::{client::Client, rc::RefGuard, string::CefString, window::WindowInfo, State, View};

/// Browser initialization settings. Specify NULL or 0 to get the recommended
/// default values. The consequences of using custom values may not be well
/// tested. Many of these and other settings can also configured using command-
/// line switches.
#[derive(Debug, Clone)]
pub struct BrowserSettings {
    /// The maximum rate in frames per second (fps) that CefRenderHandler::OnPaint
    /// will be called for a windowless browser. The actual fps may be lower if
    /// the browser cannot generate frames at the requested rate. The minimum
    /// value is 1 and the maximum value is 60 (default 30). This value can also
    /// be changed dynamically via CefBrowserHost::SetWindowlessFrameRate.
    pub windowless_frame_rate: usize,

    /// BEGIN values that map to WebPreferences settings.

    /// Font settings.
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: u32,
    pub default_fixed_font_size: u32,
    pub minimum_font_size: u32,
    pub minimum_logical_font_size: u32,

    /// Default encoding for Web content. If empty "ISO-8859-1" will be used. Also
    /// configurable using the "default-encoding" command-line switch.
    pub default_encoding: CefString,

    ///
    /// the "disable-remote-fonts" command-line switch.
    pub remote_fonts: State,

    /// Controls whether JavaScript can be executed. Also configurable using the
    /// "disable-javascript" command-line switch.
    pub javascript: State,

    /// Controls whether JavaScript can be used to close windows that were not
    /// opened via JavaScript. JavaScript can still be used to close windows that
    /// were opened via JavaScript or that have no back/forward history. Also
    /// configurable using the "disable-javascript-close-windows" command-line
    /// switch.
    pub javascript_close_windows: State,

    /// Controls whether JavaScript can access the clipboard. Also configurable
    /// using the "disable-javascript-access-clipboard" command-line switch.
    pub javascript_access_clipboard: State,

    /// Controls whether DOM pasting is supported in the editor via
    /// execCommand("paste"). The |javascript_access_clipboard| setting must also
    /// be enabled. Also configurable using the "disable-javascript-dom-paste"
    /// command-line switch.
    pub javascript_dom_paste: State,

    /// Controls whether image URLs will be loaded from the network. A cached
    /// image will still be rendered if requested. Also configurable using the
    /// "disable-image-loading" command-line switch.
    pub image_loading: State,

    /// Controls whether standalone images will be shrunk to fit the page. Also
    /// configurable using the "image-shrink-standalone-to-fit" command-line
    /// switch.
    pub image_shrink_standalone_to_fit: State,

    /// Controls whether text areas can be resized. Also configurable using the
    /// "disable-text-area-resize" command-line switch.
    pub text_area_resize: State,

    /// Controls whether the tab key can advance focus to links. Also configurable
    /// using the "disable-tab-to-links" command-line switch.
    pub tab_to_links: State,

    /// Controls whether local storage can be used. Also configurable using the
    /// "disable-local-storage" command-line switch.
    pub local_storage: State,

    /// Controls whether databases can be used. Also configurable using the
    /// "disable-databases" command-line switch.
    pub databases: State,

    /// Controls whether WebGL can be used. Note that WebGL requires hardware
    /// support and may not work on all systems even when enabled. Also
    /// configurable using the "disable-webgl" command-line switch.
    pub webgl: State,

    /// END values that map to WebPreferences settings.

    /// Background color used for the browser before a document is loaded and when
    /// no document color is specified. The alpha component must be either fully
    /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
    /// opaque then the RGB components will be used as the background color. If
    /// the alpha component is fully transparent for a windowed browser then the
    /// CefSettings.background_color value will be used. If the alpha component is
    /// fully transparent for a windowless (off-screen) browser then transparent
    /// painting will be enabled.
    pub background_color: u32,

    /// Comma delimited ordered list of language codes without any whitespace that
    /// will be used in the "Accept-Language" HTTP header. May be set globally
    /// using the CefSettings.accept_language_list value. If both values are
    /// empty then "en-US,en" will be used.
    pub accept_language_list: CefString,

    /// Controls whether the Chrome status bubble will be used. Only supported
    /// with the Chrome runtime. For details about the status bubble see
    /// https://www.chromium.org/user-experience/status-bubble/
    pub chrome_status_bubble: State,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            windowless_frame_rate: Default::default(),
            standard_font_family: Default::default(),
            fixed_font_family: Default::default(),
            serif_font_family: Default::default(),
            sans_serif_font_family: Default::default(),
            cursive_font_family: Default::default(),
            fantasy_font_family: Default::default(),
            default_font_size: Default::default(),
            default_fixed_font_size: Default::default(),
            minimum_font_size: Default::default(),
            minimum_logical_font_size: Default::default(),
            default_encoding: Default::default(),
            remote_fonts: State::STATE_DEFAULT,
            javascript: State::STATE_DEFAULT,
            javascript_close_windows: State::STATE_DEFAULT,
            javascript_access_clipboard: State::STATE_DEFAULT,
            javascript_dom_paste: State::STATE_DEFAULT,
            image_loading: State::STATE_DEFAULT,
            image_shrink_standalone_to_fit: State::STATE_DEFAULT,
            text_area_resize: State::STATE_DEFAULT,
            tab_to_links: State::STATE_DEFAULT,
            local_storage: State::STATE_DEFAULT,
            databases: State::STATE_DEFAULT,
            webgl: State::STATE_DEFAULT,
            background_color: Default::default(),
            accept_language_list: Default::default(),
            chrome_status_bubble: State::STATE_DEFAULT,
        }
    }
}

impl BrowserSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_raw(self) -> cef_browser_settings_t {
        cef_browser_settings_t {
            size: std::mem::size_of::<cef_browser_settings_t>(),
            windowless_frame_rate: self.windowless_frame_rate as c_int,
            standard_font_family: self.standard_font_family.into_raw(),
            fixed_font_family: self.fixed_font_family.into_raw(),
            serif_font_family: self.serif_font_family.into_raw(),
            sans_serif_font_family: self.sans_serif_font_family.into_raw(),
            cursive_font_family: self.cursive_font_family.into_raw(),
            fantasy_font_family: self.fantasy_font_family.into_raw(),
            default_font_size: self.default_font_size as c_int,
            default_fixed_font_size: self.default_fixed_font_size as c_int,
            minimum_font_size: self.minimum_font_size as c_int,
            minimum_logical_font_size: self.minimum_logical_font_size as c_int,
            default_encoding: self.default_encoding.into_raw(),
            remote_fonts: self.remote_fonts,
            javascript: self.javascript,
            javascript_close_windows: self.javascript_close_windows,
            javascript_access_clipboard: self.javascript_access_clipboard,
            javascript_dom_paste: self.javascript_dom_paste,
            image_loading: self.image_loading,
            image_shrink_standalone_to_fit: self.image_shrink_standalone_to_fit,
            text_area_resize: self.text_area_resize,
            tab_to_links: self.tab_to_links,
            local_storage: self.local_storage,
            databases: self.databases,
            webgl: self.webgl,
            background_color: self.background_color,
            accept_language_list: self.accept_language_list.into_raw(),
            chrome_status_bubble: self.chrome_status_bubble,
        }
    }
}

/// Create a new browser using the window parameters specified by |windowInfo|.
/// All values will be copied internally and the actual window (if any) will be
/// created on the UI thread. If |request_context| is NULL the global request
/// context will be used. This function can be called on any browser process
/// thread and will not block. The optional |extra_info| parameter provides an
/// opportunity to specify extra information specific to the created browser
/// that will be passed to cef_render_process_handler_t::on_browser_created() in
/// the render process.
pub fn create_browser<T: Client>(
    window_info: WindowInfo,
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
) -> i32 {
    let client = client.map(|c| c.to_raw()).unwrap_or(null_mut());

    unsafe {
        cef_sys::cef_browser_host_create_browser(
            &window_info.into_raw(),
            client,
            &url.into_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
        )
    }
}

/// Create a new BrowserView. The underlying cef_browser_t will not be created
/// until this view is added to the views hierarchy. The optional |extra_info|
/// parameter provides an opportunity to specify extra information specific to
/// the created browser that will be passed to
/// cef_render_process_handler_t::on_browser_created() in the render process.
#[derive(Debug, Clone)]
pub struct BrowserView(RefGuard<cef_browser_view_t>);

impl BrowserView {
    pub fn as_view(&self) -> View {
        unsafe { View(self.0.clone().convert()) }
    }
}

pub fn create_browser_view<T: Client>(
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
    // TODO delegate: *mut _cef_browser_view_delegate_t,
) -> BrowserView {
    let client = client.map(|c| c.to_raw()).unwrap_or(null_mut());

    let view = unsafe {
        cef_sys::cef_browser_view_create(
            client,
            &url.into_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };

    BrowserView(RefGuard::from_raw(view))
}
