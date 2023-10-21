use std::{ffi::c_int, ptr::null_mut};

use cef_sys::{
    cef_browser_host_create_browser, cef_browser_settings_t, cef_browser_view_create,
    cef_browser_view_t,
};

use crate::{client::Client, rc::RefGuard, string::CefString, window::WindowInfo, State, View};

/// See [cef_browser_settings_t] for more documentation.
#[derive(Debug, Clone)]
pub struct BrowserSettings {
    pub windowless_frame_rate: usize,
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
    pub default_encoding: CefString,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub webgl: State,
    pub background_color: u32,
    pub accept_language_list: CefString,
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

/// See [cef_browser_host_create_browser] for more documentation.
pub fn create_browser<T: Client>(
    window_info: WindowInfo,
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
) -> i32 {
    let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

    unsafe {
        cef_browser_host_create_browser(
            &window_info.into_raw(),
            client,
            &url.into_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
        )
    }
}

/// See [cef_browser_view_t] for more documentation.
#[derive(Debug, Clone)]
pub struct BrowserView(RefGuard<cef_browser_view_t>);

impl BrowserView {
    pub fn as_view(&self) -> View {
        unsafe { View(self.0.convert()) }
    }
}

/// See [cef_browser_view_create] for more documentation.
pub fn create_browser_view<T: Client>(
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
    // TODO delegate: *mut _cef_browser_view_delegate_t,
) -> BrowserView {
    let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

    let view = unsafe {
        cef_browser_view_create(
            client,
            &url.into_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };

    BrowserView(unsafe { RefGuard::from_raw(view) })
}
