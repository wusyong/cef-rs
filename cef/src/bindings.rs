#![allow(dead_code, non_camel_case_types, unused_variables)]
use crate::{
    rc::{ConvertParam, ConvertReturnValue, RcImpl, RefGuard},
    wrapper,
};
use cef_sys::*;

// Type aliases
pub type XDisplay = _XDisplay;
pub type XEvent = _XEvent;
pub type __time_t = ::std::os::raw::c_long;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint_least16_t = __uint16_t;
pub type AcceleratedPaintNativePixmapPlane = AcceleratedPaintNativePixmapPlaneInfo;
pub type Char = char16_t;
pub type Color = u32;
pub type CefString = CefStringUtf16;
pub type CefStringUserfree = CefStringUserfreeUtf16;
pub type CefStringUserfreeUtf16 = CefStringUtf16;
pub type CefStringUserfreeUtf8 = CefStringUtf8;
pub type CefStringUserfreeWide = CefStringWide;
pub type char16_t = __uint_least16_t;
pub type time_t = __time_t;
pub type wchar_t = ::std::os::raw::c_int;

// Struct wrappers

/// See [_cef_string_wide_t] for more documentation.
pub struct CefStringWide(_cef_string_wide_t);

impl From<_cef_string_wide_t> for CefStringWide {
    fn from(value: _cef_string_wide_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_wide_t> for CefStringWide {
    fn into(self) -> _cef_string_wide_t {
        self.0
    }
}

impl AsRef<_cef_string_wide_t> for CefStringWide {
    fn as_ref(&self) -> &_cef_string_wide_t {
        &self.0
    }
}

impl AsMut<_cef_string_wide_t> for CefStringWide {
    fn as_mut(&mut self) -> &mut _cef_string_wide_t {
        &mut self.0
    }
}

impl Default for CefStringWide {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_utf8_t] for more documentation.
pub struct CefStringUtf8(_cef_string_utf8_t);

impl From<_cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: _cef_string_utf8_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_utf8_t> for CefStringUtf8 {
    fn into(self) -> _cef_string_utf8_t {
        self.0
    }
}

impl AsRef<_cef_string_utf8_t> for CefStringUtf8 {
    fn as_ref(&self) -> &_cef_string_utf8_t {
        &self.0
    }
}

impl AsMut<_cef_string_utf8_t> for CefStringUtf8 {
    fn as_mut(&mut self) -> &mut _cef_string_utf8_t {
        &mut self.0
    }
}

impl Default for CefStringUtf8 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_utf16_t] for more documentation.
pub struct CefStringUtf16(_cef_string_utf16_t);

impl From<_cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: _cef_string_utf16_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_utf16_t> for CefStringUtf16 {
    fn into(self) -> _cef_string_utf16_t {
        self.0
    }
}

impl AsRef<_cef_string_utf16_t> for CefStringUtf16 {
    fn as_ref(&self) -> &_cef_string_utf16_t {
        &self.0
    }
}

impl AsMut<_cef_string_utf16_t> for CefStringUtf16 {
    fn as_mut(&mut self) -> &mut _cef_string_utf16_t {
        &mut self.0
    }
}

impl Default for CefStringUtf16 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_list_t] for more documentation.
pub struct CefStringList(_cef_string_list_t);

impl From<_cef_string_list_t> for CefStringList {
    fn from(value: _cef_string_list_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_list_t> for CefStringList {
    fn into(self) -> _cef_string_list_t {
        self.0
    }
}

impl AsRef<_cef_string_list_t> for CefStringList {
    fn as_ref(&self) -> &_cef_string_list_t {
        &self.0
    }
}

impl AsMut<_cef_string_list_t> for CefStringList {
    fn as_mut(&mut self) -> &mut _cef_string_list_t {
        &mut self.0
    }
}

impl Default for CefStringList {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_map_t] for more documentation.
pub struct CefStringMap(_cef_string_map_t);

impl From<_cef_string_map_t> for CefStringMap {
    fn from(value: _cef_string_map_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_map_t> for CefStringMap {
    fn into(self) -> _cef_string_map_t {
        self.0
    }
}

impl AsRef<_cef_string_map_t> for CefStringMap {
    fn as_ref(&self) -> &_cef_string_map_t {
        &self.0
    }
}

impl AsMut<_cef_string_map_t> for CefStringMap {
    fn as_mut(&mut self) -> &mut _cef_string_map_t {
        &mut self.0
    }
}

impl Default for CefStringMap {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_multimap_t] for more documentation.
pub struct CefStringMultimap(_cef_string_multimap_t);

impl From<_cef_string_multimap_t> for CefStringMultimap {
    fn from(value: _cef_string_multimap_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_string_multimap_t> for CefStringMultimap {
    fn into(self) -> _cef_string_multimap_t {
        self.0
    }
}

impl AsRef<_cef_string_multimap_t> for CefStringMultimap {
    fn as_ref(&self) -> &_cef_string_multimap_t {
        &self.0
    }
}

impl AsMut<_cef_string_multimap_t> for CefStringMultimap {
    fn as_mut(&mut self) -> &mut _cef_string_multimap_t {
        &mut self.0
    }
}

impl Default for CefStringMultimap {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_basetime_t] for more documentation.
pub struct Basetime {
    pub val: i64,
}

impl From<_cef_basetime_t> for Basetime {
    fn from(value: _cef_basetime_t) -> Self {
        Self {
            val: value.val.into(),
        }
    }
}

impl Into<_cef_basetime_t> for Basetime {
    fn into(self) -> _cef_basetime_t {
        _cef_basetime_t {
            val: self.val.into(),
        }
    }
}

impl Default for Basetime {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_time_t] for more documentation.
pub struct Time {
    pub year: ::std::os::raw::c_int,
    pub month: ::std::os::raw::c_int,
    pub day_of_week: ::std::os::raw::c_int,
    pub day_of_month: ::std::os::raw::c_int,
    pub hour: ::std::os::raw::c_int,
    pub minute: ::std::os::raw::c_int,
    pub second: ::std::os::raw::c_int,
    pub millisecond: ::std::os::raw::c_int,
}

impl From<_cef_time_t> for Time {
    fn from(value: _cef_time_t) -> Self {
        Self {
            year: value.year.into(),
            month: value.month.into(),
            day_of_week: value.day_of_week.into(),
            day_of_month: value.day_of_month.into(),
            hour: value.hour.into(),
            minute: value.minute.into(),
            second: value.second.into(),
            millisecond: value.millisecond.into(),
        }
    }
}

impl Into<_cef_time_t> for Time {
    fn into(self) -> _cef_time_t {
        _cef_time_t {
            year: self.year.into(),
            month: self.month.into(),
            day_of_week: self.day_of_week.into(),
            day_of_month: self.day_of_month.into(),
            hour: self.hour.into(),
            minute: self.minute.into(),
            second: self.second.into(),
            millisecond: self.millisecond.into(),
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_point_t] for more documentation.
pub struct Point {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}

impl From<_cef_point_t> for Point {
    fn from(value: _cef_point_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

impl Into<_cef_point_t> for Point {
    fn into(self) -> _cef_point_t {
        _cef_point_t {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_rect_t] for more documentation.
pub struct Rect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}

impl From<_cef_rect_t> for Rect {
    fn from(value: _cef_rect_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl Into<_cef_rect_t> for Rect {
    fn into(self) -> _cef_rect_t {
        _cef_rect_t {
            x: self.x.into(),
            y: self.y.into(),
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_size_t] for more documentation.
pub struct Size {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}

impl From<_cef_size_t> for Size {
    fn from(value: _cef_size_t) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl Into<_cef_size_t> for Size {
    fn into(self) -> _cef_size_t {
        _cef_size_t {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_insets_t] for more documentation.
pub struct Insets {
    pub top: ::std::os::raw::c_int,
    pub left: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
}

impl From<_cef_insets_t> for Insets {
    fn from(value: _cef_insets_t) -> Self {
        Self {
            top: value.top.into(),
            left: value.left.into(),
            bottom: value.bottom.into(),
            right: value.right.into(),
        }
    }
}

impl Into<_cef_insets_t> for Insets {
    fn into(self) -> _cef_insets_t {
        _cef_insets_t {
            top: self.top.into(),
            left: self.left.into(),
            bottom: self.bottom.into(),
            right: self.right.into(),
        }
    }
}

impl Default for Insets {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_main_args_t] for more documentation.
pub struct MainArgs {
    pub argc: ::std::os::raw::c_int,
    pub argv: *mut *mut ::std::os::raw::c_char,
}

impl From<_cef_main_args_t> for MainArgs {
    fn from(value: _cef_main_args_t) -> Self {
        Self {
            argc: value.argc.into(),
            argv: value.argv.into(),
        }
    }
}

impl Into<_cef_main_args_t> for MainArgs {
    fn into(self) -> _cef_main_args_t {
        _cef_main_args_t {
            argc: self.argc.into(),
            argv: self.argv.into(),
        }
    }
}

impl Default for MainArgs {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_window_info_t] for more documentation.
pub struct WindowInfo {
    pub window_name: CefString,
    pub bounds: Rect,
    pub parent_window: ::std::os::raw::c_ulong,
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    pub shared_texture_enabled: ::std::os::raw::c_int,
    pub external_begin_frame_enabled: ::std::os::raw::c_int,
    pub window: ::std::os::raw::c_ulong,
    pub runtime_style: RuntimeStyle,
}

impl From<_cef_window_info_t> for WindowInfo {
    fn from(value: _cef_window_info_t) -> Self {
        Self {
            window_name: value.window_name.into(),
            bounds: value.bounds.into(),
            parent_window: value.parent_window.into(),
            windowless_rendering_enabled: value.windowless_rendering_enabled.into(),
            shared_texture_enabled: value.shared_texture_enabled.into(),
            external_begin_frame_enabled: value.external_begin_frame_enabled.into(),
            window: value.window.into(),
            runtime_style: value.runtime_style.into(),
        }
    }
}

impl Into<_cef_window_info_t> for WindowInfo {
    fn into(self) -> _cef_window_info_t {
        _cef_window_info_t {
            window_name: self.window_name.into(),
            bounds: self.bounds.into(),
            parent_window: self.parent_window.into(),
            windowless_rendering_enabled: self.windowless_rendering_enabled.into(),
            shared_texture_enabled: self.shared_texture_enabled.into(),
            external_begin_frame_enabled: self.external_begin_frame_enabled.into(),
            window: self.window.into(),
            runtime_style: self.runtime_style.into(),
        }
    }
}

impl Default for WindowInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_accelerated_paint_native_pixmap_plane_info_t] for more documentation.
pub struct AcceleratedPaintNativePixmapPlaneInfo {
    pub stride: u32,
    pub offset: u64,
    pub size: u64,
    pub fd: ::std::os::raw::c_int,
}

impl From<_cef_accelerated_paint_native_pixmap_plane_info_t>
    for AcceleratedPaintNativePixmapPlaneInfo
{
    fn from(value: _cef_accelerated_paint_native_pixmap_plane_info_t) -> Self {
        Self {
            stride: value.stride.into(),
            offset: value.offset.into(),
            size: value.size.into(),
            fd: value.fd.into(),
        }
    }
}

impl Into<_cef_accelerated_paint_native_pixmap_plane_info_t>
    for AcceleratedPaintNativePixmapPlaneInfo
{
    fn into(self) -> _cef_accelerated_paint_native_pixmap_plane_info_t {
        _cef_accelerated_paint_native_pixmap_plane_info_t {
            stride: self.stride.into(),
            offset: self.offset.into(),
            size: self.size.into(),
            fd: self.fd.into(),
        }
    }
}

impl Default for AcceleratedPaintNativePixmapPlaneInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_accelerated_paint_info_t] for more documentation.
pub struct AcceleratedPaintInfo {
    pub planes: [AcceleratedPaintNativePixmapPlane; 4usize],
    pub plane_count: ::std::os::raw::c_int,
    pub modifier: u64,
    pub format: ColorType,
}

impl From<_cef_accelerated_paint_info_t> for AcceleratedPaintInfo {
    fn from(value: _cef_accelerated_paint_info_t) -> Self {
        Self {
            planes: value.planes.into(),
            plane_count: value.plane_count.into(),
            modifier: value.modifier.into(),
            format: value.format.into(),
        }
    }
}

impl Into<_cef_accelerated_paint_info_t> for AcceleratedPaintInfo {
    fn into(self) -> _cef_accelerated_paint_info_t {
        _cef_accelerated_paint_info_t {
            planes: self.planes.into(),
            plane_count: self.plane_count.into(),
            modifier: self.modifier.into(),
            format: self.format.into(),
        }
    }
}

impl Default for AcceleratedPaintInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_settings_t] for more documentation.
pub struct Settings {
    pub size: usize,
    pub no_sandbox: ::std::os::raw::c_int,
    pub browser_subprocess_path: CefString,
    pub framework_dir_path: CefString,
    pub main_bundle_path: CefString,
    pub multi_threaded_message_loop: ::std::os::raw::c_int,
    pub external_message_pump: ::std::os::raw::c_int,
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    pub command_line_args_disabled: ::std::os::raw::c_int,
    pub cache_path: CefString,
    pub root_cache_path: CefString,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub user_agent: CefString,
    pub user_agent_product: CefString,
    pub locale: CefString,
    pub log_file: CefString,
    pub log_severity: LogSeverity,
    pub log_items: LogItems,
    pub javascript_flags: CefString,
    pub resources_dir_path: CefString,
    pub locales_dir_path: CefString,
    pub remote_debugging_port: ::std::os::raw::c_int,
    pub uncaught_exception_stack_size: ::std::os::raw::c_int,
    pub background_color: Color,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
    pub chrome_policy_id: CefString,
    pub chrome_app_icon_id: ::std::os::raw::c_int,
    pub disable_signal_handlers: ::std::os::raw::c_int,
}

impl From<_cef_settings_t> for Settings {
    fn from(value: _cef_settings_t) -> Self {
        Self {
            size: value.size.into(),
            no_sandbox: value.no_sandbox.into(),
            browser_subprocess_path: value.browser_subprocess_path.into(),
            framework_dir_path: value.framework_dir_path.into(),
            main_bundle_path: value.main_bundle_path.into(),
            multi_threaded_message_loop: value.multi_threaded_message_loop.into(),
            external_message_pump: value.external_message_pump.into(),
            windowless_rendering_enabled: value.windowless_rendering_enabled.into(),
            command_line_args_disabled: value.command_line_args_disabled.into(),
            cache_path: value.cache_path.into(),
            root_cache_path: value.root_cache_path.into(),
            persist_session_cookies: value.persist_session_cookies.into(),
            user_agent: value.user_agent.into(),
            user_agent_product: value.user_agent_product.into(),
            locale: value.locale.into(),
            log_file: value.log_file.into(),
            log_severity: value.log_severity.into(),
            log_items: value.log_items.into(),
            javascript_flags: value.javascript_flags.into(),
            resources_dir_path: value.resources_dir_path.into(),
            locales_dir_path: value.locales_dir_path.into(),
            remote_debugging_port: value.remote_debugging_port.into(),
            uncaught_exception_stack_size: value.uncaught_exception_stack_size.into(),
            background_color: value.background_color.into(),
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults.into(),
            chrome_policy_id: value.chrome_policy_id.into(),
            chrome_app_icon_id: value.chrome_app_icon_id.into(),
            disable_signal_handlers: value.disable_signal_handlers.into(),
        }
    }
}

impl Into<_cef_settings_t> for Settings {
    fn into(self) -> _cef_settings_t {
        _cef_settings_t {
            size: self.size.into(),
            no_sandbox: self.no_sandbox.into(),
            browser_subprocess_path: self.browser_subprocess_path.into(),
            framework_dir_path: self.framework_dir_path.into(),
            main_bundle_path: self.main_bundle_path.into(),
            multi_threaded_message_loop: self.multi_threaded_message_loop.into(),
            external_message_pump: self.external_message_pump.into(),
            windowless_rendering_enabled: self.windowless_rendering_enabled.into(),
            command_line_args_disabled: self.command_line_args_disabled.into(),
            cache_path: self.cache_path.into(),
            root_cache_path: self.root_cache_path.into(),
            persist_session_cookies: self.persist_session_cookies.into(),
            user_agent: self.user_agent.into(),
            user_agent_product: self.user_agent_product.into(),
            locale: self.locale.into(),
            log_file: self.log_file.into(),
            log_severity: self.log_severity.into(),
            log_items: self.log_items.into(),
            javascript_flags: self.javascript_flags.into(),
            resources_dir_path: self.resources_dir_path.into(),
            locales_dir_path: self.locales_dir_path.into(),
            remote_debugging_port: self.remote_debugging_port.into(),
            uncaught_exception_stack_size: self.uncaught_exception_stack_size.into(),
            background_color: self.background_color.into(),
            accept_language_list: self.accept_language_list.into(),
            cookieable_schemes_list: self.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults.into(),
            chrome_policy_id: self.chrome_policy_id.into(),
            chrome_app_icon_id: self.chrome_app_icon_id.into(),
            disable_signal_handlers: self.disable_signal_handlers.into(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_request_context_settings_t] for more documentation.
pub struct RequestContextSettings {
    pub size: usize,
    pub cache_path: CefString,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
}

impl From<_cef_request_context_settings_t> for RequestContextSettings {
    fn from(value: _cef_request_context_settings_t) -> Self {
        Self {
            size: value.size.into(),
            cache_path: value.cache_path.into(),
            persist_session_cookies: value.persist_session_cookies.into(),
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults.into(),
        }
    }
}

impl Into<_cef_request_context_settings_t> for RequestContextSettings {
    fn into(self) -> _cef_request_context_settings_t {
        _cef_request_context_settings_t {
            size: self.size.into(),
            cache_path: self.cache_path.into(),
            persist_session_cookies: self.persist_session_cookies.into(),
            accept_language_list: self.accept_language_list.into(),
            cookieable_schemes_list: self.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults.into(),
        }
    }
}

impl Default for RequestContextSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_settings_t] for more documentation.
pub struct BrowserSettings {
    pub size: usize,
    pub windowless_frame_rate: ::std::os::raw::c_int,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: ::std::os::raw::c_int,
    pub default_fixed_font_size: ::std::os::raw::c_int,
    pub minimum_font_size: ::std::os::raw::c_int,
    pub minimum_logical_font_size: ::std::os::raw::c_int,
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
    pub background_color: Color,
    pub chrome_status_bubble: State,
    pub chrome_zoom_bubble: State,
}

impl From<_cef_browser_settings_t> for BrowserSettings {
    fn from(value: _cef_browser_settings_t) -> Self {
        Self {
            size: value.size.into(),
            windowless_frame_rate: value.windowless_frame_rate.into(),
            standard_font_family: value.standard_font_family.into(),
            fixed_font_family: value.fixed_font_family.into(),
            serif_font_family: value.serif_font_family.into(),
            sans_serif_font_family: value.sans_serif_font_family.into(),
            cursive_font_family: value.cursive_font_family.into(),
            fantasy_font_family: value.fantasy_font_family.into(),
            default_font_size: value.default_font_size.into(),
            default_fixed_font_size: value.default_fixed_font_size.into(),
            minimum_font_size: value.minimum_font_size.into(),
            minimum_logical_font_size: value.minimum_logical_font_size.into(),
            default_encoding: value.default_encoding.into(),
            remote_fonts: value.remote_fonts.into(),
            javascript: value.javascript.into(),
            javascript_close_windows: value.javascript_close_windows.into(),
            javascript_access_clipboard: value.javascript_access_clipboard.into(),
            javascript_dom_paste: value.javascript_dom_paste.into(),
            image_loading: value.image_loading.into(),
            image_shrink_standalone_to_fit: value.image_shrink_standalone_to_fit.into(),
            text_area_resize: value.text_area_resize.into(),
            tab_to_links: value.tab_to_links.into(),
            local_storage: value.local_storage.into(),
            databases: value.databases.into(),
            webgl: value.webgl.into(),
            background_color: value.background_color.into(),
            chrome_status_bubble: value.chrome_status_bubble.into(),
            chrome_zoom_bubble: value.chrome_zoom_bubble.into(),
        }
    }
}

impl Into<_cef_browser_settings_t> for BrowserSettings {
    fn into(self) -> _cef_browser_settings_t {
        _cef_browser_settings_t {
            size: self.size.into(),
            windowless_frame_rate: self.windowless_frame_rate.into(),
            standard_font_family: self.standard_font_family.into(),
            fixed_font_family: self.fixed_font_family.into(),
            serif_font_family: self.serif_font_family.into(),
            sans_serif_font_family: self.sans_serif_font_family.into(),
            cursive_font_family: self.cursive_font_family.into(),
            fantasy_font_family: self.fantasy_font_family.into(),
            default_font_size: self.default_font_size.into(),
            default_fixed_font_size: self.default_fixed_font_size.into(),
            minimum_font_size: self.minimum_font_size.into(),
            minimum_logical_font_size: self.minimum_logical_font_size.into(),
            default_encoding: self.default_encoding.into(),
            remote_fonts: self.remote_fonts.into(),
            javascript: self.javascript.into(),
            javascript_close_windows: self.javascript_close_windows.into(),
            javascript_access_clipboard: self.javascript_access_clipboard.into(),
            javascript_dom_paste: self.javascript_dom_paste.into(),
            image_loading: self.image_loading.into(),
            image_shrink_standalone_to_fit: self.image_shrink_standalone_to_fit.into(),
            text_area_resize: self.text_area_resize.into(),
            tab_to_links: self.tab_to_links.into(),
            local_storage: self.local_storage.into(),
            databases: self.databases.into(),
            webgl: self.webgl.into(),
            background_color: self.background_color.into(),
            chrome_status_bubble: self.chrome_status_bubble.into(),
            chrome_zoom_bubble: self.chrome_zoom_bubble.into(),
        }
    }
}

impl Default for BrowserSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_urlparts_t] for more documentation.
pub struct Urlparts {
    pub spec: CefString,
    pub scheme: CefString,
    pub username: CefString,
    pub password: CefString,
    pub host: CefString,
    pub port: CefString,
    pub origin: CefString,
    pub path: CefString,
    pub query: CefString,
    pub fragment: CefString,
}

impl From<_cef_urlparts_t> for Urlparts {
    fn from(value: _cef_urlparts_t) -> Self {
        Self {
            spec: value.spec.into(),
            scheme: value.scheme.into(),
            username: value.username.into(),
            password: value.password.into(),
            host: value.host.into(),
            port: value.port.into(),
            origin: value.origin.into(),
            path: value.path.into(),
            query: value.query.into(),
            fragment: value.fragment.into(),
        }
    }
}

impl Into<_cef_urlparts_t> for Urlparts {
    fn into(self) -> _cef_urlparts_t {
        _cef_urlparts_t {
            spec: self.spec.into(),
            scheme: self.scheme.into(),
            username: self.username.into(),
            password: self.password.into(),
            host: self.host.into(),
            port: self.port.into(),
            origin: self.origin.into(),
            path: self.path.into(),
            query: self.query.into(),
            fragment: self.fragment.into(),
        }
    }
}

impl Default for Urlparts {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_cookie_t] for more documentation.
pub struct Cookie {
    pub name: CefString,
    pub value: CefString,
    pub domain: CefString,
    pub path: CefString,
    pub secure: ::std::os::raw::c_int,
    pub httponly: ::std::os::raw::c_int,
    pub creation: Basetime,
    pub last_access: Basetime,
    pub has_expires: ::std::os::raw::c_int,
    pub expires: Basetime,
    pub same_site: CookieSameSite,
    pub priority: CookiePriority,
}

impl From<_cef_cookie_t> for Cookie {
    fn from(value: _cef_cookie_t) -> Self {
        Self {
            name: value.name.into(),
            value: value.value.into(),
            domain: value.domain.into(),
            path: value.path.into(),
            secure: value.secure.into(),
            httponly: value.httponly.into(),
            creation: value.creation.into(),
            last_access: value.last_access.into(),
            has_expires: value.has_expires.into(),
            expires: value.expires.into(),
            same_site: value.same_site.into(),
            priority: value.priority.into(),
        }
    }
}

impl Into<_cef_cookie_t> for Cookie {
    fn into(self) -> _cef_cookie_t {
        _cef_cookie_t {
            name: self.name.into(),
            value: self.value.into(),
            domain: self.domain.into(),
            path: self.path.into(),
            secure: self.secure.into(),
            httponly: self.httponly.into(),
            creation: self.creation.into(),
            last_access: self.last_access.into(),
            has_expires: self.has_expires.into(),
            expires: self.expires.into(),
            same_site: self.same_site.into(),
            priority: self.priority.into(),
        }
    }
}

impl Default for Cookie {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_draggable_region_t] for more documentation.
pub struct DraggableRegion {
    pub bounds: Rect,
    pub draggable: ::std::os::raw::c_int,
}

impl From<_cef_draggable_region_t> for DraggableRegion {
    fn from(value: _cef_draggable_region_t) -> Self {
        Self {
            bounds: value.bounds.into(),
            draggable: value.draggable.into(),
        }
    }
}

impl Into<_cef_draggable_region_t> for DraggableRegion {
    fn into(self) -> _cef_draggable_region_t {
        _cef_draggable_region_t {
            bounds: self.bounds.into(),
            draggable: self.draggable.into(),
        }
    }
}

impl Default for DraggableRegion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_screen_info_t] for more documentation.
pub struct ScreenInfo {
    pub device_scale_factor: f32,
    pub depth: ::std::os::raw::c_int,
    pub depth_per_component: ::std::os::raw::c_int,
    pub is_monochrome: ::std::os::raw::c_int,
    pub rect: Rect,
    pub available_rect: Rect,
}

impl From<_cef_screen_info_t> for ScreenInfo {
    fn from(value: _cef_screen_info_t) -> Self {
        Self {
            device_scale_factor: value.device_scale_factor.into(),
            depth: value.depth.into(),
            depth_per_component: value.depth_per_component.into(),
            is_monochrome: value.is_monochrome.into(),
            rect: value.rect.into(),
            available_rect: value.available_rect.into(),
        }
    }
}

impl Into<_cef_screen_info_t> for ScreenInfo {
    fn into(self) -> _cef_screen_info_t {
        _cef_screen_info_t {
            device_scale_factor: self.device_scale_factor.into(),
            depth: self.depth.into(),
            depth_per_component: self.depth_per_component.into(),
            is_monochrome: self.is_monochrome.into(),
            rect: self.rect.into(),
            available_rect: self.available_rect.into(),
        }
    }
}

impl Default for ScreenInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_linux_window_properties_t] for more documentation.
pub struct LinuxWindowProperties {
    pub wayland_app_id: CefString,
    pub wm_class_class: CefString,
    pub wm_class_name: CefString,
    pub wm_role_name: CefString,
}

impl From<_cef_linux_window_properties_t> for LinuxWindowProperties {
    fn from(value: _cef_linux_window_properties_t) -> Self {
        Self {
            wayland_app_id: value.wayland_app_id.into(),
            wm_class_class: value.wm_class_class.into(),
            wm_class_name: value.wm_class_name.into(),
            wm_role_name: value.wm_role_name.into(),
        }
    }
}

impl Into<_cef_linux_window_properties_t> for LinuxWindowProperties {
    fn into(self) -> _cef_linux_window_properties_t {
        _cef_linux_window_properties_t {
            wayland_app_id: self.wayland_app_id.into(),
            wm_class_class: self.wm_class_class.into(),
            wm_class_name: self.wm_class_name.into(),
            wm_role_name: self.wm_role_name.into(),
        }
    }
}

impl Default for LinuxWindowProperties {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_mouse_event_t] for more documentation.
pub struct MouseEvent {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub modifiers: u32,
}

impl From<_cef_mouse_event_t> for MouseEvent {
    fn from(value: _cef_mouse_event_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            modifiers: value.modifiers.into(),
        }
    }
}

impl Into<_cef_mouse_event_t> for MouseEvent {
    fn into(self) -> _cef_mouse_event_t {
        _cef_mouse_event_t {
            x: self.x.into(),
            y: self.y.into(),
            modifiers: self.modifiers.into(),
        }
    }
}

impl Default for MouseEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_touch_event_t] for more documentation.
pub struct TouchEvent {
    pub id: ::std::os::raw::c_int,
    pub x: f32,
    pub y: f32,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation_angle: f32,
    pub pressure: f32,
    pub type_: TouchEventType,
    pub modifiers: u32,
    pub pointer_type: PointerType,
}

impl From<_cef_touch_event_t> for TouchEvent {
    fn from(value: _cef_touch_event_t) -> Self {
        Self {
            id: value.id.into(),
            x: value.x.into(),
            y: value.y.into(),
            radius_x: value.radius_x.into(),
            radius_y: value.radius_y.into(),
            rotation_angle: value.rotation_angle.into(),
            pressure: value.pressure.into(),
            type_: value.type_.into(),
            modifiers: value.modifiers.into(),
            pointer_type: value.pointer_type.into(),
        }
    }
}

impl Into<_cef_touch_event_t> for TouchEvent {
    fn into(self) -> _cef_touch_event_t {
        _cef_touch_event_t {
            id: self.id.into(),
            x: self.x.into(),
            y: self.y.into(),
            radius_x: self.radius_x.into(),
            radius_y: self.radius_y.into(),
            rotation_angle: self.rotation_angle.into(),
            pressure: self.pressure.into(),
            type_: self.type_.into(),
            modifiers: self.modifiers.into(),
            pointer_type: self.pointer_type.into(),
        }
    }
}

impl Default for TouchEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_key_event_t] for more documentation.
pub struct KeyEvent {
    pub type_: KeyEventType,
    pub modifiers: u32,
    pub windows_key_code: ::std::os::raw::c_int,
    pub native_key_code: ::std::os::raw::c_int,
    pub is_system_key: ::std::os::raw::c_int,
    pub character: char16_t,
    pub unmodified_character: char16_t,
    pub focus_on_editable_field: ::std::os::raw::c_int,
}

impl From<_cef_key_event_t> for KeyEvent {
    fn from(value: _cef_key_event_t) -> Self {
        Self {
            type_: value.type_.into(),
            modifiers: value.modifiers.into(),
            windows_key_code: value.windows_key_code.into(),
            native_key_code: value.native_key_code.into(),
            is_system_key: value.is_system_key.into(),
            character: value.character.into(),
            unmodified_character: value.unmodified_character.into(),
            focus_on_editable_field: value.focus_on_editable_field.into(),
        }
    }
}

impl Into<_cef_key_event_t> for KeyEvent {
    fn into(self) -> _cef_key_event_t {
        _cef_key_event_t {
            type_: self.type_.into(),
            modifiers: self.modifiers.into(),
            windows_key_code: self.windows_key_code.into(),
            native_key_code: self.native_key_code.into(),
            is_system_key: self.is_system_key.into(),
            character: self.character.into(),
            unmodified_character: self.unmodified_character.into(),
            focus_on_editable_field: self.focus_on_editable_field.into(),
        }
    }
}

impl Default for KeyEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_popup_features_t] for more documentation.
pub struct PopupFeatures {
    pub x: ::std::os::raw::c_int,
    pub x_set: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub y_set: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub width_set: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub height_set: ::std::os::raw::c_int,
    pub is_popup: ::std::os::raw::c_int,
}

impl From<_cef_popup_features_t> for PopupFeatures {
    fn from(value: _cef_popup_features_t) -> Self {
        Self {
            x: value.x.into(),
            x_set: value.xSet.into(),
            y: value.y.into(),
            y_set: value.ySet.into(),
            width: value.width.into(),
            width_set: value.widthSet.into(),
            height: value.height.into(),
            height_set: value.heightSet.into(),
            is_popup: value.isPopup.into(),
        }
    }
}

impl Into<_cef_popup_features_t> for PopupFeatures {
    fn into(self) -> _cef_popup_features_t {
        _cef_popup_features_t {
            x: self.x.into(),
            xSet: self.x_set.into(),
            y: self.y.into(),
            ySet: self.y_set.into(),
            width: self.width.into(),
            widthSet: self.width_set.into(),
            height: self.height.into(),
            heightSet: self.height_set.into(),
            isPopup: self.is_popup.into(),
        }
    }
}

impl Default for PopupFeatures {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_cursor_info_t] for more documentation.
pub struct CursorInfo {
    pub hotspot: Point,
    pub image_scale_factor: f32,
    pub buffer: *mut ::std::os::raw::c_void,
    pub size: Size,
}

impl From<_cef_cursor_info_t> for CursorInfo {
    fn from(value: _cef_cursor_info_t) -> Self {
        Self {
            hotspot: value.hotspot.into(),
            image_scale_factor: value.image_scale_factor.into(),
            buffer: value.buffer.into(),
            size: value.size.into(),
        }
    }
}

impl Into<_cef_cursor_info_t> for CursorInfo {
    fn into(self) -> _cef_cursor_info_t {
        _cef_cursor_info_t {
            hotspot: self.hotspot.into(),
            image_scale_factor: self.image_scale_factor.into(),
            buffer: self.buffer.into(),
            size: self.size.into(),
        }
    }
}

impl Default for CursorInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_pdf_print_settings_t] for more documentation.
pub struct PdfPrintSettings {
    pub landscape: ::std::os::raw::c_int,
    pub print_background: ::std::os::raw::c_int,
    pub scale: f64,
    pub paper_width: f64,
    pub paper_height: f64,
    pub prefer_css_page_size: ::std::os::raw::c_int,
    pub margin_type: PdfPrintMarginType,
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub page_ranges: CefString,
    pub display_header_footer: ::std::os::raw::c_int,
    pub header_template: CefString,
    pub footer_template: CefString,
    pub generate_tagged_pdf: ::std::os::raw::c_int,
    pub generate_document_outline: ::std::os::raw::c_int,
}

impl From<_cef_pdf_print_settings_t> for PdfPrintSettings {
    fn from(value: _cef_pdf_print_settings_t) -> Self {
        Self {
            landscape: value.landscape.into(),
            print_background: value.print_background.into(),
            scale: value.scale.into(),
            paper_width: value.paper_width.into(),
            paper_height: value.paper_height.into(),
            prefer_css_page_size: value.prefer_css_page_size.into(),
            margin_type: value.margin_type.into(),
            margin_top: value.margin_top.into(),
            margin_right: value.margin_right.into(),
            margin_bottom: value.margin_bottom.into(),
            margin_left: value.margin_left.into(),
            page_ranges: value.page_ranges.into(),
            display_header_footer: value.display_header_footer.into(),
            header_template: value.header_template.into(),
            footer_template: value.footer_template.into(),
            generate_tagged_pdf: value.generate_tagged_pdf.into(),
            generate_document_outline: value.generate_document_outline.into(),
        }
    }
}

impl Into<_cef_pdf_print_settings_t> for PdfPrintSettings {
    fn into(self) -> _cef_pdf_print_settings_t {
        _cef_pdf_print_settings_t {
            landscape: self.landscape.into(),
            print_background: self.print_background.into(),
            scale: self.scale.into(),
            paper_width: self.paper_width.into(),
            paper_height: self.paper_height.into(),
            prefer_css_page_size: self.prefer_css_page_size.into(),
            margin_type: self.margin_type.into(),
            margin_top: self.margin_top.into(),
            margin_right: self.margin_right.into(),
            margin_bottom: self.margin_bottom.into(),
            margin_left: self.margin_left.into(),
            page_ranges: self.page_ranges.into(),
            display_header_footer: self.display_header_footer.into(),
            header_template: self.header_template.into(),
            footer_template: self.footer_template.into(),
            generate_tagged_pdf: self.generate_tagged_pdf.into(),
            generate_document_outline: self.generate_document_outline.into(),
        }
    }
}

impl Default for PdfPrintSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_box_layout_settings_t] for more documentation.
pub struct BoxLayoutSettings {
    pub horizontal: ::std::os::raw::c_int,
    pub inside_border_horizontal_spacing: ::std::os::raw::c_int,
    pub inside_border_vertical_spacing: ::std::os::raw::c_int,
    pub inside_border_insets: Insets,
    pub between_child_spacing: ::std::os::raw::c_int,
    pub main_axis_alignment: AxisAlignment,
    pub cross_axis_alignment: AxisAlignment,
    pub minimum_cross_axis_size: ::std::os::raw::c_int,
    pub default_flex: ::std::os::raw::c_int,
}

impl From<_cef_box_layout_settings_t> for BoxLayoutSettings {
    fn from(value: _cef_box_layout_settings_t) -> Self {
        Self {
            horizontal: value.horizontal.into(),
            inside_border_horizontal_spacing: value.inside_border_horizontal_spacing.into(),
            inside_border_vertical_spacing: value.inside_border_vertical_spacing.into(),
            inside_border_insets: value.inside_border_insets.into(),
            between_child_spacing: value.between_child_spacing.into(),
            main_axis_alignment: value.main_axis_alignment.into(),
            cross_axis_alignment: value.cross_axis_alignment.into(),
            minimum_cross_axis_size: value.minimum_cross_axis_size.into(),
            default_flex: value.default_flex.into(),
        }
    }
}

impl Into<_cef_box_layout_settings_t> for BoxLayoutSettings {
    fn into(self) -> _cef_box_layout_settings_t {
        _cef_box_layout_settings_t {
            horizontal: self.horizontal.into(),
            inside_border_horizontal_spacing: self.inside_border_horizontal_spacing.into(),
            inside_border_vertical_spacing: self.inside_border_vertical_spacing.into(),
            inside_border_insets: self.inside_border_insets.into(),
            between_child_spacing: self.between_child_spacing.into(),
            main_axis_alignment: self.main_axis_alignment.into(),
            cross_axis_alignment: self.cross_axis_alignment.into(),
            minimum_cross_axis_size: self.minimum_cross_axis_size.into(),
            default_flex: self.default_flex.into(),
        }
    }
}

impl Default for BoxLayoutSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_range_t] for more documentation.
pub struct Range {
    pub from: u32,
    pub to: u32,
}

impl From<_cef_range_t> for Range {
    fn from(value: _cef_range_t) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}

impl Into<_cef_range_t> for Range {
    fn into(self) -> _cef_range_t {
        _cef_range_t {
            from: self.from.into(),
            to: self.to.into(),
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_composition_underline_t] for more documentation.
pub struct CompositionUnderline {
    pub range: Range,
    pub color: Color,
    pub background_color: Color,
    pub thick: ::std::os::raw::c_int,
    pub style: CompositionUnderlineStyle,
}

impl From<_cef_composition_underline_t> for CompositionUnderline {
    fn from(value: _cef_composition_underline_t) -> Self {
        Self {
            range: value.range.into(),
            color: value.color.into(),
            background_color: value.background_color.into(),
            thick: value.thick.into(),
            style: value.style.into(),
        }
    }
}

impl Into<_cef_composition_underline_t> for CompositionUnderline {
    fn into(self) -> _cef_composition_underline_t {
        _cef_composition_underline_t {
            range: self.range.into(),
            color: self.color.into(),
            background_color: self.background_color.into(),
            thick: self.thick.into(),
            style: self.style.into(),
        }
    }
}

impl Default for CompositionUnderline {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_audio_parameters_t] for more documentation.
pub struct AudioParameters {
    pub channel_layout: ChannelLayout,
    pub sample_rate: ::std::os::raw::c_int,
    pub frames_per_buffer: ::std::os::raw::c_int,
}

impl From<_cef_audio_parameters_t> for AudioParameters {
    fn from(value: _cef_audio_parameters_t) -> Self {
        Self {
            channel_layout: value.channel_layout.into(),
            sample_rate: value.sample_rate.into(),
            frames_per_buffer: value.frames_per_buffer.into(),
        }
    }
}

impl Into<_cef_audio_parameters_t> for AudioParameters {
    fn into(self) -> _cef_audio_parameters_t {
        _cef_audio_parameters_t {
            channel_layout: self.channel_layout.into(),
            sample_rate: self.sample_rate.into(),
            frames_per_buffer: self.frames_per_buffer.into(),
        }
    }
}

impl Default for AudioParameters {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_sink_device_info_t] for more documentation.
pub struct MediaSinkDeviceInfo {
    pub ip_address: CefString,
    pub port: ::std::os::raw::c_int,
    pub model_name: CefString,
}

impl From<_cef_media_sink_device_info_t> for MediaSinkDeviceInfo {
    fn from(value: _cef_media_sink_device_info_t) -> Self {
        Self {
            ip_address: value.ip_address.into(),
            port: value.port.into(),
            model_name: value.model_name.into(),
        }
    }
}

impl Into<_cef_media_sink_device_info_t> for MediaSinkDeviceInfo {
    fn into(self) -> _cef_media_sink_device_info_t {
        _cef_media_sink_device_info_t {
            ip_address: self.ip_address.into(),
            port: self.port.into(),
            model_name: self.model_name.into(),
        }
    }
}

impl Default for MediaSinkDeviceInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_touch_handle_state_t] for more documentation.
pub struct TouchHandleState {
    pub touch_handle_id: ::std::os::raw::c_int,
    pub flags: u32,
    pub enabled: ::std::os::raw::c_int,
    pub orientation: HorizontalAlignment,
    pub mirror_vertical: ::std::os::raw::c_int,
    pub mirror_horizontal: ::std::os::raw::c_int,
    pub origin: Point,
    pub alpha: f32,
}

impl From<_cef_touch_handle_state_t> for TouchHandleState {
    fn from(value: _cef_touch_handle_state_t) -> Self {
        Self {
            touch_handle_id: value.touch_handle_id.into(),
            flags: value.flags.into(),
            enabled: value.enabled.into(),
            orientation: value.orientation.into(),
            mirror_vertical: value.mirror_vertical.into(),
            mirror_horizontal: value.mirror_horizontal.into(),
            origin: value.origin.into(),
            alpha: value.alpha.into(),
        }
    }
}

impl Into<_cef_touch_handle_state_t> for TouchHandleState {
    fn into(self) -> _cef_touch_handle_state_t {
        _cef_touch_handle_state_t {
            touch_handle_id: self.touch_handle_id.into(),
            flags: self.flags.into(),
            enabled: self.enabled.into(),
            orientation: self.orientation.into(),
            mirror_vertical: self.mirror_vertical.into(),
            mirror_horizontal: self.mirror_horizontal.into(),
            origin: self.origin.into(),
            alpha: self.alpha.into(),
        }
    }
}

impl Default for TouchHandleState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_task_info_t] for more documentation.
pub struct TaskInfo {
    pub id: i64,
    pub type_: TaskType,
    pub is_killable: ::std::os::raw::c_int,
    pub title: CefString,
    pub cpu_usage: f64,
    pub number_of_processors: ::std::os::raw::c_int,
    pub memory: i64,
    pub gpu_memory: i64,
    pub is_gpu_memory_inflated: ::std::os::raw::c_int,
}

impl From<_cef_task_info_t> for TaskInfo {
    fn from(value: _cef_task_info_t) -> Self {
        Self {
            id: value.id.into(),
            type_: value.type_.into(),
            is_killable: value.is_killable.into(),
            title: value.title.into(),
            cpu_usage: value.cpu_usage.into(),
            number_of_processors: value.number_of_processors.into(),
            memory: value.memory.into(),
            gpu_memory: value.gpu_memory.into(),
            is_gpu_memory_inflated: value.is_gpu_memory_inflated.into(),
        }
    }
}

impl Into<_cef_task_info_t> for TaskInfo {
    fn into(self) -> _cef_task_info_t {
        _cef_task_info_t {
            id: self.id.into(),
            type_: self.type_.into(),
            is_killable: self.is_killable.into(),
            title: self.title.into(),
            cpu_usage: self.cpu_usage.into(),
            number_of_processors: self.number_of_processors.into(),
            memory: self.memory.into(),
            gpu_memory: self.gpu_memory.into(),
            is_gpu_memory_inflated: self.is_gpu_memory_inflated.into(),
        }
    }
}

impl Default for TaskInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_base_ref_counted_t] for more documentation.
pub struct BaseRefCounted(_cef_base_ref_counted_t);

impl From<_cef_base_ref_counted_t> for BaseRefCounted {
    fn from(value: _cef_base_ref_counted_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_base_ref_counted_t> for BaseRefCounted {
    fn into(self) -> _cef_base_ref_counted_t {
        self.0
    }
}

impl AsRef<_cef_base_ref_counted_t> for BaseRefCounted {
    fn as_ref(&self) -> &_cef_base_ref_counted_t {
        &self.0
    }
}

impl AsMut<_cef_base_ref_counted_t> for BaseRefCounted {
    fn as_mut(&mut self) -> &mut _cef_base_ref_counted_t {
        &mut self.0
    }
}

impl Default for BaseRefCounted {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_base_scoped_t] for more documentation.
pub struct BaseScoped(_cef_base_scoped_t);

impl From<_cef_base_scoped_t> for BaseScoped {
    fn from(value: _cef_base_scoped_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_base_scoped_t> for BaseScoped {
    fn into(self) -> _cef_base_scoped_t {
        self.0
    }
}

impl AsRef<_cef_base_scoped_t> for BaseScoped {
    fn as_ref(&self) -> &_cef_base_scoped_t {
        &self.0
    }
}

impl AsMut<_cef_base_scoped_t> for BaseScoped {
    fn as_mut(&mut self) -> &mut _cef_base_scoped_t {
        &mut self.0
    }
}

impl Default for BaseScoped {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_dev_tools_message_observer_t] for more documentation."]
    #[derive(Clone)]
    pub struct DevToolsMessageObserver(_cef_dev_tools_message_observer_t);

    pub fn on_dev_tools_message(
        &self,
        browser: &mut Browser,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn on_dev_tools_method_result(
        &self,
        browser: &mut Browser,
        message_id: ::std::os::raw::c_int,
        success: ::std::os::raw::c_int,
        result: *const ::std::os::raw::c_void,
        result_size: usize,
    );
    pub fn on_dev_tools_event(
        &self,
        browser: &mut Browser,
        method: &CefString,
        params: *const ::std::os::raw::c_void,
        params_size: usize,
    );
    pub fn on_dev_tools_agent_attached(&self, browser: &mut Browser);
    pub fn on_dev_tools_agent_detached(&self, browser: &mut Browser);
);

pub trait ImplDevToolsMessageObserver: Sized {
    fn on_dev_tools_message(
        &self,
        browser: &mut Browser,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_dev_tools_method_result(
        &self,
        browser: &mut Browser,
        message_id: ::std::os::raw::c_int,
        success: ::std::os::raw::c_int,
        result: *const ::std::os::raw::c_void,
        result_size: usize,
    ) {
    }
    fn on_dev_tools_event(
        &self,
        browser: &mut Browser,
        method: &CefString,
        params: *const ::std::os::raw::c_void,
        params_size: usize,
    ) {
    }
    fn on_dev_tools_agent_attached(&self, browser: &mut Browser) {}
    fn on_dev_tools_agent_detached(&self, browser: &mut Browser) {}

    fn into_raw(self) -> *mut _cef_dev_tools_message_observer_t {
        let mut object: _cef_dev_tools_message_observer_t = unsafe { std::mem::zeroed() };
        impl_cef_dev_tools_message_observer_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_dev_tools_message_observer_t {
    use super::*;

    pub fn init_methods<I: ImplDevToolsMessageObserver>(
        object: &mut _cef_dev_tools_message_observer_t,
    ) {
        object.on_dev_tools_message = Some(on_dev_tools_message::<I>);
        object.on_dev_tools_method_result = Some(on_dev_tools_method_result::<I>);
        object.on_dev_tools_event = Some(on_dev_tools_event::<I>);
        object.on_dev_tools_agent_attached = Some(on_dev_tools_agent_attached::<I>);
        object.on_dev_tools_agent_detached = Some(on_dev_tools_agent_detached::<I>);
    }

    extern "C" fn on_dev_tools_message<I: ImplDevToolsMessageObserver>(
        self_: *mut _cef_dev_tools_message_observer_t,
        browser: *mut _cef_browser_t,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_dev_tools_message(browser, message, message_size)
            .into()
    }

    extern "C" fn on_dev_tools_method_result<I: ImplDevToolsMessageObserver>(
        self_: *mut _cef_dev_tools_message_observer_t,
        browser: *mut _cef_browser_t,
        message_id: ::std::os::raw::c_int,
        success: ::std::os::raw::c_int,
        result: *const ::std::os::raw::c_void,
        result_size: usize,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_dev_tools_method_result(browser, message_id, success, result, result_size)
    }

    extern "C" fn on_dev_tools_event<I: ImplDevToolsMessageObserver>(
        self_: *mut _cef_dev_tools_message_observer_t,
        browser: *mut _cef_browser_t,
        method: *const cef_string_t,
        params: *const ::std::os::raw::c_void,
        params_size: usize,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let method = &CefString::from(method);
        obj.interface
            .on_dev_tools_event(browser, method, params, params_size)
    }

    extern "C" fn on_dev_tools_agent_attached<I: ImplDevToolsMessageObserver>(
        self_: *mut _cef_dev_tools_message_observer_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_dev_tools_agent_attached(browser)
    }

    extern "C" fn on_dev_tools_agent_detached<I: ImplDevToolsMessageObserver>(
        self_: *mut _cef_dev_tools_message_observer_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_dev_tools_agent_detached(browser)
    }
}

wrapper!(
    #[doc = "See [_cef_value_t] for more documentation."]
    #[derive(Clone)]
    pub struct Value(_cef_value_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_owned(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut Value) -> ::std::os::raw::c_int;
    pub fn is_equal(&self, that: &mut Value) -> ::std::os::raw::c_int;
    pub fn copy(&self) -> Value;
    pub fn get_type(&self) -> ValueType;
    pub fn get_bool(&self) -> ::std::os::raw::c_int;
    pub fn get_int(&self) -> ::std::os::raw::c_int;
    pub fn get_double(&self) -> f64;
    pub fn get_string(&self) -> CefStringUserfree;
    pub fn get_binary(&self) -> BinaryValue;
    pub fn get_dictionary(&self) -> DictionaryValue;
    pub fn get_list(&self) -> ListValue;
    pub fn set_null(&self) -> ::std::os::raw::c_int;
    pub fn set_bool(&self, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_int(&self, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_double(&self, value: f64) -> ::std::os::raw::c_int;
    pub fn set_string(&self, value: &CefString) -> ::std::os::raw::c_int;
    pub fn set_binary(&self, value: &mut BinaryValue) -> ::std::os::raw::c_int;
    pub fn set_dictionary(&self, value: &mut DictionaryValue) -> ::std::os::raw::c_int;
    pub fn set_list(&self, value: &mut ListValue) -> ::std::os::raw::c_int;
);

pub trait ImplValue: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_owned(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut Value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_equal(&self, that: &mut Value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self) -> Value {
        Default::default()
    }
    fn get_type(&self) -> ValueType {
        Default::default()
    }
    fn get_bool(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_int(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_double(&self) -> f64 {
        Default::default()
    }
    fn get_string(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_binary(&self) -> BinaryValue {
        Default::default()
    }
    fn get_dictionary(&self) -> DictionaryValue {
        Default::default()
    }
    fn get_list(&self) -> ListValue {
        Default::default()
    }
    fn set_null(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_bool(&self, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_int(&self, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_double(&self, value: f64) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_string(&self, value: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_binary(&self, value: &mut BinaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_dictionary(&self, value: &mut DictionaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_list(&self, value: &mut ListValue) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_value_t {
        let mut object: _cef_value_t = unsafe { std::mem::zeroed() };
        impl_cef_value_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_value_t {
    use super::*;

    pub fn init_methods<I: ImplValue>(object: &mut _cef_value_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_owned = Some(is_owned::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.is_same = Some(is_same::<I>);
        object.is_equal = Some(is_equal::<I>);
        object.copy = Some(copy::<I>);
        object.get_type = Some(get_type::<I>);
        object.get_bool = Some(get_bool::<I>);
        object.get_int = Some(get_int::<I>);
        object.get_double = Some(get_double::<I>);
        object.get_string = Some(get_string::<I>);
        object.get_binary = Some(get_binary::<I>);
        object.get_dictionary = Some(get_dictionary::<I>);
        object.get_list = Some(get_list::<I>);
        object.set_null = Some(set_null::<I>);
        object.set_bool = Some(set_bool::<I>);
        object.set_int = Some(set_int::<I>);
        object.set_double = Some(set_double::<I>);
        object.set_string = Some(set_string::<I>);
        object.set_binary = Some(set_binary::<I>);
        object.set_dictionary = Some(set_dictionary::<I>);
        object.set_list = Some(set_list::<I>);
    }

    extern "C" fn is_valid<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_owned<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_owned().into()
    }

    extern "C" fn is_read_only<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn is_same<I: ImplValue>(
        self_: *mut _cef_value_t,
        that: *mut _cef_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut Value(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn is_equal<I: ImplValue>(
        self_: *mut _cef_value_t,
        that: *mut _cef_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut Value(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_equal(that).into()
    }

    extern "C" fn copy<I: ImplValue>(self_: *mut _cef_value_t) -> *mut _cef_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy().into()
    }

    extern "C" fn get_type<I: ImplValue>(self_: *mut _cef_value_t) -> cef_value_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type().into()
    }

    extern "C" fn get_bool<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bool().into()
    }

    extern "C" fn get_int<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_int().into()
    }

    extern "C" fn get_double<I: ImplValue>(self_: *mut _cef_value_t) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_double().into()
    }

    extern "C" fn get_string<I: ImplValue>(self_: *mut _cef_value_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_string().into()
    }

    extern "C" fn get_binary<I: ImplValue>(self_: *mut _cef_value_t) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_binary().into()
    }

    extern "C" fn get_dictionary<I: ImplValue>(
        self_: *mut _cef_value_t,
    ) -> *mut _cef_dictionary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_dictionary().into()
    }

    extern "C" fn get_list<I: ImplValue>(self_: *mut _cef_value_t) -> *mut _cef_list_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_list().into()
    }

    extern "C" fn set_null<I: ImplValue>(self_: *mut _cef_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_null().into()
    }

    extern "C" fn set_bool<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_bool(value).into()
    }

    extern "C" fn set_int<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_int(value).into()
    }

    extern "C" fn set_double<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: f64,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_double(value).into()
    }

    extern "C" fn set_string<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &CefString::from(value);
        obj.interface.set_string(value).into()
    }

    extern "C" fn set_binary<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut BinaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_binary(value).into()
    }

    extern "C" fn set_dictionary<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_dictionary(value).into()
    }

    extern "C" fn set_list<I: ImplValue>(
        self_: *mut _cef_value_t,
        value: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut ListValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_list(value).into()
    }
}

wrapper!(
    #[doc = "See [_cef_binary_value_t] for more documentation."]
    #[derive(Clone)]
    pub struct BinaryValue(_cef_binary_value_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_owned(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut BinaryValue) -> ::std::os::raw::c_int;
    pub fn is_equal(&self, that: &mut BinaryValue) -> ::std::os::raw::c_int;
    pub fn copy(&self) -> BinaryValue;
    pub fn get_raw_data(&self) -> *const ::std::os::raw::c_void;
    pub fn get_size(&self) -> usize;
    pub fn get_data(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        data_offset: usize,
    ) -> usize;
);

pub trait ImplBinaryValue: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_owned(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut BinaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_equal(&self, that: &mut BinaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self) -> BinaryValue {
        Default::default()
    }
    fn get_raw_data(&self) -> *const ::std::os::raw::c_void {
        Default::default()
    }
    fn get_size(&self) -> usize {
        Default::default()
    }
    fn get_data(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        data_offset: usize,
    ) -> usize {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_binary_value_t {
        let mut object: _cef_binary_value_t = unsafe { std::mem::zeroed() };
        impl_cef_binary_value_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_binary_value_t {
    use super::*;

    pub fn init_methods<I: ImplBinaryValue>(object: &mut _cef_binary_value_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_owned = Some(is_owned::<I>);
        object.is_same = Some(is_same::<I>);
        object.is_equal = Some(is_equal::<I>);
        object.copy = Some(copy::<I>);
        object.get_raw_data = Some(get_raw_data::<I>);
        object.get_size = Some(get_size::<I>);
        object.get_data = Some(get_data::<I>);
    }

    extern "C" fn is_valid<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_owned<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_owned().into()
    }

    extern "C" fn is_same<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
        that: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut BinaryValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn is_equal<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
        that: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut BinaryValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_equal(that).into()
    }

    extern "C" fn copy<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy().into()
    }

    extern "C" fn get_raw_data<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
    ) -> *const ::std::os::raw::c_void {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_raw_data().into()
    }

    extern "C" fn get_size<I: ImplBinaryValue>(self_: *mut _cef_binary_value_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_size().into()
    }

    extern "C" fn get_data<I: ImplBinaryValue>(
        self_: *mut _cef_binary_value_t,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        data_offset: usize,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_data(buffer, buffer_size, data_offset)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_dictionary_value_t] for more documentation."]
    #[derive(Clone)]
    pub struct DictionaryValue(_cef_dictionary_value_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_owned(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut DictionaryValue) -> ::std::os::raw::c_int;
    pub fn is_equal(&self, that: &mut DictionaryValue) -> ::std::os::raw::c_int;
    pub fn copy(&self, exclude_empty_children: ::std::os::raw::c_int) -> DictionaryValue;
    pub fn get_size(&self) -> usize;
    pub fn clear(&self) -> ::std::os::raw::c_int;
    pub fn has_key(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn get_keys(&self, keys: &mut CefStringList) -> ::std::os::raw::c_int;
    pub fn remove(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn get_type(&self, key: &CefString) -> ValueType;
    pub fn get_value(&self, key: &CefString) -> Value;
    pub fn get_bool(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn get_int(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn get_double(&self, key: &CefString) -> f64;
    pub fn get_string(&self, key: &CefString) -> CefStringUserfree;
    pub fn get_binary(&self, key: &CefString) -> BinaryValue;
    pub fn get_dictionary(&self, key: &CefString) -> DictionaryValue;
    pub fn get_list(&self, key: &CefString) -> ListValue;
    pub fn set_value(&self, key: &CefString, value: &mut Value) -> ::std::os::raw::c_int;
    pub fn set_null(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn set_bool(&self, key: &CefString, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_int(&self, key: &CefString, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_double(&self, key: &CefString, value: f64) -> ::std::os::raw::c_int;
    pub fn set_string(&self, key: &CefString, value: &CefString) -> ::std::os::raw::c_int;
    pub fn set_binary(&self, key: &CefString, value: &mut BinaryValue) -> ::std::os::raw::c_int;
    pub fn set_dictionary(
        &self,
        key: &CefString,
        value: &mut DictionaryValue,
    ) -> ::std::os::raw::c_int;
    pub fn set_list(&self, key: &CefString, value: &mut ListValue) -> ::std::os::raw::c_int;
);

pub trait ImplDictionaryValue: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_owned(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut DictionaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_equal(&self, that: &mut DictionaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self, exclude_empty_children: ::std::os::raw::c_int) -> DictionaryValue {
        Default::default()
    }
    fn get_size(&self) -> usize {
        Default::default()
    }
    fn clear(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_key(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_keys(&self, keys: &mut CefStringList) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_type(&self, key: &CefString) -> ValueType {
        Default::default()
    }
    fn get_value(&self, key: &CefString) -> Value {
        Default::default()
    }
    fn get_bool(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_int(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_double(&self, key: &CefString) -> f64 {
        Default::default()
    }
    fn get_string(&self, key: &CefString) -> CefStringUserfree {
        Default::default()
    }
    fn get_binary(&self, key: &CefString) -> BinaryValue {
        Default::default()
    }
    fn get_dictionary(&self, key: &CefString) -> DictionaryValue {
        Default::default()
    }
    fn get_list(&self, key: &CefString) -> ListValue {
        Default::default()
    }
    fn set_value(&self, key: &CefString, value: &mut Value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_null(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_bool(&self, key: &CefString, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_int(&self, key: &CefString, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_double(&self, key: &CefString, value: f64) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_string(&self, key: &CefString, value: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_binary(&self, key: &CefString, value: &mut BinaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_dictionary(
        &self,
        key: &CefString,
        value: &mut DictionaryValue,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_list(&self, key: &CefString, value: &mut ListValue) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_dictionary_value_t {
        let mut object: _cef_dictionary_value_t = unsafe { std::mem::zeroed() };
        impl_cef_dictionary_value_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_dictionary_value_t {
    use super::*;

    pub fn init_methods<I: ImplDictionaryValue>(object: &mut _cef_dictionary_value_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_owned = Some(is_owned::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.is_same = Some(is_same::<I>);
        object.is_equal = Some(is_equal::<I>);
        object.copy = Some(copy::<I>);
        object.get_size = Some(get_size::<I>);
        object.clear = Some(clear::<I>);
        object.has_key = Some(has_key::<I>);
        object.get_keys = Some(get_keys::<I>);
        object.remove = Some(remove::<I>);
        object.get_type = Some(get_type::<I>);
        object.get_value = Some(get_value::<I>);
        object.get_bool = Some(get_bool::<I>);
        object.get_int = Some(get_int::<I>);
        object.get_double = Some(get_double::<I>);
        object.get_string = Some(get_string::<I>);
        object.get_binary = Some(get_binary::<I>);
        object.get_dictionary = Some(get_dictionary::<I>);
        object.get_list = Some(get_list::<I>);
        object.set_value = Some(set_value::<I>);
        object.set_null = Some(set_null::<I>);
        object.set_bool = Some(set_bool::<I>);
        object.set_int = Some(set_int::<I>);
        object.set_double = Some(set_double::<I>);
        object.set_string = Some(set_string::<I>);
        object.set_binary = Some(set_binary::<I>);
        object.set_dictionary = Some(set_dictionary::<I>);
        object.set_list = Some(set_list::<I>);
    }

    extern "C" fn is_valid<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_owned<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_owned().into()
    }

    extern "C" fn is_read_only<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn is_same<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        that: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn is_equal<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        that: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_equal(that).into()
    }

    extern "C" fn copy<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        exclude_empty_children: ::std::os::raw::c_int,
    ) -> *mut _cef_dictionary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy(exclude_empty_children).into()
    }

    extern "C" fn get_size<I: ImplDictionaryValue>(self_: *mut _cef_dictionary_value_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_size().into()
    }

    extern "C" fn clear<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear().into()
    }

    extern "C" fn has_key<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.has_key(key).into()
    }

    extern "C" fn get_keys<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        keys: cef_string_list_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let keys = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(keys) });
        obj.interface.get_keys(keys).into()
    }

    extern "C" fn remove<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.remove(key).into()
    }

    extern "C" fn get_type<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> cef_value_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_type(key).into()
    }

    extern "C" fn get_value<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> *mut _cef_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_value(key).into()
    }

    extern "C" fn get_bool<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_bool(key).into()
    }

    extern "C" fn get_int<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_int(key).into()
    }

    extern "C" fn get_double<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_double(key).into()
    }

    extern "C" fn get_string<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_string(key).into()
    }

    extern "C" fn get_binary<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_binary(key).into()
    }

    extern "C" fn get_dictionary<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> *mut _cef_dictionary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_dictionary(key).into()
    }

    extern "C" fn get_list<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> *mut _cef_list_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_list(key).into()
    }

    extern "C" fn set_value<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: *mut _cef_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_value(key, value).into()
    }

    extern "C" fn set_null<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.set_null(key).into()
    }

    extern "C" fn set_bool<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.set_bool(key, value).into()
    }

    extern "C" fn set_int<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.set_int(key, value).into()
    }

    extern "C" fn set_double<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: f64,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.set_double(key, value).into()
    }

    extern "C" fn set_string<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &CefString::from(value);
        obj.interface.set_string(key, value).into()
    }

    extern "C" fn set_binary<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &mut BinaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_binary(key, value).into()
    }

    extern "C" fn set_dictionary<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_dictionary(key, value).into()
    }

    extern "C" fn set_list<I: ImplDictionaryValue>(
        self_: *mut _cef_dictionary_value_t,
        key: *const cef_string_t,
        value: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &mut ListValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_list(key, value).into()
    }
}

wrapper!(
    #[doc = "See [_cef_list_value_t] for more documentation."]
    #[derive(Clone)]
    pub struct ListValue(_cef_list_value_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_owned(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut ListValue) -> ::std::os::raw::c_int;
    pub fn is_equal(&self, that: &mut ListValue) -> ::std::os::raw::c_int;
    pub fn copy(&self) -> ListValue;
    pub fn set_size(&self, size: usize) -> ::std::os::raw::c_int;
    pub fn get_size(&self) -> usize;
    pub fn clear(&self) -> ::std::os::raw::c_int;
    pub fn remove(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn get_type(&self, index: usize) -> ValueType;
    pub fn get_value(&self, index: usize) -> Value;
    pub fn get_bool(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn get_int(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn get_double(&self, index: usize) -> f64;
    pub fn get_string(&self, index: usize) -> CefStringUserfree;
    pub fn get_binary(&self, index: usize) -> BinaryValue;
    pub fn get_dictionary(&self, index: usize) -> DictionaryValue;
    pub fn get_list(&self, index: usize) -> ListValue;
    pub fn set_value(&self, index: usize, value: &mut Value) -> ::std::os::raw::c_int;
    pub fn set_null(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_bool(&self, index: usize, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_int(&self, index: usize, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn set_double(&self, index: usize, value: f64) -> ::std::os::raw::c_int;
    pub fn set_string(&self, index: usize, value: &CefString) -> ::std::os::raw::c_int;
    pub fn set_binary(&self, index: usize, value: &mut BinaryValue) -> ::std::os::raw::c_int;
    pub fn set_dictionary(
        &self,
        index: usize,
        value: &mut DictionaryValue,
    ) -> ::std::os::raw::c_int;
    pub fn set_list(&self, index: usize, value: &mut ListValue) -> ::std::os::raw::c_int;
);

pub trait ImplListValue: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_owned(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut ListValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_equal(&self, that: &mut ListValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self) -> ListValue {
        Default::default()
    }
    fn set_size(&self, size: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_size(&self) -> usize {
        Default::default()
    }
    fn clear(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_type(&self, index: usize) -> ValueType {
        Default::default()
    }
    fn get_value(&self, index: usize) -> Value {
        Default::default()
    }
    fn get_bool(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_int(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_double(&self, index: usize) -> f64 {
        Default::default()
    }
    fn get_string(&self, index: usize) -> CefStringUserfree {
        Default::default()
    }
    fn get_binary(&self, index: usize) -> BinaryValue {
        Default::default()
    }
    fn get_dictionary(&self, index: usize) -> DictionaryValue {
        Default::default()
    }
    fn get_list(&self, index: usize) -> ListValue {
        Default::default()
    }
    fn set_value(&self, index: usize, value: &mut Value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_null(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_bool(&self, index: usize, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_int(&self, index: usize, value: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_double(&self, index: usize, value: f64) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_string(&self, index: usize, value: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_binary(&self, index: usize, value: &mut BinaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_dictionary(&self, index: usize, value: &mut DictionaryValue) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_list(&self, index: usize, value: &mut ListValue) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_list_value_t {
        let mut object: _cef_list_value_t = unsafe { std::mem::zeroed() };
        impl_cef_list_value_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_list_value_t {
    use super::*;

    pub fn init_methods<I: ImplListValue>(object: &mut _cef_list_value_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_owned = Some(is_owned::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.is_same = Some(is_same::<I>);
        object.is_equal = Some(is_equal::<I>);
        object.copy = Some(copy::<I>);
        object.set_size = Some(set_size::<I>);
        object.get_size = Some(get_size::<I>);
        object.clear = Some(clear::<I>);
        object.remove = Some(remove::<I>);
        object.get_type = Some(get_type::<I>);
        object.get_value = Some(get_value::<I>);
        object.get_bool = Some(get_bool::<I>);
        object.get_int = Some(get_int::<I>);
        object.get_double = Some(get_double::<I>);
        object.get_string = Some(get_string::<I>);
        object.get_binary = Some(get_binary::<I>);
        object.get_dictionary = Some(get_dictionary::<I>);
        object.get_list = Some(get_list::<I>);
        object.set_value = Some(set_value::<I>);
        object.set_null = Some(set_null::<I>);
        object.set_bool = Some(set_bool::<I>);
        object.set_int = Some(set_int::<I>);
        object.set_double = Some(set_double::<I>);
        object.set_string = Some(set_string::<I>);
        object.set_binary = Some(set_binary::<I>);
        object.set_dictionary = Some(set_dictionary::<I>);
        object.set_list = Some(set_list::<I>);
    }

    extern "C" fn is_valid<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_owned<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_owned().into()
    }

    extern "C" fn is_read_only<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn is_same<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        that: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut ListValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn is_equal<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        that: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut ListValue(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_equal(that).into()
    }

    extern "C" fn copy<I: ImplListValue>(self_: *mut _cef_list_value_t) -> *mut _cef_list_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy().into()
    }

    extern "C" fn set_size<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_size(size).into()
    }

    extern "C" fn get_size<I: ImplListValue>(self_: *mut _cef_list_value_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_size().into()
    }

    extern "C" fn clear<I: ImplListValue>(self_: *mut _cef_list_value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear().into()
    }

    extern "C" fn remove<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove(index).into()
    }

    extern "C" fn get_type<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> cef_value_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type(index).into()
    }

    extern "C" fn get_value<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> *mut _cef_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_value(index).into()
    }

    extern "C" fn get_bool<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bool(index).into()
    }

    extern "C" fn get_int<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_int(index).into()
    }

    extern "C" fn get_double<I: ImplListValue>(self_: *mut _cef_list_value_t, index: usize) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_double(index).into()
    }

    extern "C" fn get_string<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_string(index).into()
    }

    extern "C" fn get_binary<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_binary(index).into()
    }

    extern "C" fn get_dictionary<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> *mut _cef_dictionary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_dictionary(index).into()
    }

    extern "C" fn get_list<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> *mut _cef_list_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_list(index).into()
    }

    extern "C" fn set_value<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: *mut _cef_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_value(index, value).into()
    }

    extern "C" fn set_null<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_null(index).into()
    }

    extern "C" fn set_bool<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_bool(index, value).into()
    }

    extern "C" fn set_int<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_int(index, value).into()
    }

    extern "C" fn set_double<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: f64,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_double(index, value).into()
    }

    extern "C" fn set_string<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &CefString::from(value);
        obj.interface.set_string(index, value).into()
    }

    extern "C" fn set_binary<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: *mut _cef_binary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut BinaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_binary(index, value).into()
    }

    extern "C" fn set_dictionary<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_dictionary(index, value).into()
    }

    extern "C" fn set_list<I: ImplListValue>(
        self_: *mut _cef_list_value_t,
        index: usize,
        value: *mut _cef_list_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut ListValue(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_list(index, value).into()
    }
}

wrapper!(
    #[doc = "See [_cef_image_t] for more documentation."]
    #[derive(Clone)]
    pub struct Image(_cef_image_t);

    pub fn is_empty(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut Image) -> ::std::os::raw::c_int;
    pub fn add_bitmap(
        &self,
        scale_factor: f32,
        pixel_width: ::std::os::raw::c_int,
        pixel_height: ::std::os::raw::c_int,
        color_type: ColorType,
        alpha_type: AlphaType,
        pixel_data: *const ::std::os::raw::c_void,
        pixel_data_size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn add_png(
        &self,
        scale_factor: f32,
        png_data: *const ::std::os::raw::c_void,
        png_data_size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn add_jpeg(
        &self,
        scale_factor: f32,
        jpeg_data: *const ::std::os::raw::c_void,
        jpeg_data_size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn get_width(&self) -> usize;
    pub fn get_height(&self) -> usize;
    pub fn has_representation(&self, scale_factor: f32) -> ::std::os::raw::c_int;
    pub fn remove_representation(&self, scale_factor: f32) -> ::std::os::raw::c_int;
    pub fn get_representation_info(
        &self,
        scale_factor: f32,
        actual_scale_factor: *mut f32,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_as_bitmap(
        &self,
        scale_factor: f32,
        color_type: ColorType,
        alpha_type: AlphaType,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue;
    pub fn get_as_png(
        &self,
        scale_factor: f32,
        with_transparency: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue;
    pub fn get_as_jpeg(
        &self,
        scale_factor: f32,
        quality: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue;
);

pub trait ImplImage: Sized {
    fn is_empty(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut Image) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_bitmap(
        &self,
        scale_factor: f32,
        pixel_width: ::std::os::raw::c_int,
        pixel_height: ::std::os::raw::c_int,
        color_type: ColorType,
        alpha_type: AlphaType,
        pixel_data: *const ::std::os::raw::c_void,
        pixel_data_size: usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_png(
        &self,
        scale_factor: f32,
        png_data: *const ::std::os::raw::c_void,
        png_data_size: usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_jpeg(
        &self,
        scale_factor: f32,
        jpeg_data: *const ::std::os::raw::c_void,
        jpeg_data_size: usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_width(&self) -> usize {
        Default::default()
    }
    fn get_height(&self) -> usize {
        Default::default()
    }
    fn has_representation(&self, scale_factor: f32) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove_representation(&self, scale_factor: f32) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_representation_info(
        &self,
        scale_factor: f32,
        actual_scale_factor: *mut f32,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_as_bitmap(
        &self,
        scale_factor: f32,
        color_type: ColorType,
        alpha_type: AlphaType,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue {
        Default::default()
    }
    fn get_as_png(
        &self,
        scale_factor: f32,
        with_transparency: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue {
        Default::default()
    }
    fn get_as_jpeg(
        &self,
        scale_factor: f32,
        quality: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> BinaryValue {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_image_t {
        let mut object: _cef_image_t = unsafe { std::mem::zeroed() };
        impl_cef_image_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_image_t {
    use super::*;

    pub fn init_methods<I: ImplImage>(object: &mut _cef_image_t) {
        object.is_empty = Some(is_empty::<I>);
        object.is_same = Some(is_same::<I>);
        object.add_bitmap = Some(add_bitmap::<I>);
        object.add_png = Some(add_png::<I>);
        object.add_jpeg = Some(add_jpeg::<I>);
        object.get_width = Some(get_width::<I>);
        object.get_height = Some(get_height::<I>);
        object.has_representation = Some(has_representation::<I>);
        object.remove_representation = Some(remove_representation::<I>);
        object.get_representation_info = Some(get_representation_info::<I>);
        object.get_as_bitmap = Some(get_as_bitmap::<I>);
        object.get_as_png = Some(get_as_png::<I>);
        object.get_as_jpeg = Some(get_as_jpeg::<I>);
    }

    extern "C" fn is_empty<I: ImplImage>(self_: *mut _cef_image_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_empty().into()
    }

    extern "C" fn is_same<I: ImplImage>(
        self_: *mut _cef_image_t,
        that: *mut _cef_image_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut Image(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn add_bitmap<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        pixel_width: ::std::os::raw::c_int,
        pixel_height: ::std::os::raw::c_int,
        color_type: cef_color_type_t,
        alpha_type: cef_alpha_type_t,
        pixel_data: *const ::std::os::raw::c_void,
        pixel_data_size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .add_bitmap(
                scale_factor,
                pixel_width,
                pixel_height,
                color_type,
                alpha_type,
                pixel_data,
                pixel_data_size,
            )
            .into()
    }

    extern "C" fn add_png<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        png_data: *const ::std::os::raw::c_void,
        png_data_size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .add_png(scale_factor, png_data, png_data_size)
            .into()
    }

    extern "C" fn add_jpeg<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        jpeg_data: *const ::std::os::raw::c_void,
        jpeg_data_size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .add_jpeg(scale_factor, jpeg_data, jpeg_data_size)
            .into()
    }

    extern "C" fn get_width<I: ImplImage>(self_: *mut _cef_image_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_width().into()
    }

    extern "C" fn get_height<I: ImplImage>(self_: *mut _cef_image_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_height().into()
    }

    extern "C" fn has_representation<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_representation(scale_factor).into()
    }

    extern "C" fn remove_representation<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_representation(scale_factor).into()
    }

    extern "C" fn get_representation_info<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        actual_scale_factor: *mut f32,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_representation_info(scale_factor, actual_scale_factor, pixel_width, pixel_height)
            .into()
    }

    extern "C" fn get_as_bitmap<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        color_type: cef_color_type_t,
        alpha_type: cef_alpha_type_t,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_as_bitmap(
                scale_factor,
                color_type,
                alpha_type,
                pixel_width,
                pixel_height,
            )
            .into()
    }

    extern "C" fn get_as_png<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        with_transparency: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_as_png(scale_factor, with_transparency, pixel_width, pixel_height)
            .into()
    }

    extern "C" fn get_as_jpeg<I: ImplImage>(
        self_: *mut _cef_image_t,
        scale_factor: f32,
        quality: ::std::os::raw::c_int,
        pixel_width: *mut ::std::os::raw::c_int,
        pixel_height: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_as_jpeg(scale_factor, quality, pixel_width, pixel_height)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_read_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct ReadHandler(_cef_read_handler_t);

    pub fn read(&self, ptr: *mut ::std::os::raw::c_void, size: usize, n: usize) -> usize;
    pub fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tell(&self) -> i64;
    pub fn eof(&self) -> ::std::os::raw::c_int;
    pub fn may_block(&self) -> ::std::os::raw::c_int;
);

pub trait ImplReadHandler: Sized {
    fn read(&self, ptr: *mut ::std::os::raw::c_void, size: usize, n: usize) -> usize {
        Default::default()
    }
    fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn tell(&self) -> i64 {
        Default::default()
    }
    fn eof(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn may_block(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_read_handler_t {
        let mut object: _cef_read_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_read_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_read_handler_t {
    use super::*;

    pub fn init_methods<I: ImplReadHandler>(object: &mut _cef_read_handler_t) {
        object.read = Some(read::<I>);
        object.seek = Some(seek::<I>);
        object.tell = Some(tell::<I>);
        object.eof = Some(eof::<I>);
        object.may_block = Some(may_block::<I>);
    }

    extern "C" fn read<I: ImplReadHandler>(
        self_: *mut _cef_read_handler_t,
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        n: usize,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.read(ptr, size, n).into()
    }

    extern "C" fn seek<I: ImplReadHandler>(
        self_: *mut _cef_read_handler_t,
        offset: i64,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.seek(offset, whence).into()
    }

    extern "C" fn tell<I: ImplReadHandler>(self_: *mut _cef_read_handler_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.tell().into()
    }

    extern "C" fn eof<I: ImplReadHandler>(
        self_: *mut _cef_read_handler_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.eof().into()
    }

    extern "C" fn may_block<I: ImplReadHandler>(
        self_: *mut _cef_read_handler_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.may_block().into()
    }
}

wrapper!(
    #[doc = "See [_cef_stream_reader_t] for more documentation."]
    #[derive(Clone)]
    pub struct StreamReader(_cef_stream_reader_t);

    pub fn read(&self, ptr: *mut ::std::os::raw::c_void, size: usize, n: usize) -> usize;
    pub fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tell(&self) -> i64;
    pub fn eof(&self) -> ::std::os::raw::c_int;
    pub fn may_block(&self) -> ::std::os::raw::c_int;
);

pub trait ImplStreamReader: Sized {
    fn read(&self, ptr: *mut ::std::os::raw::c_void, size: usize, n: usize) -> usize {
        Default::default()
    }
    fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn tell(&self) -> i64 {
        Default::default()
    }
    fn eof(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn may_block(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_stream_reader_t {
        let mut object: _cef_stream_reader_t = unsafe { std::mem::zeroed() };
        impl_cef_stream_reader_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_stream_reader_t {
    use super::*;

    pub fn init_methods<I: ImplStreamReader>(object: &mut _cef_stream_reader_t) {
        object.read = Some(read::<I>);
        object.seek = Some(seek::<I>);
        object.tell = Some(tell::<I>);
        object.eof = Some(eof::<I>);
        object.may_block = Some(may_block::<I>);
    }

    extern "C" fn read<I: ImplStreamReader>(
        self_: *mut _cef_stream_reader_t,
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        n: usize,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.read(ptr, size, n).into()
    }

    extern "C" fn seek<I: ImplStreamReader>(
        self_: *mut _cef_stream_reader_t,
        offset: i64,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.seek(offset, whence).into()
    }

    extern "C" fn tell<I: ImplStreamReader>(self_: *mut _cef_stream_reader_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.tell().into()
    }

    extern "C" fn eof<I: ImplStreamReader>(
        self_: *mut _cef_stream_reader_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.eof().into()
    }

    extern "C" fn may_block<I: ImplStreamReader>(
        self_: *mut _cef_stream_reader_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.may_block().into()
    }
}

wrapper!(
    #[doc = "See [_cef_write_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct WriteHandler(_cef_write_handler_t);

    pub fn write(&self, ptr: *const ::std::os::raw::c_void, size: usize, n: usize) -> usize;
    pub fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tell(&self) -> i64;
    pub fn flush(&self) -> ::std::os::raw::c_int;
    pub fn may_block(&self) -> ::std::os::raw::c_int;
);

pub trait ImplWriteHandler: Sized {
    fn write(&self, ptr: *const ::std::os::raw::c_void, size: usize, n: usize) -> usize {
        Default::default()
    }
    fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn tell(&self) -> i64 {
        Default::default()
    }
    fn flush(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn may_block(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_write_handler_t {
        let mut object: _cef_write_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_write_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_write_handler_t {
    use super::*;

    pub fn init_methods<I: ImplWriteHandler>(object: &mut _cef_write_handler_t) {
        object.write = Some(write::<I>);
        object.seek = Some(seek::<I>);
        object.tell = Some(tell::<I>);
        object.flush = Some(flush::<I>);
        object.may_block = Some(may_block::<I>);
    }

    extern "C" fn write<I: ImplWriteHandler>(
        self_: *mut _cef_write_handler_t,
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        n: usize,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.write(ptr, size, n).into()
    }

    extern "C" fn seek<I: ImplWriteHandler>(
        self_: *mut _cef_write_handler_t,
        offset: i64,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.seek(offset, whence).into()
    }

    extern "C" fn tell<I: ImplWriteHandler>(self_: *mut _cef_write_handler_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.tell().into()
    }

    extern "C" fn flush<I: ImplWriteHandler>(
        self_: *mut _cef_write_handler_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.flush().into()
    }

    extern "C" fn may_block<I: ImplWriteHandler>(
        self_: *mut _cef_write_handler_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.may_block().into()
    }
}

wrapper!(
    #[doc = "See [_cef_stream_writer_t] for more documentation."]
    #[derive(Clone)]
    pub struct StreamWriter(_cef_stream_writer_t);

    pub fn write(&self, ptr: *const ::std::os::raw::c_void, size: usize, n: usize) -> usize;
    pub fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tell(&self) -> i64;
    pub fn flush(&self) -> ::std::os::raw::c_int;
    pub fn may_block(&self) -> ::std::os::raw::c_int;
);

pub trait ImplStreamWriter: Sized {
    fn write(&self, ptr: *const ::std::os::raw::c_void, size: usize, n: usize) -> usize {
        Default::default()
    }
    fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn tell(&self) -> i64 {
        Default::default()
    }
    fn flush(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn may_block(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_stream_writer_t {
        let mut object: _cef_stream_writer_t = unsafe { std::mem::zeroed() };
        impl_cef_stream_writer_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_stream_writer_t {
    use super::*;

    pub fn init_methods<I: ImplStreamWriter>(object: &mut _cef_stream_writer_t) {
        object.write = Some(write::<I>);
        object.seek = Some(seek::<I>);
        object.tell = Some(tell::<I>);
        object.flush = Some(flush::<I>);
        object.may_block = Some(may_block::<I>);
    }

    extern "C" fn write<I: ImplStreamWriter>(
        self_: *mut _cef_stream_writer_t,
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        n: usize,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.write(ptr, size, n).into()
    }

    extern "C" fn seek<I: ImplStreamWriter>(
        self_: *mut _cef_stream_writer_t,
        offset: i64,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.seek(offset, whence).into()
    }

    extern "C" fn tell<I: ImplStreamWriter>(self_: *mut _cef_stream_writer_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.tell().into()
    }

    extern "C" fn flush<I: ImplStreamWriter>(
        self_: *mut _cef_stream_writer_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.flush().into()
    }

    extern "C" fn may_block<I: ImplStreamWriter>(
        self_: *mut _cef_stream_writer_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.may_block().into()
    }
}

wrapper!(
    #[doc = "See [_cef_drag_data_t] for more documentation."]
    #[derive(Clone)]
    pub struct DragData(_cef_drag_data_t);

    pub fn clone(&self) -> DragData;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn is_link(&self) -> ::std::os::raw::c_int;
    pub fn is_fragment(&self) -> ::std::os::raw::c_int;
    pub fn is_file(&self) -> ::std::os::raw::c_int;
    pub fn get_link_url(&self) -> CefStringUserfree;
    pub fn get_link_title(&self) -> CefStringUserfree;
    pub fn get_link_metadata(&self) -> CefStringUserfree;
    pub fn get_fragment_text(&self) -> CefStringUserfree;
    pub fn get_fragment_html(&self) -> CefStringUserfree;
    pub fn get_fragment_base_url(&self) -> CefStringUserfree;
    pub fn get_file_name(&self) -> CefStringUserfree;
    pub fn get_file_contents(&self, writer: &mut StreamWriter) -> usize;
    pub fn get_file_names(&self, names: &mut CefStringList) -> ::std::os::raw::c_int;
    pub fn get_file_paths(&self, paths: &mut CefStringList) -> ::std::os::raw::c_int;
    pub fn set_link_url(&self, url: &CefString);
    pub fn set_link_title(&self, title: &CefString);
    pub fn set_link_metadata(&self, data: &CefString);
    pub fn set_fragment_text(&self, text: &CefString);
    pub fn set_fragment_html(&self, html: &CefString);
    pub fn set_fragment_base_url(&self, base_url: &CefString);
    pub fn reset_file_contents(&self);
    pub fn add_file(&self, path: &CefString, display_name: &CefString);
    pub fn clear_filenames(&self);
    pub fn get_image(&self) -> Image;
    pub fn get_image_hotspot(&self) -> Point;
    pub fn has_image(&self) -> ::std::os::raw::c_int;
);

pub trait ImplDragData: Sized {
    fn clone(&self) -> DragData {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_link(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_fragment(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_file(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_link_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_link_title(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_link_metadata(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_fragment_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_fragment_html(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_fragment_base_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_file_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_file_contents(&self, writer: &mut StreamWriter) -> usize {
        Default::default()
    }
    fn get_file_names(&self, names: &mut CefStringList) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_file_paths(&self, paths: &mut CefStringList) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_link_url(&self, url: &CefString) {}
    fn set_link_title(&self, title: &CefString) {}
    fn set_link_metadata(&self, data: &CefString) {}
    fn set_fragment_text(&self, text: &CefString) {}
    fn set_fragment_html(&self, html: &CefString) {}
    fn set_fragment_base_url(&self, base_url: &CefString) {}
    fn reset_file_contents(&self) {}
    fn add_file(&self, path: &CefString, display_name: &CefString) {}
    fn clear_filenames(&self) {}
    fn get_image(&self) -> Image {
        Default::default()
    }
    fn get_image_hotspot(&self) -> Point {
        Default::default()
    }
    fn has_image(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_drag_data_t {
        let mut object: _cef_drag_data_t = unsafe { std::mem::zeroed() };
        impl_cef_drag_data_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_drag_data_t {
    use super::*;

    pub fn init_methods<I: ImplDragData>(object: &mut _cef_drag_data_t) {
        object.clone = Some(clone::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.is_link = Some(is_link::<I>);
        object.is_fragment = Some(is_fragment::<I>);
        object.is_file = Some(is_file::<I>);
        object.get_link_url = Some(get_link_url::<I>);
        object.get_link_title = Some(get_link_title::<I>);
        object.get_link_metadata = Some(get_link_metadata::<I>);
        object.get_fragment_text = Some(get_fragment_text::<I>);
        object.get_fragment_html = Some(get_fragment_html::<I>);
        object.get_fragment_base_url = Some(get_fragment_base_url::<I>);
        object.get_file_name = Some(get_file_name::<I>);
        object.get_file_contents = Some(get_file_contents::<I>);
        object.get_file_names = Some(get_file_names::<I>);
        object.get_file_paths = Some(get_file_paths::<I>);
        object.set_link_url = Some(set_link_url::<I>);
        object.set_link_title = Some(set_link_title::<I>);
        object.set_link_metadata = Some(set_link_metadata::<I>);
        object.set_fragment_text = Some(set_fragment_text::<I>);
        object.set_fragment_html = Some(set_fragment_html::<I>);
        object.set_fragment_base_url = Some(set_fragment_base_url::<I>);
        object.reset_file_contents = Some(reset_file_contents::<I>);
        object.add_file = Some(add_file::<I>);
        object.clear_filenames = Some(clear_filenames::<I>);
        object.get_image = Some(get_image::<I>);
        object.get_image_hotspot = Some(get_image_hotspot::<I>);
        object.has_image = Some(has_image::<I>);
    }

    extern "C" fn clone<I: ImplDragData>(self_: *mut _cef_drag_data_t) -> *mut _cef_drag_data_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clone().into()
    }

    extern "C" fn is_read_only<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn is_link<I: ImplDragData>(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_link().into()
    }

    extern "C" fn is_fragment<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_fragment().into()
    }

    extern "C" fn is_file<I: ImplDragData>(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_file().into()
    }

    extern "C" fn get_link_url<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_link_url().into()
    }

    extern "C" fn get_link_title<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_link_title().into()
    }

    extern "C" fn get_link_metadata<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_link_metadata().into()
    }

    extern "C" fn get_fragment_text<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_fragment_text().into()
    }

    extern "C" fn get_fragment_html<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_fragment_html().into()
    }

    extern "C" fn get_fragment_base_url<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_fragment_base_url().into()
    }

    extern "C" fn get_file_name<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_file_name().into()
    }

    extern "C" fn get_file_contents<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        writer: *mut _cef_stream_writer_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let writer = &mut StreamWriter(unsafe { RefGuard::from_raw_add_ref(writer) });
        obj.interface.get_file_contents(writer).into()
    }

    extern "C" fn get_file_names<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        names: cef_string_list_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let names = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(names) });
        obj.interface.get_file_names(names).into()
    }

    extern "C" fn get_file_paths<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        paths: cef_string_list_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let paths = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(paths) });
        obj.interface.get_file_paths(paths).into()
    }

    extern "C" fn set_link_url<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.set_link_url(url)
    }

    extern "C" fn set_link_title<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        title: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let title = &CefString::from(title);
        obj.interface.set_link_title(title)
    }

    extern "C" fn set_link_metadata<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        data: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let data = &CefString::from(data);
        obj.interface.set_link_metadata(data)
    }

    extern "C" fn set_fragment_text<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        text: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let text = &CefString::from(text);
        obj.interface.set_fragment_text(text)
    }

    extern "C" fn set_fragment_html<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        html: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let html = &CefString::from(html);
        obj.interface.set_fragment_html(html)
    }

    extern "C" fn set_fragment_base_url<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        base_url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let base_url = &CefString::from(base_url);
        obj.interface.set_fragment_base_url(base_url)
    }

    extern "C" fn reset_file_contents<I: ImplDragData>(self_: *mut _cef_drag_data_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.reset_file_contents()
    }

    extern "C" fn add_file<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
        path: *const cef_string_t,
        display_name: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let path = &CefString::from(path);
        let display_name = &CefString::from(display_name);
        obj.interface.add_file(path, display_name)
    }

    extern "C" fn clear_filenames<I: ImplDragData>(self_: *mut _cef_drag_data_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear_filenames()
    }

    extern "C" fn get_image<I: ImplDragData>(self_: *mut _cef_drag_data_t) -> *mut _cef_image_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_image().into()
    }

    extern "C" fn get_image_hotspot<I: ImplDragData>(self_: *mut _cef_drag_data_t) -> cef_point_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_image_hotspot().into()
    }

    extern "C" fn has_image<I: ImplDragData>(
        self_: *mut _cef_drag_data_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_image().into()
    }
}

wrapper!(
    #[doc = "See [_cef_domvisitor_t] for more documentation."]
    #[derive(Clone)]
    pub struct Domvisitor(_cef_domvisitor_t);

    pub fn visit(&self, document: &mut Domdocument);
);

pub trait ImplDomvisitor: Sized {
    fn visit(&self, document: &mut Domdocument) {}

    fn into_raw(self) -> *mut _cef_domvisitor_t {
        let mut object: _cef_domvisitor_t = unsafe { std::mem::zeroed() };
        impl_cef_domvisitor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_domvisitor_t {
    use super::*;

    pub fn init_methods<I: ImplDomvisitor>(object: &mut _cef_domvisitor_t) {
        object.visit = Some(visit::<I>);
    }

    extern "C" fn visit<I: ImplDomvisitor>(
        self_: *mut _cef_domvisitor_t,
        document: *mut _cef_domdocument_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let document = &mut Domdocument(unsafe { RefGuard::from_raw_add_ref(document) });
        obj.interface.visit(document)
    }
}

wrapper!(
    #[doc = "See [_cef_domdocument_t] for more documentation."]
    #[derive(Clone)]
    pub struct Domdocument(_cef_domdocument_t);

    pub fn get_type(&self) -> DomDocumentType;
    pub fn get_document(&self) -> Domnode;
    pub fn get_body(&self) -> Domnode;
    pub fn get_head(&self) -> Domnode;
    pub fn get_title(&self) -> CefStringUserfree;
    pub fn get_element_by_id(&self, id: &CefString) -> Domnode;
    pub fn get_focused_node(&self) -> Domnode;
    pub fn has_selection(&self) -> ::std::os::raw::c_int;
    pub fn get_selection_start_offset(&self) -> ::std::os::raw::c_int;
    pub fn get_selection_end_offset(&self) -> ::std::os::raw::c_int;
    pub fn get_selection_as_markup(&self) -> CefStringUserfree;
    pub fn get_selection_as_text(&self) -> CefStringUserfree;
    pub fn get_base_url(&self) -> CefStringUserfree;
    pub fn get_complete_url(&self, partial_url: &CefString) -> CefStringUserfree;
);

pub trait ImplDomdocument: Sized {
    fn get_type(&self) -> DomDocumentType {
        Default::default()
    }
    fn get_document(&self) -> Domnode {
        Default::default()
    }
    fn get_body(&self) -> Domnode {
        Default::default()
    }
    fn get_head(&self) -> Domnode {
        Default::default()
    }
    fn get_title(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_element_by_id(&self, id: &CefString) -> Domnode {
        Default::default()
    }
    fn get_focused_node(&self) -> Domnode {
        Default::default()
    }
    fn has_selection(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_selection_start_offset(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_selection_end_offset(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_selection_as_markup(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_selection_as_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_base_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_complete_url(&self, partial_url: &CefString) -> CefStringUserfree {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_domdocument_t {
        let mut object: _cef_domdocument_t = unsafe { std::mem::zeroed() };
        impl_cef_domdocument_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_domdocument_t {
    use super::*;

    pub fn init_methods<I: ImplDomdocument>(object: &mut _cef_domdocument_t) {
        object.get_type = Some(get_type::<I>);
        object.get_document = Some(get_document::<I>);
        object.get_body = Some(get_body::<I>);
        object.get_head = Some(get_head::<I>);
        object.get_title = Some(get_title::<I>);
        object.get_element_by_id = Some(get_element_by_id::<I>);
        object.get_focused_node = Some(get_focused_node::<I>);
        object.has_selection = Some(has_selection::<I>);
        object.get_selection_start_offset = Some(get_selection_start_offset::<I>);
        object.get_selection_end_offset = Some(get_selection_end_offset::<I>);
        object.get_selection_as_markup = Some(get_selection_as_markup::<I>);
        object.get_selection_as_text = Some(get_selection_as_text::<I>);
        object.get_base_url = Some(get_base_url::<I>);
        object.get_complete_url = Some(get_complete_url::<I>);
    }

    extern "C" fn get_type<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> cef_dom_document_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type().into()
    }

    extern "C" fn get_document<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_document().into()
    }

    extern "C" fn get_body<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_body().into()
    }

    extern "C" fn get_head<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_head().into()
    }

    extern "C" fn get_title<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_title().into()
    }

    extern "C" fn get_element_by_id<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
        id: *const cef_string_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let id = &CefString::from(id);
        obj.interface.get_element_by_id(id).into()
    }

    extern "C" fn get_focused_node<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_focused_node().into()
    }

    extern "C" fn has_selection<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_selection().into()
    }

    extern "C" fn get_selection_start_offset<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_selection_start_offset().into()
    }

    extern "C" fn get_selection_end_offset<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_selection_end_offset().into()
    }

    extern "C" fn get_selection_as_markup<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_selection_as_markup().into()
    }

    extern "C" fn get_selection_as_text<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_selection_as_text().into()
    }

    extern "C" fn get_base_url<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_base_url().into()
    }

    extern "C" fn get_complete_url<I: ImplDomdocument>(
        self_: *mut _cef_domdocument_t,
        partial_url: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let partial_url = &CefString::from(partial_url);
        obj.interface.get_complete_url(partial_url).into()
    }
}

wrapper!(
    #[doc = "See [_cef_domnode_t] for more documentation."]
    #[derive(Clone)]
    pub struct Domnode(_cef_domnode_t);

    pub fn get_type(&self) -> DomNodeType;
    pub fn is_text(&self) -> ::std::os::raw::c_int;
    pub fn is_element(&self) -> ::std::os::raw::c_int;
    pub fn is_editable(&self) -> ::std::os::raw::c_int;
    pub fn is_form_control_element(&self) -> ::std::os::raw::c_int;
    pub fn get_form_control_element_type(&self) -> DomFormControlType;
    pub fn is_same(&self, that: &mut Domnode) -> ::std::os::raw::c_int;
    pub fn get_name(&self) -> CefStringUserfree;
    pub fn get_value(&self) -> CefStringUserfree;
    pub fn set_value(&self, value: &CefString) -> ::std::os::raw::c_int;
    pub fn get_as_markup(&self) -> CefStringUserfree;
    pub fn get_document(&self) -> Domdocument;
    pub fn get_parent(&self) -> Domnode;
    pub fn get_previous_sibling(&self) -> Domnode;
    pub fn get_next_sibling(&self) -> Domnode;
    pub fn has_children(&self) -> ::std::os::raw::c_int;
    pub fn get_first_child(&self) -> Domnode;
    pub fn get_last_child(&self) -> Domnode;
    pub fn get_element_tag_name(&self) -> CefStringUserfree;
    pub fn has_element_attributes(&self) -> ::std::os::raw::c_int;
    pub fn has_element_attribute(&self, attr_name: &CefString) -> ::std::os::raw::c_int;
    pub fn get_element_attribute(&self, attr_name: &CefString) -> CefStringUserfree;
    pub fn get_element_attributes(&self, attr_map: &mut CefStringMap);
    pub fn set_element_attribute(
        &self,
        attr_name: &CefString,
        value: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn get_element_inner_text(&self) -> CefStringUserfree;
    pub fn get_element_bounds(&self) -> Rect;
);

pub trait ImplDomnode: Sized {
    fn get_type(&self) -> DomNodeType {
        Default::default()
    }
    fn is_text(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_element(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_editable(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_form_control_element(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_form_control_element_type(&self) -> DomFormControlType {
        Default::default()
    }
    fn is_same(&self, that: &mut Domnode) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_value(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_value(&self, value: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_as_markup(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_document(&self) -> Domdocument {
        Default::default()
    }
    fn get_parent(&self) -> Domnode {
        Default::default()
    }
    fn get_previous_sibling(&self) -> Domnode {
        Default::default()
    }
    fn get_next_sibling(&self) -> Domnode {
        Default::default()
    }
    fn has_children(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_first_child(&self) -> Domnode {
        Default::default()
    }
    fn get_last_child(&self) -> Domnode {
        Default::default()
    }
    fn get_element_tag_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn has_element_attributes(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_element_attribute(&self, attr_name: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_element_attribute(&self, attr_name: &CefString) -> CefStringUserfree {
        Default::default()
    }
    fn get_element_attributes(&self, attr_map: &mut CefStringMap) {}
    fn set_element_attribute(
        &self,
        attr_name: &CefString,
        value: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_element_inner_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_element_bounds(&self) -> Rect {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_domnode_t {
        let mut object: _cef_domnode_t = unsafe { std::mem::zeroed() };
        impl_cef_domnode_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_domnode_t {
    use super::*;

    pub fn init_methods<I: ImplDomnode>(object: &mut _cef_domnode_t) {
        object.get_type = Some(get_type::<I>);
        object.is_text = Some(is_text::<I>);
        object.is_element = Some(is_element::<I>);
        object.is_editable = Some(is_editable::<I>);
        object.is_form_control_element = Some(is_form_control_element::<I>);
        object.get_form_control_element_type = Some(get_form_control_element_type::<I>);
        object.is_same = Some(is_same::<I>);
        object.get_name = Some(get_name::<I>);
        object.get_value = Some(get_value::<I>);
        object.set_value = Some(set_value::<I>);
        object.get_as_markup = Some(get_as_markup::<I>);
        object.get_document = Some(get_document::<I>);
        object.get_parent = Some(get_parent::<I>);
        object.get_previous_sibling = Some(get_previous_sibling::<I>);
        object.get_next_sibling = Some(get_next_sibling::<I>);
        object.has_children = Some(has_children::<I>);
        object.get_first_child = Some(get_first_child::<I>);
        object.get_last_child = Some(get_last_child::<I>);
        object.get_element_tag_name = Some(get_element_tag_name::<I>);
        object.has_element_attributes = Some(has_element_attributes::<I>);
        object.has_element_attribute = Some(has_element_attribute::<I>);
        object.get_element_attribute = Some(get_element_attribute::<I>);
        object.get_element_attributes = Some(get_element_attributes::<I>);
        object.set_element_attribute = Some(set_element_attribute::<I>);
        object.get_element_inner_text = Some(get_element_inner_text::<I>);
        object.get_element_bounds = Some(get_element_bounds::<I>);
    }

    extern "C" fn get_type<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> cef_dom_node_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type().into()
    }

    extern "C" fn is_text<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_text().into()
    }

    extern "C" fn is_element<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_element().into()
    }

    extern "C" fn is_editable<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_editable().into()
    }

    extern "C" fn is_form_control_element<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_form_control_element().into()
    }

    extern "C" fn get_form_control_element_type<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> cef_dom_form_control_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_form_control_element_type().into()
    }

    extern "C" fn is_same<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        that: *mut _cef_domnode_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut Domnode(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn get_name<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_name().into()
    }

    extern "C" fn get_value<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_value().into()
    }

    extern "C" fn set_value<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        value: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &CefString::from(value);
        obj.interface.set_value(value).into()
    }

    extern "C" fn get_as_markup<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_as_markup().into()
    }

    extern "C" fn get_document<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> *mut _cef_domdocument_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_document().into()
    }

    extern "C" fn get_parent<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_parent().into()
    }

    extern "C" fn get_previous_sibling<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_previous_sibling().into()
    }

    extern "C" fn get_next_sibling<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_next_sibling().into()
    }

    extern "C" fn has_children<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_children().into()
    }

    extern "C" fn get_first_child<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_first_child().into()
    }

    extern "C" fn get_last_child<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> *mut _cef_domnode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_last_child().into()
    }

    extern "C" fn get_element_tag_name<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_element_tag_name().into()
    }

    extern "C" fn has_element_attributes<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_element_attributes().into()
    }

    extern "C" fn has_element_attribute<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        attr_name: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let attr_name = &CefString::from(attr_name);
        obj.interface.has_element_attribute(attr_name).into()
    }

    extern "C" fn get_element_attribute<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        attr_name: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let attr_name = &CefString::from(attr_name);
        obj.interface.get_element_attribute(attr_name).into()
    }

    extern "C" fn get_element_attributes<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        attr_map: cef_string_map_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let attr_map = &mut CefStringMap(unsafe { RefGuard::from_raw_add_ref(attr_map) });
        obj.interface.get_element_attributes(attr_map)
    }

    extern "C" fn set_element_attribute<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
        attr_name: *const cef_string_t,
        value: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let attr_name = &CefString::from(attr_name);
        let value = &CefString::from(value);
        obj.interface.set_element_attribute(attr_name, value).into()
    }

    extern "C" fn get_element_inner_text<I: ImplDomnode>(
        self_: *mut _cef_domnode_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_element_inner_text().into()
    }

    extern "C" fn get_element_bounds<I: ImplDomnode>(self_: *mut _cef_domnode_t) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_element_bounds().into()
    }
}

wrapper!(
    #[doc = "See [_cef_shared_memory_region_t] for more documentation."]
    #[derive(Clone)]
    pub struct SharedMemoryRegion(_cef_shared_memory_region_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn size(&self) -> usize;
    pub fn memory(&self) -> *mut ::std::os::raw::c_void;
);

pub trait ImplSharedMemoryRegion: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn size(&self) -> usize {
        Default::default()
    }
    fn memory(&self) -> *mut ::std::os::raw::c_void {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_shared_memory_region_t {
        let mut object: _cef_shared_memory_region_t = unsafe { std::mem::zeroed() };
        impl_cef_shared_memory_region_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_shared_memory_region_t {
    use super::*;

    pub fn init_methods<I: ImplSharedMemoryRegion>(object: &mut _cef_shared_memory_region_t) {
        object.is_valid = Some(is_valid::<I>);
        object.size = Some(size::<I>);
        object.memory = Some(memory::<I>);
    }

    extern "C" fn is_valid<I: ImplSharedMemoryRegion>(
        self_: *mut _cef_shared_memory_region_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn size<I: ImplSharedMemoryRegion>(
        self_: *mut _cef_shared_memory_region_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.size().into()
    }

    extern "C" fn memory<I: ImplSharedMemoryRegion>(
        self_: *mut _cef_shared_memory_region_t,
    ) -> *mut ::std::os::raw::c_void {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.memory().into()
    }
}

wrapper!(
    #[doc = "See [_cef_process_message_t] for more documentation."]
    #[derive(Clone)]
    pub struct ProcessMessage(_cef_process_message_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn copy(&self) -> ProcessMessage;
    pub fn get_name(&self) -> CefStringUserfree;
    pub fn get_argument_list(&self) -> ListValue;
    pub fn get_shared_memory_region(&self) -> SharedMemoryRegion;
);

pub trait ImplProcessMessage: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self) -> ProcessMessage {
        Default::default()
    }
    fn get_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_argument_list(&self) -> ListValue {
        Default::default()
    }
    fn get_shared_memory_region(&self) -> SharedMemoryRegion {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_process_message_t {
        let mut object: _cef_process_message_t = unsafe { std::mem::zeroed() };
        impl_cef_process_message_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_process_message_t {
    use super::*;

    pub fn init_methods<I: ImplProcessMessage>(object: &mut _cef_process_message_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.copy = Some(copy::<I>);
        object.get_name = Some(get_name::<I>);
        object.get_argument_list = Some(get_argument_list::<I>);
        object.get_shared_memory_region = Some(get_shared_memory_region::<I>);
    }

    extern "C" fn is_valid<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_read_only<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn copy<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> *mut _cef_process_message_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy().into()
    }

    extern "C" fn get_name<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_name().into()
    }

    extern "C" fn get_argument_list<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> *mut _cef_list_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_argument_list().into()
    }

    extern "C" fn get_shared_memory_region<I: ImplProcessMessage>(
        self_: *mut _cef_process_message_t,
    ) -> *mut _cef_shared_memory_region_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_shared_memory_region().into()
    }
}

wrapper!(
    #[doc = "See [_cef_request_t] for more documentation."]
    #[derive(Clone)]
    pub struct Request(_cef_request_t);

    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn get_url(&self) -> CefStringUserfree;
    pub fn set_url(&self, url: &CefString);
    pub fn get_method(&self) -> CefStringUserfree;
    pub fn set_method(&self, method: &CefString);
    pub fn set_referrer(&self, referrer_url: &CefString, policy: ReferrerPolicy);
    pub fn get_referrer_url(&self) -> CefStringUserfree;
    pub fn get_referrer_policy(&self) -> ReferrerPolicy;
    pub fn get_post_data(&self) -> PostData;
    pub fn set_post_data(&self, post_data: &mut PostData);
    pub fn get_header_map(&self, header_map: &mut CefStringMultimap);
    pub fn set_header_map(&self, header_map: &mut CefStringMultimap);
    pub fn get_header_by_name(&self, name: &CefString) -> CefStringUserfree;
    pub fn set_header_by_name(
        &self,
        name: &CefString,
        value: &CefString,
        overwrite: ::std::os::raw::c_int,
    );
    pub fn set(
        &self,
        url: &CefString,
        method: &CefString,
        post_data: &mut PostData,
        header_map: &mut CefStringMultimap,
    );
    pub fn get_flags(&self) -> ::std::os::raw::c_int;
    pub fn set_flags(&self, flags: ::std::os::raw::c_int);
    pub fn get_first_party_for_cookies(&self) -> CefStringUserfree;
    pub fn set_first_party_for_cookies(&self, url: &CefString);
    pub fn get_resource_type(&self) -> ResourceType;
    pub fn get_transition_type(&self) -> TransitionType;
    pub fn get_identifier(&self) -> u64;
);

pub trait ImplRequest: Sized {
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_url(&self, url: &CefString) {}
    fn get_method(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_method(&self, method: &CefString) {}
    fn set_referrer(&self, referrer_url: &CefString, policy: ReferrerPolicy) {}
    fn get_referrer_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_referrer_policy(&self) -> ReferrerPolicy {
        Default::default()
    }
    fn get_post_data(&self) -> PostData {
        Default::default()
    }
    fn set_post_data(&self, post_data: &mut PostData) {}
    fn get_header_map(&self, header_map: &mut CefStringMultimap) {}
    fn set_header_map(&self, header_map: &mut CefStringMultimap) {}
    fn get_header_by_name(&self, name: &CefString) -> CefStringUserfree {
        Default::default()
    }
    fn set_header_by_name(
        &self,
        name: &CefString,
        value: &CefString,
        overwrite: ::std::os::raw::c_int,
    ) {
    }
    fn set(
        &self,
        url: &CefString,
        method: &CefString,
        post_data: &mut PostData,
        header_map: &mut CefStringMultimap,
    ) {
    }
    fn get_flags(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_flags(&self, flags: ::std::os::raw::c_int) {}
    fn get_first_party_for_cookies(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_first_party_for_cookies(&self, url: &CefString) {}
    fn get_resource_type(&self) -> ResourceType {
        Default::default()
    }
    fn get_transition_type(&self) -> TransitionType {
        Default::default()
    }
    fn get_identifier(&self) -> u64 {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_request_t {
        let mut object: _cef_request_t = unsafe { std::mem::zeroed() };
        impl_cef_request_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_request_t {
    use super::*;

    pub fn init_methods<I: ImplRequest>(object: &mut _cef_request_t) {
        object.is_read_only = Some(is_read_only::<I>);
        object.get_url = Some(get_url::<I>);
        object.set_url = Some(set_url::<I>);
        object.get_method = Some(get_method::<I>);
        object.set_method = Some(set_method::<I>);
        object.set_referrer = Some(set_referrer::<I>);
        object.get_referrer_url = Some(get_referrer_url::<I>);
        object.get_referrer_policy = Some(get_referrer_policy::<I>);
        object.get_post_data = Some(get_post_data::<I>);
        object.set_post_data = Some(set_post_data::<I>);
        object.get_header_map = Some(get_header_map::<I>);
        object.set_header_map = Some(set_header_map::<I>);
        object.get_header_by_name = Some(get_header_by_name::<I>);
        object.set_header_by_name = Some(set_header_by_name::<I>);
        object.set = Some(set::<I>);
        object.get_flags = Some(get_flags::<I>);
        object.set_flags = Some(set_flags::<I>);
        object.get_first_party_for_cookies = Some(get_first_party_for_cookies::<I>);
        object.set_first_party_for_cookies = Some(set_first_party_for_cookies::<I>);
        object.get_resource_type = Some(get_resource_type::<I>);
        object.get_transition_type = Some(get_transition_type::<I>);
        object.get_identifier = Some(get_identifier::<I>);
    }

    extern "C" fn is_read_only<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn get_url<I: ImplRequest>(self_: *mut _cef_request_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_url().into()
    }

    extern "C" fn set_url<I: ImplRequest>(self_: *mut _cef_request_t, url: *const cef_string_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.set_url(url)
    }

    extern "C" fn get_method<I: ImplRequest>(self_: *mut _cef_request_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_method().into()
    }

    extern "C" fn set_method<I: ImplRequest>(
        self_: *mut _cef_request_t,
        method: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let method = &CefString::from(method);
        obj.interface.set_method(method)
    }

    extern "C" fn set_referrer<I: ImplRequest>(
        self_: *mut _cef_request_t,
        referrer_url: *const cef_string_t,
        policy: cef_referrer_policy_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let referrer_url = &CefString::from(referrer_url);
        obj.interface.set_referrer(referrer_url, policy)
    }

    extern "C" fn get_referrer_url<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_referrer_url().into()
    }

    extern "C" fn get_referrer_policy<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> cef_referrer_policy_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_referrer_policy().into()
    }

    extern "C" fn get_post_data<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> *mut _cef_post_data_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_post_data().into()
    }

    extern "C" fn set_post_data<I: ImplRequest>(
        self_: *mut _cef_request_t,
        post_data: *mut _cef_post_data_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let post_data = &mut PostData(unsafe { RefGuard::from_raw_add_ref(post_data) });
        obj.interface.set_post_data(post_data)
    }

    extern "C" fn get_header_map<I: ImplRequest>(
        self_: *mut _cef_request_t,
        header_map: cef_string_multimap_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let header_map = &mut CefStringMultimap(unsafe { RefGuard::from_raw_add_ref(header_map) });
        obj.interface.get_header_map(header_map)
    }

    extern "C" fn set_header_map<I: ImplRequest>(
        self_: *mut _cef_request_t,
        header_map: cef_string_multimap_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let header_map = &mut CefStringMultimap(unsafe { RefGuard::from_raw_add_ref(header_map) });
        obj.interface.set_header_map(header_map)
    }

    extern "C" fn get_header_by_name<I: ImplRequest>(
        self_: *mut _cef_request_t,
        name: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.get_header_by_name(name).into()
    }

    extern "C" fn set_header_by_name<I: ImplRequest>(
        self_: *mut _cef_request_t,
        name: *const cef_string_t,
        value: *const cef_string_t,
        overwrite: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let value = &CefString::from(value);
        obj.interface.set_header_by_name(name, value, overwrite)
    }

    extern "C" fn set<I: ImplRequest>(
        self_: *mut _cef_request_t,
        url: *const cef_string_t,
        method: *const cef_string_t,
        post_data: *mut _cef_post_data_t,
        header_map: cef_string_multimap_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        let method = &CefString::from(method);
        let post_data = &mut PostData(unsafe { RefGuard::from_raw_add_ref(post_data) });
        let header_map = &mut CefStringMultimap(unsafe { RefGuard::from_raw_add_ref(header_map) });
        obj.interface.set(url, method, post_data, header_map)
    }

    extern "C" fn get_flags<I: ImplRequest>(self_: *mut _cef_request_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_flags().into()
    }

    extern "C" fn set_flags<I: ImplRequest>(
        self_: *mut _cef_request_t,
        flags: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_flags(flags)
    }

    extern "C" fn get_first_party_for_cookies<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_first_party_for_cookies().into()
    }

    extern "C" fn set_first_party_for_cookies<I: ImplRequest>(
        self_: *mut _cef_request_t,
        url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.set_first_party_for_cookies(url)
    }

    extern "C" fn get_resource_type<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> cef_resource_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_resource_type().into()
    }

    extern "C" fn get_transition_type<I: ImplRequest>(
        self_: *mut _cef_request_t,
    ) -> cef_transition_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_transition_type().into()
    }

    extern "C" fn get_identifier<I: ImplRequest>(self_: *mut _cef_request_t) -> u64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_identifier().into()
    }
}

wrapper!(
    #[doc = "See [_cef_post_data_t] for more documentation."]
    #[derive(Clone)]
    pub struct PostData(_cef_post_data_t);

    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn has_excluded_elements(&self) -> ::std::os::raw::c_int;
    pub fn get_element_count(&self) -> usize;
    pub fn get_elements(&self, elements_count: *mut usize, elements: *mut &mut PostDataElement);
    pub fn remove_element(&self, element: &mut PostDataElement) -> ::std::os::raw::c_int;
    pub fn add_element(&self, element: &mut PostDataElement) -> ::std::os::raw::c_int;
    pub fn remove_elements(&self);
);

pub trait ImplPostData: Sized {
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_excluded_elements(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_element_count(&self) -> usize {
        Default::default()
    }
    fn get_elements(&self, elements_count: *mut usize, elements: *mut &mut PostDataElement) {}
    fn remove_element(&self, element: &mut PostDataElement) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_element(&self, element: &mut PostDataElement) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove_elements(&self) {}

    fn into_raw(self) -> *mut _cef_post_data_t {
        let mut object: _cef_post_data_t = unsafe { std::mem::zeroed() };
        impl_cef_post_data_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_post_data_t {
    use super::*;

    pub fn init_methods<I: ImplPostData>(object: &mut _cef_post_data_t) {
        object.is_read_only = Some(is_read_only::<I>);
        object.has_excluded_elements = Some(has_excluded_elements::<I>);
        object.get_element_count = Some(get_element_count::<I>);
        object.get_elements = Some(get_elements::<I>);
        object.remove_element = Some(remove_element::<I>);
        object.add_element = Some(add_element::<I>);
        object.remove_elements = Some(remove_elements::<I>);
    }

    extern "C" fn is_read_only<I: ImplPostData>(
        self_: *mut _cef_post_data_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn has_excluded_elements<I: ImplPostData>(
        self_: *mut _cef_post_data_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_excluded_elements().into()
    }

    extern "C" fn get_element_count<I: ImplPostData>(self_: *mut _cef_post_data_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_element_count().into()
    }

    extern "C" fn get_elements<I: ImplPostData>(
        self_: *mut _cef_post_data_t,
        elements_count: *mut usize,
        elements: *mut *mut _cef_post_data_element_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_elements(elements_count, elements)
    }

    extern "C" fn remove_element<I: ImplPostData>(
        self_: *mut _cef_post_data_t,
        element: *mut _cef_post_data_element_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let element = &mut PostDataElement(unsafe { RefGuard::from_raw_add_ref(element) });
        obj.interface.remove_element(element).into()
    }

    extern "C" fn add_element<I: ImplPostData>(
        self_: *mut _cef_post_data_t,
        element: *mut _cef_post_data_element_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let element = &mut PostDataElement(unsafe { RefGuard::from_raw_add_ref(element) });
        obj.interface.add_element(element).into()
    }

    extern "C" fn remove_elements<I: ImplPostData>(self_: *mut _cef_post_data_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_elements()
    }
}

wrapper!(
    #[doc = "See [_cef_post_data_element_t] for more documentation."]
    #[derive(Clone)]
    pub struct PostDataElement(_cef_post_data_element_t);

    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn set_to_empty(&self);
    pub fn set_to_file(&self, file_name: &CefString);
    pub fn set_to_bytes(&self, size: usize, bytes: *const ::std::os::raw::c_void);
    pub fn get_type(&self) -> PostdataelementType;
    pub fn get_file(&self) -> CefStringUserfree;
    pub fn get_bytes_count(&self) -> usize;
    pub fn get_bytes(&self, size: usize, bytes: *mut ::std::os::raw::c_void) -> usize;
);

pub trait ImplPostDataElement: Sized {
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_to_empty(&self) {}
    fn set_to_file(&self, file_name: &CefString) {}
    fn set_to_bytes(&self, size: usize, bytes: *const ::std::os::raw::c_void) {}
    fn get_type(&self) -> PostdataelementType {
        Default::default()
    }
    fn get_file(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_bytes_count(&self) -> usize {
        Default::default()
    }
    fn get_bytes(&self, size: usize, bytes: *mut ::std::os::raw::c_void) -> usize {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_post_data_element_t {
        let mut object: _cef_post_data_element_t = unsafe { std::mem::zeroed() };
        impl_cef_post_data_element_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_post_data_element_t {
    use super::*;

    pub fn init_methods<I: ImplPostDataElement>(object: &mut _cef_post_data_element_t) {
        object.is_read_only = Some(is_read_only::<I>);
        object.set_to_empty = Some(set_to_empty::<I>);
        object.set_to_file = Some(set_to_file::<I>);
        object.set_to_bytes = Some(set_to_bytes::<I>);
        object.get_type = Some(get_type::<I>);
        object.get_file = Some(get_file::<I>);
        object.get_bytes_count = Some(get_bytes_count::<I>);
        object.get_bytes = Some(get_bytes::<I>);
    }

    extern "C" fn is_read_only<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn set_to_empty<I: ImplPostDataElement>(self_: *mut _cef_post_data_element_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_to_empty()
    }

    extern "C" fn set_to_file<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
        file_name: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let file_name = &CefString::from(file_name);
        obj.interface.set_to_file(file_name)
    }

    extern "C" fn set_to_bytes<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
        size: usize,
        bytes: *const ::std::os::raw::c_void,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_to_bytes(size, bytes)
    }

    extern "C" fn get_type<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
    ) -> cef_postdataelement_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type().into()
    }

    extern "C" fn get_file<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_file().into()
    }

    extern "C" fn get_bytes_count<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bytes_count().into()
    }

    extern "C" fn get_bytes<I: ImplPostDataElement>(
        self_: *mut _cef_post_data_element_t,
        size: usize,
        bytes: *mut ::std::os::raw::c_void,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bytes(size, bytes).into()
    }
}

wrapper!(
    #[doc = "See [_cef_string_visitor_t] for more documentation."]
    #[derive(Clone)]
    pub struct CefStringVisitor(_cef_string_visitor_t);

    pub fn visit(&self, string: &CefString);
);

pub trait ImplCefStringVisitor: Sized {
    fn visit(&self, string: &CefString) {}

    fn into_raw(self) -> *mut _cef_string_visitor_t {
        let mut object: _cef_string_visitor_t = unsafe { std::mem::zeroed() };
        impl_cef_string_visitor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_string_visitor_t {
    use super::*;

    pub fn init_methods<I: ImplCefStringVisitor>(object: &mut _cef_string_visitor_t) {
        object.visit = Some(visit::<I>);
    }

    extern "C" fn visit<I: ImplCefStringVisitor>(
        self_: *mut _cef_string_visitor_t,
        string: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let string = &CefString::from(string);
        obj.interface.visit(string)
    }
}

/// See [_cef_urlrequest_client_t] for more documentation.
pub struct UrlrequestClient(_cef_urlrequest_client_t);

impl From<_cef_urlrequest_client_t> for UrlrequestClient {
    fn from(value: _cef_urlrequest_client_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_urlrequest_client_t> for UrlrequestClient {
    fn into(self) -> _cef_urlrequest_client_t {
        self.0
    }
}

impl AsRef<_cef_urlrequest_client_t> for UrlrequestClient {
    fn as_ref(&self) -> &_cef_urlrequest_client_t {
        &self.0
    }
}

impl AsMut<_cef_urlrequest_client_t> for UrlrequestClient {
    fn as_mut(&mut self) -> &mut _cef_urlrequest_client_t {
        &mut self.0
    }
}

impl Default for UrlrequestClient {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_urlrequest_t] for more documentation.
pub struct Urlrequest(_cef_urlrequest_t);

impl From<_cef_urlrequest_t> for Urlrequest {
    fn from(value: _cef_urlrequest_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_urlrequest_t> for Urlrequest {
    fn into(self) -> _cef_urlrequest_t {
        self.0
    }
}

impl AsRef<_cef_urlrequest_t> for Urlrequest {
    fn as_ref(&self) -> &_cef_urlrequest_t {
        &self.0
    }
}

impl AsMut<_cef_urlrequest_t> for Urlrequest {
    fn as_mut(&mut self) -> &mut _cef_urlrequest_t {
        &mut self.0
    }
}

impl Default for Urlrequest {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_frame_t] for more documentation."]
    #[derive(Clone)]
    pub struct Frame(_cef_frame_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn undo(&self);
    pub fn redo(&self);
    pub fn cut(&self);
    pub fn copy(&self);
    pub fn paste(&self);
    pub fn paste_and_match_style(&self);
    pub fn del(&self);
    pub fn select_all(&self);
    pub fn view_source(&self);
    pub fn get_source(&self, visitor: &mut CefStringVisitor);
    pub fn get_text(&self, visitor: &mut CefStringVisitor);
    pub fn load_request(&self, request: &mut Request);
    pub fn load_url(&self, url: &CefString);
    pub fn execute_java_script(
        &self,
        code: &CefString,
        script_url: &CefString,
        start_line: ::std::os::raw::c_int,
    );
    pub fn is_main(&self) -> ::std::os::raw::c_int;
    pub fn is_focused(&self) -> ::std::os::raw::c_int;
    pub fn get_name(&self) -> CefStringUserfree;
    pub fn get_identifier(&self) -> CefStringUserfree;
    pub fn get_parent(&self) -> Frame;
    pub fn get_url(&self) -> CefStringUserfree;
    pub fn get_browser(&self) -> Browser;
    pub fn get_v8context(&self) -> V8context;
    pub fn visit_dom(&self, visitor: &mut Domvisitor);
    pub fn create_urlrequest(
        &self,
        request: &mut Request,
        client: &mut UrlrequestClient,
    ) -> Urlrequest;
    pub fn send_process_message(&self, target_process: ProcessId, message: &mut ProcessMessage);
);

pub trait ImplFrame: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn undo(&self) {}
    fn redo(&self) {}
    fn cut(&self) {}
    fn copy(&self) {}
    fn paste(&self) {}
    fn paste_and_match_style(&self) {}
    fn del(&self) {}
    fn select_all(&self) {}
    fn view_source(&self) {}
    fn get_source(&self, visitor: &mut CefStringVisitor) {}
    fn get_text(&self, visitor: &mut CefStringVisitor) {}
    fn load_request(&self, request: &mut Request) {}
    fn load_url(&self, url: &CefString) {}
    fn execute_java_script(
        &self,
        code: &CefString,
        script_url: &CefString,
        start_line: ::std::os::raw::c_int,
    ) {
    }
    fn is_main(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_focused(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_identifier(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_parent(&self) -> Frame {
        Default::default()
    }
    fn get_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_browser(&self) -> Browser {
        Default::default()
    }
    fn get_v8context(&self) -> V8context {
        Default::default()
    }
    fn visit_dom(&self, visitor: &mut Domvisitor) {}
    fn create_urlrequest(
        &self,
        request: &mut Request,
        client: &mut UrlrequestClient,
    ) -> Urlrequest {
        Default::default()
    }
    fn send_process_message(&self, target_process: ProcessId, message: &mut ProcessMessage) {}

    fn into_raw(self) -> *mut _cef_frame_t {
        let mut object: _cef_frame_t = unsafe { std::mem::zeroed() };
        impl_cef_frame_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_frame_t {
    use super::*;

    pub fn init_methods<I: ImplFrame>(object: &mut _cef_frame_t) {
        object.is_valid = Some(is_valid::<I>);
        object.undo = Some(undo::<I>);
        object.redo = Some(redo::<I>);
        object.cut = Some(cut::<I>);
        object.copy = Some(copy::<I>);
        object.paste = Some(paste::<I>);
        object.paste_and_match_style = Some(paste_and_match_style::<I>);
        object.del = Some(del::<I>);
        object.select_all = Some(select_all::<I>);
        object.view_source = Some(view_source::<I>);
        object.get_source = Some(get_source::<I>);
        object.get_text = Some(get_text::<I>);
        object.load_request = Some(load_request::<I>);
        object.load_url = Some(load_url::<I>);
        object.execute_java_script = Some(execute_java_script::<I>);
        object.is_main = Some(is_main::<I>);
        object.is_focused = Some(is_focused::<I>);
        object.get_name = Some(get_name::<I>);
        object.get_identifier = Some(get_identifier::<I>);
        object.get_parent = Some(get_parent::<I>);
        object.get_url = Some(get_url::<I>);
        object.get_browser = Some(get_browser::<I>);
        object.get_v8context = Some(get_v8context::<I>);
        object.visit_dom = Some(visit_dom::<I>);
        object.create_urlrequest = Some(create_urlrequest::<I>);
        object.send_process_message = Some(send_process_message::<I>);
    }

    extern "C" fn is_valid<I: ImplFrame>(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn undo<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.undo()
    }

    extern "C" fn redo<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.redo()
    }

    extern "C" fn cut<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cut()
    }

    extern "C" fn copy<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy()
    }

    extern "C" fn paste<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.paste()
    }

    extern "C" fn paste_and_match_style<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.paste_and_match_style()
    }

    extern "C" fn del<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.del()
    }

    extern "C" fn select_all<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.select_all()
    }

    extern "C" fn view_source<I: ImplFrame>(self_: *mut _cef_frame_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.view_source()
    }

    extern "C" fn get_source<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        visitor: *mut _cef_string_visitor_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let visitor = &mut CefStringVisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface.get_source(visitor)
    }

    extern "C" fn get_text<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        visitor: *mut _cef_string_visitor_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let visitor = &mut CefStringVisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface.get_text(visitor)
    }

    extern "C" fn load_request<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        request: *mut _cef_request_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface.load_request(request)
    }

    extern "C" fn load_url<I: ImplFrame>(self_: *mut _cef_frame_t, url: *const cef_string_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.load_url(url)
    }

    extern "C" fn execute_java_script<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        code: *const cef_string_t,
        script_url: *const cef_string_t,
        start_line: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let code = &CefString::from(code);
        let script_url = &CefString::from(script_url);
        obj.interface
            .execute_java_script(code, script_url, start_line)
    }

    extern "C" fn is_main<I: ImplFrame>(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_main().into()
    }

    extern "C" fn is_focused<I: ImplFrame>(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_focused().into()
    }

    extern "C" fn get_name<I: ImplFrame>(self_: *mut _cef_frame_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_name().into()
    }

    extern "C" fn get_identifier<I: ImplFrame>(self_: *mut _cef_frame_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_identifier().into()
    }

    extern "C" fn get_parent<I: ImplFrame>(self_: *mut _cef_frame_t) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_parent().into()
    }

    extern "C" fn get_url<I: ImplFrame>(self_: *mut _cef_frame_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_url().into()
    }

    extern "C" fn get_browser<I: ImplFrame>(self_: *mut _cef_frame_t) -> *mut _cef_browser_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser().into()
    }

    extern "C" fn get_v8context<I: ImplFrame>(self_: *mut _cef_frame_t) -> *mut _cef_v8context_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_v8context().into()
    }

    extern "C" fn visit_dom<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        visitor: *mut _cef_domvisitor_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let visitor = &mut Domvisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface.visit_dom(visitor)
    }

    extern "C" fn create_urlrequest<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        client: *mut _cef_urlrequest_client_t,
    ) -> *mut _cef_urlrequest_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let client = &mut UrlrequestClient(unsafe { RefGuard::from_raw_add_ref(client) });
        obj.interface.create_urlrequest(request, client).into()
    }

    extern "C" fn send_process_message<I: ImplFrame>(
        self_: *mut _cef_frame_t,
        target_process: cef_process_id_t,
        message: *mut _cef_process_message_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let message = &mut ProcessMessage(unsafe { RefGuard::from_raw_add_ref(message) });
        obj.interface.send_process_message(target_process, message)
    }
}

wrapper!(
    #[doc = "See [_cef_x509cert_principal_t] for more documentation."]
    #[derive(Clone)]
    pub struct X509certPrincipal(_cef_x509cert_principal_t);

    pub fn get_display_name(&self) -> CefStringUserfree;
    pub fn get_common_name(&self) -> CefStringUserfree;
    pub fn get_locality_name(&self) -> CefStringUserfree;
    pub fn get_state_or_province_name(&self) -> CefStringUserfree;
    pub fn get_country_name(&self) -> CefStringUserfree;
    pub fn get_organization_names(&self, names: &mut CefStringList);
    pub fn get_organization_unit_names(&self, names: &mut CefStringList);
);

pub trait ImplX509certPrincipal: Sized {
    fn get_display_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_common_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_locality_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_state_or_province_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_country_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_organization_names(&self, names: &mut CefStringList) {}
    fn get_organization_unit_names(&self, names: &mut CefStringList) {}

    fn into_raw(self) -> *mut _cef_x509cert_principal_t {
        let mut object: _cef_x509cert_principal_t = unsafe { std::mem::zeroed() };
        impl_cef_x509cert_principal_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_x509cert_principal_t {
    use super::*;

    pub fn init_methods<I: ImplX509certPrincipal>(object: &mut _cef_x509cert_principal_t) {
        object.get_display_name = Some(get_display_name::<I>);
        object.get_common_name = Some(get_common_name::<I>);
        object.get_locality_name = Some(get_locality_name::<I>);
        object.get_state_or_province_name = Some(get_state_or_province_name::<I>);
        object.get_country_name = Some(get_country_name::<I>);
        object.get_organization_names = Some(get_organization_names::<I>);
        object.get_organization_unit_names = Some(get_organization_unit_names::<I>);
    }

    extern "C" fn get_display_name<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_display_name().into()
    }

    extern "C" fn get_common_name<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_common_name().into()
    }

    extern "C" fn get_locality_name<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_locality_name().into()
    }

    extern "C" fn get_state_or_province_name<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_state_or_province_name().into()
    }

    extern "C" fn get_country_name<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_country_name().into()
    }

    extern "C" fn get_organization_names<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
        names: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let names = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(names) });
        obj.interface.get_organization_names(names)
    }

    extern "C" fn get_organization_unit_names<I: ImplX509certPrincipal>(
        self_: *mut _cef_x509cert_principal_t,
        names: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let names = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(names) });
        obj.interface.get_organization_unit_names(names)
    }
}

wrapper!(
    #[doc = "See [_cef_x509certificate_t] for more documentation."]
    #[derive(Clone)]
    pub struct X509certificate(_cef_x509certificate_t);

    pub fn get_subject(&self) -> X509certPrincipal;
    pub fn get_issuer(&self) -> X509certPrincipal;
    pub fn get_serial_number(&self) -> BinaryValue;
    pub fn get_valid_start(&self) -> Basetime;
    pub fn get_valid_expiry(&self) -> Basetime;
    pub fn get_derencoded(&self) -> BinaryValue;
    pub fn get_pemencoded(&self) -> BinaryValue;
    pub fn get_issuer_chain_size(&self) -> usize;
    pub fn get_derencoded_issuer_chain(
        &self,
        chain_count: *mut usize,
        chain: *mut &mut BinaryValue,
    );
    pub fn get_pemencoded_issuer_chain(
        &self,
        chain_count: *mut usize,
        chain: *mut &mut BinaryValue,
    );
);

pub trait ImplX509certificate: Sized {
    fn get_subject(&self) -> X509certPrincipal {
        Default::default()
    }
    fn get_issuer(&self) -> X509certPrincipal {
        Default::default()
    }
    fn get_serial_number(&self) -> BinaryValue {
        Default::default()
    }
    fn get_valid_start(&self) -> Basetime {
        Default::default()
    }
    fn get_valid_expiry(&self) -> Basetime {
        Default::default()
    }
    fn get_derencoded(&self) -> BinaryValue {
        Default::default()
    }
    fn get_pemencoded(&self) -> BinaryValue {
        Default::default()
    }
    fn get_issuer_chain_size(&self) -> usize {
        Default::default()
    }
    fn get_derencoded_issuer_chain(&self, chain_count: *mut usize, chain: *mut &mut BinaryValue) {}
    fn get_pemencoded_issuer_chain(&self, chain_count: *mut usize, chain: *mut &mut BinaryValue) {}

    fn into_raw(self) -> *mut _cef_x509certificate_t {
        let mut object: _cef_x509certificate_t = unsafe { std::mem::zeroed() };
        impl_cef_x509certificate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_x509certificate_t {
    use super::*;

    pub fn init_methods<I: ImplX509certificate>(object: &mut _cef_x509certificate_t) {
        object.get_subject = Some(get_subject::<I>);
        object.get_issuer = Some(get_issuer::<I>);
        object.get_serial_number = Some(get_serial_number::<I>);
        object.get_valid_start = Some(get_valid_start::<I>);
        object.get_valid_expiry = Some(get_valid_expiry::<I>);
        object.get_derencoded = Some(get_derencoded::<I>);
        object.get_pemencoded = Some(get_pemencoded::<I>);
        object.get_issuer_chain_size = Some(get_issuer_chain_size::<I>);
        object.get_derencoded_issuer_chain = Some(get_derencoded_issuer_chain::<I>);
        object.get_pemencoded_issuer_chain = Some(get_pemencoded_issuer_chain::<I>);
    }

    extern "C" fn get_subject<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> *mut _cef_x509cert_principal_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_subject().into()
    }

    extern "C" fn get_issuer<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> *mut _cef_x509cert_principal_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_issuer().into()
    }

    extern "C" fn get_serial_number<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_serial_number().into()
    }

    extern "C" fn get_valid_start<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_valid_start().into()
    }

    extern "C" fn get_valid_expiry<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_valid_expiry().into()
    }

    extern "C" fn get_derencoded<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_derencoded().into()
    }

    extern "C" fn get_pemencoded<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> *mut _cef_binary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_pemencoded().into()
    }

    extern "C" fn get_issuer_chain_size<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_issuer_chain_size().into()
    }

    extern "C" fn get_derencoded_issuer_chain<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
        chain_count: *mut usize,
        chain: *mut *mut _cef_binary_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_derencoded_issuer_chain(chain_count, chain)
    }

    extern "C" fn get_pemencoded_issuer_chain<I: ImplX509certificate>(
        self_: *mut _cef_x509certificate_t,
        chain_count: *mut usize,
        chain: *mut *mut _cef_binary_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_pemencoded_issuer_chain(chain_count, chain)
    }
}

wrapper!(
    #[doc = "See [_cef_sslstatus_t] for more documentation."]
    #[derive(Clone)]
    pub struct Sslstatus(_cef_sslstatus_t);

    pub fn is_secure_connection(&self) -> ::std::os::raw::c_int;
    pub fn get_cert_status(&self) -> CertStatus;
    pub fn get_sslversion(&self) -> SslVersion;
    pub fn get_content_status(&self) -> SslContentStatus;
    pub fn get_x509certificate(&self) -> X509certificate;
);

pub trait ImplSslstatus: Sized {
    fn is_secure_connection(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_cert_status(&self) -> CertStatus {
        Default::default()
    }
    fn get_sslversion(&self) -> SslVersion {
        Default::default()
    }
    fn get_content_status(&self) -> SslContentStatus {
        Default::default()
    }
    fn get_x509certificate(&self) -> X509certificate {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_sslstatus_t {
        let mut object: _cef_sslstatus_t = unsafe { std::mem::zeroed() };
        impl_cef_sslstatus_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_sslstatus_t {
    use super::*;

    pub fn init_methods<I: ImplSslstatus>(object: &mut _cef_sslstatus_t) {
        object.is_secure_connection = Some(is_secure_connection::<I>);
        object.get_cert_status = Some(get_cert_status::<I>);
        object.get_sslversion = Some(get_sslversion::<I>);
        object.get_content_status = Some(get_content_status::<I>);
        object.get_x509certificate = Some(get_x509certificate::<I>);
    }

    extern "C" fn is_secure_connection<I: ImplSslstatus>(
        self_: *mut _cef_sslstatus_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_secure_connection().into()
    }

    extern "C" fn get_cert_status<I: ImplSslstatus>(
        self_: *mut _cef_sslstatus_t,
    ) -> cef_cert_status_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_cert_status().into()
    }

    extern "C" fn get_sslversion<I: ImplSslstatus>(
        self_: *mut _cef_sslstatus_t,
    ) -> cef_ssl_version_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_sslversion().into()
    }

    extern "C" fn get_content_status<I: ImplSslstatus>(
        self_: *mut _cef_sslstatus_t,
    ) -> cef_ssl_content_status_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_content_status().into()
    }

    extern "C" fn get_x509certificate<I: ImplSslstatus>(
        self_: *mut _cef_sslstatus_t,
    ) -> *mut _cef_x509certificate_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_x509certificate().into()
    }
}

wrapper!(
    #[doc = "See [_cef_navigation_entry_t] for more documentation."]
    #[derive(Clone)]
    pub struct NavigationEntry(_cef_navigation_entry_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn get_url(&self) -> CefStringUserfree;
    pub fn get_display_url(&self) -> CefStringUserfree;
    pub fn get_original_url(&self) -> CefStringUserfree;
    pub fn get_title(&self) -> CefStringUserfree;
    pub fn get_transition_type(&self) -> TransitionType;
    pub fn has_post_data(&self) -> ::std::os::raw::c_int;
    pub fn get_completion_time(&self) -> Basetime;
    pub fn get_http_status_code(&self) -> ::std::os::raw::c_int;
    pub fn get_sslstatus(&self) -> Sslstatus;
);

pub trait ImplNavigationEntry: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_display_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_original_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_title(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_transition_type(&self) -> TransitionType {
        Default::default()
    }
    fn has_post_data(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_completion_time(&self) -> Basetime {
        Default::default()
    }
    fn get_http_status_code(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_sslstatus(&self) -> Sslstatus {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_navigation_entry_t {
        let mut object: _cef_navigation_entry_t = unsafe { std::mem::zeroed() };
        impl_cef_navigation_entry_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_navigation_entry_t {
    use super::*;

    pub fn init_methods<I: ImplNavigationEntry>(object: &mut _cef_navigation_entry_t) {
        object.is_valid = Some(is_valid::<I>);
        object.get_url = Some(get_url::<I>);
        object.get_display_url = Some(get_display_url::<I>);
        object.get_original_url = Some(get_original_url::<I>);
        object.get_title = Some(get_title::<I>);
        object.get_transition_type = Some(get_transition_type::<I>);
        object.has_post_data = Some(has_post_data::<I>);
        object.get_completion_time = Some(get_completion_time::<I>);
        object.get_http_status_code = Some(get_http_status_code::<I>);
        object.get_sslstatus = Some(get_sslstatus::<I>);
    }

    extern "C" fn is_valid<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn get_url<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_url().into()
    }

    extern "C" fn get_display_url<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_display_url().into()
    }

    extern "C" fn get_original_url<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_original_url().into()
    }

    extern "C" fn get_title<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_title().into()
    }

    extern "C" fn get_transition_type<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_transition_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_transition_type().into()
    }

    extern "C" fn has_post_data<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_post_data().into()
    }

    extern "C" fn get_completion_time<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_completion_time().into()
    }

    extern "C" fn get_http_status_code<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_http_status_code().into()
    }

    extern "C" fn get_sslstatus<I: ImplNavigationEntry>(
        self_: *mut _cef_navigation_entry_t,
    ) -> *mut _cef_sslstatus_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_sslstatus().into()
    }
}

wrapper!(
    #[doc = "See [_cef_registration_t] for more documentation."]
    #[derive(Clone)]
    pub struct Registration(_cef_registration_t);
);

pub trait ImplRegistration: Sized {
    fn into_raw(self) -> *mut _cef_registration_t {
        let mut object: _cef_registration_t = unsafe { std::mem::zeroed() };
        impl_cef_registration_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_registration_t {
    use super::*;

    pub fn init_methods<I: ImplRegistration>(object: &mut _cef_registration_t) {}
}

wrapper!(
    #[doc = "See [_cef_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct Callback(_cef_callback_t);

    pub fn cont(&self);
    pub fn cancel(&self);
);

pub trait ImplCallback: Sized {
    fn cont(&self) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_callback_t {
        let mut object: _cef_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_callback_t {
    use super::*;

    pub fn init_methods<I: ImplCallback>(object: &mut _cef_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplCallback>(self_: *mut _cef_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont()
    }

    extern "C" fn cancel<I: ImplCallback>(self_: *mut _cef_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_completion_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct CompletionCallback(_cef_completion_callback_t);

    pub fn on_complete(&self);
);

pub trait ImplCompletionCallback: Sized {
    fn on_complete(&self) {}

    fn into_raw(self) -> *mut _cef_completion_callback_t {
        let mut object: _cef_completion_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_completion_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_completion_callback_t {
    use super::*;

    pub fn init_methods<I: ImplCompletionCallback>(object: &mut _cef_completion_callback_t) {
        object.on_complete = Some(on_complete::<I>);
    }

    extern "C" fn on_complete<I: ImplCompletionCallback>(self_: *mut _cef_completion_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_complete()
    }
}

wrapper!(
    #[doc = "See [_cef_cookie_manager_t] for more documentation."]
    #[derive(Clone)]
    pub struct CookieManager(_cef_cookie_manager_t);

    pub fn visit_all_cookies(&self, visitor: &mut CookieVisitor) -> ::std::os::raw::c_int;
    pub fn visit_url_cookies(
        &self,
        url: &CefString,
        include_http_only: ::std::os::raw::c_int,
        visitor: &mut CookieVisitor,
    ) -> ::std::os::raw::c_int;
    pub fn set_cookie(
        &self,
        url: &CefString,
        cookie: &Cookie,
        callback: &mut SetCookieCallback,
    ) -> ::std::os::raw::c_int;
    pub fn delete_cookies(
        &self,
        url: &CefString,
        cookie_name: &CefString,
        callback: &mut DeleteCookiesCallback,
    ) -> ::std::os::raw::c_int;
    pub fn flush_store(&self, callback: &mut CompletionCallback) -> ::std::os::raw::c_int;
);

pub trait ImplCookieManager: Sized {
    fn visit_all_cookies(&self, visitor: &mut CookieVisitor) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn visit_url_cookies(
        &self,
        url: &CefString,
        include_http_only: ::std::os::raw::c_int,
        visitor: &mut CookieVisitor,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_cookie(
        &self,
        url: &CefString,
        cookie: &Cookie,
        callback: &mut SetCookieCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn delete_cookies(
        &self,
        url: &CefString,
        cookie_name: &CefString,
        callback: &mut DeleteCookiesCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn flush_store(&self, callback: &mut CompletionCallback) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_cookie_manager_t {
        let mut object: _cef_cookie_manager_t = unsafe { std::mem::zeroed() };
        impl_cef_cookie_manager_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_cookie_manager_t {
    use super::*;

    pub fn init_methods<I: ImplCookieManager>(object: &mut _cef_cookie_manager_t) {
        object.visit_all_cookies = Some(visit_all_cookies::<I>);
        object.visit_url_cookies = Some(visit_url_cookies::<I>);
        object.set_cookie = Some(set_cookie::<I>);
        object.delete_cookies = Some(delete_cookies::<I>);
        object.flush_store = Some(flush_store::<I>);
    }

    extern "C" fn visit_all_cookies<I: ImplCookieManager>(
        self_: *mut _cef_cookie_manager_t,
        visitor: *mut _cef_cookie_visitor_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let visitor = &mut CookieVisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface.visit_all_cookies(visitor).into()
    }

    extern "C" fn visit_url_cookies<I: ImplCookieManager>(
        self_: *mut _cef_cookie_manager_t,
        url: *const cef_string_t,
        include_http_only: ::std::os::raw::c_int,
        visitor: *mut _cef_cookie_visitor_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        let visitor = &mut CookieVisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface
            .visit_url_cookies(url, include_http_only, visitor)
            .into()
    }

    extern "C" fn set_cookie<I: ImplCookieManager>(
        self_: *mut _cef_cookie_manager_t,
        url: *const cef_string_t,
        cookie: *const _cef_cookie_t,
        callback: *mut _cef_set_cookie_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        let cookie = &Cookie(unsafe { RefGuard::from_raw_add_ref(cookie) });
        let callback = &mut SetCookieCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.set_cookie(url, cookie, callback).into()
    }

    extern "C" fn delete_cookies<I: ImplCookieManager>(
        self_: *mut _cef_cookie_manager_t,
        url: *const cef_string_t,
        cookie_name: *const cef_string_t,
        callback: *mut _cef_delete_cookies_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        let cookie_name = &CefString::from(cookie_name);
        let callback = &mut DeleteCookiesCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .delete_cookies(url, cookie_name, callback)
            .into()
    }

    extern "C" fn flush_store<I: ImplCookieManager>(
        self_: *mut _cef_cookie_manager_t,
        callback: *mut _cef_completion_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.flush_store(callback).into()
    }
}

wrapper!(
    #[doc = "See [_cef_cookie_visitor_t] for more documentation."]
    #[derive(Clone)]
    pub struct CookieVisitor(_cef_cookie_visitor_t);

    pub fn visit(
        &self,
        cookie: &Cookie,
        count: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
        delete_cookie: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplCookieVisitor: Sized {
    fn visit(
        &self,
        cookie: &Cookie,
        count: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
        delete_cookie: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_cookie_visitor_t {
        let mut object: _cef_cookie_visitor_t = unsafe { std::mem::zeroed() };
        impl_cef_cookie_visitor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_cookie_visitor_t {
    use super::*;

    pub fn init_methods<I: ImplCookieVisitor>(object: &mut _cef_cookie_visitor_t) {
        object.visit = Some(visit::<I>);
    }

    extern "C" fn visit<I: ImplCookieVisitor>(
        self_: *mut _cef_cookie_visitor_t,
        cookie: *const _cef_cookie_t,
        count: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
        delete_cookie: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let cookie = &Cookie(unsafe { RefGuard::from_raw_add_ref(cookie) });
        obj.interface
            .visit(cookie, count, total, delete_cookie)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_set_cookie_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct SetCookieCallback(_cef_set_cookie_callback_t);

    pub fn on_complete(&self, success: ::std::os::raw::c_int);
);

pub trait ImplSetCookieCallback: Sized {
    fn on_complete(&self, success: ::std::os::raw::c_int) {}

    fn into_raw(self) -> *mut _cef_set_cookie_callback_t {
        let mut object: _cef_set_cookie_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_set_cookie_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_set_cookie_callback_t {
    use super::*;

    pub fn init_methods<I: ImplSetCookieCallback>(object: &mut _cef_set_cookie_callback_t) {
        object.on_complete = Some(on_complete::<I>);
    }

    extern "C" fn on_complete<I: ImplSetCookieCallback>(
        self_: *mut _cef_set_cookie_callback_t,
        success: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_complete(success)
    }
}

wrapper!(
    #[doc = "See [_cef_delete_cookies_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct DeleteCookiesCallback(_cef_delete_cookies_callback_t);

    pub fn on_complete(&self, num_deleted: ::std::os::raw::c_int);
);

pub trait ImplDeleteCookiesCallback: Sized {
    fn on_complete(&self, num_deleted: ::std::os::raw::c_int) {}

    fn into_raw(self) -> *mut _cef_delete_cookies_callback_t {
        let mut object: _cef_delete_cookies_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_delete_cookies_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_delete_cookies_callback_t {
    use super::*;

    pub fn init_methods<I: ImplDeleteCookiesCallback>(object: &mut _cef_delete_cookies_callback_t) {
        object.on_complete = Some(on_complete::<I>);
    }

    extern "C" fn on_complete<I: ImplDeleteCookiesCallback>(
        self_: *mut _cef_delete_cookies_callback_t,
        num_deleted: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_complete(num_deleted)
    }
}

wrapper!(
    #[doc = "See [_cef_media_router_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaRouter(_cef_media_router_t);

    pub fn add_observer(&self, observer: &mut MediaObserver) -> Registration;
    pub fn get_source(&self, urn: &CefString) -> MediaSource;
    pub fn notify_current_sinks(&self);
    pub fn create_route(
        &self,
        source: &mut MediaSource,
        sink: &mut MediaSink,
        callback: &mut MediaRouteCreateCallback,
    );
    pub fn notify_current_routes(&self);
);

pub trait ImplMediaRouter: Sized {
    fn add_observer(&self, observer: &mut MediaObserver) -> Registration {
        Default::default()
    }
    fn get_source(&self, urn: &CefString) -> MediaSource {
        Default::default()
    }
    fn notify_current_sinks(&self) {}
    fn create_route(
        &self,
        source: &mut MediaSource,
        sink: &mut MediaSink,
        callback: &mut MediaRouteCreateCallback,
    ) {
    }
    fn notify_current_routes(&self) {}

    fn into_raw(self) -> *mut _cef_media_router_t {
        let mut object: _cef_media_router_t = unsafe { std::mem::zeroed() };
        impl_cef_media_router_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_router_t {
    use super::*;

    pub fn init_methods<I: ImplMediaRouter>(object: &mut _cef_media_router_t) {
        object.add_observer = Some(add_observer::<I>);
        object.get_source = Some(get_source::<I>);
        object.notify_current_sinks = Some(notify_current_sinks::<I>);
        object.create_route = Some(create_route::<I>);
        object.notify_current_routes = Some(notify_current_routes::<I>);
    }

    extern "C" fn add_observer<I: ImplMediaRouter>(
        self_: *mut _cef_media_router_t,
        observer: *mut _cef_media_observer_t,
    ) -> *mut _cef_registration_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let observer = &mut MediaObserver(unsafe { RefGuard::from_raw_add_ref(observer) });
        obj.interface.add_observer(observer).into()
    }

    extern "C" fn get_source<I: ImplMediaRouter>(
        self_: *mut _cef_media_router_t,
        urn: *const cef_string_t,
    ) -> *mut _cef_media_source_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let urn = &CefString::from(urn);
        obj.interface.get_source(urn).into()
    }

    extern "C" fn notify_current_sinks<I: ImplMediaRouter>(self_: *mut _cef_media_router_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.notify_current_sinks()
    }

    extern "C" fn create_route<I: ImplMediaRouter>(
        self_: *mut _cef_media_router_t,
        source: *mut _cef_media_source_t,
        sink: *mut _cef_media_sink_t,
        callback: *mut _cef_media_route_create_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let source = &mut MediaSource(unsafe { RefGuard::from_raw_add_ref(source) });
        let sink = &mut MediaSink(unsafe { RefGuard::from_raw_add_ref(sink) });
        let callback =
            &mut MediaRouteCreateCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.create_route(source, sink, callback)
    }

    extern "C" fn notify_current_routes<I: ImplMediaRouter>(self_: *mut _cef_media_router_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.notify_current_routes()
    }
}

wrapper!(
    #[doc = "See [_cef_media_observer_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaObserver(_cef_media_observer_t);

    pub fn on_sinks(&self, sinks_count: usize, sinks: *const &mut MediaSink);
    pub fn on_routes(&self, routes_count: usize, routes: *const &mut MediaRoute);
    pub fn on_route_state_changed(&self, route: &mut MediaRoute, state: MediaRouteConnectionState);
    pub fn on_route_message_received(
        &self,
        route: &mut MediaRoute,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    );
);

pub trait ImplMediaObserver: Sized {
    fn on_sinks(&self, sinks_count: usize, sinks: *const &mut MediaSink) {}
    fn on_routes(&self, routes_count: usize, routes: *const &mut MediaRoute) {}
    fn on_route_state_changed(&self, route: &mut MediaRoute, state: MediaRouteConnectionState) {}
    fn on_route_message_received(
        &self,
        route: &mut MediaRoute,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) {
    }

    fn into_raw(self) -> *mut _cef_media_observer_t {
        let mut object: _cef_media_observer_t = unsafe { std::mem::zeroed() };
        impl_cef_media_observer_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_observer_t {
    use super::*;

    pub fn init_methods<I: ImplMediaObserver>(object: &mut _cef_media_observer_t) {
        object.on_sinks = Some(on_sinks::<I>);
        object.on_routes = Some(on_routes::<I>);
        object.on_route_state_changed = Some(on_route_state_changed::<I>);
        object.on_route_message_received = Some(on_route_message_received::<I>);
    }

    extern "C" fn on_sinks<I: ImplMediaObserver>(
        self_: *mut _cef_media_observer_t,
        sinks_count: usize,
        sinks: *const *mut _cef_media_sink_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_sinks(sinks_count, sinks)
    }

    extern "C" fn on_routes<I: ImplMediaObserver>(
        self_: *mut _cef_media_observer_t,
        routes_count: usize,
        routes: *const *mut _cef_media_route_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_routes(routes_count, routes)
    }

    extern "C" fn on_route_state_changed<I: ImplMediaObserver>(
        self_: *mut _cef_media_observer_t,
        route: *mut _cef_media_route_t,
        state: cef_media_route_connection_state_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let route = &mut MediaRoute(unsafe { RefGuard::from_raw_add_ref(route) });
        obj.interface.on_route_state_changed(route, state)
    }

    extern "C" fn on_route_message_received<I: ImplMediaObserver>(
        self_: *mut _cef_media_observer_t,
        route: *mut _cef_media_route_t,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let route = &mut MediaRoute(unsafe { RefGuard::from_raw_add_ref(route) });
        obj.interface
            .on_route_message_received(route, message, message_size)
    }
}

wrapper!(
    #[doc = "See [_cef_media_route_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaRoute(_cef_media_route_t);

    pub fn get_id(&self) -> CefStringUserfree;
    pub fn get_source(&self) -> MediaSource;
    pub fn get_sink(&self) -> MediaSink;
    pub fn send_route_message(&self, message: *const ::std::os::raw::c_void, message_size: usize);
    pub fn terminate(&self);
);

pub trait ImplMediaRoute: Sized {
    fn get_id(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_source(&self) -> MediaSource {
        Default::default()
    }
    fn get_sink(&self) -> MediaSink {
        Default::default()
    }
    fn send_route_message(&self, message: *const ::std::os::raw::c_void, message_size: usize) {}
    fn terminate(&self) {}

    fn into_raw(self) -> *mut _cef_media_route_t {
        let mut object: _cef_media_route_t = unsafe { std::mem::zeroed() };
        impl_cef_media_route_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_route_t {
    use super::*;

    pub fn init_methods<I: ImplMediaRoute>(object: &mut _cef_media_route_t) {
        object.get_id = Some(get_id::<I>);
        object.get_source = Some(get_source::<I>);
        object.get_sink = Some(get_sink::<I>);
        object.send_route_message = Some(send_route_message::<I>);
        object.terminate = Some(terminate::<I>);
    }

    extern "C" fn get_id<I: ImplMediaRoute>(
        self_: *mut _cef_media_route_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn get_source<I: ImplMediaRoute>(
        self_: *mut _cef_media_route_t,
    ) -> *mut _cef_media_source_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_source().into()
    }

    extern "C" fn get_sink<I: ImplMediaRoute>(
        self_: *mut _cef_media_route_t,
    ) -> *mut _cef_media_sink_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_sink().into()
    }

    extern "C" fn send_route_message<I: ImplMediaRoute>(
        self_: *mut _cef_media_route_t,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.send_route_message(message, message_size)
    }

    extern "C" fn terminate<I: ImplMediaRoute>(self_: *mut _cef_media_route_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.terminate()
    }
}

wrapper!(
    #[doc = "See [_cef_media_route_create_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaRouteCreateCallback(_cef_media_route_create_callback_t);

    pub fn on_media_route_create_finished(
        &self,
        result: MediaRouteCreateResult,
        error: &CefString,
        route: &mut MediaRoute,
    );
);

pub trait ImplMediaRouteCreateCallback: Sized {
    fn on_media_route_create_finished(
        &self,
        result: MediaRouteCreateResult,
        error: &CefString,
        route: &mut MediaRoute,
    ) {
    }

    fn into_raw(self) -> *mut _cef_media_route_create_callback_t {
        let mut object: _cef_media_route_create_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_media_route_create_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_route_create_callback_t {
    use super::*;

    pub fn init_methods<I: ImplMediaRouteCreateCallback>(
        object: &mut _cef_media_route_create_callback_t,
    ) {
        object.on_media_route_create_finished = Some(on_media_route_create_finished::<I>);
    }

    extern "C" fn on_media_route_create_finished<I: ImplMediaRouteCreateCallback>(
        self_: *mut _cef_media_route_create_callback_t,
        result: cef_media_route_create_result_t,
        error: *const cef_string_t,
        route: *mut _cef_media_route_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let error = &CefString::from(error);
        let route = &mut MediaRoute(unsafe { RefGuard::from_raw_add_ref(route) });
        obj.interface
            .on_media_route_create_finished(result, error, route)
    }
}

wrapper!(
    #[doc = "See [_cef_media_sink_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaSink(_cef_media_sink_t);

    pub fn get_id(&self) -> CefStringUserfree;
    pub fn get_name(&self) -> CefStringUserfree;
    pub fn get_icon_type(&self) -> MediaSinkIconType;
    pub fn get_device_info(&self, callback: &mut MediaSinkDeviceInfoCallback);
    pub fn is_cast_sink(&self) -> ::std::os::raw::c_int;
    pub fn is_dial_sink(&self) -> ::std::os::raw::c_int;
    pub fn is_compatible_with(&self, source: &mut MediaSource) -> ::std::os::raw::c_int;
);

pub trait ImplMediaSink: Sized {
    fn get_id(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_icon_type(&self) -> MediaSinkIconType {
        Default::default()
    }
    fn get_device_info(&self, callback: &mut MediaSinkDeviceInfoCallback) {}
    fn is_cast_sink(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_dial_sink(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_compatible_with(&self, source: &mut MediaSource) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_media_sink_t {
        let mut object: _cef_media_sink_t = unsafe { std::mem::zeroed() };
        impl_cef_media_sink_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_sink_t {
    use super::*;

    pub fn init_methods<I: ImplMediaSink>(object: &mut _cef_media_sink_t) {
        object.get_id = Some(get_id::<I>);
        object.get_name = Some(get_name::<I>);
        object.get_icon_type = Some(get_icon_type::<I>);
        object.get_device_info = Some(get_device_info::<I>);
        object.is_cast_sink = Some(is_cast_sink::<I>);
        object.is_dial_sink = Some(is_dial_sink::<I>);
        object.is_compatible_with = Some(is_compatible_with::<I>);
    }

    extern "C" fn get_id<I: ImplMediaSink>(self_: *mut _cef_media_sink_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn get_name<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_name().into()
    }

    extern "C" fn get_icon_type<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
    ) -> cef_media_sink_icon_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_icon_type().into()
    }

    extern "C" fn get_device_info<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
        callback: *mut _cef_media_sink_device_info_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback =
            &mut MediaSinkDeviceInfoCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.get_device_info(callback)
    }

    extern "C" fn is_cast_sink<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_cast_sink().into()
    }

    extern "C" fn is_dial_sink<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_dial_sink().into()
    }

    extern "C" fn is_compatible_with<I: ImplMediaSink>(
        self_: *mut _cef_media_sink_t,
        source: *mut _cef_media_source_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let source = &mut MediaSource(unsafe { RefGuard::from_raw_add_ref(source) });
        obj.interface.is_compatible_with(source).into()
    }
}

wrapper!(
    #[doc = "See [_cef_media_sink_device_info_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaSinkDeviceInfoCallback(_cef_media_sink_device_info_callback_t);

    pub fn on_media_sink_device_info(&self, device_info: &MediaSinkDeviceInfo);
);

pub trait ImplMediaSinkDeviceInfoCallback: Sized {
    fn on_media_sink_device_info(&self, device_info: &MediaSinkDeviceInfo) {}

    fn into_raw(self) -> *mut _cef_media_sink_device_info_callback_t {
        let mut object: _cef_media_sink_device_info_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_media_sink_device_info_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_sink_device_info_callback_t {
    use super::*;

    pub fn init_methods<I: ImplMediaSinkDeviceInfoCallback>(
        object: &mut _cef_media_sink_device_info_callback_t,
    ) {
        object.on_media_sink_device_info = Some(on_media_sink_device_info::<I>);
    }

    extern "C" fn on_media_sink_device_info<I: ImplMediaSinkDeviceInfoCallback>(
        self_: *mut _cef_media_sink_device_info_callback_t,
        device_info: *const _cef_media_sink_device_info_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let device_info = &MediaSinkDeviceInfo(unsafe { RefGuard::from_raw_add_ref(device_info) });
        obj.interface.on_media_sink_device_info(device_info)
    }
}

wrapper!(
    #[doc = "See [_cef_media_source_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaSource(_cef_media_source_t);

    pub fn get_id(&self) -> CefStringUserfree;
    pub fn is_cast_source(&self) -> ::std::os::raw::c_int;
    pub fn is_dial_source(&self) -> ::std::os::raw::c_int;
);

pub trait ImplMediaSource: Sized {
    fn get_id(&self) -> CefStringUserfree {
        Default::default()
    }
    fn is_cast_source(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_dial_source(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_media_source_t {
        let mut object: _cef_media_source_t = unsafe { std::mem::zeroed() };
        impl_cef_media_source_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_source_t {
    use super::*;

    pub fn init_methods<I: ImplMediaSource>(object: &mut _cef_media_source_t) {
        object.get_id = Some(get_id::<I>);
        object.is_cast_source = Some(is_cast_source::<I>);
        object.is_dial_source = Some(is_dial_source::<I>);
    }

    extern "C" fn get_id<I: ImplMediaSource>(
        self_: *mut _cef_media_source_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn is_cast_source<I: ImplMediaSource>(
        self_: *mut _cef_media_source_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_cast_source().into()
    }

    extern "C" fn is_dial_source<I: ImplMediaSource>(
        self_: *mut _cef_media_source_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_dial_source().into()
    }
}

/// See [_cef_preference_registrar_t] for more documentation.
pub struct PreferenceRegistrar(_cef_preference_registrar_t);

impl From<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn from(value: _cef_preference_registrar_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn into(self) -> _cef_preference_registrar_t {
        self.0
    }
}

impl AsRef<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn as_ref(&self) -> &_cef_preference_registrar_t {
        &self.0
    }
}

impl AsMut<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn as_mut(&mut self) -> &mut _cef_preference_registrar_t {
        &mut self.0
    }
}

impl Default for PreferenceRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_preference_manager_t] for more documentation."]
    #[derive(Clone)]
    pub struct PreferenceManager(_cef_preference_manager_t);

    pub fn has_preference(&self, name: &CefString) -> ::std::os::raw::c_int;
    pub fn get_preference(&self, name: &CefString) -> Value;
    pub fn get_all_preferences(&self, include_defaults: ::std::os::raw::c_int) -> DictionaryValue;
    pub fn can_set_preference(&self, name: &CefString) -> ::std::os::raw::c_int;
    pub fn set_preference(
        &self,
        name: &CefString,
        value: &mut Value,
        error: &mut CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplPreferenceManager: Sized {
    fn has_preference(&self, name: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_preference(&self, name: &CefString) -> Value {
        Default::default()
    }
    fn get_all_preferences(&self, include_defaults: ::std::os::raw::c_int) -> DictionaryValue {
        Default::default()
    }
    fn can_set_preference(&self, name: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_preference(
        &self,
        name: &CefString,
        value: &mut Value,
        error: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_preference_manager_t {
        let mut object: _cef_preference_manager_t = unsafe { std::mem::zeroed() };
        impl_cef_preference_manager_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_preference_manager_t {
    use super::*;

    pub fn init_methods<I: ImplPreferenceManager>(object: &mut _cef_preference_manager_t) {
        object.has_preference = Some(has_preference::<I>);
        object.get_preference = Some(get_preference::<I>);
        object.get_all_preferences = Some(get_all_preferences::<I>);
        object.can_set_preference = Some(can_set_preference::<I>);
        object.set_preference = Some(set_preference::<I>);
    }

    extern "C" fn has_preference<I: ImplPreferenceManager>(
        self_: *mut _cef_preference_manager_t,
        name: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.has_preference(name).into()
    }

    extern "C" fn get_preference<I: ImplPreferenceManager>(
        self_: *mut _cef_preference_manager_t,
        name: *const cef_string_t,
    ) -> *mut _cef_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.get_preference(name).into()
    }

    extern "C" fn get_all_preferences<I: ImplPreferenceManager>(
        self_: *mut _cef_preference_manager_t,
        include_defaults: ::std::os::raw::c_int,
    ) -> *mut _cef_dictionary_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_all_preferences(include_defaults).into()
    }

    extern "C" fn can_set_preference<I: ImplPreferenceManager>(
        self_: *mut _cef_preference_manager_t,
        name: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.can_set_preference(name).into()
    }

    extern "C" fn set_preference<I: ImplPreferenceManager>(
        self_: *mut _cef_preference_manager_t,
        name: *const cef_string_t,
        value: *mut _cef_value_t,
        error: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        let error = &mut CefString::from(error);
        obj.interface.set_preference(name, value, error).into()
    }
}

wrapper!(
    #[doc = "See [_cef_resolve_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResolveCallback(_cef_resolve_callback_t);

    pub fn on_resolve_completed(&self, result: Errorcode, resolved_ips: &mut CefStringList);
);

pub trait ImplResolveCallback: Sized {
    fn on_resolve_completed(&self, result: Errorcode, resolved_ips: &mut CefStringList) {}

    fn into_raw(self) -> *mut _cef_resolve_callback_t {
        let mut object: _cef_resolve_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_resolve_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resolve_callback_t {
    use super::*;

    pub fn init_methods<I: ImplResolveCallback>(object: &mut _cef_resolve_callback_t) {
        object.on_resolve_completed = Some(on_resolve_completed::<I>);
    }

    extern "C" fn on_resolve_completed<I: ImplResolveCallback>(
        self_: *mut _cef_resolve_callback_t,
        result: cef_errorcode_t,
        resolved_ips: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let resolved_ips = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(resolved_ips) });
        obj.interface.on_resolve_completed(result, resolved_ips)
    }
}

wrapper!(
    #[doc = "See [_cef_request_context_t] for more documentation."]
    #[derive(Clone)]
    pub struct RequestContext(_cef_request_context_t);

    pub fn is_same(&self, other: &mut RequestContext) -> ::std::os::raw::c_int;
    pub fn is_sharing_with(&self, other: &mut RequestContext) -> ::std::os::raw::c_int;
    pub fn is_global(&self) -> ::std::os::raw::c_int;
    pub fn get_handler(&self) -> RequestContextHandler;
    pub fn get_cache_path(&self) -> CefStringUserfree;
    pub fn get_cookie_manager(&self, callback: &mut CompletionCallback) -> CookieManager;
    pub fn register_scheme_handler_factory(
        &self,
        scheme_name: &CefString,
        domain_name: &CefString,
        factory: &mut SchemeHandlerFactory,
    ) -> ::std::os::raw::c_int;
    pub fn clear_scheme_handler_factories(&self) -> ::std::os::raw::c_int;
    pub fn clear_certificate_exceptions(&self, callback: &mut CompletionCallback);
    pub fn clear_http_auth_credentials(&self, callback: &mut CompletionCallback);
    pub fn close_all_connections(&self, callback: &mut CompletionCallback);
    pub fn resolve_host(&self, origin: &CefString, callback: &mut ResolveCallback);
    pub fn get_media_router(&self, callback: &mut CompletionCallback) -> MediaRouter;
    pub fn get_website_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
    ) -> Value;
    pub fn set_website_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
        value: &mut Value,
    );
    pub fn get_content_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
    ) -> ContentSettingValues;
    pub fn set_content_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
        value: ContentSettingValues,
    );
    pub fn set_chrome_color_scheme(&self, variant: ColorVariant, user_color: &mut Color);
    pub fn get_chrome_color_scheme_mode(&self) -> ColorVariant;
    pub fn get_chrome_color_scheme_color(&self) -> Color;
    pub fn get_chrome_color_scheme_variant(&self) -> ColorVariant;
);

pub trait ImplRequestContext: ImplPreferenceManager {
    fn is_same(&self, other: &mut RequestContext) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_sharing_with(&self, other: &mut RequestContext) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_global(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_handler(&self) -> RequestContextHandler {
        Default::default()
    }
    fn get_cache_path(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_cookie_manager(&self, callback: &mut CompletionCallback) -> CookieManager {
        Default::default()
    }
    fn register_scheme_handler_factory(
        &self,
        scheme_name: &CefString,
        domain_name: &CefString,
        factory: &mut SchemeHandlerFactory,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn clear_scheme_handler_factories(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn clear_certificate_exceptions(&self, callback: &mut CompletionCallback) {}
    fn clear_http_auth_credentials(&self, callback: &mut CompletionCallback) {}
    fn close_all_connections(&self, callback: &mut CompletionCallback) {}
    fn resolve_host(&self, origin: &CefString, callback: &mut ResolveCallback) {}
    fn get_media_router(&self, callback: &mut CompletionCallback) -> MediaRouter {
        Default::default()
    }
    fn get_website_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
    ) -> Value {
        Default::default()
    }
    fn set_website_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
        value: &mut Value,
    ) {
    }
    fn get_content_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
    ) -> ContentSettingValues {
        Default::default()
    }
    fn set_content_setting(
        &self,
        requesting_url: &CefString,
        top_level_url: &CefString,
        content_type: ContentSettingTypes,
        value: ContentSettingValues,
    ) {
    }
    fn set_chrome_color_scheme(&self, variant: ColorVariant, user_color: &mut Color) {}
    fn get_chrome_color_scheme_mode(&self) -> ColorVariant {
        Default::default()
    }
    fn get_chrome_color_scheme_color(&self) -> Color {
        Default::default()
    }
    fn get_chrome_color_scheme_variant(&self) -> ColorVariant {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_request_context_t {
        let mut object: _cef_request_context_t = unsafe { std::mem::zeroed() };
        impl_cef_preference_manager_t::init_methods::<Self>(&mut object.base);
        impl_cef_request_context_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_request_context_t {
    use super::*;

    pub fn init_methods<I: ImplRequestContext>(object: &mut _cef_request_context_t) {
        object.is_same = Some(is_same::<I>);
        object.is_sharing_with = Some(is_sharing_with::<I>);
        object.is_global = Some(is_global::<I>);
        object.get_handler = Some(get_handler::<I>);
        object.get_cache_path = Some(get_cache_path::<I>);
        object.get_cookie_manager = Some(get_cookie_manager::<I>);
        object.register_scheme_handler_factory = Some(register_scheme_handler_factory::<I>);
        object.clear_scheme_handler_factories = Some(clear_scheme_handler_factories::<I>);
        object.clear_certificate_exceptions = Some(clear_certificate_exceptions::<I>);
        object.clear_http_auth_credentials = Some(clear_http_auth_credentials::<I>);
        object.close_all_connections = Some(close_all_connections::<I>);
        object.resolve_host = Some(resolve_host::<I>);
        object.get_media_router = Some(get_media_router::<I>);
        object.get_website_setting = Some(get_website_setting::<I>);
        object.set_website_setting = Some(set_website_setting::<I>);
        object.get_content_setting = Some(get_content_setting::<I>);
        object.set_content_setting = Some(set_content_setting::<I>);
        object.set_chrome_color_scheme = Some(set_chrome_color_scheme::<I>);
        object.get_chrome_color_scheme_mode = Some(get_chrome_color_scheme_mode::<I>);
        object.get_chrome_color_scheme_color = Some(get_chrome_color_scheme_color::<I>);
        object.get_chrome_color_scheme_variant = Some(get_chrome_color_scheme_variant::<I>);
    }

    extern "C" fn is_same<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        other: *mut _cef_request_context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let other = &mut RequestContext(unsafe { RefGuard::from_raw_add_ref(other) });
        obj.interface.is_same(other).into()
    }

    extern "C" fn is_sharing_with<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        other: *mut _cef_request_context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let other = &mut RequestContext(unsafe { RefGuard::from_raw_add_ref(other) });
        obj.interface.is_sharing_with(other).into()
    }

    extern "C" fn is_global<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_global().into()
    }

    extern "C" fn get_handler<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> *mut _cef_request_context_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_handler().into()
    }

    extern "C" fn get_cache_path<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_cache_path().into()
    }

    extern "C" fn get_cookie_manager<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        callback: *mut _cef_completion_callback_t,
    ) -> *mut _cef_cookie_manager_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.get_cookie_manager(callback).into()
    }

    extern "C" fn register_scheme_handler_factory<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        scheme_name: *const cef_string_t,
        domain_name: *const cef_string_t,
        factory: *mut _cef_scheme_handler_factory_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let scheme_name = &CefString::from(scheme_name);
        let domain_name = &CefString::from(domain_name);
        let factory = &mut SchemeHandlerFactory(unsafe { RefGuard::from_raw_add_ref(factory) });
        obj.interface
            .register_scheme_handler_factory(scheme_name, domain_name, factory)
            .into()
    }

    extern "C" fn clear_scheme_handler_factories<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear_scheme_handler_factories().into()
    }

    extern "C" fn clear_certificate_exceptions<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        callback: *mut _cef_completion_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.clear_certificate_exceptions(callback)
    }

    extern "C" fn clear_http_auth_credentials<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        callback: *mut _cef_completion_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.clear_http_auth_credentials(callback)
    }

    extern "C" fn close_all_connections<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        callback: *mut _cef_completion_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.close_all_connections(callback)
    }

    extern "C" fn resolve_host<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        origin: *const cef_string_t,
        callback: *mut _cef_resolve_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let origin = &CefString::from(origin);
        let callback = &mut ResolveCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.resolve_host(origin, callback)
    }

    extern "C" fn get_media_router<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        callback: *mut _cef_completion_callback_t,
    ) -> *mut _cef_media_router_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut CompletionCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.get_media_router(callback).into()
    }

    extern "C" fn get_website_setting<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        requesting_url: *const cef_string_t,
        top_level_url: *const cef_string_t,
        content_type: cef_content_setting_types_t,
    ) -> *mut _cef_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let requesting_url = &CefString::from(requesting_url);
        let top_level_url = &CefString::from(top_level_url);
        obj.interface
            .get_website_setting(requesting_url, top_level_url, content_type)
            .into()
    }

    extern "C" fn set_website_setting<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        requesting_url: *const cef_string_t,
        top_level_url: *const cef_string_t,
        content_type: cef_content_setting_types_t,
        value: *mut _cef_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let requesting_url = &CefString::from(requesting_url);
        let top_level_url = &CefString::from(top_level_url);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface
            .set_website_setting(requesting_url, top_level_url, content_type, value)
    }

    extern "C" fn get_content_setting<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        requesting_url: *const cef_string_t,
        top_level_url: *const cef_string_t,
        content_type: cef_content_setting_types_t,
    ) -> cef_content_setting_values_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let requesting_url = &CefString::from(requesting_url);
        let top_level_url = &CefString::from(top_level_url);
        obj.interface
            .get_content_setting(requesting_url, top_level_url, content_type)
            .into()
    }

    extern "C" fn set_content_setting<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        requesting_url: *const cef_string_t,
        top_level_url: *const cef_string_t,
        content_type: cef_content_setting_types_t,
        value: cef_content_setting_values_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let requesting_url = &CefString::from(requesting_url);
        let top_level_url = &CefString::from(top_level_url);
        obj.interface
            .set_content_setting(requesting_url, top_level_url, content_type, value)
    }

    extern "C" fn set_chrome_color_scheme<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
        variant: cef_color_variant_t,
        user_color: cef_color_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_chrome_color_scheme(variant, user_color)
    }

    extern "C" fn get_chrome_color_scheme_mode<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> cef_color_variant_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_chrome_color_scheme_mode().into()
    }

    extern "C" fn get_chrome_color_scheme_color<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> cef_color_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_chrome_color_scheme_color().into()
    }

    extern "C" fn get_chrome_color_scheme_variant<I: ImplRequestContext>(
        self_: *mut _cef_request_context_t,
    ) -> cef_color_variant_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_chrome_color_scheme_variant().into()
    }
}

wrapper!(
    #[doc = "See [_cef_browser_t] for more documentation."]
    #[derive(Clone)]
    pub struct Browser(_cef_browser_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn get_host(&self) -> BrowserHost;
    pub fn can_go_back(&self) -> ::std::os::raw::c_int;
    pub fn go_back(&self);
    pub fn can_go_forward(&self) -> ::std::os::raw::c_int;
    pub fn go_forward(&self);
    pub fn is_loading(&self) -> ::std::os::raw::c_int;
    pub fn reload(&self);
    pub fn reload_ignore_cache(&self);
    pub fn stop_load(&self);
    pub fn get_identifier(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut Browser) -> ::std::os::raw::c_int;
    pub fn is_popup(&self) -> ::std::os::raw::c_int;
    pub fn has_document(&self) -> ::std::os::raw::c_int;
    pub fn get_main_frame(&self) -> Frame;
    pub fn get_focused_frame(&self) -> Frame;
    pub fn get_frame_by_identifier(&self, identifier: &CefString) -> Frame;
    pub fn get_frame_by_name(&self, name: &CefString) -> Frame;
    pub fn get_frame_count(&self) -> usize;
    pub fn get_frame_identifiers(&self, identifiers: &mut CefStringList);
    pub fn get_frame_names(&self, names: &mut CefStringList);
);

pub trait ImplBrowser: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_host(&self) -> BrowserHost {
        Default::default()
    }
    fn can_go_back(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn go_back(&self) {}
    fn can_go_forward(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn go_forward(&self) {}
    fn is_loading(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn reload(&self) {}
    fn reload_ignore_cache(&self) {}
    fn stop_load(&self) {}
    fn get_identifier(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut Browser) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_popup(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_document(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_main_frame(&self) -> Frame {
        Default::default()
    }
    fn get_focused_frame(&self) -> Frame {
        Default::default()
    }
    fn get_frame_by_identifier(&self, identifier: &CefString) -> Frame {
        Default::default()
    }
    fn get_frame_by_name(&self, name: &CefString) -> Frame {
        Default::default()
    }
    fn get_frame_count(&self) -> usize {
        Default::default()
    }
    fn get_frame_identifiers(&self, identifiers: &mut CefStringList) {}
    fn get_frame_names(&self, names: &mut CefStringList) {}

    fn into_raw(self) -> *mut _cef_browser_t {
        let mut object: _cef_browser_t = unsafe { std::mem::zeroed() };
        impl_cef_browser_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_browser_t {
    use super::*;

    pub fn init_methods<I: ImplBrowser>(object: &mut _cef_browser_t) {
        object.is_valid = Some(is_valid::<I>);
        object.get_host = Some(get_host::<I>);
        object.can_go_back = Some(can_go_back::<I>);
        object.go_back = Some(go_back::<I>);
        object.can_go_forward = Some(can_go_forward::<I>);
        object.go_forward = Some(go_forward::<I>);
        object.is_loading = Some(is_loading::<I>);
        object.reload = Some(reload::<I>);
        object.reload_ignore_cache = Some(reload_ignore_cache::<I>);
        object.stop_load = Some(stop_load::<I>);
        object.get_identifier = Some(get_identifier::<I>);
        object.is_same = Some(is_same::<I>);
        object.is_popup = Some(is_popup::<I>);
        object.has_document = Some(has_document::<I>);
        object.get_main_frame = Some(get_main_frame::<I>);
        object.get_focused_frame = Some(get_focused_frame::<I>);
        object.get_frame_by_identifier = Some(get_frame_by_identifier::<I>);
        object.get_frame_by_name = Some(get_frame_by_name::<I>);
        object.get_frame_count = Some(get_frame_count::<I>);
        object.get_frame_identifiers = Some(get_frame_identifiers::<I>);
        object.get_frame_names = Some(get_frame_names::<I>);
    }

    extern "C" fn is_valid<I: ImplBrowser>(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn get_host<I: ImplBrowser>(self_: *mut _cef_browser_t) -> *mut _cef_browser_host_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_host().into()
    }

    extern "C" fn can_go_back<I: ImplBrowser>(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.can_go_back().into()
    }

    extern "C" fn go_back<I: ImplBrowser>(self_: *mut _cef_browser_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.go_back()
    }

    extern "C" fn can_go_forward<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.can_go_forward().into()
    }

    extern "C" fn go_forward<I: ImplBrowser>(self_: *mut _cef_browser_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.go_forward()
    }

    extern "C" fn is_loading<I: ImplBrowser>(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_loading().into()
    }

    extern "C" fn reload<I: ImplBrowser>(self_: *mut _cef_browser_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.reload()
    }

    extern "C" fn reload_ignore_cache<I: ImplBrowser>(self_: *mut _cef_browser_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.reload_ignore_cache()
    }

    extern "C" fn stop_load<I: ImplBrowser>(self_: *mut _cef_browser_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.stop_load()
    }

    extern "C" fn get_identifier<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_identifier().into()
    }

    extern "C" fn is_same<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
        that: *mut _cef_browser_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut Browser(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn is_popup<I: ImplBrowser>(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_popup().into()
    }

    extern "C" fn has_document<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_document().into()
    }

    extern "C" fn get_main_frame<I: ImplBrowser>(self_: *mut _cef_browser_t) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_main_frame().into()
    }

    extern "C" fn get_focused_frame<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
    ) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_focused_frame().into()
    }

    extern "C" fn get_frame_by_identifier<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
        identifier: *const cef_string_t,
    ) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let identifier = &CefString::from(identifier);
        obj.interface.get_frame_by_identifier(identifier).into()
    }

    extern "C" fn get_frame_by_name<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
        name: *const cef_string_t,
    ) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.get_frame_by_name(name).into()
    }

    extern "C" fn get_frame_count<I: ImplBrowser>(self_: *mut _cef_browser_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame_count().into()
    }

    extern "C" fn get_frame_identifiers<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
        identifiers: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let identifiers = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(identifiers) });
        obj.interface.get_frame_identifiers(identifiers)
    }

    extern "C" fn get_frame_names<I: ImplBrowser>(
        self_: *mut _cef_browser_t,
        names: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let names = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(names) });
        obj.interface.get_frame_names(names)
    }
}

wrapper!(
    #[doc = "See [_cef_run_file_dialog_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct RunFileDialogCallback(_cef_run_file_dialog_callback_t);

    pub fn on_file_dialog_dismissed(&self, file_paths: &mut CefStringList);
);

pub trait ImplRunFileDialogCallback: Sized {
    fn on_file_dialog_dismissed(&self, file_paths: &mut CefStringList) {}

    fn into_raw(self) -> *mut _cef_run_file_dialog_callback_t {
        let mut object: _cef_run_file_dialog_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_run_file_dialog_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_run_file_dialog_callback_t {
    use super::*;

    pub fn init_methods<I: ImplRunFileDialogCallback>(
        object: &mut _cef_run_file_dialog_callback_t,
    ) {
        object.on_file_dialog_dismissed = Some(on_file_dialog_dismissed::<I>);
    }

    extern "C" fn on_file_dialog_dismissed<I: ImplRunFileDialogCallback>(
        self_: *mut _cef_run_file_dialog_callback_t,
        file_paths: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let file_paths = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(file_paths) });
        obj.interface.on_file_dialog_dismissed(file_paths)
    }
}

wrapper!(
    #[doc = "See [_cef_navigation_entry_visitor_t] for more documentation."]
    #[derive(Clone)]
    pub struct NavigationEntryVisitor(_cef_navigation_entry_visitor_t);

    pub fn visit(
        &self,
        entry: &mut NavigationEntry,
        current: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplNavigationEntryVisitor: Sized {
    fn visit(
        &self,
        entry: &mut NavigationEntry,
        current: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_navigation_entry_visitor_t {
        let mut object: _cef_navigation_entry_visitor_t = unsafe { std::mem::zeroed() };
        impl_cef_navigation_entry_visitor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_navigation_entry_visitor_t {
    use super::*;

    pub fn init_methods<I: ImplNavigationEntryVisitor>(
        object: &mut _cef_navigation_entry_visitor_t,
    ) {
        object.visit = Some(visit::<I>);
    }

    extern "C" fn visit<I: ImplNavigationEntryVisitor>(
        self_: *mut _cef_navigation_entry_visitor_t,
        entry: *mut _cef_navigation_entry_t,
        current: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        total: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let entry = &mut NavigationEntry(unsafe { RefGuard::from_raw_add_ref(entry) });
        obj.interface.visit(entry, current, index, total).into()
    }
}

wrapper!(
    #[doc = "See [_cef_pdf_print_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct PdfPrintCallback(_cef_pdf_print_callback_t);

    pub fn on_pdf_print_finished(&self, path: &CefString, ok: ::std::os::raw::c_int);
);

pub trait ImplPdfPrintCallback: Sized {
    fn on_pdf_print_finished(&self, path: &CefString, ok: ::std::os::raw::c_int) {}

    fn into_raw(self) -> *mut _cef_pdf_print_callback_t {
        let mut object: _cef_pdf_print_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_pdf_print_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_pdf_print_callback_t {
    use super::*;

    pub fn init_methods<I: ImplPdfPrintCallback>(object: &mut _cef_pdf_print_callback_t) {
        object.on_pdf_print_finished = Some(on_pdf_print_finished::<I>);
    }

    extern "C" fn on_pdf_print_finished<I: ImplPdfPrintCallback>(
        self_: *mut _cef_pdf_print_callback_t,
        path: *const cef_string_t,
        ok: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let path = &CefString::from(path);
        obj.interface.on_pdf_print_finished(path, ok)
    }
}

wrapper!(
    #[doc = "See [_cef_download_image_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct DownloadImageCallback(_cef_download_image_callback_t);

    pub fn on_download_image_finished(
        &self,
        image_url: &CefString,
        http_status_code: ::std::os::raw::c_int,
        image: &mut Image,
    );
);

pub trait ImplDownloadImageCallback: Sized {
    fn on_download_image_finished(
        &self,
        image_url: &CefString,
        http_status_code: ::std::os::raw::c_int,
        image: &mut Image,
    ) {
    }

    fn into_raw(self) -> *mut _cef_download_image_callback_t {
        let mut object: _cef_download_image_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_download_image_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_download_image_callback_t {
    use super::*;

    pub fn init_methods<I: ImplDownloadImageCallback>(object: &mut _cef_download_image_callback_t) {
        object.on_download_image_finished = Some(on_download_image_finished::<I>);
    }

    extern "C" fn on_download_image_finished<I: ImplDownloadImageCallback>(
        self_: *mut _cef_download_image_callback_t,
        image_url: *const cef_string_t,
        http_status_code: ::std::os::raw::c_int,
        image: *mut _cef_image_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let image_url = &CefString::from(image_url);
        let image = &mut Image(unsafe { RefGuard::from_raw_add_ref(image) });
        obj.interface
            .on_download_image_finished(image_url, http_status_code, image)
    }
}

wrapper!(
    #[doc = "See [_cef_browser_host_t] for more documentation."]
    #[derive(Clone)]
    pub struct BrowserHost(_cef_browser_host_t);

    pub fn get_browser(&self) -> Browser;
    pub fn close_browser(&self, force_close: ::std::os::raw::c_int);
    pub fn try_close_browser(&self) -> ::std::os::raw::c_int;
    pub fn is_ready_to_be_closed(&self) -> ::std::os::raw::c_int;
    pub fn set_focus(&self, focus: ::std::os::raw::c_int);
    pub fn get_window_handle(&self) -> ::std::os::raw::c_ulong;
    pub fn get_opener_window_handle(&self) -> ::std::os::raw::c_ulong;
    pub fn get_opener_identifier(&self) -> ::std::os::raw::c_int;
    pub fn has_view(&self) -> ::std::os::raw::c_int;
    pub fn get_client(&self) -> Client;
    pub fn get_request_context(&self) -> RequestContext;
    pub fn can_zoom(&self, command: ZoomCommand) -> ::std::os::raw::c_int;
    pub fn zoom(&self, command: ZoomCommand);
    pub fn get_default_zoom_level(&self) -> f64;
    pub fn get_zoom_level(&self) -> f64;
    pub fn set_zoom_level(&self, zoom_level: f64);
    pub fn run_file_dialog(
        &self,
        mode: FileDialogMode,
        title: &CefString,
        default_file_path: &CefString,
        accept_filters: &mut CefStringList,
        callback: &mut RunFileDialogCallback,
    );
    pub fn start_download(&self, url: &CefString);
    pub fn download_image(
        &self,
        image_url: &CefString,
        is_favicon: ::std::os::raw::c_int,
        max_image_size: u32,
        bypass_cache: ::std::os::raw::c_int,
        callback: &mut DownloadImageCallback,
    );
    pub fn print(&self);
    pub fn print_to_pdf(
        &self,
        path: &CefString,
        settings: &PdfPrintSettings,
        callback: &mut PdfPrintCallback,
    );
    pub fn find(
        &self,
        search_text: &CefString,
        forward: ::std::os::raw::c_int,
        match_case: ::std::os::raw::c_int,
        find_next: ::std::os::raw::c_int,
    );
    pub fn stop_finding(&self, clear_selection: ::std::os::raw::c_int);
    pub fn show_dev_tools(
        &self,
        window_info: &WindowInfo,
        client: &mut Client,
        settings: &BrowserSettings,
        inspect_element_at: &Point,
    );
    pub fn close_dev_tools(&self);
    pub fn has_dev_tools(&self) -> ::std::os::raw::c_int;
    pub fn send_dev_tools_message(
        &self,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn execute_dev_tools_method(
        &self,
        message_id: ::std::os::raw::c_int,
        method: &CefString,
        params: &mut DictionaryValue,
    ) -> ::std::os::raw::c_int;
    pub fn add_dev_tools_message_observer(
        &self,
        observer: &mut DevToolsMessageObserver,
    ) -> Registration;
    pub fn get_navigation_entries(
        &self,
        visitor: &mut NavigationEntryVisitor,
        current_only: ::std::os::raw::c_int,
    );
    pub fn replace_misspelling(&self, word: &CefString);
    pub fn add_word_to_dictionary(&self, word: &CefString);
    pub fn is_window_rendering_disabled(&self) -> ::std::os::raw::c_int;
    pub fn was_resized(&self);
    pub fn was_hidden(&self, hidden: ::std::os::raw::c_int);
    pub fn notify_screen_info_changed(&self);
    pub fn invalidate(&self, type_: PaintElementType);
    pub fn send_external_begin_frame(&self);
    pub fn send_key_event(&self, event: &KeyEvent);
    pub fn send_mouse_click_event(
        &self,
        event: &MouseEvent,
        type_: MouseButtonType,
        mouse_up: ::std::os::raw::c_int,
        click_count: ::std::os::raw::c_int,
    );
    pub fn send_mouse_move_event(&self, event: &MouseEvent, mouse_leave: ::std::os::raw::c_int);
    pub fn send_mouse_wheel_event(
        &self,
        event: &MouseEvent,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    );
    pub fn send_touch_event(&self, event: &TouchEvent);
    pub fn send_capture_lost_event(&self);
    pub fn notify_move_or_resize_started(&self);
    pub fn get_windowless_frame_rate(&self) -> ::std::os::raw::c_int;
    pub fn set_windowless_frame_rate(&self, frame_rate: ::std::os::raw::c_int);
    pub fn ime_set_composition(
        &self,
        text: &CefString,
        underlines_count: usize,
        underlines: &CompositionUnderline,
        replacement_range: &Range,
        selection_range: &Range,
    );
    pub fn ime_commit_text(
        &self,
        text: &CefString,
        replacement_range: &Range,
        relative_cursor_pos: ::std::os::raw::c_int,
    );
    pub fn ime_finish_composing_text(&self, keep_selection: ::std::os::raw::c_int);
    pub fn ime_cancel_composition(&self);
    pub fn drag_target_drag_enter(
        &self,
        drag_data: &mut DragData,
        event: &MouseEvent,
        allowed_ops: DragOperationsMask,
    );
    pub fn drag_target_drag_over(&self, event: &MouseEvent, allowed_ops: DragOperationsMask);
    pub fn drag_target_drag_leave(&self);
    pub fn drag_target_drop(&self, event: &MouseEvent);
    pub fn drag_source_ended_at(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        op: DragOperationsMask,
    );
    pub fn drag_source_system_drag_ended(&self);
    pub fn get_visible_navigation_entry(&self) -> NavigationEntry;
    pub fn set_accessibility_state(&self, accessibility_state: State);
    pub fn set_auto_resize_enabled(
        &self,
        enabled: ::std::os::raw::c_int,
        min_size: &Size,
        max_size: &Size,
    );
    pub fn set_audio_muted(&self, mute: ::std::os::raw::c_int);
    pub fn is_audio_muted(&self) -> ::std::os::raw::c_int;
    pub fn is_fullscreen(&self) -> ::std::os::raw::c_int;
    pub fn exit_fullscreen(&self, will_cause_resize: ::std::os::raw::c_int);
    pub fn can_execute_chrome_command(
        &self,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn execute_chrome_command(
        &self,
        command_id: ::std::os::raw::c_int,
        disposition: WindowOpenDisposition,
    );
    pub fn is_render_process_unresponsive(&self) -> ::std::os::raw::c_int;
    pub fn get_runtime_style(&self) -> RuntimeStyle;
);

pub trait ImplBrowserHost: Sized {
    fn get_browser(&self) -> Browser {
        Default::default()
    }
    fn close_browser(&self, force_close: ::std::os::raw::c_int) {}
    fn try_close_browser(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_ready_to_be_closed(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_focus(&self, focus: ::std::os::raw::c_int) {}
    fn get_window_handle(&self) -> ::std::os::raw::c_ulong {
        Default::default()
    }
    fn get_opener_window_handle(&self) -> ::std::os::raw::c_ulong {
        Default::default()
    }
    fn get_opener_identifier(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_view(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_client(&self) -> Client {
        Default::default()
    }
    fn get_request_context(&self) -> RequestContext {
        Default::default()
    }
    fn can_zoom(&self, command: ZoomCommand) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn zoom(&self, command: ZoomCommand) {}
    fn get_default_zoom_level(&self) -> f64 {
        Default::default()
    }
    fn get_zoom_level(&self) -> f64 {
        Default::default()
    }
    fn set_zoom_level(&self, zoom_level: f64) {}
    fn run_file_dialog(
        &self,
        mode: FileDialogMode,
        title: &CefString,
        default_file_path: &CefString,
        accept_filters: &mut CefStringList,
        callback: &mut RunFileDialogCallback,
    ) {
    }
    fn start_download(&self, url: &CefString) {}
    fn download_image(
        &self,
        image_url: &CefString,
        is_favicon: ::std::os::raw::c_int,
        max_image_size: u32,
        bypass_cache: ::std::os::raw::c_int,
        callback: &mut DownloadImageCallback,
    ) {
    }
    fn print(&self) {}
    fn print_to_pdf(
        &self,
        path: &CefString,
        settings: &PdfPrintSettings,
        callback: &mut PdfPrintCallback,
    ) {
    }
    fn find(
        &self,
        search_text: &CefString,
        forward: ::std::os::raw::c_int,
        match_case: ::std::os::raw::c_int,
        find_next: ::std::os::raw::c_int,
    ) {
    }
    fn stop_finding(&self, clear_selection: ::std::os::raw::c_int) {}
    fn show_dev_tools(
        &self,
        window_info: &WindowInfo,
        client: &mut Client,
        settings: &BrowserSettings,
        inspect_element_at: &Point,
    ) {
    }
    fn close_dev_tools(&self) {}
    fn has_dev_tools(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn send_dev_tools_message(
        &self,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn execute_dev_tools_method(
        &self,
        message_id: ::std::os::raw::c_int,
        method: &CefString,
        params: &mut DictionaryValue,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_dev_tools_message_observer(
        &self,
        observer: &mut DevToolsMessageObserver,
    ) -> Registration {
        Default::default()
    }
    fn get_navigation_entries(
        &self,
        visitor: &mut NavigationEntryVisitor,
        current_only: ::std::os::raw::c_int,
    ) {
    }
    fn replace_misspelling(&self, word: &CefString) {}
    fn add_word_to_dictionary(&self, word: &CefString) {}
    fn is_window_rendering_disabled(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn was_resized(&self) {}
    fn was_hidden(&self, hidden: ::std::os::raw::c_int) {}
    fn notify_screen_info_changed(&self) {}
    fn invalidate(&self, type_: PaintElementType) {}
    fn send_external_begin_frame(&self) {}
    fn send_key_event(&self, event: &KeyEvent) {}
    fn send_mouse_click_event(
        &self,
        event: &MouseEvent,
        type_: MouseButtonType,
        mouse_up: ::std::os::raw::c_int,
        click_count: ::std::os::raw::c_int,
    ) {
    }
    fn send_mouse_move_event(&self, event: &MouseEvent, mouse_leave: ::std::os::raw::c_int) {}
    fn send_mouse_wheel_event(
        &self,
        event: &MouseEvent,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    ) {
    }
    fn send_touch_event(&self, event: &TouchEvent) {}
    fn send_capture_lost_event(&self) {}
    fn notify_move_or_resize_started(&self) {}
    fn get_windowless_frame_rate(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_windowless_frame_rate(&self, frame_rate: ::std::os::raw::c_int) {}
    fn ime_set_composition(
        &self,
        text: &CefString,
        underlines_count: usize,
        underlines: &CompositionUnderline,
        replacement_range: &Range,
        selection_range: &Range,
    ) {
    }
    fn ime_commit_text(
        &self,
        text: &CefString,
        replacement_range: &Range,
        relative_cursor_pos: ::std::os::raw::c_int,
    ) {
    }
    fn ime_finish_composing_text(&self, keep_selection: ::std::os::raw::c_int) {}
    fn ime_cancel_composition(&self) {}
    fn drag_target_drag_enter(
        &self,
        drag_data: &mut DragData,
        event: &MouseEvent,
        allowed_ops: DragOperationsMask,
    ) {
    }
    fn drag_target_drag_over(&self, event: &MouseEvent, allowed_ops: DragOperationsMask) {}
    fn drag_target_drag_leave(&self) {}
    fn drag_target_drop(&self, event: &MouseEvent) {}
    fn drag_source_ended_at(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        op: DragOperationsMask,
    ) {
    }
    fn drag_source_system_drag_ended(&self) {}
    fn get_visible_navigation_entry(&self) -> NavigationEntry {
        Default::default()
    }
    fn set_accessibility_state(&self, accessibility_state: State) {}
    fn set_auto_resize_enabled(
        &self,
        enabled: ::std::os::raw::c_int,
        min_size: &Size,
        max_size: &Size,
    ) {
    }
    fn set_audio_muted(&self, mute: ::std::os::raw::c_int) {}
    fn is_audio_muted(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_fullscreen(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn exit_fullscreen(&self, will_cause_resize: ::std::os::raw::c_int) {}
    fn can_execute_chrome_command(
        &self,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn execute_chrome_command(
        &self,
        command_id: ::std::os::raw::c_int,
        disposition: WindowOpenDisposition,
    ) {
    }
    fn is_render_process_unresponsive(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_runtime_style(&self) -> RuntimeStyle {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_browser_host_t {
        let mut object: _cef_browser_host_t = unsafe { std::mem::zeroed() };
        impl_cef_browser_host_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_browser_host_t {
    use super::*;

    pub fn init_methods<I: ImplBrowserHost>(object: &mut _cef_browser_host_t) {
        object.get_browser = Some(get_browser::<I>);
        object.close_browser = Some(close_browser::<I>);
        object.try_close_browser = Some(try_close_browser::<I>);
        object.is_ready_to_be_closed = Some(is_ready_to_be_closed::<I>);
        object.set_focus = Some(set_focus::<I>);
        object.get_window_handle = Some(get_window_handle::<I>);
        object.get_opener_window_handle = Some(get_opener_window_handle::<I>);
        object.get_opener_identifier = Some(get_opener_identifier::<I>);
        object.has_view = Some(has_view::<I>);
        object.get_client = Some(get_client::<I>);
        object.get_request_context = Some(get_request_context::<I>);
        object.can_zoom = Some(can_zoom::<I>);
        object.zoom = Some(zoom::<I>);
        object.get_default_zoom_level = Some(get_default_zoom_level::<I>);
        object.get_zoom_level = Some(get_zoom_level::<I>);
        object.set_zoom_level = Some(set_zoom_level::<I>);
        object.run_file_dialog = Some(run_file_dialog::<I>);
        object.start_download = Some(start_download::<I>);
        object.download_image = Some(download_image::<I>);
        object.print = Some(print::<I>);
        object.print_to_pdf = Some(print_to_pdf::<I>);
        object.find = Some(find::<I>);
        object.stop_finding = Some(stop_finding::<I>);
        object.show_dev_tools = Some(show_dev_tools::<I>);
        object.close_dev_tools = Some(close_dev_tools::<I>);
        object.has_dev_tools = Some(has_dev_tools::<I>);
        object.send_dev_tools_message = Some(send_dev_tools_message::<I>);
        object.execute_dev_tools_method = Some(execute_dev_tools_method::<I>);
        object.add_dev_tools_message_observer = Some(add_dev_tools_message_observer::<I>);
        object.get_navigation_entries = Some(get_navigation_entries::<I>);
        object.replace_misspelling = Some(replace_misspelling::<I>);
        object.add_word_to_dictionary = Some(add_word_to_dictionary::<I>);
        object.is_window_rendering_disabled = Some(is_window_rendering_disabled::<I>);
        object.was_resized = Some(was_resized::<I>);
        object.was_hidden = Some(was_hidden::<I>);
        object.notify_screen_info_changed = Some(notify_screen_info_changed::<I>);
        object.invalidate = Some(invalidate::<I>);
        object.send_external_begin_frame = Some(send_external_begin_frame::<I>);
        object.send_key_event = Some(send_key_event::<I>);
        object.send_mouse_click_event = Some(send_mouse_click_event::<I>);
        object.send_mouse_move_event = Some(send_mouse_move_event::<I>);
        object.send_mouse_wheel_event = Some(send_mouse_wheel_event::<I>);
        object.send_touch_event = Some(send_touch_event::<I>);
        object.send_capture_lost_event = Some(send_capture_lost_event::<I>);
        object.notify_move_or_resize_started = Some(notify_move_or_resize_started::<I>);
        object.get_windowless_frame_rate = Some(get_windowless_frame_rate::<I>);
        object.set_windowless_frame_rate = Some(set_windowless_frame_rate::<I>);
        object.ime_set_composition = Some(ime_set_composition::<I>);
        object.ime_commit_text = Some(ime_commit_text::<I>);
        object.ime_finish_composing_text = Some(ime_finish_composing_text::<I>);
        object.ime_cancel_composition = Some(ime_cancel_composition::<I>);
        object.drag_target_drag_enter = Some(drag_target_drag_enter::<I>);
        object.drag_target_drag_over = Some(drag_target_drag_over::<I>);
        object.drag_target_drag_leave = Some(drag_target_drag_leave::<I>);
        object.drag_target_drop = Some(drag_target_drop::<I>);
        object.drag_source_ended_at = Some(drag_source_ended_at::<I>);
        object.drag_source_system_drag_ended = Some(drag_source_system_drag_ended::<I>);
        object.get_visible_navigation_entry = Some(get_visible_navigation_entry::<I>);
        object.set_accessibility_state = Some(set_accessibility_state::<I>);
        object.set_auto_resize_enabled = Some(set_auto_resize_enabled::<I>);
        object.set_audio_muted = Some(set_audio_muted::<I>);
        object.is_audio_muted = Some(is_audio_muted::<I>);
        object.is_fullscreen = Some(is_fullscreen::<I>);
        object.exit_fullscreen = Some(exit_fullscreen::<I>);
        object.can_execute_chrome_command = Some(can_execute_chrome_command::<I>);
        object.execute_chrome_command = Some(execute_chrome_command::<I>);
        object.is_render_process_unresponsive = Some(is_render_process_unresponsive::<I>);
        object.get_runtime_style = Some(get_runtime_style::<I>);
    }

    extern "C" fn get_browser<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> *mut _cef_browser_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser().into()
    }

    extern "C" fn close_browser<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        force_close: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.close_browser(force_close)
    }

    extern "C" fn try_close_browser<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.try_close_browser().into()
    }

    extern "C" fn is_ready_to_be_closed<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_ready_to_be_closed().into()
    }

    extern "C" fn set_focus<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        focus: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_focus(focus)
    }

    extern "C" fn get_window_handle<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_ulong {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window_handle().into()
    }

    extern "C" fn get_opener_window_handle<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_ulong {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_opener_window_handle().into()
    }

    extern "C" fn get_opener_identifier<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_opener_identifier().into()
    }

    extern "C" fn has_view<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_view().into()
    }

    extern "C" fn get_client<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> *mut _cef_client_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_client().into()
    }

    extern "C" fn get_request_context<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> *mut _cef_request_context_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_request_context().into()
    }

    extern "C" fn can_zoom<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        command: cef_zoom_command_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.can_zoom(command).into()
    }

    extern "C" fn zoom<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        command: cef_zoom_command_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.zoom(command)
    }

    extern "C" fn get_default_zoom_level<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_default_zoom_level().into()
    }

    extern "C" fn get_zoom_level<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_zoom_level().into()
    }

    extern "C" fn set_zoom_level<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        zoom_level: f64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_zoom_level(zoom_level)
    }

    extern "C" fn run_file_dialog<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        mode: cef_file_dialog_mode_t,
        title: *const cef_string_t,
        default_file_path: *const cef_string_t,
        accept_filters: cef_string_list_t,
        callback: *mut _cef_run_file_dialog_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let title = &CefString::from(title);
        let default_file_path = &CefString::from(default_file_path);
        let accept_filters =
            &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(accept_filters) });
        let callback = &mut RunFileDialogCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .run_file_dialog(mode, title, default_file_path, accept_filters, callback)
    }

    extern "C" fn start_download<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.start_download(url)
    }

    extern "C" fn download_image<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        image_url: *const cef_string_t,
        is_favicon: ::std::os::raw::c_int,
        max_image_size: u32,
        bypass_cache: ::std::os::raw::c_int,
        callback: *mut _cef_download_image_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let image_url = &CefString::from(image_url);
        let callback = &mut DownloadImageCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.download_image(
            image_url,
            is_favicon,
            max_image_size,
            bypass_cache,
            callback,
        )
    }

    extern "C" fn print<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.print()
    }

    extern "C" fn print_to_pdf<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        path: *const cef_string_t,
        settings: *const _cef_pdf_print_settings_t,
        callback: *mut _cef_pdf_print_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let path = &CefString::from(path);
        let settings = &PdfPrintSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        let callback = &mut PdfPrintCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.print_to_pdf(path, settings, callback)
    }

    extern "C" fn find<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        search_text: *const cef_string_t,
        forward: ::std::os::raw::c_int,
        match_case: ::std::os::raw::c_int,
        find_next: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let search_text = &CefString::from(search_text);
        obj.interface
            .find(search_text, forward, match_case, find_next)
    }

    extern "C" fn stop_finding<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        clear_selection: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.stop_finding(clear_selection)
    }

    extern "C" fn show_dev_tools<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        window_info: *const _cef_window_info_t,
        client: *mut _cef_client_t,
        settings: *const _cef_browser_settings_t,
        inspect_element_at: *const cef_point_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window_info = &WindowInfo(unsafe { RefGuard::from_raw_add_ref(window_info) });
        let client = &mut Client(unsafe { RefGuard::from_raw_add_ref(client) });
        let settings = &BrowserSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        let inspect_element_at = &Point(unsafe { RefGuard::from_raw_add_ref(inspect_element_at) });
        obj.interface
            .show_dev_tools(window_info, client, settings, inspect_element_at)
    }

    extern "C" fn close_dev_tools<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.close_dev_tools()
    }

    extern "C" fn has_dev_tools<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_dev_tools().into()
    }

    extern "C" fn send_dev_tools_message<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        message: *const ::std::os::raw::c_void,
        message_size: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .send_dev_tools_message(message, message_size)
            .into()
    }

    extern "C" fn execute_dev_tools_method<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        message_id: ::std::os::raw::c_int,
        method: *const cef_string_t,
        params: *mut _cef_dictionary_value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let method = &CefString::from(method);
        let params = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(params) });
        obj.interface
            .execute_dev_tools_method(message_id, method, params)
            .into()
    }

    extern "C" fn add_dev_tools_message_observer<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        observer: *mut _cef_dev_tools_message_observer_t,
    ) -> *mut _cef_registration_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let observer =
            &mut DevToolsMessageObserver(unsafe { RefGuard::from_raw_add_ref(observer) });
        obj.interface
            .add_dev_tools_message_observer(observer)
            .into()
    }

    extern "C" fn get_navigation_entries<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        visitor: *mut _cef_navigation_entry_visitor_t,
        current_only: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let visitor = &mut NavigationEntryVisitor(unsafe { RefGuard::from_raw_add_ref(visitor) });
        obj.interface.get_navigation_entries(visitor, current_only)
    }

    extern "C" fn replace_misspelling<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        word: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let word = &CefString::from(word);
        obj.interface.replace_misspelling(word)
    }

    extern "C" fn add_word_to_dictionary<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        word: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let word = &CefString::from(word);
        obj.interface.add_word_to_dictionary(word)
    }

    extern "C" fn is_window_rendering_disabled<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_window_rendering_disabled().into()
    }

    extern "C" fn was_resized<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.was_resized()
    }

    extern "C" fn was_hidden<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        hidden: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.was_hidden(hidden)
    }

    extern "C" fn notify_screen_info_changed<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.notify_screen_info_changed()
    }

    extern "C" fn invalidate<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        type_: cef_paint_element_type_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.invalidate(type_)
    }

    extern "C" fn send_external_begin_frame<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.send_external_begin_frame()
    }

    extern "C" fn send_key_event<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_key_event_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &KeyEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.send_key_event(event)
    }

    extern "C" fn send_mouse_click_event<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_mouse_event_t,
        type_: cef_mouse_button_type_t,
        mouse_up: ::std::os::raw::c_int,
        click_count: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface
            .send_mouse_click_event(event, type_, mouse_up, click_count)
    }

    extern "C" fn send_mouse_move_event<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_mouse_event_t,
        mouse_leave: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.send_mouse_move_event(event, mouse_leave)
    }

    extern "C" fn send_mouse_wheel_event<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_mouse_event_t,
        delta_x: ::std::os::raw::c_int,
        delta_y: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface
            .send_mouse_wheel_event(event, delta_x, delta_y)
    }

    extern "C" fn send_touch_event<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_touch_event_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &TouchEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.send_touch_event(event)
    }

    extern "C" fn send_capture_lost_event<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.send_capture_lost_event()
    }

    extern "C" fn notify_move_or_resize_started<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.notify_move_or_resize_started()
    }

    extern "C" fn get_windowless_frame_rate<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_windowless_frame_rate().into()
    }

    extern "C" fn set_windowless_frame_rate<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        frame_rate: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_windowless_frame_rate(frame_rate)
    }

    extern "C" fn ime_set_composition<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        text: *const cef_string_t,
        underlines_count: usize,
        underlines: *const cef_composition_underline_t,
        replacement_range: *const cef_range_t,
        selection_range: *const cef_range_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let text = &CefString::from(text);
        let underlines = &CompositionUnderline(unsafe { RefGuard::from_raw_add_ref(underlines) });
        let replacement_range = &Range(unsafe { RefGuard::from_raw_add_ref(replacement_range) });
        let selection_range = &Range(unsafe { RefGuard::from_raw_add_ref(selection_range) });
        obj.interface.ime_set_composition(
            text,
            underlines_count,
            underlines,
            replacement_range,
            selection_range,
        )
    }

    extern "C" fn ime_commit_text<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        text: *const cef_string_t,
        replacement_range: *const cef_range_t,
        relative_cursor_pos: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let text = &CefString::from(text);
        let replacement_range = &Range(unsafe { RefGuard::from_raw_add_ref(replacement_range) });
        obj.interface
            .ime_commit_text(text, replacement_range, relative_cursor_pos)
    }

    extern "C" fn ime_finish_composing_text<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        keep_selection: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.ime_finish_composing_text(keep_selection)
    }

    extern "C" fn ime_cancel_composition<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.ime_cancel_composition()
    }

    extern "C" fn drag_target_drag_enter<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        drag_data: *mut _cef_drag_data_t,
        event: *const cef_mouse_event_t,
        allowed_ops: cef_drag_operations_mask_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let drag_data = &mut DragData(unsafe { RefGuard::from_raw_add_ref(drag_data) });
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface
            .drag_target_drag_enter(drag_data, event, allowed_ops)
    }

    extern "C" fn drag_target_drag_over<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_mouse_event_t,
        allowed_ops: cef_drag_operations_mask_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.drag_target_drag_over(event, allowed_ops)
    }

    extern "C" fn drag_target_drag_leave<I: ImplBrowserHost>(self_: *mut _cef_browser_host_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.drag_target_drag_leave()
    }

    extern "C" fn drag_target_drop<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        event: *const cef_mouse_event_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let event = &MouseEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.drag_target_drop(event)
    }

    extern "C" fn drag_source_ended_at<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        op: cef_drag_operations_mask_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.drag_source_ended_at(x, y, op)
    }

    extern "C" fn drag_source_system_drag_ended<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.drag_source_system_drag_ended()
    }

    extern "C" fn get_visible_navigation_entry<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> *mut _cef_navigation_entry_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_visible_navigation_entry().into()
    }

    extern "C" fn set_accessibility_state<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        accessibility_state: cef_state_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_accessibility_state(accessibility_state)
    }

    extern "C" fn set_auto_resize_enabled<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        enabled: ::std::os::raw::c_int,
        min_size: *const cef_size_t,
        max_size: *const cef_size_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let min_size = &Size(unsafe { RefGuard::from_raw_add_ref(min_size) });
        let max_size = &Size(unsafe { RefGuard::from_raw_add_ref(max_size) });
        obj.interface
            .set_auto_resize_enabled(enabled, min_size, max_size)
    }

    extern "C" fn set_audio_muted<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        mute: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_audio_muted(mute)
    }

    extern "C" fn is_audio_muted<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_audio_muted().into()
    }

    extern "C" fn is_fullscreen<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_fullscreen().into()
    }

    extern "C" fn exit_fullscreen<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        will_cause_resize: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.exit_fullscreen(will_cause_resize)
    }

    extern "C" fn can_execute_chrome_command<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.can_execute_chrome_command(command_id).into()
    }

    extern "C" fn execute_chrome_command<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
        command_id: ::std::os::raw::c_int,
        disposition: cef_window_open_disposition_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .execute_chrome_command(command_id, disposition)
    }

    extern "C" fn is_render_process_unresponsive<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_render_process_unresponsive().into()
    }

    extern "C" fn get_runtime_style<I: ImplBrowserHost>(
        self_: *mut _cef_browser_host_t,
    ) -> cef_runtime_style_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_runtime_style().into()
    }
}

wrapper!(
    #[doc = "See [_cef_audio_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct AudioHandler(_cef_audio_handler_t);

    pub fn get_audio_parameters(
        &self,
        browser: &mut Browser,
        params: &mut AudioParameters,
    ) -> ::std::os::raw::c_int;
    pub fn on_audio_stream_started(
        &self,
        browser: &mut Browser,
        params: &AudioParameters,
        channels: ::std::os::raw::c_int,
    );
    pub fn on_audio_stream_packet(
        &self,
        browser: &mut Browser,
        data: *mut *const f32,
        frames: ::std::os::raw::c_int,
        pts: i64,
    );
    pub fn on_audio_stream_stopped(&self, browser: &mut Browser);
    pub fn on_audio_stream_error(&self, browser: &mut Browser, message: &CefString);
);

pub trait ImplAudioHandler: Sized {
    fn get_audio_parameters(
        &self,
        browser: &mut Browser,
        params: &mut AudioParameters,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_audio_stream_started(
        &self,
        browser: &mut Browser,
        params: &AudioParameters,
        channels: ::std::os::raw::c_int,
    ) {
    }
    fn on_audio_stream_packet(
        &self,
        browser: &mut Browser,
        data: *mut *const f32,
        frames: ::std::os::raw::c_int,
        pts: i64,
    ) {
    }
    fn on_audio_stream_stopped(&self, browser: &mut Browser) {}
    fn on_audio_stream_error(&self, browser: &mut Browser, message: &CefString) {}

    fn into_raw(self) -> *mut _cef_audio_handler_t {
        let mut object: _cef_audio_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_audio_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_audio_handler_t {
    use super::*;

    pub fn init_methods<I: ImplAudioHandler>(object: &mut _cef_audio_handler_t) {
        object.get_audio_parameters = Some(get_audio_parameters::<I>);
        object.on_audio_stream_started = Some(on_audio_stream_started::<I>);
        object.on_audio_stream_packet = Some(on_audio_stream_packet::<I>);
        object.on_audio_stream_stopped = Some(on_audio_stream_stopped::<I>);
        object.on_audio_stream_error = Some(on_audio_stream_error::<I>);
    }

    extern "C" fn get_audio_parameters<I: ImplAudioHandler>(
        self_: *mut _cef_audio_handler_t,
        browser: *mut _cef_browser_t,
        params: *mut cef_audio_parameters_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let params = &mut AudioParameters(unsafe { RefGuard::from_raw_add_ref(params) });
        obj.interface.get_audio_parameters(browser, params).into()
    }

    extern "C" fn on_audio_stream_started<I: ImplAudioHandler>(
        self_: *mut _cef_audio_handler_t,
        browser: *mut _cef_browser_t,
        params: *const cef_audio_parameters_t,
        channels: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let params = &AudioParameters(unsafe { RefGuard::from_raw_add_ref(params) });
        obj.interface
            .on_audio_stream_started(browser, params, channels)
    }

    extern "C" fn on_audio_stream_packet<I: ImplAudioHandler>(
        self_: *mut _cef_audio_handler_t,
        browser: *mut _cef_browser_t,
        data: *mut *const f32,
        frames: ::std::os::raw::c_int,
        pts: i64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_audio_stream_packet(browser, data, frames, pts)
    }

    extern "C" fn on_audio_stream_stopped<I: ImplAudioHandler>(
        self_: *mut _cef_audio_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_audio_stream_stopped(browser)
    }

    extern "C" fn on_audio_stream_error<I: ImplAudioHandler>(
        self_: *mut _cef_audio_handler_t,
        browser: *mut _cef_browser_t,
        message: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let message = &CefString::from(message);
        obj.interface.on_audio_stream_error(browser, message)
    }
}

wrapper!(
    #[doc = "See [_cef_command_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct CommandHandler(_cef_command_handler_t);

    pub fn on_chrome_command(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
        disposition: WindowOpenDisposition,
    ) -> ::std::os::raw::c_int;
    pub fn is_chrome_app_menu_item_visible(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn is_chrome_app_menu_item_enabled(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn is_chrome_page_action_icon_visible(
        &self,
        icon_type: ChromePageActionIconType,
    ) -> ::std::os::raw::c_int;
    pub fn is_chrome_toolbar_button_visible(
        &self,
        button_type: ChromeToolbarButtonType,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplCommandHandler: Sized {
    fn on_chrome_command(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
        disposition: WindowOpenDisposition,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_chrome_app_menu_item_visible(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_chrome_app_menu_item_enabled(
        &self,
        browser: &mut Browser,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_chrome_page_action_icon_visible(
        &self,
        icon_type: ChromePageActionIconType,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_chrome_toolbar_button_visible(
        &self,
        button_type: ChromeToolbarButtonType,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_command_handler_t {
        let mut object: _cef_command_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_command_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_command_handler_t {
    use super::*;

    pub fn init_methods<I: ImplCommandHandler>(object: &mut _cef_command_handler_t) {
        object.on_chrome_command = Some(on_chrome_command::<I>);
        object.is_chrome_app_menu_item_visible = Some(is_chrome_app_menu_item_visible::<I>);
        object.is_chrome_app_menu_item_enabled = Some(is_chrome_app_menu_item_enabled::<I>);
        object.is_chrome_page_action_icon_visible = Some(is_chrome_page_action_icon_visible::<I>);
        object.is_chrome_toolbar_button_visible = Some(is_chrome_toolbar_button_visible::<I>);
    }

    extern "C" fn on_chrome_command<I: ImplCommandHandler>(
        self_: *mut _cef_command_handler_t,
        browser: *mut _cef_browser_t,
        command_id: ::std::os::raw::c_int,
        disposition: cef_window_open_disposition_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_chrome_command(browser, command_id, disposition)
            .into()
    }

    extern "C" fn is_chrome_app_menu_item_visible<I: ImplCommandHandler>(
        self_: *mut _cef_command_handler_t,
        browser: *mut _cef_browser_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .is_chrome_app_menu_item_visible(browser, command_id)
            .into()
    }

    extern "C" fn is_chrome_app_menu_item_enabled<I: ImplCommandHandler>(
        self_: *mut _cef_command_handler_t,
        browser: *mut _cef_browser_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .is_chrome_app_menu_item_enabled(browser, command_id)
            .into()
    }

    extern "C" fn is_chrome_page_action_icon_visible<I: ImplCommandHandler>(
        self_: *mut _cef_command_handler_t,
        icon_type: cef_chrome_page_action_icon_type_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .is_chrome_page_action_icon_visible(icon_type)
            .into()
    }

    extern "C" fn is_chrome_toolbar_button_visible<I: ImplCommandHandler>(
        self_: *mut _cef_command_handler_t,
        button_type: cef_chrome_toolbar_button_type_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .is_chrome_toolbar_button_visible(button_type)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_menu_model_delegate_t] for more documentation."]
    #[derive(Clone)]
    pub struct MenuModelDelegate(_cef_menu_model_delegate_t);

    pub fn execute_command(
        &self,
        menu_model: &mut MenuModel,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    );
    pub fn mouse_outside_menu(&self, menu_model: &mut MenuModel, screen_point: &Point);
    pub fn unhandled_open_submenu(&self, menu_model: &mut MenuModel, is_rtl: ::std::os::raw::c_int);
    pub fn unhandled_close_submenu(
        &self,
        menu_model: &mut MenuModel,
        is_rtl: ::std::os::raw::c_int,
    );
    pub fn menu_will_show(&self, menu_model: &mut MenuModel);
    pub fn menu_closed(&self, menu_model: &mut MenuModel);
    pub fn format_label(
        &self,
        menu_model: &mut MenuModel,
        label: &mut CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplMenuModelDelegate: Sized {
    fn execute_command(
        &self,
        menu_model: &mut MenuModel,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    ) {
    }
    fn mouse_outside_menu(&self, menu_model: &mut MenuModel, screen_point: &Point) {}
    fn unhandled_open_submenu(&self, menu_model: &mut MenuModel, is_rtl: ::std::os::raw::c_int) {}
    fn unhandled_close_submenu(&self, menu_model: &mut MenuModel, is_rtl: ::std::os::raw::c_int) {}
    fn menu_will_show(&self, menu_model: &mut MenuModel) {}
    fn menu_closed(&self, menu_model: &mut MenuModel) {}
    fn format_label(
        &self,
        menu_model: &mut MenuModel,
        label: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_menu_model_delegate_t {
        let mut object: _cef_menu_model_delegate_t = unsafe { std::mem::zeroed() };
        impl_cef_menu_model_delegate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_menu_model_delegate_t {
    use super::*;

    pub fn init_methods<I: ImplMenuModelDelegate>(object: &mut _cef_menu_model_delegate_t) {
        object.execute_command = Some(execute_command::<I>);
        object.mouse_outside_menu = Some(mouse_outside_menu::<I>);
        object.unhandled_open_submenu = Some(unhandled_open_submenu::<I>);
        object.unhandled_close_submenu = Some(unhandled_close_submenu::<I>);
        object.menu_will_show = Some(menu_will_show::<I>);
        object.menu_closed = Some(menu_closed::<I>);
        object.format_label = Some(format_label::<I>);
    }

    extern "C" fn execute_command<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        event_flags: cef_event_flags_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        obj.interface
            .execute_command(menu_model, command_id, event_flags)
    }

    extern "C" fn mouse_outside_menu<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
        screen_point: *const cef_point_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        let screen_point = &Point(unsafe { RefGuard::from_raw_add_ref(screen_point) });
        obj.interface.mouse_outside_menu(menu_model, screen_point)
    }

    extern "C" fn unhandled_open_submenu<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
        is_rtl: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        obj.interface.unhandled_open_submenu(menu_model, is_rtl)
    }

    extern "C" fn unhandled_close_submenu<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
        is_rtl: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        obj.interface.unhandled_close_submenu(menu_model, is_rtl)
    }

    extern "C" fn menu_will_show<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        obj.interface.menu_will_show(menu_model)
    }

    extern "C" fn menu_closed<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        obj.interface.menu_closed(menu_model)
    }

    extern "C" fn format_label<I: ImplMenuModelDelegate>(
        self_: *mut _cef_menu_model_delegate_t,
        menu_model: *mut _cef_menu_model_t,
        label: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        let label = &mut CefString::from(label);
        obj.interface.format_label(menu_model, label).into()
    }
}

wrapper!(
    #[doc = "See [_cef_menu_model_t] for more documentation."]
    #[derive(Clone)]
    pub struct MenuModel(_cef_menu_model_t);

    pub fn is_sub_menu(&self) -> ::std::os::raw::c_int;
    pub fn clear(&self) -> ::std::os::raw::c_int;
    pub fn get_count(&self) -> usize;
    pub fn add_separator(&self) -> ::std::os::raw::c_int;
    pub fn add_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn add_check_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn add_radio_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn add_sub_menu(&self, command_id: ::std::os::raw::c_int, label: &CefString) -> MenuModel;
    pub fn insert_separator_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn insert_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn insert_check_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn insert_radio_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn insert_sub_menu_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> MenuModel;
    pub fn remove(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn remove_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn get_index_of(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn get_command_id_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_command_id_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_label(&self, command_id: ::std::os::raw::c_int) -> CefStringUserfree;
    pub fn get_label_at(&self, index: usize) -> CefStringUserfree;
    pub fn set_label(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn set_label_at(&self, index: usize, label: &CefString) -> ::std::os::raw::c_int;
    pub fn get_type(&self, command_id: ::std::os::raw::c_int) -> MenuItemType;
    pub fn get_type_at(&self, index: usize) -> MenuItemType;
    pub fn get_group_id(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn get_group_id_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_group_id(
        &self,
        command_id: ::std::os::raw::c_int,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_group_id_at(
        &self,
        index: usize,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_sub_menu(&self, command_id: ::std::os::raw::c_int) -> MenuModel;
    pub fn get_sub_menu_at(&self, index: usize) -> MenuModel;
    pub fn is_visible(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn is_visible_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_visible(
        &self,
        command_id: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_visible_at(
        &self,
        index: usize,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn is_enabled(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn is_enabled_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_enabled(
        &self,
        command_id: ::std::os::raw::c_int,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_enabled_at(
        &self,
        index: usize,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn is_checked(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn is_checked_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_checked(
        &self,
        command_id: ::std::os::raw::c_int,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_checked_at(
        &self,
        index: usize,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn has_accelerator(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn has_accelerator_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn set_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_accelerator_at(
        &self,
        index: usize,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn remove_accelerator(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn remove_accelerator_at(&self, index: usize) -> ::std::os::raw::c_int;
    pub fn get_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_accelerator_at(
        &self,
        index: usize,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn set_color(
        &self,
        command_id: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int;
    pub fn set_color_at(
        &self,
        index: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int;
    pub fn get_color(
        &self,
        command_id: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int;
    pub fn get_color_at(
        &self,
        index: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int;
    pub fn set_font_list(
        &self,
        command_id: ::std::os::raw::c_int,
        font_list: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn set_font_list_at(
        &self,
        index: ::std::os::raw::c_int,
        font_list: &CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplMenuModel: Sized {
    fn is_sub_menu(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn clear(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_count(&self) -> usize {
        Default::default()
    }
    fn add_separator(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_check_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_radio_item(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn add_sub_menu(&self, command_id: ::std::os::raw::c_int, label: &CefString) -> MenuModel {
        Default::default()
    }
    fn insert_separator_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn insert_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn insert_check_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn insert_radio_item_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn insert_sub_menu_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> MenuModel {
        Default::default()
    }
    fn remove(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_index_of(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_command_id_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_command_id_at(
        &self,
        index: usize,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_label(&self, command_id: ::std::os::raw::c_int) -> CefStringUserfree {
        Default::default()
    }
    fn get_label_at(&self, index: usize) -> CefStringUserfree {
        Default::default()
    }
    fn set_label(
        &self,
        command_id: ::std::os::raw::c_int,
        label: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_label_at(&self, index: usize, label: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_type(&self, command_id: ::std::os::raw::c_int) -> MenuItemType {
        Default::default()
    }
    fn get_type_at(&self, index: usize) -> MenuItemType {
        Default::default()
    }
    fn get_group_id(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_group_id_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_group_id(
        &self,
        command_id: ::std::os::raw::c_int,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_group_id_at(
        &self,
        index: usize,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_sub_menu(&self, command_id: ::std::os::raw::c_int) -> MenuModel {
        Default::default()
    }
    fn get_sub_menu_at(&self, index: usize) -> MenuModel {
        Default::default()
    }
    fn is_visible(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_visible_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_visible(
        &self,
        command_id: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_visible_at(
        &self,
        index: usize,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_enabled(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_enabled_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_enabled(
        &self,
        command_id: ::std::os::raw::c_int,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_enabled_at(
        &self,
        index: usize,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_checked(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_checked_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_checked(
        &self,
        command_id: ::std::os::raw::c_int,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_checked_at(
        &self,
        index: usize,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_accelerator(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_accelerator_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_accelerator_at(
        &self,
        index: usize,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove_accelerator(&self, command_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn remove_accelerator_at(&self, index: usize) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_accelerator_at(
        &self,
        index: usize,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_color(
        &self,
        command_id: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_color_at(
        &self,
        index: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_color(
        &self,
        command_id: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_color_at(
        &self,
        index: ::std::os::raw::c_int,
        color_type: MenuColorType,
        color: &mut Color,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_font_list(
        &self,
        command_id: ::std::os::raw::c_int,
        font_list: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_font_list_at(
        &self,
        index: ::std::os::raw::c_int,
        font_list: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_menu_model_t {
        let mut object: _cef_menu_model_t = unsafe { std::mem::zeroed() };
        impl_cef_menu_model_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_menu_model_t {
    use super::*;

    pub fn init_methods<I: ImplMenuModel>(object: &mut _cef_menu_model_t) {
        object.is_sub_menu = Some(is_sub_menu::<I>);
        object.clear = Some(clear::<I>);
        object.get_count = Some(get_count::<I>);
        object.add_separator = Some(add_separator::<I>);
        object.add_item = Some(add_item::<I>);
        object.add_check_item = Some(add_check_item::<I>);
        object.add_radio_item = Some(add_radio_item::<I>);
        object.add_sub_menu = Some(add_sub_menu::<I>);
        object.insert_separator_at = Some(insert_separator_at::<I>);
        object.insert_item_at = Some(insert_item_at::<I>);
        object.insert_check_item_at = Some(insert_check_item_at::<I>);
        object.insert_radio_item_at = Some(insert_radio_item_at::<I>);
        object.insert_sub_menu_at = Some(insert_sub_menu_at::<I>);
        object.remove = Some(remove::<I>);
        object.remove_at = Some(remove_at::<I>);
        object.get_index_of = Some(get_index_of::<I>);
        object.get_command_id_at = Some(get_command_id_at::<I>);
        object.set_command_id_at = Some(set_command_id_at::<I>);
        object.get_label = Some(get_label::<I>);
        object.get_label_at = Some(get_label_at::<I>);
        object.set_label = Some(set_label::<I>);
        object.set_label_at = Some(set_label_at::<I>);
        object.get_type = Some(get_type::<I>);
        object.get_type_at = Some(get_type_at::<I>);
        object.get_group_id = Some(get_group_id::<I>);
        object.get_group_id_at = Some(get_group_id_at::<I>);
        object.set_group_id = Some(set_group_id::<I>);
        object.set_group_id_at = Some(set_group_id_at::<I>);
        object.get_sub_menu = Some(get_sub_menu::<I>);
        object.get_sub_menu_at = Some(get_sub_menu_at::<I>);
        object.is_visible = Some(is_visible::<I>);
        object.is_visible_at = Some(is_visible_at::<I>);
        object.set_visible = Some(set_visible::<I>);
        object.set_visible_at = Some(set_visible_at::<I>);
        object.is_enabled = Some(is_enabled::<I>);
        object.is_enabled_at = Some(is_enabled_at::<I>);
        object.set_enabled = Some(set_enabled::<I>);
        object.set_enabled_at = Some(set_enabled_at::<I>);
        object.is_checked = Some(is_checked::<I>);
        object.is_checked_at = Some(is_checked_at::<I>);
        object.set_checked = Some(set_checked::<I>);
        object.set_checked_at = Some(set_checked_at::<I>);
        object.has_accelerator = Some(has_accelerator::<I>);
        object.has_accelerator_at = Some(has_accelerator_at::<I>);
        object.set_accelerator = Some(set_accelerator::<I>);
        object.set_accelerator_at = Some(set_accelerator_at::<I>);
        object.remove_accelerator = Some(remove_accelerator::<I>);
        object.remove_accelerator_at = Some(remove_accelerator_at::<I>);
        object.get_accelerator = Some(get_accelerator::<I>);
        object.get_accelerator_at = Some(get_accelerator_at::<I>);
        object.set_color = Some(set_color::<I>);
        object.set_color_at = Some(set_color_at::<I>);
        object.get_color = Some(get_color::<I>);
        object.get_color_at = Some(get_color_at::<I>);
        object.set_font_list = Some(set_font_list::<I>);
        object.set_font_list_at = Some(set_font_list_at::<I>);
    }

    extern "C" fn is_sub_menu<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_sub_menu().into()
    }

    extern "C" fn clear<I: ImplMenuModel>(self_: *mut _cef_menu_model_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear().into()
    }

    extern "C" fn get_count<I: ImplMenuModel>(self_: *mut _cef_menu_model_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_count().into()
    }

    extern "C" fn add_separator<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.add_separator().into()
    }

    extern "C" fn add_item<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface.add_item(command_id, label).into()
    }

    extern "C" fn add_check_item<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface.add_check_item(command_id, label).into()
    }

    extern "C" fn add_radio_item<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface
            .add_radio_item(command_id, label, group_id)
            .into()
    }

    extern "C" fn add_sub_menu<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> *mut _cef_menu_model_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface.add_sub_menu(command_id, label).into()
    }

    extern "C" fn insert_separator_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.insert_separator_at(index).into()
    }

    extern "C" fn insert_item_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface
            .insert_item_at(index, command_id, label)
            .into()
    }

    extern "C" fn insert_check_item_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface
            .insert_check_item_at(index, command_id, label)
            .into()
    }

    extern "C" fn insert_radio_item_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface
            .insert_radio_item_at(index, command_id, label, group_id)
            .into()
    }

    extern "C" fn insert_sub_menu_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> *mut _cef_menu_model_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface
            .insert_sub_menu_at(index, command_id, label)
            .into()
    }

    extern "C" fn remove<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove(command_id).into()
    }

    extern "C" fn remove_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_at(index).into()
    }

    extern "C" fn get_index_of<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_index_of(command_id).into()
    }

    extern "C" fn get_command_id_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_command_id_at(index).into()
    }

    extern "C" fn set_command_id_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_command_id_at(index, command_id).into()
    }

    extern "C" fn get_label<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_label(command_id).into()
    }

    extern "C" fn get_label_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_label_at(index).into()
    }

    extern "C" fn set_label<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface.set_label(command_id, label).into()
    }

    extern "C" fn set_label_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        label: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let label = &CefString::from(label);
        obj.interface.set_label_at(index, label).into()
    }

    extern "C" fn get_type<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> cef_menu_item_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type(command_id).into()
    }

    extern "C" fn get_type_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> cef_menu_item_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type_at(index).into()
    }

    extern "C" fn get_group_id<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_group_id(command_id).into()
    }

    extern "C" fn get_group_id_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_group_id_at(index).into()
    }

    extern "C" fn set_group_id<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_group_id(command_id, group_id).into()
    }

    extern "C" fn set_group_id_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        group_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_group_id_at(index, group_id).into()
    }

    extern "C" fn get_sub_menu<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> *mut _cef_menu_model_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_sub_menu(command_id).into()
    }

    extern "C" fn get_sub_menu_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> *mut _cef_menu_model_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_sub_menu_at(index).into()
    }

    extern "C" fn is_visible<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_visible(command_id).into()
    }

    extern "C" fn is_visible_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_visible_at(index).into()
    }

    extern "C" fn set_visible<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_visible(command_id, visible).into()
    }

    extern "C" fn set_visible_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        visible: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_visible_at(index, visible).into()
    }

    extern "C" fn is_enabled<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_enabled(command_id).into()
    }

    extern "C" fn is_enabled_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_enabled_at(index).into()
    }

    extern "C" fn set_enabled<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_enabled(command_id, enabled).into()
    }

    extern "C" fn set_enabled_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_enabled_at(index, enabled).into()
    }

    extern "C" fn is_checked<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_checked(command_id).into()
    }

    extern "C" fn is_checked_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_checked_at(index).into()
    }

    extern "C" fn set_checked<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_checked(command_id, checked).into()
    }

    extern "C" fn set_checked_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        checked: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_checked_at(index, checked).into()
    }

    extern "C" fn has_accelerator<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_accelerator(command_id).into()
    }

    extern "C" fn has_accelerator_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_accelerator_at(index).into()
    }

    extern "C" fn set_accelerator<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .set_accelerator(
                command_id,
                key_code,
                shift_pressed,
                ctrl_pressed,
                alt_pressed,
            )
            .into()
    }

    extern "C" fn set_accelerator_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .set_accelerator_at(index, key_code, shift_pressed, ctrl_pressed, alt_pressed)
            .into()
    }

    extern "C" fn remove_accelerator<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_accelerator(command_id).into()
    }

    extern "C" fn remove_accelerator_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_accelerator_at(index).into()
    }

    extern "C" fn get_accelerator<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_accelerator(
                command_id,
                key_code,
                shift_pressed,
                ctrl_pressed,
                alt_pressed,
            )
            .into()
    }

    extern "C" fn get_accelerator_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: usize,
        key_code: *mut ::std::os::raw::c_int,
        shift_pressed: *mut ::std::os::raw::c_int,
        ctrl_pressed: *mut ::std::os::raw::c_int,
        alt_pressed: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_accelerator_at(index, key_code, shift_pressed, ctrl_pressed, alt_pressed)
            .into()
    }

    extern "C" fn set_color<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        color_type: cef_menu_color_type_t,
        color: cef_color_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .set_color(command_id, color_type, color)
            .into()
    }

    extern "C" fn set_color_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: ::std::os::raw::c_int,
        color_type: cef_menu_color_type_t,
        color: cef_color_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_color_at(index, color_type, color).into()
    }

    extern "C" fn get_color<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        color_type: cef_menu_color_type_t,
        color: *mut cef_color_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_color(command_id, color_type, color)
            .into()
    }

    extern "C" fn get_color_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: ::std::os::raw::c_int,
        color_type: cef_menu_color_type_t,
        color: *mut cef_color_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_color_at(index, color_type, color).into()
    }

    extern "C" fn set_font_list<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        command_id: ::std::os::raw::c_int,
        font_list: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let font_list = &CefString::from(font_list);
        obj.interface.set_font_list(command_id, font_list).into()
    }

    extern "C" fn set_font_list_at<I: ImplMenuModel>(
        self_: *mut _cef_menu_model_t,
        index: ::std::os::raw::c_int,
        font_list: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let font_list = &CefString::from(font_list);
        obj.interface.set_font_list_at(index, font_list).into()
    }
}

wrapper!(
    #[doc = "See [_cef_run_context_menu_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct RunContextMenuCallback(_cef_run_context_menu_callback_t);

    pub fn cont(&self, command_id: ::std::os::raw::c_int, event_flags: EventFlags);
    pub fn cancel(&self);
);

pub trait ImplRunContextMenuCallback: Sized {
    fn cont(&self, command_id: ::std::os::raw::c_int, event_flags: EventFlags) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_run_context_menu_callback_t {
        let mut object: _cef_run_context_menu_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_run_context_menu_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_run_context_menu_callback_t {
    use super::*;

    pub fn init_methods<I: ImplRunContextMenuCallback>(
        object: &mut _cef_run_context_menu_callback_t,
    ) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplRunContextMenuCallback>(
        self_: *mut _cef_run_context_menu_callback_t,
        command_id: ::std::os::raw::c_int,
        event_flags: cef_event_flags_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(command_id, event_flags)
    }

    extern "C" fn cancel<I: ImplRunContextMenuCallback>(
        self_: *mut _cef_run_context_menu_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_run_quick_menu_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct RunQuickMenuCallback(_cef_run_quick_menu_callback_t);

    pub fn cont(&self, command_id: ::std::os::raw::c_int, event_flags: EventFlags);
    pub fn cancel(&self);
);

pub trait ImplRunQuickMenuCallback: Sized {
    fn cont(&self, command_id: ::std::os::raw::c_int, event_flags: EventFlags) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_run_quick_menu_callback_t {
        let mut object: _cef_run_quick_menu_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_run_quick_menu_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_run_quick_menu_callback_t {
    use super::*;

    pub fn init_methods<I: ImplRunQuickMenuCallback>(object: &mut _cef_run_quick_menu_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplRunQuickMenuCallback>(
        self_: *mut _cef_run_quick_menu_callback_t,
        command_id: ::std::os::raw::c_int,
        event_flags: cef_event_flags_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(command_id, event_flags)
    }

    extern "C" fn cancel<I: ImplRunQuickMenuCallback>(self_: *mut _cef_run_quick_menu_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_context_menu_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct ContextMenuHandler(_cef_context_menu_handler_t);

    pub fn on_before_context_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        model: &mut MenuModel,
    );
    pub fn run_context_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        model: &mut MenuModel,
        callback: &mut RunContextMenuCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_context_menu_command(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    ) -> ::std::os::raw::c_int;
    pub fn on_context_menu_dismissed(&self, browser: &mut Browser, frame: &mut Frame);
    pub fn run_quick_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        location: &Point,
        size: &Size,
        edit_state_flags: QuickMenuEditStateFlags,
        callback: &mut RunQuickMenuCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_quick_menu_command(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    ) -> ::std::os::raw::c_int;
    pub fn on_quick_menu_dismissed(&self, browser: &mut Browser, frame: &mut Frame);
);

pub trait ImplContextMenuHandler: Sized {
    fn on_before_context_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        model: &mut MenuModel,
    ) {
    }
    fn run_context_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        model: &mut MenuModel,
        callback: &mut RunContextMenuCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_context_menu_command(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        params: &mut ContextMenuParams,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_context_menu_dismissed(&self, browser: &mut Browser, frame: &mut Frame) {}
    fn run_quick_menu(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        location: &Point,
        size: &Size,
        edit_state_flags: QuickMenuEditStateFlags,
        callback: &mut RunQuickMenuCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_quick_menu_command(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        command_id: ::std::os::raw::c_int,
        event_flags: EventFlags,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_quick_menu_dismissed(&self, browser: &mut Browser, frame: &mut Frame) {}

    fn into_raw(self) -> *mut _cef_context_menu_handler_t {
        let mut object: _cef_context_menu_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_context_menu_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_context_menu_handler_t {
    use super::*;

    pub fn init_methods<I: ImplContextMenuHandler>(object: &mut _cef_context_menu_handler_t) {
        object.on_before_context_menu = Some(on_before_context_menu::<I>);
        object.run_context_menu = Some(run_context_menu::<I>);
        object.on_context_menu_command = Some(on_context_menu_command::<I>);
        object.on_context_menu_dismissed = Some(on_context_menu_dismissed::<I>);
        object.run_quick_menu = Some(run_quick_menu::<I>);
        object.on_quick_menu_command = Some(on_quick_menu_command::<I>);
        object.on_quick_menu_dismissed = Some(on_quick_menu_dismissed::<I>);
    }

    extern "C" fn on_before_context_menu<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        params: *mut _cef_context_menu_params_t,
        model: *mut _cef_menu_model_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let params = &mut ContextMenuParams(unsafe { RefGuard::from_raw_add_ref(params) });
        let model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(model) });
        obj.interface
            .on_before_context_menu(browser, frame, params, model)
    }

    extern "C" fn run_context_menu<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        params: *mut _cef_context_menu_params_t,
        model: *mut _cef_menu_model_t,
        callback: *mut _cef_run_context_menu_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let params = &mut ContextMenuParams(unsafe { RefGuard::from_raw_add_ref(params) });
        let model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(model) });
        let callback = &mut RunContextMenuCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .run_context_menu(browser, frame, params, model, callback)
            .into()
    }

    extern "C" fn on_context_menu_command<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        params: *mut _cef_context_menu_params_t,
        command_id: ::std::os::raw::c_int,
        event_flags: cef_event_flags_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let params = &mut ContextMenuParams(unsafe { RefGuard::from_raw_add_ref(params) });
        obj.interface
            .on_context_menu_command(browser, frame, params, command_id, event_flags)
            .into()
    }

    extern "C" fn on_context_menu_dismissed<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_context_menu_dismissed(browser, frame)
    }

    extern "C" fn run_quick_menu<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        location: *const cef_point_t,
        size: *const cef_size_t,
        edit_state_flags: cef_quick_menu_edit_state_flags_t,
        callback: *mut _cef_run_quick_menu_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let location = &Point(unsafe { RefGuard::from_raw_add_ref(location) });
        let size = &Size(unsafe { RefGuard::from_raw_add_ref(size) });
        let callback = &mut RunQuickMenuCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .run_quick_menu(browser, frame, location, size, edit_state_flags, callback)
            .into()
    }

    extern "C" fn on_quick_menu_command<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        command_id: ::std::os::raw::c_int,
        event_flags: cef_event_flags_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface
            .on_quick_menu_command(browser, frame, command_id, event_flags)
            .into()
    }

    extern "C" fn on_quick_menu_dismissed<I: ImplContextMenuHandler>(
        self_: *mut _cef_context_menu_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_quick_menu_dismissed(browser, frame)
    }
}

wrapper!(
    #[doc = "See [_cef_context_menu_params_t] for more documentation."]
    #[derive(Clone)]
    pub struct ContextMenuParams(_cef_context_menu_params_t);

    pub fn get_xcoord(&self) -> ::std::os::raw::c_int;
    pub fn get_ycoord(&self) -> ::std::os::raw::c_int;
    pub fn get_type_flags(&self) -> ContextMenuTypeFlags;
    pub fn get_link_url(&self) -> CefStringUserfree;
    pub fn get_unfiltered_link_url(&self) -> CefStringUserfree;
    pub fn get_source_url(&self) -> CefStringUserfree;
    pub fn has_image_contents(&self) -> ::std::os::raw::c_int;
    pub fn get_title_text(&self) -> CefStringUserfree;
    pub fn get_page_url(&self) -> CefStringUserfree;
    pub fn get_frame_url(&self) -> CefStringUserfree;
    pub fn get_frame_charset(&self) -> CefStringUserfree;
    pub fn get_media_type(&self) -> ContextMenuMediaType;
    pub fn get_media_state_flags(&self) -> ContextMenuMediaStateFlags;
    pub fn get_selection_text(&self) -> CefStringUserfree;
    pub fn get_misspelled_word(&self) -> CefStringUserfree;
    pub fn get_dictionary_suggestions(
        &self,
        suggestions: &mut CefStringList,
    ) -> ::std::os::raw::c_int;
    pub fn is_editable(&self) -> ::std::os::raw::c_int;
    pub fn is_spell_check_enabled(&self) -> ::std::os::raw::c_int;
    pub fn get_edit_state_flags(&self) -> ContextMenuEditStateFlags;
    pub fn is_custom_menu(&self) -> ::std::os::raw::c_int;
);

pub trait ImplContextMenuParams: Sized {
    fn get_xcoord(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_ycoord(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_type_flags(&self) -> ContextMenuTypeFlags {
        Default::default()
    }
    fn get_link_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_unfiltered_link_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_source_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn has_image_contents(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_title_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_page_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_frame_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_frame_charset(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_media_type(&self) -> ContextMenuMediaType {
        Default::default()
    }
    fn get_media_state_flags(&self) -> ContextMenuMediaStateFlags {
        Default::default()
    }
    fn get_selection_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_misspelled_word(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_dictionary_suggestions(&self, suggestions: &mut CefStringList) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_editable(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_spell_check_enabled(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_edit_state_flags(&self) -> ContextMenuEditStateFlags {
        Default::default()
    }
    fn is_custom_menu(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_context_menu_params_t {
        let mut object: _cef_context_menu_params_t = unsafe { std::mem::zeroed() };
        impl_cef_context_menu_params_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_context_menu_params_t {
    use super::*;

    pub fn init_methods<I: ImplContextMenuParams>(object: &mut _cef_context_menu_params_t) {
        object.get_xcoord = Some(get_xcoord::<I>);
        object.get_ycoord = Some(get_ycoord::<I>);
        object.get_type_flags = Some(get_type_flags::<I>);
        object.get_link_url = Some(get_link_url::<I>);
        object.get_unfiltered_link_url = Some(get_unfiltered_link_url::<I>);
        object.get_source_url = Some(get_source_url::<I>);
        object.has_image_contents = Some(has_image_contents::<I>);
        object.get_title_text = Some(get_title_text::<I>);
        object.get_page_url = Some(get_page_url::<I>);
        object.get_frame_url = Some(get_frame_url::<I>);
        object.get_frame_charset = Some(get_frame_charset::<I>);
        object.get_media_type = Some(get_media_type::<I>);
        object.get_media_state_flags = Some(get_media_state_flags::<I>);
        object.get_selection_text = Some(get_selection_text::<I>);
        object.get_misspelled_word = Some(get_misspelled_word::<I>);
        object.get_dictionary_suggestions = Some(get_dictionary_suggestions::<I>);
        object.is_editable = Some(is_editable::<I>);
        object.is_spell_check_enabled = Some(is_spell_check_enabled::<I>);
        object.get_edit_state_flags = Some(get_edit_state_flags::<I>);
        object.is_custom_menu = Some(is_custom_menu::<I>);
    }

    extern "C" fn get_xcoord<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_xcoord().into()
    }

    extern "C" fn get_ycoord<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_ycoord().into()
    }

    extern "C" fn get_type_flags<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_context_menu_type_flags_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type_flags().into()
    }

    extern "C" fn get_link_url<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_link_url().into()
    }

    extern "C" fn get_unfiltered_link_url<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_unfiltered_link_url().into()
    }

    extern "C" fn get_source_url<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_source_url().into()
    }

    extern "C" fn has_image_contents<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_image_contents().into()
    }

    extern "C" fn get_title_text<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_title_text().into()
    }

    extern "C" fn get_page_url<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_page_url().into()
    }

    extern "C" fn get_frame_url<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame_url().into()
    }

    extern "C" fn get_frame_charset<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame_charset().into()
    }

    extern "C" fn get_media_type<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_context_menu_media_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_media_type().into()
    }

    extern "C" fn get_media_state_flags<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_context_menu_media_state_flags_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_media_state_flags().into()
    }

    extern "C" fn get_selection_text<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_selection_text().into()
    }

    extern "C" fn get_misspelled_word<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_misspelled_word().into()
    }

    extern "C" fn get_dictionary_suggestions<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
        suggestions: cef_string_list_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let suggestions = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(suggestions) });
        obj.interface.get_dictionary_suggestions(suggestions).into()
    }

    extern "C" fn is_editable<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_editable().into()
    }

    extern "C" fn is_spell_check_enabled<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_spell_check_enabled().into()
    }

    extern "C" fn get_edit_state_flags<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> cef_context_menu_edit_state_flags_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_edit_state_flags().into()
    }

    extern "C" fn is_custom_menu<I: ImplContextMenuParams>(
        self_: *mut _cef_context_menu_params_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_custom_menu().into()
    }
}

wrapper!(
    #[doc = "See [_cef_file_dialog_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct FileDialogCallback(_cef_file_dialog_callback_t);

    pub fn cont(&self, file_paths: &mut CefStringList);
    pub fn cancel(&self);
);

pub trait ImplFileDialogCallback: Sized {
    fn cont(&self, file_paths: &mut CefStringList) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_file_dialog_callback_t {
        let mut object: _cef_file_dialog_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_file_dialog_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_file_dialog_callback_t {
    use super::*;

    pub fn init_methods<I: ImplFileDialogCallback>(object: &mut _cef_file_dialog_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplFileDialogCallback>(
        self_: *mut _cef_file_dialog_callback_t,
        file_paths: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let file_paths = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(file_paths) });
        obj.interface.cont(file_paths)
    }

    extern "C" fn cancel<I: ImplFileDialogCallback>(self_: *mut _cef_file_dialog_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_dialog_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct DialogHandler(_cef_dialog_handler_t);

    pub fn on_file_dialog(
        &self,
        browser: &mut Browser,
        mode: FileDialogMode,
        title: &CefString,
        default_file_path: &CefString,
        accept_filters: &mut CefStringList,
        accept_extensions: &mut CefStringList,
        accept_descriptions: &mut CefStringList,
        callback: &mut FileDialogCallback,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplDialogHandler: Sized {
    fn on_file_dialog(
        &self,
        browser: &mut Browser,
        mode: FileDialogMode,
        title: &CefString,
        default_file_path: &CefString,
        accept_filters: &mut CefStringList,
        accept_extensions: &mut CefStringList,
        accept_descriptions: &mut CefStringList,
        callback: &mut FileDialogCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_dialog_handler_t {
        let mut object: _cef_dialog_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_dialog_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_dialog_handler_t {
    use super::*;

    pub fn init_methods<I: ImplDialogHandler>(object: &mut _cef_dialog_handler_t) {
        object.on_file_dialog = Some(on_file_dialog::<I>);
    }

    extern "C" fn on_file_dialog<I: ImplDialogHandler>(
        self_: *mut _cef_dialog_handler_t,
        browser: *mut _cef_browser_t,
        mode: cef_file_dialog_mode_t,
        title: *const cef_string_t,
        default_file_path: *const cef_string_t,
        accept_filters: cef_string_list_t,
        accept_extensions: cef_string_list_t,
        accept_descriptions: cef_string_list_t,
        callback: *mut _cef_file_dialog_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let title = &CefString::from(title);
        let default_file_path = &CefString::from(default_file_path);
        let accept_filters =
            &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(accept_filters) });
        let accept_extensions =
            &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(accept_extensions) });
        let accept_descriptions =
            &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(accept_descriptions) });
        let callback = &mut FileDialogCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_file_dialog(
                browser,
                mode,
                title,
                default_file_path,
                accept_filters,
                accept_extensions,
                accept_descriptions,
                callback,
            )
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_display_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct DisplayHandler(_cef_display_handler_t);

    pub fn on_address_change(&self, browser: &mut Browser, frame: &mut Frame, url: &CefString);
    pub fn on_title_change(&self, browser: &mut Browser, title: &CefString);
    pub fn on_favicon_urlchange(&self, browser: &mut Browser, icon_urls: &mut CefStringList);
    pub fn on_fullscreen_mode_change(
        &self,
        browser: &mut Browser,
        fullscreen: ::std::os::raw::c_int,
    );
    pub fn on_tooltip(&self, browser: &mut Browser, text: &mut CefString) -> ::std::os::raw::c_int;
    pub fn on_status_message(&self, browser: &mut Browser, value: &CefString);
    pub fn on_console_message(
        &self,
        browser: &mut Browser,
        level: LogSeverity,
        message: &CefString,
        source: &CefString,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_auto_resize(&self, browser: &mut Browser, new_size: &Size) -> ::std::os::raw::c_int;
    pub fn on_loading_progress_change(&self, browser: &mut Browser, progress: f64);
    pub fn on_cursor_change(
        &self,
        browser: &mut Browser,
        cursor: ::std::os::raw::c_ulong,
        type_: CursorType,
        custom_cursor_info: &CursorInfo,
    ) -> ::std::os::raw::c_int;
    pub fn on_media_access_change(
        &self,
        browser: &mut Browser,
        has_video_access: ::std::os::raw::c_int,
        has_audio_access: ::std::os::raw::c_int,
    );
);

pub trait ImplDisplayHandler: Sized {
    fn on_address_change(&self, browser: &mut Browser, frame: &mut Frame, url: &CefString) {}
    fn on_title_change(&self, browser: &mut Browser, title: &CefString) {}
    fn on_favicon_urlchange(&self, browser: &mut Browser, icon_urls: &mut CefStringList) {}
    fn on_fullscreen_mode_change(&self, browser: &mut Browser, fullscreen: ::std::os::raw::c_int) {}
    fn on_tooltip(&self, browser: &mut Browser, text: &mut CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_status_message(&self, browser: &mut Browser, value: &CefString) {}
    fn on_console_message(
        &self,
        browser: &mut Browser,
        level: LogSeverity,
        message: &CefString,
        source: &CefString,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_auto_resize(&self, browser: &mut Browser, new_size: &Size) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_loading_progress_change(&self, browser: &mut Browser, progress: f64) {}
    fn on_cursor_change(
        &self,
        browser: &mut Browser,
        cursor: ::std::os::raw::c_ulong,
        type_: CursorType,
        custom_cursor_info: &CursorInfo,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_media_access_change(
        &self,
        browser: &mut Browser,
        has_video_access: ::std::os::raw::c_int,
        has_audio_access: ::std::os::raw::c_int,
    ) {
    }

    fn into_raw(self) -> *mut _cef_display_handler_t {
        let mut object: _cef_display_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_display_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_display_handler_t {
    use super::*;

    pub fn init_methods<I: ImplDisplayHandler>(object: &mut _cef_display_handler_t) {
        object.on_address_change = Some(on_address_change::<I>);
        object.on_title_change = Some(on_title_change::<I>);
        object.on_favicon_urlchange = Some(on_favicon_urlchange::<I>);
        object.on_fullscreen_mode_change = Some(on_fullscreen_mode_change::<I>);
        object.on_tooltip = Some(on_tooltip::<I>);
        object.on_status_message = Some(on_status_message::<I>);
        object.on_console_message = Some(on_console_message::<I>);
        object.on_auto_resize = Some(on_auto_resize::<I>);
        object.on_loading_progress_change = Some(on_loading_progress_change::<I>);
        object.on_cursor_change = Some(on_cursor_change::<I>);
        object.on_media_access_change = Some(on_media_access_change::<I>);
    }

    extern "C" fn on_address_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let url = &CefString::from(url);
        obj.interface.on_address_change(browser, frame, url)
    }

    extern "C" fn on_title_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        title: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let title = &CefString::from(title);
        obj.interface.on_title_change(browser, title)
    }

    extern "C" fn on_favicon_urlchange<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        icon_urls: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let icon_urls = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(icon_urls) });
        obj.interface.on_favicon_urlchange(browser, icon_urls)
    }

    extern "C" fn on_fullscreen_mode_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        fullscreen: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_fullscreen_mode_change(browser, fullscreen)
    }

    extern "C" fn on_tooltip<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        text: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let text = &mut CefString::from(text);
        obj.interface.on_tooltip(browser, text).into()
    }

    extern "C" fn on_status_message<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        value: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let value = &CefString::from(value);
        obj.interface.on_status_message(browser, value)
    }

    extern "C" fn on_console_message<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        level: cef_log_severity_t,
        message: *const cef_string_t,
        source: *const cef_string_t,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let message = &CefString::from(message);
        let source = &CefString::from(source);
        obj.interface
            .on_console_message(browser, level, message, source, line)
            .into()
    }

    extern "C" fn on_auto_resize<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        new_size: *const cef_size_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let new_size = &Size(unsafe { RefGuard::from_raw_add_ref(new_size) });
        obj.interface.on_auto_resize(browser, new_size).into()
    }

    extern "C" fn on_loading_progress_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        progress: f64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_loading_progress_change(browser, progress)
    }

    extern "C" fn on_cursor_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        cursor: ::std::os::raw::c_ulong,
        type_: cef_cursor_type_t,
        custom_cursor_info: *const cef_cursor_info_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let custom_cursor_info =
            &CursorInfo(unsafe { RefGuard::from_raw_add_ref(custom_cursor_info) });
        obj.interface
            .on_cursor_change(browser, cursor, type_, custom_cursor_info)
            .into()
    }

    extern "C" fn on_media_access_change<I: ImplDisplayHandler>(
        self_: *mut _cef_display_handler_t,
        browser: *mut _cef_browser_t,
        has_video_access: ::std::os::raw::c_int,
        has_audio_access: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_media_access_change(browser, has_video_access, has_audio_access)
    }
}

wrapper!(
    #[doc = "See [_cef_download_item_t] for more documentation."]
    #[derive(Clone)]
    pub struct DownloadItem(_cef_download_item_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_in_progress(&self) -> ::std::os::raw::c_int;
    pub fn is_complete(&self) -> ::std::os::raw::c_int;
    pub fn is_canceled(&self) -> ::std::os::raw::c_int;
    pub fn is_interrupted(&self) -> ::std::os::raw::c_int;
    pub fn get_interrupt_reason(&self) -> DownloadInterruptReason;
    pub fn get_current_speed(&self) -> i64;
    pub fn get_percent_complete(&self) -> ::std::os::raw::c_int;
    pub fn get_total_bytes(&self) -> i64;
    pub fn get_received_bytes(&self) -> i64;
    pub fn get_start_time(&self) -> Basetime;
    pub fn get_end_time(&self) -> Basetime;
    pub fn get_full_path(&self) -> CefStringUserfree;
    pub fn get_id(&self) -> u32;
    pub fn get_url(&self) -> CefStringUserfree;
    pub fn get_original_url(&self) -> CefStringUserfree;
    pub fn get_suggested_file_name(&self) -> CefStringUserfree;
    pub fn get_content_disposition(&self) -> CefStringUserfree;
    pub fn get_mime_type(&self) -> CefStringUserfree;
);

pub trait ImplDownloadItem: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_in_progress(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_complete(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_canceled(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_interrupted(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_interrupt_reason(&self) -> DownloadInterruptReason {
        Default::default()
    }
    fn get_current_speed(&self) -> i64 {
        Default::default()
    }
    fn get_percent_complete(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_total_bytes(&self) -> i64 {
        Default::default()
    }
    fn get_received_bytes(&self) -> i64 {
        Default::default()
    }
    fn get_start_time(&self) -> Basetime {
        Default::default()
    }
    fn get_end_time(&self) -> Basetime {
        Default::default()
    }
    fn get_full_path(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_id(&self) -> u32 {
        Default::default()
    }
    fn get_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_original_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_suggested_file_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_content_disposition(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_mime_type(&self) -> CefStringUserfree {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_download_item_t {
        let mut object: _cef_download_item_t = unsafe { std::mem::zeroed() };
        impl_cef_download_item_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_download_item_t {
    use super::*;

    pub fn init_methods<I: ImplDownloadItem>(object: &mut _cef_download_item_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_in_progress = Some(is_in_progress::<I>);
        object.is_complete = Some(is_complete::<I>);
        object.is_canceled = Some(is_canceled::<I>);
        object.is_interrupted = Some(is_interrupted::<I>);
        object.get_interrupt_reason = Some(get_interrupt_reason::<I>);
        object.get_current_speed = Some(get_current_speed::<I>);
        object.get_percent_complete = Some(get_percent_complete::<I>);
        object.get_total_bytes = Some(get_total_bytes::<I>);
        object.get_received_bytes = Some(get_received_bytes::<I>);
        object.get_start_time = Some(get_start_time::<I>);
        object.get_end_time = Some(get_end_time::<I>);
        object.get_full_path = Some(get_full_path::<I>);
        object.get_id = Some(get_id::<I>);
        object.get_url = Some(get_url::<I>);
        object.get_original_url = Some(get_original_url::<I>);
        object.get_suggested_file_name = Some(get_suggested_file_name::<I>);
        object.get_content_disposition = Some(get_content_disposition::<I>);
        object.get_mime_type = Some(get_mime_type::<I>);
    }

    extern "C" fn is_valid<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_in_progress<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_in_progress().into()
    }

    extern "C" fn is_complete<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_complete().into()
    }

    extern "C" fn is_canceled<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_canceled().into()
    }

    extern "C" fn is_interrupted<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_interrupted().into()
    }

    extern "C" fn get_interrupt_reason<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_download_interrupt_reason_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_interrupt_reason().into()
    }

    extern "C" fn get_current_speed<I: ImplDownloadItem>(self_: *mut _cef_download_item_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_current_speed().into()
    }

    extern "C" fn get_percent_complete<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_percent_complete().into()
    }

    extern "C" fn get_total_bytes<I: ImplDownloadItem>(self_: *mut _cef_download_item_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_total_bytes().into()
    }

    extern "C" fn get_received_bytes<I: ImplDownloadItem>(self_: *mut _cef_download_item_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_received_bytes().into()
    }

    extern "C" fn get_start_time<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_start_time().into()
    }

    extern "C" fn get_end_time<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_end_time().into()
    }

    extern "C" fn get_full_path<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_full_path().into()
    }

    extern "C" fn get_id<I: ImplDownloadItem>(self_: *mut _cef_download_item_t) -> u32 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn get_url<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_url().into()
    }

    extern "C" fn get_original_url<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_original_url().into()
    }

    extern "C" fn get_suggested_file_name<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_suggested_file_name().into()
    }

    extern "C" fn get_content_disposition<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_content_disposition().into()
    }

    extern "C" fn get_mime_type<I: ImplDownloadItem>(
        self_: *mut _cef_download_item_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_mime_type().into()
    }
}

wrapper!(
    #[doc = "See [_cef_before_download_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct BeforeDownloadCallback(_cef_before_download_callback_t);

    pub fn cont(&self, download_path: &CefString, show_dialog: ::std::os::raw::c_int);
);

pub trait ImplBeforeDownloadCallback: Sized {
    fn cont(&self, download_path: &CefString, show_dialog: ::std::os::raw::c_int) {}

    fn into_raw(self) -> *mut _cef_before_download_callback_t {
        let mut object: _cef_before_download_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_before_download_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_before_download_callback_t {
    use super::*;

    pub fn init_methods<I: ImplBeforeDownloadCallback>(
        object: &mut _cef_before_download_callback_t,
    ) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplBeforeDownloadCallback>(
        self_: *mut _cef_before_download_callback_t,
        download_path: *const cef_string_t,
        show_dialog: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let download_path = &CefString::from(download_path);
        obj.interface.cont(download_path, show_dialog)
    }
}

wrapper!(
    #[doc = "See [_cef_download_item_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct DownloadItemCallback(_cef_download_item_callback_t);

    pub fn cancel(&self);
    pub fn pause(&self);
    pub fn resume(&self);
);

pub trait ImplDownloadItemCallback: Sized {
    fn cancel(&self) {}
    fn pause(&self) {}
    fn resume(&self) {}

    fn into_raw(self) -> *mut _cef_download_item_callback_t {
        let mut object: _cef_download_item_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_download_item_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_download_item_callback_t {
    use super::*;

    pub fn init_methods<I: ImplDownloadItemCallback>(object: &mut _cef_download_item_callback_t) {
        object.cancel = Some(cancel::<I>);
        object.pause = Some(pause::<I>);
        object.resume = Some(resume::<I>);
    }

    extern "C" fn cancel<I: ImplDownloadItemCallback>(self_: *mut _cef_download_item_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }

    extern "C" fn pause<I: ImplDownloadItemCallback>(self_: *mut _cef_download_item_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.pause()
    }

    extern "C" fn resume<I: ImplDownloadItemCallback>(self_: *mut _cef_download_item_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.resume()
    }
}

wrapper!(
    #[doc = "See [_cef_download_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct DownloadHandler(_cef_download_handler_t);

    pub fn can_download(
        &self,
        browser: &mut Browser,
        url: &CefString,
        request_method: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn on_before_download(
        &self,
        browser: &mut Browser,
        download_item: &mut DownloadItem,
        suggested_name: &CefString,
        callback: &mut BeforeDownloadCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_download_updated(
        &self,
        browser: &mut Browser,
        download_item: &mut DownloadItem,
        callback: &mut DownloadItemCallback,
    );
);

pub trait ImplDownloadHandler: Sized {
    fn can_download(
        &self,
        browser: &mut Browser,
        url: &CefString,
        request_method: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_before_download(
        &self,
        browser: &mut Browser,
        download_item: &mut DownloadItem,
        suggested_name: &CefString,
        callback: &mut BeforeDownloadCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_download_updated(
        &self,
        browser: &mut Browser,
        download_item: &mut DownloadItem,
        callback: &mut DownloadItemCallback,
    ) {
    }

    fn into_raw(self) -> *mut _cef_download_handler_t {
        let mut object: _cef_download_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_download_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_download_handler_t {
    use super::*;

    pub fn init_methods<I: ImplDownloadHandler>(object: &mut _cef_download_handler_t) {
        object.can_download = Some(can_download::<I>);
        object.on_before_download = Some(on_before_download::<I>);
        object.on_download_updated = Some(on_download_updated::<I>);
    }

    extern "C" fn can_download<I: ImplDownloadHandler>(
        self_: *mut _cef_download_handler_t,
        browser: *mut _cef_browser_t,
        url: *const cef_string_t,
        request_method: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let url = &CefString::from(url);
        let request_method = &CefString::from(request_method);
        obj.interface
            .can_download(browser, url, request_method)
            .into()
    }

    extern "C" fn on_before_download<I: ImplDownloadHandler>(
        self_: *mut _cef_download_handler_t,
        browser: *mut _cef_browser_t,
        download_item: *mut _cef_download_item_t,
        suggested_name: *const cef_string_t,
        callback: *mut _cef_before_download_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let download_item = &mut DownloadItem(unsafe { RefGuard::from_raw_add_ref(download_item) });
        let suggested_name = &CefString::from(suggested_name);
        let callback = &mut BeforeDownloadCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_before_download(browser, download_item, suggested_name, callback)
            .into()
    }

    extern "C" fn on_download_updated<I: ImplDownloadHandler>(
        self_: *mut _cef_download_handler_t,
        browser: *mut _cef_browser_t,
        download_item: *mut _cef_download_item_t,
        callback: *mut _cef_download_item_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let download_item = &mut DownloadItem(unsafe { RefGuard::from_raw_add_ref(download_item) });
        let callback = &mut DownloadItemCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_download_updated(browser, download_item, callback)
    }
}

wrapper!(
    #[doc = "See [_cef_drag_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct DragHandler(_cef_drag_handler_t);

    pub fn on_drag_enter(
        &self,
        browser: &mut Browser,
        drag_data: &mut DragData,
        mask: DragOperationsMask,
    ) -> ::std::os::raw::c_int;
    pub fn on_draggable_regions_changed(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        regions_count: usize,
        regions: &DraggableRegion,
    );
);

pub trait ImplDragHandler: Sized {
    fn on_drag_enter(
        &self,
        browser: &mut Browser,
        drag_data: &mut DragData,
        mask: DragOperationsMask,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_draggable_regions_changed(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        regions_count: usize,
        regions: &DraggableRegion,
    ) {
    }

    fn into_raw(self) -> *mut _cef_drag_handler_t {
        let mut object: _cef_drag_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_drag_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_drag_handler_t {
    use super::*;

    pub fn init_methods<I: ImplDragHandler>(object: &mut _cef_drag_handler_t) {
        object.on_drag_enter = Some(on_drag_enter::<I>);
        object.on_draggable_regions_changed = Some(on_draggable_regions_changed::<I>);
    }

    extern "C" fn on_drag_enter<I: ImplDragHandler>(
        self_: *mut _cef_drag_handler_t,
        browser: *mut _cef_browser_t,
        drag_data: *mut _cef_drag_data_t,
        mask: cef_drag_operations_mask_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let drag_data = &mut DragData(unsafe { RefGuard::from_raw_add_ref(drag_data) });
        obj.interface.on_drag_enter(browser, drag_data, mask).into()
    }

    extern "C" fn on_draggable_regions_changed<I: ImplDragHandler>(
        self_: *mut _cef_drag_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        regions_count: usize,
        regions: *const cef_draggable_region_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let regions = &DraggableRegion(unsafe { RefGuard::from_raw_add_ref(regions) });
        obj.interface
            .on_draggable_regions_changed(browser, frame, regions_count, regions)
    }
}

wrapper!(
    #[doc = "See [_cef_find_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct FindHandler(_cef_find_handler_t);

    pub fn on_find_result(
        &self,
        browser: &mut Browser,
        identifier: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        selection_rect: &Rect,
        active_match_ordinal: ::std::os::raw::c_int,
        final_update: ::std::os::raw::c_int,
    );
);

pub trait ImplFindHandler: Sized {
    fn on_find_result(
        &self,
        browser: &mut Browser,
        identifier: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        selection_rect: &Rect,
        active_match_ordinal: ::std::os::raw::c_int,
        final_update: ::std::os::raw::c_int,
    ) {
    }

    fn into_raw(self) -> *mut _cef_find_handler_t {
        let mut object: _cef_find_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_find_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_find_handler_t {
    use super::*;

    pub fn init_methods<I: ImplFindHandler>(object: &mut _cef_find_handler_t) {
        object.on_find_result = Some(on_find_result::<I>);
    }

    extern "C" fn on_find_result<I: ImplFindHandler>(
        self_: *mut _cef_find_handler_t,
        browser: *mut _cef_browser_t,
        identifier: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        selection_rect: *const cef_rect_t,
        active_match_ordinal: ::std::os::raw::c_int,
        final_update: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let selection_rect = &Rect(unsafe { RefGuard::from_raw_add_ref(selection_rect) });
        obj.interface.on_find_result(
            browser,
            identifier,
            count,
            selection_rect,
            active_match_ordinal,
            final_update,
        )
    }
}

wrapper!(
    #[doc = "See [_cef_focus_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct FocusHandler(_cef_focus_handler_t);

    pub fn on_take_focus(&self, browser: &mut Browser, next: ::std::os::raw::c_int);
    pub fn on_set_focus(&self, browser: &mut Browser, source: FocusSource)
        -> ::std::os::raw::c_int;
    pub fn on_got_focus(&self, browser: &mut Browser);
);

pub trait ImplFocusHandler: Sized {
    fn on_take_focus(&self, browser: &mut Browser, next: ::std::os::raw::c_int) {}
    fn on_set_focus(&self, browser: &mut Browser, source: FocusSource) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_got_focus(&self, browser: &mut Browser) {}

    fn into_raw(self) -> *mut _cef_focus_handler_t {
        let mut object: _cef_focus_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_focus_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_focus_handler_t {
    use super::*;

    pub fn init_methods<I: ImplFocusHandler>(object: &mut _cef_focus_handler_t) {
        object.on_take_focus = Some(on_take_focus::<I>);
        object.on_set_focus = Some(on_set_focus::<I>);
        object.on_got_focus = Some(on_got_focus::<I>);
    }

    extern "C" fn on_take_focus<I: ImplFocusHandler>(
        self_: *mut _cef_focus_handler_t,
        browser: *mut _cef_browser_t,
        next: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_take_focus(browser, next)
    }

    extern "C" fn on_set_focus<I: ImplFocusHandler>(
        self_: *mut _cef_focus_handler_t,
        browser: *mut _cef_browser_t,
        source: cef_focus_source_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_set_focus(browser, source).into()
    }

    extern "C" fn on_got_focus<I: ImplFocusHandler>(
        self_: *mut _cef_focus_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_got_focus(browser)
    }
}

wrapper!(
    #[doc = "See [_cef_frame_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct FrameHandler(_cef_frame_handler_t);

    pub fn on_frame_created(&self, browser: &mut Browser, frame: &mut Frame);
    pub fn on_frame_destroyed(&self, browser: &mut Browser, frame: &mut Frame);
    pub fn on_frame_attached(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        reattached: ::std::os::raw::c_int,
    );
    pub fn on_frame_detached(&self, browser: &mut Browser, frame: &mut Frame);
    pub fn on_main_frame_changed(
        &self,
        browser: &mut Browser,
        old_frame: &mut Frame,
        new_frame: &mut Frame,
    );
);

pub trait ImplFrameHandler: Sized {
    fn on_frame_created(&self, browser: &mut Browser, frame: &mut Frame) {}
    fn on_frame_destroyed(&self, browser: &mut Browser, frame: &mut Frame) {}
    fn on_frame_attached(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        reattached: ::std::os::raw::c_int,
    ) {
    }
    fn on_frame_detached(&self, browser: &mut Browser, frame: &mut Frame) {}
    fn on_main_frame_changed(
        &self,
        browser: &mut Browser,
        old_frame: &mut Frame,
        new_frame: &mut Frame,
    ) {
    }

    fn into_raw(self) -> *mut _cef_frame_handler_t {
        let mut object: _cef_frame_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_frame_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_frame_handler_t {
    use super::*;

    pub fn init_methods<I: ImplFrameHandler>(object: &mut _cef_frame_handler_t) {
        object.on_frame_created = Some(on_frame_created::<I>);
        object.on_frame_destroyed = Some(on_frame_destroyed::<I>);
        object.on_frame_attached = Some(on_frame_attached::<I>);
        object.on_frame_detached = Some(on_frame_detached::<I>);
        object.on_main_frame_changed = Some(on_main_frame_changed::<I>);
    }

    extern "C" fn on_frame_created<I: ImplFrameHandler>(
        self_: *mut _cef_frame_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_frame_created(browser, frame)
    }

    extern "C" fn on_frame_destroyed<I: ImplFrameHandler>(
        self_: *mut _cef_frame_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_frame_destroyed(browser, frame)
    }

    extern "C" fn on_frame_attached<I: ImplFrameHandler>(
        self_: *mut _cef_frame_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        reattached: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_frame_attached(browser, frame, reattached)
    }

    extern "C" fn on_frame_detached<I: ImplFrameHandler>(
        self_: *mut _cef_frame_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_frame_detached(browser, frame)
    }

    extern "C" fn on_main_frame_changed<I: ImplFrameHandler>(
        self_: *mut _cef_frame_handler_t,
        browser: *mut _cef_browser_t,
        old_frame: *mut _cef_frame_t,
        new_frame: *mut _cef_frame_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let old_frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(old_frame) });
        let new_frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(new_frame) });
        obj.interface
            .on_main_frame_changed(browser, old_frame, new_frame)
    }
}

wrapper!(
    #[doc = "See [_cef_jsdialog_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct JsdialogCallback(_cef_jsdialog_callback_t);

    pub fn cont(&self, success: ::std::os::raw::c_int, user_input: &CefString);
);

pub trait ImplJsdialogCallback: Sized {
    fn cont(&self, success: ::std::os::raw::c_int, user_input: &CefString) {}

    fn into_raw(self) -> *mut _cef_jsdialog_callback_t {
        let mut object: _cef_jsdialog_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_jsdialog_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_jsdialog_callback_t {
    use super::*;

    pub fn init_methods<I: ImplJsdialogCallback>(object: &mut _cef_jsdialog_callback_t) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplJsdialogCallback>(
        self_: *mut _cef_jsdialog_callback_t,
        success: ::std::os::raw::c_int,
        user_input: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let user_input = &CefString::from(user_input);
        obj.interface.cont(success, user_input)
    }
}

wrapper!(
    #[doc = "See [_cef_jsdialog_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct JsdialogHandler(_cef_jsdialog_handler_t);

    pub fn on_jsdialog(
        &self,
        browser: &mut Browser,
        origin_url: &CefString,
        dialog_type: JsdialogType,
        message_text: &CefString,
        default_prompt_text: &CefString,
        callback: &mut JsdialogCallback,
        suppress_message: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_before_unload_dialog(
        &self,
        browser: &mut Browser,
        message_text: &CefString,
        is_reload: ::std::os::raw::c_int,
        callback: &mut JsdialogCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_reset_dialog_state(&self, browser: &mut Browser);
    pub fn on_dialog_closed(&self, browser: &mut Browser);
);

pub trait ImplJsdialogHandler: Sized {
    fn on_jsdialog(
        &self,
        browser: &mut Browser,
        origin_url: &CefString,
        dialog_type: JsdialogType,
        message_text: &CefString,
        default_prompt_text: &CefString,
        callback: &mut JsdialogCallback,
        suppress_message: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_before_unload_dialog(
        &self,
        browser: &mut Browser,
        message_text: &CefString,
        is_reload: ::std::os::raw::c_int,
        callback: &mut JsdialogCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_reset_dialog_state(&self, browser: &mut Browser) {}
    fn on_dialog_closed(&self, browser: &mut Browser) {}

    fn into_raw(self) -> *mut _cef_jsdialog_handler_t {
        let mut object: _cef_jsdialog_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_jsdialog_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_jsdialog_handler_t {
    use super::*;

    pub fn init_methods<I: ImplJsdialogHandler>(object: &mut _cef_jsdialog_handler_t) {
        object.on_jsdialog = Some(on_jsdialog::<I>);
        object.on_before_unload_dialog = Some(on_before_unload_dialog::<I>);
        object.on_reset_dialog_state = Some(on_reset_dialog_state::<I>);
        object.on_dialog_closed = Some(on_dialog_closed::<I>);
    }

    extern "C" fn on_jsdialog<I: ImplJsdialogHandler>(
        self_: *mut _cef_jsdialog_handler_t,
        browser: *mut _cef_browser_t,
        origin_url: *const cef_string_t,
        dialog_type: cef_jsdialog_type_t,
        message_text: *const cef_string_t,
        default_prompt_text: *const cef_string_t,
        callback: *mut _cef_jsdialog_callback_t,
        suppress_message: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let origin_url = &CefString::from(origin_url);
        let message_text = &CefString::from(message_text);
        let default_prompt_text = &CefString::from(default_prompt_text);
        let callback = &mut JsdialogCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_jsdialog(
                browser,
                origin_url,
                dialog_type,
                message_text,
                default_prompt_text,
                callback,
                suppress_message,
            )
            .into()
    }

    extern "C" fn on_before_unload_dialog<I: ImplJsdialogHandler>(
        self_: *mut _cef_jsdialog_handler_t,
        browser: *mut _cef_browser_t,
        message_text: *const cef_string_t,
        is_reload: ::std::os::raw::c_int,
        callback: *mut _cef_jsdialog_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let message_text = &CefString::from(message_text);
        let callback = &mut JsdialogCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_before_unload_dialog(browser, message_text, is_reload, callback)
            .into()
    }

    extern "C" fn on_reset_dialog_state<I: ImplJsdialogHandler>(
        self_: *mut _cef_jsdialog_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_reset_dialog_state(browser)
    }

    extern "C" fn on_dialog_closed<I: ImplJsdialogHandler>(
        self_: *mut _cef_jsdialog_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_dialog_closed(browser)
    }
}

wrapper!(
    #[doc = "See [_cef_keyboard_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct KeyboardHandler(_cef_keyboard_handler_t);

    pub fn on_pre_key_event(
        &self,
        browser: &mut Browser,
        event: &KeyEvent,
        os_event: *mut XEvent,
        is_keyboard_shortcut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_key_event(
        &self,
        browser: &mut Browser,
        event: &KeyEvent,
        os_event: *mut XEvent,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplKeyboardHandler: Sized {
    fn on_pre_key_event(
        &self,
        browser: &mut Browser,
        event: &KeyEvent,
        os_event: *mut XEvent,
        is_keyboard_shortcut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_key_event(
        &self,
        browser: &mut Browser,
        event: &KeyEvent,
        os_event: *mut XEvent,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_keyboard_handler_t {
        let mut object: _cef_keyboard_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_keyboard_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_keyboard_handler_t {
    use super::*;

    pub fn init_methods<I: ImplKeyboardHandler>(object: &mut _cef_keyboard_handler_t) {
        object.on_pre_key_event = Some(on_pre_key_event::<I>);
        object.on_key_event = Some(on_key_event::<I>);
    }

    extern "C" fn on_pre_key_event<I: ImplKeyboardHandler>(
        self_: *mut _cef_keyboard_handler_t,
        browser: *mut _cef_browser_t,
        event: *const cef_key_event_t,
        os_event: *mut XEvent,
        is_keyboard_shortcut: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let event = &KeyEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface
            .on_pre_key_event(browser, event, os_event, is_keyboard_shortcut)
            .into()
    }

    extern "C" fn on_key_event<I: ImplKeyboardHandler>(
        self_: *mut _cef_keyboard_handler_t,
        browser: *mut _cef_browser_t,
        event: *const cef_key_event_t,
        os_event: *mut XEvent,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let event = &KeyEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.on_key_event(browser, event, os_event).into()
    }
}

wrapper!(
    #[doc = "See [_cef_life_span_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct LifeSpanHandler(_cef_life_span_handler_t);

    pub fn on_before_popup(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        popup_id: ::std::os::raw::c_int,
        target_url: &CefString,
        target_frame_name: &CefString,
        target_disposition: WindowOpenDisposition,
        user_gesture: ::std::os::raw::c_int,
        popup_features: &PopupFeatures,
        window_info: &mut WindowInfo,
        client: *mut &mut Client,
        settings: &mut BrowserSettings,
        extra_info: *mut &mut DictionaryValue,
        no_javascript_access: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_before_popup_aborted(&self, browser: &mut Browser, popup_id: ::std::os::raw::c_int);
    pub fn on_before_dev_tools_popup(
        &self,
        browser: &mut Browser,
        window_info: &mut WindowInfo,
        client: *mut &mut Client,
        settings: &mut BrowserSettings,
        extra_info: *mut &mut DictionaryValue,
        use_default_window: *mut ::std::os::raw::c_int,
    );
    pub fn on_after_created(&self, browser: &mut Browser);
    pub fn do_close(&self, browser: &mut Browser) -> ::std::os::raw::c_int;
    pub fn on_before_close(&self, browser: &mut Browser);
);

pub trait ImplLifeSpanHandler: Sized {
    fn on_before_popup(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        popup_id: ::std::os::raw::c_int,
        target_url: &CefString,
        target_frame_name: &CefString,
        target_disposition: WindowOpenDisposition,
        user_gesture: ::std::os::raw::c_int,
        popup_features: &PopupFeatures,
        window_info: &mut WindowInfo,
        client: *mut &mut Client,
        settings: &mut BrowserSettings,
        extra_info: *mut &mut DictionaryValue,
        no_javascript_access: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_before_popup_aborted(&self, browser: &mut Browser, popup_id: ::std::os::raw::c_int) {}
    fn on_before_dev_tools_popup(
        &self,
        browser: &mut Browser,
        window_info: &mut WindowInfo,
        client: *mut &mut Client,
        settings: &mut BrowserSettings,
        extra_info: *mut &mut DictionaryValue,
        use_default_window: *mut ::std::os::raw::c_int,
    ) {
    }
    fn on_after_created(&self, browser: &mut Browser) {}
    fn do_close(&self, browser: &mut Browser) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_before_close(&self, browser: &mut Browser) {}

    fn into_raw(self) -> *mut _cef_life_span_handler_t {
        let mut object: _cef_life_span_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_life_span_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_life_span_handler_t {
    use super::*;

    pub fn init_methods<I: ImplLifeSpanHandler>(object: &mut _cef_life_span_handler_t) {
        object.on_before_popup = Some(on_before_popup::<I>);
        object.on_before_popup_aborted = Some(on_before_popup_aborted::<I>);
        object.on_before_dev_tools_popup = Some(on_before_dev_tools_popup::<I>);
        object.on_after_created = Some(on_after_created::<I>);
        object.do_close = Some(do_close::<I>);
        object.on_before_close = Some(on_before_close::<I>);
    }

    extern "C" fn on_before_popup<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        popup_id: ::std::os::raw::c_int,
        target_url: *const cef_string_t,
        target_frame_name: *const cef_string_t,
        target_disposition: cef_window_open_disposition_t,
        user_gesture: ::std::os::raw::c_int,
        popup_features: *const cef_popup_features_t,
        window_info: *mut _cef_window_info_t,
        client: *mut *mut _cef_client_t,
        settings: *mut _cef_browser_settings_t,
        extra_info: *mut *mut _cef_dictionary_value_t,
        no_javascript_access: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let target_url = &CefString::from(target_url);
        let target_frame_name = &CefString::from(target_frame_name);
        let popup_features = &PopupFeatures(unsafe { RefGuard::from_raw_add_ref(popup_features) });
        let window_info = &mut WindowInfo(unsafe { RefGuard::from_raw_add_ref(window_info) });
        let settings = &mut BrowserSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        obj.interface
            .on_before_popup(
                browser,
                frame,
                popup_id,
                target_url,
                target_frame_name,
                target_disposition,
                user_gesture,
                popup_features,
                window_info,
                client,
                settings,
                extra_info,
                no_javascript_access,
            )
            .into()
    }

    extern "C" fn on_before_popup_aborted<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
        popup_id: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_before_popup_aborted(browser, popup_id)
    }

    extern "C" fn on_before_dev_tools_popup<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
        window_info: *mut _cef_window_info_t,
        client: *mut *mut _cef_client_t,
        settings: *mut _cef_browser_settings_t,
        extra_info: *mut *mut _cef_dictionary_value_t,
        use_default_window: *mut ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let window_info = &mut WindowInfo(unsafe { RefGuard::from_raw_add_ref(window_info) });
        let settings = &mut BrowserSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        obj.interface.on_before_dev_tools_popup(
            browser,
            window_info,
            client,
            settings,
            extra_info,
            use_default_window,
        )
    }

    extern "C" fn on_after_created<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_after_created(browser)
    }

    extern "C" fn do_close<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.do_close(browser).into()
    }

    extern "C" fn on_before_close<I: ImplLifeSpanHandler>(
        self_: *mut _cef_life_span_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_before_close(browser)
    }
}

wrapper!(
    #[doc = "See [_cef_load_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct LoadHandler(_cef_load_handler_t);

    pub fn on_loading_state_change(
        &self,
        browser: &mut Browser,
        is_loading: ::std::os::raw::c_int,
        can_go_back: ::std::os::raw::c_int,
        can_go_forward: ::std::os::raw::c_int,
    );
    pub fn on_load_start(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        transition_type: TransitionType,
    );
    pub fn on_load_end(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        http_status_code: ::std::os::raw::c_int,
    );
    pub fn on_load_error(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        error_code: Errorcode,
        error_text: &CefString,
        failed_url: &CefString,
    );
);

pub trait ImplLoadHandler: Sized {
    fn on_loading_state_change(
        &self,
        browser: &mut Browser,
        is_loading: ::std::os::raw::c_int,
        can_go_back: ::std::os::raw::c_int,
        can_go_forward: ::std::os::raw::c_int,
    ) {
    }
    fn on_load_start(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        transition_type: TransitionType,
    ) {
    }
    fn on_load_end(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        http_status_code: ::std::os::raw::c_int,
    ) {
    }
    fn on_load_error(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        error_code: Errorcode,
        error_text: &CefString,
        failed_url: &CefString,
    ) {
    }

    fn into_raw(self) -> *mut _cef_load_handler_t {
        let mut object: _cef_load_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_load_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_load_handler_t {
    use super::*;

    pub fn init_methods<I: ImplLoadHandler>(object: &mut _cef_load_handler_t) {
        object.on_loading_state_change = Some(on_loading_state_change::<I>);
        object.on_load_start = Some(on_load_start::<I>);
        object.on_load_end = Some(on_load_end::<I>);
        object.on_load_error = Some(on_load_error::<I>);
    }

    extern "C" fn on_loading_state_change<I: ImplLoadHandler>(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        is_loading: ::std::os::raw::c_int,
        can_go_back: ::std::os::raw::c_int,
        can_go_forward: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_loading_state_change(browser, is_loading, can_go_back, can_go_forward)
    }

    extern "C" fn on_load_start<I: ImplLoadHandler>(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        transition_type: cef_transition_type_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_load_start(browser, frame, transition_type)
    }

    extern "C" fn on_load_end<I: ImplLoadHandler>(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        http_status_code: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        obj.interface.on_load_end(browser, frame, http_status_code)
    }

    extern "C" fn on_load_error<I: ImplLoadHandler>(
        self_: *mut _cef_load_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        error_code: cef_errorcode_t,
        error_text: *const cef_string_t,
        failed_url: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let error_text = &CefString::from(error_text);
        let failed_url = &CefString::from(failed_url);
        obj.interface
            .on_load_error(browser, frame, error_code, error_text, failed_url)
    }
}

wrapper!(
    #[doc = "See [_cef_media_access_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct MediaAccessCallback(_cef_media_access_callback_t);

    pub fn cont(&self, allowed_permissions: u32);
    pub fn cancel(&self);
);

pub trait ImplMediaAccessCallback: Sized {
    fn cont(&self, allowed_permissions: u32) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_media_access_callback_t {
        let mut object: _cef_media_access_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_media_access_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_media_access_callback_t {
    use super::*;

    pub fn init_methods<I: ImplMediaAccessCallback>(object: &mut _cef_media_access_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplMediaAccessCallback>(
        self_: *mut _cef_media_access_callback_t,
        allowed_permissions: u32,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(allowed_permissions)
    }

    extern "C" fn cancel<I: ImplMediaAccessCallback>(self_: *mut _cef_media_access_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_permission_prompt_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct PermissionPromptCallback(_cef_permission_prompt_callback_t);

    pub fn cont(&self, result: PermissionRequestResult);
);

pub trait ImplPermissionPromptCallback: Sized {
    fn cont(&self, result: PermissionRequestResult) {}

    fn into_raw(self) -> *mut _cef_permission_prompt_callback_t {
        let mut object: _cef_permission_prompt_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_permission_prompt_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_permission_prompt_callback_t {
    use super::*;

    pub fn init_methods<I: ImplPermissionPromptCallback>(
        object: &mut _cef_permission_prompt_callback_t,
    ) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplPermissionPromptCallback>(
        self_: *mut _cef_permission_prompt_callback_t,
        result: cef_permission_request_result_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(result)
    }
}

wrapper!(
    #[doc = "See [_cef_permission_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct PermissionHandler(_cef_permission_handler_t);

    pub fn on_request_media_access_permission(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        requesting_origin: &CefString,
        requested_permissions: u32,
        callback: &mut MediaAccessCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_show_permission_prompt(
        &self,
        browser: &mut Browser,
        prompt_id: u64,
        requesting_origin: &CefString,
        requested_permissions: u32,
        callback: &mut PermissionPromptCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_dismiss_permission_prompt(
        &self,
        browser: &mut Browser,
        prompt_id: u64,
        result: PermissionRequestResult,
    );
);

pub trait ImplPermissionHandler: Sized {
    fn on_request_media_access_permission(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        requesting_origin: &CefString,
        requested_permissions: u32,
        callback: &mut MediaAccessCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_show_permission_prompt(
        &self,
        browser: &mut Browser,
        prompt_id: u64,
        requesting_origin: &CefString,
        requested_permissions: u32,
        callback: &mut PermissionPromptCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_dismiss_permission_prompt(
        &self,
        browser: &mut Browser,
        prompt_id: u64,
        result: PermissionRequestResult,
    ) {
    }

    fn into_raw(self) -> *mut _cef_permission_handler_t {
        let mut object: _cef_permission_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_permission_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_permission_handler_t {
    use super::*;

    pub fn init_methods<I: ImplPermissionHandler>(object: &mut _cef_permission_handler_t) {
        object.on_request_media_access_permission = Some(on_request_media_access_permission::<I>);
        object.on_show_permission_prompt = Some(on_show_permission_prompt::<I>);
        object.on_dismiss_permission_prompt = Some(on_dismiss_permission_prompt::<I>);
    }

    extern "C" fn on_request_media_access_permission<I: ImplPermissionHandler>(
        self_: *mut _cef_permission_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        requesting_origin: *const cef_string_t,
        requested_permissions: u32,
        callback: *mut _cef_media_access_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let requesting_origin = &CefString::from(requesting_origin);
        let callback = &mut MediaAccessCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_request_media_access_permission(
                browser,
                frame,
                requesting_origin,
                requested_permissions,
                callback,
            )
            .into()
    }

    extern "C" fn on_show_permission_prompt<I: ImplPermissionHandler>(
        self_: *mut _cef_permission_handler_t,
        browser: *mut _cef_browser_t,
        prompt_id: u64,
        requesting_origin: *const cef_string_t,
        requested_permissions: u32,
        callback: *mut _cef_permission_prompt_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let requesting_origin = &CefString::from(requesting_origin);
        let callback =
            &mut PermissionPromptCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_show_permission_prompt(
                browser,
                prompt_id,
                requesting_origin,
                requested_permissions,
                callback,
            )
            .into()
    }

    extern "C" fn on_dismiss_permission_prompt<I: ImplPermissionHandler>(
        self_: *mut _cef_permission_handler_t,
        browser: *mut _cef_browser_t,
        prompt_id: u64,
        result: cef_permission_request_result_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_dismiss_permission_prompt(browser, prompt_id, result)
    }
}

wrapper!(
    #[doc = "See [_cef_print_settings_t] for more documentation."]
    #[derive(Clone)]
    pub struct PrintSettings(_cef_print_settings_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn set_orientation(&self, landscape: ::std::os::raw::c_int);
    pub fn is_landscape(&self) -> ::std::os::raw::c_int;
    pub fn set_printer_printable_area(
        &self,
        physical_size_device_units: &Size,
        printable_area_device_units: &Rect,
        landscape_needs_flip: ::std::os::raw::c_int,
    );
    pub fn set_device_name(&self, name: &CefString);
    pub fn get_device_name(&self) -> CefStringUserfree;
    pub fn set_dpi(&self, dpi: ::std::os::raw::c_int);
    pub fn get_dpi(&self) -> ::std::os::raw::c_int;
    pub fn set_page_ranges(&self, ranges_count: usize, ranges: &Range);
    pub fn get_page_ranges_count(&self) -> usize;
    pub fn get_page_ranges(&self, ranges_count: *mut usize, ranges: &mut Range);
    pub fn set_selection_only(&self, selection_only: ::std::os::raw::c_int);
    pub fn is_selection_only(&self) -> ::std::os::raw::c_int;
    pub fn set_collate(&self, collate: ::std::os::raw::c_int);
    pub fn will_collate(&self) -> ::std::os::raw::c_int;
    pub fn set_color_model(&self, model: ColorModel);
    pub fn get_color_model(&self) -> ColorModel;
    pub fn set_copies(&self, copies: ::std::os::raw::c_int);
    pub fn get_copies(&self) -> ::std::os::raw::c_int;
    pub fn set_duplex_mode(&self, mode: DuplexMode);
    pub fn get_duplex_mode(&self) -> DuplexMode;
);

pub trait ImplPrintSettings: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_orientation(&self, landscape: ::std::os::raw::c_int) {}
    fn is_landscape(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_printer_printable_area(
        &self,
        physical_size_device_units: &Size,
        printable_area_device_units: &Rect,
        landscape_needs_flip: ::std::os::raw::c_int,
    ) {
    }
    fn set_device_name(&self, name: &CefString) {}
    fn get_device_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_dpi(&self, dpi: ::std::os::raw::c_int) {}
    fn get_dpi(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_page_ranges(&self, ranges_count: usize, ranges: &Range) {}
    fn get_page_ranges_count(&self) -> usize {
        Default::default()
    }
    fn get_page_ranges(&self, ranges_count: *mut usize, ranges: &mut Range) {}
    fn set_selection_only(&self, selection_only: ::std::os::raw::c_int) {}
    fn is_selection_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_collate(&self, collate: ::std::os::raw::c_int) {}
    fn will_collate(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_color_model(&self, model: ColorModel) {}
    fn get_color_model(&self) -> ColorModel {
        Default::default()
    }
    fn set_copies(&self, copies: ::std::os::raw::c_int) {}
    fn get_copies(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_duplex_mode(&self, mode: DuplexMode) {}
    fn get_duplex_mode(&self) -> DuplexMode {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_print_settings_t {
        let mut object: _cef_print_settings_t = unsafe { std::mem::zeroed() };
        impl_cef_print_settings_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_print_settings_t {
    use super::*;

    pub fn init_methods<I: ImplPrintSettings>(object: &mut _cef_print_settings_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.set_orientation = Some(set_orientation::<I>);
        object.is_landscape = Some(is_landscape::<I>);
        object.set_printer_printable_area = Some(set_printer_printable_area::<I>);
        object.set_device_name = Some(set_device_name::<I>);
        object.get_device_name = Some(get_device_name::<I>);
        object.set_dpi = Some(set_dpi::<I>);
        object.get_dpi = Some(get_dpi::<I>);
        object.set_page_ranges = Some(set_page_ranges::<I>);
        object.get_page_ranges_count = Some(get_page_ranges_count::<I>);
        object.get_page_ranges = Some(get_page_ranges::<I>);
        object.set_selection_only = Some(set_selection_only::<I>);
        object.is_selection_only = Some(is_selection_only::<I>);
        object.set_collate = Some(set_collate::<I>);
        object.will_collate = Some(will_collate::<I>);
        object.set_color_model = Some(set_color_model::<I>);
        object.get_color_model = Some(get_color_model::<I>);
        object.set_copies = Some(set_copies::<I>);
        object.get_copies = Some(get_copies::<I>);
        object.set_duplex_mode = Some(set_duplex_mode::<I>);
        object.get_duplex_mode = Some(get_duplex_mode::<I>);
    }

    extern "C" fn is_valid<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_read_only<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn set_orientation<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        landscape: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_orientation(landscape)
    }

    extern "C" fn is_landscape<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_landscape().into()
    }

    extern "C" fn set_printer_printable_area<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        physical_size_device_units: *const cef_size_t,
        printable_area_device_units: *const cef_rect_t,
        landscape_needs_flip: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let physical_size_device_units =
            &Size(unsafe { RefGuard::from_raw_add_ref(physical_size_device_units) });
        let printable_area_device_units =
            &Rect(unsafe { RefGuard::from_raw_add_ref(printable_area_device_units) });
        obj.interface.set_printer_printable_area(
            physical_size_device_units,
            printable_area_device_units,
            landscape_needs_flip,
        )
    }

    extern "C" fn set_device_name<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        name: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.set_device_name(name)
    }

    extern "C" fn get_device_name<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_device_name().into()
    }

    extern "C" fn set_dpi<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        dpi: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_dpi(dpi)
    }

    extern "C" fn get_dpi<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_dpi().into()
    }

    extern "C" fn set_page_ranges<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        ranges_count: usize,
        ranges: *const cef_range_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let ranges = &Range(unsafe { RefGuard::from_raw_add_ref(ranges) });
        obj.interface.set_page_ranges(ranges_count, ranges)
    }

    extern "C" fn get_page_ranges_count<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_page_ranges_count().into()
    }

    extern "C" fn get_page_ranges<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        ranges_count: *mut usize,
        ranges: *mut cef_range_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let ranges = &mut Range(unsafe { RefGuard::from_raw_add_ref(ranges) });
        obj.interface.get_page_ranges(ranges_count, ranges)
    }

    extern "C" fn set_selection_only<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        selection_only: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_selection_only(selection_only)
    }

    extern "C" fn is_selection_only<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_selection_only().into()
    }

    extern "C" fn set_collate<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        collate: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_collate(collate)
    }

    extern "C" fn will_collate<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.will_collate().into()
    }

    extern "C" fn set_color_model<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        model: cef_color_model_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_color_model(model)
    }

    extern "C" fn get_color_model<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> cef_color_model_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_color_model().into()
    }

    extern "C" fn set_copies<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        copies: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_copies(copies)
    }

    extern "C" fn get_copies<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_copies().into()
    }

    extern "C" fn set_duplex_mode<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
        mode: cef_duplex_mode_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_duplex_mode(mode)
    }

    extern "C" fn get_duplex_mode<I: ImplPrintSettings>(
        self_: *mut _cef_print_settings_t,
    ) -> cef_duplex_mode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_duplex_mode().into()
    }
}

wrapper!(
    #[doc = "See [_cef_print_dialog_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct PrintDialogCallback(_cef_print_dialog_callback_t);

    pub fn cont(&self, settings: &mut PrintSettings);
    pub fn cancel(&self);
);

pub trait ImplPrintDialogCallback: Sized {
    fn cont(&self, settings: &mut PrintSettings) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_print_dialog_callback_t {
        let mut object: _cef_print_dialog_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_print_dialog_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_print_dialog_callback_t {
    use super::*;

    pub fn init_methods<I: ImplPrintDialogCallback>(object: &mut _cef_print_dialog_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplPrintDialogCallback>(
        self_: *mut _cef_print_dialog_callback_t,
        settings: *mut _cef_print_settings_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let settings = &mut PrintSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        obj.interface.cont(settings)
    }

    extern "C" fn cancel<I: ImplPrintDialogCallback>(self_: *mut _cef_print_dialog_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_print_job_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct PrintJobCallback(_cef_print_job_callback_t);

    pub fn cont(&self);
);

pub trait ImplPrintJobCallback: Sized {
    fn cont(&self) {}

    fn into_raw(self) -> *mut _cef_print_job_callback_t {
        let mut object: _cef_print_job_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_print_job_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_print_job_callback_t {
    use super::*;

    pub fn init_methods<I: ImplPrintJobCallback>(object: &mut _cef_print_job_callback_t) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplPrintJobCallback>(self_: *mut _cef_print_job_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont()
    }
}

wrapper!(
    #[doc = "See [_cef_print_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct PrintHandler(_cef_print_handler_t);

    pub fn on_print_start(&self, browser: &mut Browser);
    pub fn on_print_settings(
        &self,
        browser: &mut Browser,
        settings: &mut PrintSettings,
        get_defaults: ::std::os::raw::c_int,
    );
    pub fn on_print_dialog(
        &self,
        browser: &mut Browser,
        has_selection: ::std::os::raw::c_int,
        callback: &mut PrintDialogCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_print_job(
        &self,
        browser: &mut Browser,
        document_name: &CefString,
        pdf_file_path: &CefString,
        callback: &mut PrintJobCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_print_reset(&self, browser: &mut Browser);
    pub fn get_pdf_paper_size(
        &self,
        browser: &mut Browser,
        device_units_per_inch: ::std::os::raw::c_int,
    ) -> Size;
);

pub trait ImplPrintHandler: Sized {
    fn on_print_start(&self, browser: &mut Browser) {}
    fn on_print_settings(
        &self,
        browser: &mut Browser,
        settings: &mut PrintSettings,
        get_defaults: ::std::os::raw::c_int,
    ) {
    }
    fn on_print_dialog(
        &self,
        browser: &mut Browser,
        has_selection: ::std::os::raw::c_int,
        callback: &mut PrintDialogCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_print_job(
        &self,
        browser: &mut Browser,
        document_name: &CefString,
        pdf_file_path: &CefString,
        callback: &mut PrintJobCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_print_reset(&self, browser: &mut Browser) {}
    fn get_pdf_paper_size(
        &self,
        browser: &mut Browser,
        device_units_per_inch: ::std::os::raw::c_int,
    ) -> Size {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_print_handler_t {
        let mut object: _cef_print_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_print_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_print_handler_t {
    use super::*;

    pub fn init_methods<I: ImplPrintHandler>(object: &mut _cef_print_handler_t) {
        object.on_print_start = Some(on_print_start::<I>);
        object.on_print_settings = Some(on_print_settings::<I>);
        object.on_print_dialog = Some(on_print_dialog::<I>);
        object.on_print_job = Some(on_print_job::<I>);
        object.on_print_reset = Some(on_print_reset::<I>);
        object.get_pdf_paper_size = Some(get_pdf_paper_size::<I>);
    }

    extern "C" fn on_print_start<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_print_start(browser)
    }

    extern "C" fn on_print_settings<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
        settings: *mut _cef_print_settings_t,
        get_defaults: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let settings = &mut PrintSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        obj.interface
            .on_print_settings(browser, settings, get_defaults)
    }

    extern "C" fn on_print_dialog<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
        has_selection: ::std::os::raw::c_int,
        callback: *mut _cef_print_dialog_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let callback = &mut PrintDialogCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_print_dialog(browser, has_selection, callback)
            .into()
    }

    extern "C" fn on_print_job<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
        document_name: *const cef_string_t,
        pdf_file_path: *const cef_string_t,
        callback: *mut _cef_print_job_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let document_name = &CefString::from(document_name);
        let pdf_file_path = &CefString::from(pdf_file_path);
        let callback = &mut PrintJobCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_print_job(browser, document_name, pdf_file_path, callback)
            .into()
    }

    extern "C" fn on_print_reset<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_print_reset(browser)
    }

    extern "C" fn get_pdf_paper_size<I: ImplPrintHandler>(
        self_: *mut _cef_print_handler_t,
        browser: *mut _cef_browser_t,
        device_units_per_inch: ::std::os::raw::c_int,
    ) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .get_pdf_paper_size(browser, device_units_per_inch)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_accessibility_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct AccessibilityHandler(_cef_accessibility_handler_t);

    pub fn on_accessibility_tree_change(&self, value: &mut Value);
    pub fn on_accessibility_location_change(&self, value: &mut Value);
);

pub trait ImplAccessibilityHandler: Sized {
    fn on_accessibility_tree_change(&self, value: &mut Value) {}
    fn on_accessibility_location_change(&self, value: &mut Value) {}

    fn into_raw(self) -> *mut _cef_accessibility_handler_t {
        let mut object: _cef_accessibility_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_accessibility_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_accessibility_handler_t {
    use super::*;

    pub fn init_methods<I: ImplAccessibilityHandler>(object: &mut _cef_accessibility_handler_t) {
        object.on_accessibility_tree_change = Some(on_accessibility_tree_change::<I>);
        object.on_accessibility_location_change = Some(on_accessibility_location_change::<I>);
    }

    extern "C" fn on_accessibility_tree_change<I: ImplAccessibilityHandler>(
        self_: *mut _cef_accessibility_handler_t,
        value: *mut _cef_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.on_accessibility_tree_change(value)
    }

    extern "C" fn on_accessibility_location_change<I: ImplAccessibilityHandler>(
        self_: *mut _cef_accessibility_handler_t,
        value: *mut _cef_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut Value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.on_accessibility_location_change(value)
    }
}

wrapper!(
    #[doc = "See [_cef_render_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct RenderHandler(_cef_render_handler_t);

    pub fn get_accessibility_handler(&self) -> AccessibilityHandler;
    pub fn get_root_screen_rect(
        &self,
        browser: &mut Browser,
        rect: &mut Rect,
    ) -> ::std::os::raw::c_int;
    pub fn get_view_rect(&self, browser: &mut Browser, rect: &mut Rect);
    pub fn get_screen_point(
        &self,
        browser: &mut Browser,
        view_x: ::std::os::raw::c_int,
        view_y: ::std::os::raw::c_int,
        screen_x: *mut ::std::os::raw::c_int,
        screen_y: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_screen_info(
        &self,
        browser: &mut Browser,
        screen_info: &mut ScreenInfo,
    ) -> ::std::os::raw::c_int;
    pub fn on_popup_show(&self, browser: &mut Browser, show: ::std::os::raw::c_int);
    pub fn on_popup_size(&self, browser: &mut Browser, rect: &Rect);
    pub fn on_paint(
        &self,
        browser: &mut Browser,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: &Rect,
        buffer: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
    pub fn on_accelerated_paint(
        &self,
        browser: &mut Browser,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: &Rect,
        info: &AcceleratedPaintInfo,
    );
    pub fn get_touch_handle_size(
        &self,
        browser: &mut Browser,
        orientation: HorizontalAlignment,
        size: &mut Size,
    );
    pub fn on_touch_handle_state_changed(&self, browser: &mut Browser, state: &TouchHandleState);
    pub fn start_dragging(
        &self,
        browser: &mut Browser,
        drag_data: &mut DragData,
        allowed_ops: DragOperationsMask,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn update_drag_cursor(&self, browser: &mut Browser, operation: DragOperationsMask);
    pub fn on_scroll_offset_changed(&self, browser: &mut Browser, x: f64, y: f64);
    pub fn on_ime_composition_range_changed(
        &self,
        browser: &mut Browser,
        selected_range: &Range,
        character_bounds_count: usize,
        character_bounds: &Rect,
    );
    pub fn on_text_selection_changed(
        &self,
        browser: &mut Browser,
        selected_text: &CefString,
        selected_range: &Range,
    );
    pub fn on_virtual_keyboard_requested(&self, browser: &mut Browser, input_mode: TextInputMode);
);

pub trait ImplRenderHandler: Sized {
    fn get_accessibility_handler(&self) -> AccessibilityHandler {
        Default::default()
    }
    fn get_root_screen_rect(
        &self,
        browser: &mut Browser,
        rect: &mut Rect,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_view_rect(&self, browser: &mut Browser, rect: &mut Rect) {}
    fn get_screen_point(
        &self,
        browser: &mut Browser,
        view_x: ::std::os::raw::c_int,
        view_y: ::std::os::raw::c_int,
        screen_x: *mut ::std::os::raw::c_int,
        screen_y: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_screen_info(
        &self,
        browser: &mut Browser,
        screen_info: &mut ScreenInfo,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_popup_show(&self, browser: &mut Browser, show: ::std::os::raw::c_int) {}
    fn on_popup_size(&self, browser: &mut Browser, rect: &Rect) {}
    fn on_paint(
        &self,
        browser: &mut Browser,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: &Rect,
        buffer: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
    }
    fn on_accelerated_paint(
        &self,
        browser: &mut Browser,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: &Rect,
        info: &AcceleratedPaintInfo,
    ) {
    }
    fn get_touch_handle_size(
        &self,
        browser: &mut Browser,
        orientation: HorizontalAlignment,
        size: &mut Size,
    ) {
    }
    fn on_touch_handle_state_changed(&self, browser: &mut Browser, state: &TouchHandleState) {}
    fn start_dragging(
        &self,
        browser: &mut Browser,
        drag_data: &mut DragData,
        allowed_ops: DragOperationsMask,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn update_drag_cursor(&self, browser: &mut Browser, operation: DragOperationsMask) {}
    fn on_scroll_offset_changed(&self, browser: &mut Browser, x: f64, y: f64) {}
    fn on_ime_composition_range_changed(
        &self,
        browser: &mut Browser,
        selected_range: &Range,
        character_bounds_count: usize,
        character_bounds: &Rect,
    ) {
    }
    fn on_text_selection_changed(
        &self,
        browser: &mut Browser,
        selected_text: &CefString,
        selected_range: &Range,
    ) {
    }
    fn on_virtual_keyboard_requested(&self, browser: &mut Browser, input_mode: TextInputMode) {}

    fn into_raw(self) -> *mut _cef_render_handler_t {
        let mut object: _cef_render_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_render_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_render_handler_t {
    use super::*;

    pub fn init_methods<I: ImplRenderHandler>(object: &mut _cef_render_handler_t) {
        object.get_accessibility_handler = Some(get_accessibility_handler::<I>);
        object.get_root_screen_rect = Some(get_root_screen_rect::<I>);
        object.get_view_rect = Some(get_view_rect::<I>);
        object.get_screen_point = Some(get_screen_point::<I>);
        object.get_screen_info = Some(get_screen_info::<I>);
        object.on_popup_show = Some(on_popup_show::<I>);
        object.on_popup_size = Some(on_popup_size::<I>);
        object.on_paint = Some(on_paint::<I>);
        object.on_accelerated_paint = Some(on_accelerated_paint::<I>);
        object.get_touch_handle_size = Some(get_touch_handle_size::<I>);
        object.on_touch_handle_state_changed = Some(on_touch_handle_state_changed::<I>);
        object.start_dragging = Some(start_dragging::<I>);
        object.update_drag_cursor = Some(update_drag_cursor::<I>);
        object.on_scroll_offset_changed = Some(on_scroll_offset_changed::<I>);
        object.on_ime_composition_range_changed = Some(on_ime_composition_range_changed::<I>);
        object.on_text_selection_changed = Some(on_text_selection_changed::<I>);
        object.on_virtual_keyboard_requested = Some(on_virtual_keyboard_requested::<I>);
    }

    extern "C" fn get_accessibility_handler<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
    ) -> *mut _cef_accessibility_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_accessibility_handler().into()
    }

    extern "C" fn get_root_screen_rect<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        rect: *mut cef_rect_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let rect = &mut Rect(unsafe { RefGuard::from_raw_add_ref(rect) });
        obj.interface.get_root_screen_rect(browser, rect).into()
    }

    extern "C" fn get_view_rect<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        rect: *mut cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let rect = &mut Rect(unsafe { RefGuard::from_raw_add_ref(rect) });
        obj.interface.get_view_rect(browser, rect)
    }

    extern "C" fn get_screen_point<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        view_x: ::std::os::raw::c_int,
        view_y: ::std::os::raw::c_int,
        screen_x: *mut ::std::os::raw::c_int,
        screen_y: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .get_screen_point(browser, view_x, view_y, screen_x, screen_y)
            .into()
    }

    extern "C" fn get_screen_info<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        screen_info: *mut cef_screen_info_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let screen_info = &mut ScreenInfo(unsafe { RefGuard::from_raw_add_ref(screen_info) });
        obj.interface.get_screen_info(browser, screen_info).into()
    }

    extern "C" fn on_popup_show<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        show: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_popup_show(browser, show)
    }

    extern "C" fn on_popup_size<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        rect: *const cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let rect = &Rect(unsafe { RefGuard::from_raw_add_ref(rect) });
        obj.interface.on_popup_size(browser, rect)
    }

    extern "C" fn on_paint<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        type_: cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: *const cef_rect_t,
        buffer: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let dirty_rects = &Rect(unsafe { RefGuard::from_raw_add_ref(dirty_rects) });
        obj.interface.on_paint(
            browser,
            type_,
            dirty_rects_count,
            dirty_rects,
            buffer,
            width,
            height,
        )
    }

    extern "C" fn on_accelerated_paint<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        type_: cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: *const cef_rect_t,
        info: *const cef_accelerated_paint_info_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let dirty_rects = &Rect(unsafe { RefGuard::from_raw_add_ref(dirty_rects) });
        let info = &AcceleratedPaintInfo(unsafe { RefGuard::from_raw_add_ref(info) });
        obj.interface
            .on_accelerated_paint(browser, type_, dirty_rects_count, dirty_rects, info)
    }

    extern "C" fn get_touch_handle_size<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        orientation: cef_horizontal_alignment_t,
        size: *mut cef_size_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let size = &mut Size(unsafe { RefGuard::from_raw_add_ref(size) });
        obj.interface
            .get_touch_handle_size(browser, orientation, size)
    }

    extern "C" fn on_touch_handle_state_changed<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        state: *const cef_touch_handle_state_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let state = &TouchHandleState(unsafe { RefGuard::from_raw_add_ref(state) });
        obj.interface.on_touch_handle_state_changed(browser, state)
    }

    extern "C" fn start_dragging<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        drag_data: *mut _cef_drag_data_t,
        allowed_ops: cef_drag_operations_mask_t,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let drag_data = &mut DragData(unsafe { RefGuard::from_raw_add_ref(drag_data) });
        obj.interface
            .start_dragging(browser, drag_data, allowed_ops, x, y)
            .into()
    }

    extern "C" fn update_drag_cursor<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        operation: cef_drag_operations_mask_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.update_drag_cursor(browser, operation)
    }

    extern "C" fn on_scroll_offset_changed<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        x: f64,
        y: f64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_scroll_offset_changed(browser, x, y)
    }

    extern "C" fn on_ime_composition_range_changed<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        selected_range: *const cef_range_t,
        character_bounds_count: usize,
        character_bounds: *const cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let selected_range = &Range(unsafe { RefGuard::from_raw_add_ref(selected_range) });
        let character_bounds = &Rect(unsafe { RefGuard::from_raw_add_ref(character_bounds) });
        obj.interface.on_ime_composition_range_changed(
            browser,
            selected_range,
            character_bounds_count,
            character_bounds,
        )
    }

    extern "C" fn on_text_selection_changed<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        selected_text: *const cef_string_t,
        selected_range: *const cef_range_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let selected_text = &CefString::from(selected_text);
        let selected_range = &Range(unsafe { RefGuard::from_raw_add_ref(selected_range) });
        obj.interface
            .on_text_selection_changed(browser, selected_text, selected_range)
    }

    extern "C" fn on_virtual_keyboard_requested<I: ImplRenderHandler>(
        self_: *mut _cef_render_handler_t,
        browser: *mut _cef_browser_t,
        input_mode: cef_text_input_mode_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface
            .on_virtual_keyboard_requested(browser, input_mode)
    }
}

wrapper!(
    #[doc = "See [_cef_auth_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct AuthCallback(_cef_auth_callback_t);

    pub fn cont(&self, username: &CefString, password: &CefString);
    pub fn cancel(&self);
);

pub trait ImplAuthCallback: Sized {
    fn cont(&self, username: &CefString, password: &CefString) {}
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_auth_callback_t {
        let mut object: _cef_auth_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_auth_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_auth_callback_t {
    use super::*;

    pub fn init_methods<I: ImplAuthCallback>(object: &mut _cef_auth_callback_t) {
        object.cont = Some(cont::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn cont<I: ImplAuthCallback>(
        self_: *mut _cef_auth_callback_t,
        username: *const cef_string_t,
        password: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let username = &CefString::from(username);
        let password = &CefString::from(password);
        obj.interface.cont(username, password)
    }

    extern "C" fn cancel<I: ImplAuthCallback>(self_: *mut _cef_auth_callback_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_response_t] for more documentation."]
    #[derive(Clone)]
    pub struct Response(_cef_response_t);

    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn get_error(&self) -> Errorcode;
    pub fn set_error(&self, error: Errorcode);
    pub fn get_status(&self) -> ::std::os::raw::c_int;
    pub fn set_status(&self, status: ::std::os::raw::c_int);
    pub fn get_status_text(&self) -> CefStringUserfree;
    pub fn set_status_text(&self, status_text: &CefString);
    pub fn get_mime_type(&self) -> CefStringUserfree;
    pub fn set_mime_type(&self, mime_type: &CefString);
    pub fn get_charset(&self) -> CefStringUserfree;
    pub fn set_charset(&self, charset: &CefString);
    pub fn get_header_by_name(&self, name: &CefString) -> CefStringUserfree;
    pub fn set_header_by_name(
        &self,
        name: &CefString,
        value: &CefString,
        overwrite: ::std::os::raw::c_int,
    );
    pub fn get_header_map(&self, header_map: &mut CefStringMultimap);
    pub fn set_header_map(&self, header_map: &mut CefStringMultimap);
    pub fn get_url(&self) -> CefStringUserfree;
    pub fn set_url(&self, url: &CefString);
);

pub trait ImplResponse: Sized {
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_error(&self) -> Errorcode {
        Default::default()
    }
    fn set_error(&self, error: Errorcode) {}
    fn get_status(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_status(&self, status: ::std::os::raw::c_int) {}
    fn get_status_text(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_status_text(&self, status_text: &CefString) {}
    fn get_mime_type(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_mime_type(&self, mime_type: &CefString) {}
    fn get_charset(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_charset(&self, charset: &CefString) {}
    fn get_header_by_name(&self, name: &CefString) -> CefStringUserfree {
        Default::default()
    }
    fn set_header_by_name(
        &self,
        name: &CefString,
        value: &CefString,
        overwrite: ::std::os::raw::c_int,
    ) {
    }
    fn get_header_map(&self, header_map: &mut CefStringMultimap) {}
    fn set_header_map(&self, header_map: &mut CefStringMultimap) {}
    fn get_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_url(&self, url: &CefString) {}

    fn into_raw(self) -> *mut _cef_response_t {
        let mut object: _cef_response_t = unsafe { std::mem::zeroed() };
        impl_cef_response_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_response_t {
    use super::*;

    pub fn init_methods<I: ImplResponse>(object: &mut _cef_response_t) {
        object.is_read_only = Some(is_read_only::<I>);
        object.get_error = Some(get_error::<I>);
        object.set_error = Some(set_error::<I>);
        object.get_status = Some(get_status::<I>);
        object.set_status = Some(set_status::<I>);
        object.get_status_text = Some(get_status_text::<I>);
        object.set_status_text = Some(set_status_text::<I>);
        object.get_mime_type = Some(get_mime_type::<I>);
        object.set_mime_type = Some(set_mime_type::<I>);
        object.get_charset = Some(get_charset::<I>);
        object.set_charset = Some(set_charset::<I>);
        object.get_header_by_name = Some(get_header_by_name::<I>);
        object.set_header_by_name = Some(set_header_by_name::<I>);
        object.get_header_map = Some(get_header_map::<I>);
        object.set_header_map = Some(set_header_map::<I>);
        object.get_url = Some(get_url::<I>);
        object.set_url = Some(set_url::<I>);
    }

    extern "C" fn is_read_only<I: ImplResponse>(
        self_: *mut _cef_response_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn get_error<I: ImplResponse>(self_: *mut _cef_response_t) -> cef_errorcode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_error().into()
    }

    extern "C" fn set_error<I: ImplResponse>(self_: *mut _cef_response_t, error: cef_errorcode_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_error(error)
    }

    extern "C" fn get_status<I: ImplResponse>(
        self_: *mut _cef_response_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_status().into()
    }

    extern "C" fn set_status<I: ImplResponse>(
        self_: *mut _cef_response_t,
        status: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_status(status)
    }

    extern "C" fn get_status_text<I: ImplResponse>(
        self_: *mut _cef_response_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_status_text().into()
    }

    extern "C" fn set_status_text<I: ImplResponse>(
        self_: *mut _cef_response_t,
        status_text: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let status_text = &CefString::from(status_text);
        obj.interface.set_status_text(status_text)
    }

    extern "C" fn get_mime_type<I: ImplResponse>(
        self_: *mut _cef_response_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_mime_type().into()
    }

    extern "C" fn set_mime_type<I: ImplResponse>(
        self_: *mut _cef_response_t,
        mime_type: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let mime_type = &CefString::from(mime_type);
        obj.interface.set_mime_type(mime_type)
    }

    extern "C" fn get_charset<I: ImplResponse>(
        self_: *mut _cef_response_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_charset().into()
    }

    extern "C" fn set_charset<I: ImplResponse>(
        self_: *mut _cef_response_t,
        charset: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let charset = &CefString::from(charset);
        obj.interface.set_charset(charset)
    }

    extern "C" fn get_header_by_name<I: ImplResponse>(
        self_: *mut _cef_response_t,
        name: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.get_header_by_name(name).into()
    }

    extern "C" fn set_header_by_name<I: ImplResponse>(
        self_: *mut _cef_response_t,
        name: *const cef_string_t,
        value: *const cef_string_t,
        overwrite: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let value = &CefString::from(value);
        obj.interface.set_header_by_name(name, value, overwrite)
    }

    extern "C" fn get_header_map<I: ImplResponse>(
        self_: *mut _cef_response_t,
        header_map: cef_string_multimap_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let header_map = &mut CefStringMultimap(unsafe { RefGuard::from_raw_add_ref(header_map) });
        obj.interface.get_header_map(header_map)
    }

    extern "C" fn set_header_map<I: ImplResponse>(
        self_: *mut _cef_response_t,
        header_map: cef_string_multimap_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let header_map = &mut CefStringMultimap(unsafe { RefGuard::from_raw_add_ref(header_map) });
        obj.interface.set_header_map(header_map)
    }

    extern "C" fn get_url<I: ImplResponse>(self_: *mut _cef_response_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_url().into()
    }

    extern "C" fn set_url<I: ImplResponse>(self_: *mut _cef_response_t, url: *const cef_string_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let url = &CefString::from(url);
        obj.interface.set_url(url)
    }
}

wrapper!(
    #[doc = "See [_cef_resource_skip_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResourceSkipCallback(_cef_resource_skip_callback_t);

    pub fn cont(&self, bytes_skipped: i64);
);

pub trait ImplResourceSkipCallback: Sized {
    fn cont(&self, bytes_skipped: i64) {}

    fn into_raw(self) -> *mut _cef_resource_skip_callback_t {
        let mut object: _cef_resource_skip_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_resource_skip_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resource_skip_callback_t {
    use super::*;

    pub fn init_methods<I: ImplResourceSkipCallback>(object: &mut _cef_resource_skip_callback_t) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplResourceSkipCallback>(
        self_: *mut _cef_resource_skip_callback_t,
        bytes_skipped: i64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(bytes_skipped)
    }
}

wrapper!(
    #[doc = "See [_cef_resource_read_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResourceReadCallback(_cef_resource_read_callback_t);

    pub fn cont(&self, bytes_read: ::std::os::raw::c_int);
);

pub trait ImplResourceReadCallback: Sized {
    fn cont(&self, bytes_read: ::std::os::raw::c_int) {}

    fn into_raw(self) -> *mut _cef_resource_read_callback_t {
        let mut object: _cef_resource_read_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_resource_read_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resource_read_callback_t {
    use super::*;

    pub fn init_methods<I: ImplResourceReadCallback>(object: &mut _cef_resource_read_callback_t) {
        object.cont = Some(cont::<I>);
    }

    extern "C" fn cont<I: ImplResourceReadCallback>(
        self_: *mut _cef_resource_read_callback_t,
        bytes_read: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cont(bytes_read)
    }
}

wrapper!(
    #[doc = "See [_cef_resource_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResourceHandler(_cef_resource_handler_t);

    pub fn open(
        &self,
        request: &mut Request,
        handle_request: *mut ::std::os::raw::c_int,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int;
    pub fn process_request(
        &self,
        request: &mut Request,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int;
    pub fn get_response_headers(
        &self,
        response: &mut Response,
        response_length: *mut i64,
        redirect_url: &mut CefString,
    );
    pub fn skip(
        &self,
        bytes_to_skip: i64,
        bytes_skipped: *mut i64,
        callback: &mut ResourceSkipCallback,
    ) -> ::std::os::raw::c_int;
    pub fn read(
        &self,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: &mut ResourceReadCallback,
    ) -> ::std::os::raw::c_int;
    pub fn read_response(
        &self,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int;
    pub fn cancel(&self);
);

pub trait ImplResourceHandler: Sized {
    fn open(
        &self,
        request: &mut Request,
        handle_request: *mut ::std::os::raw::c_int,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn process_request(
        &self,
        request: &mut Request,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_response_headers(
        &self,
        response: &mut Response,
        response_length: *mut i64,
        redirect_url: &mut CefString,
    ) {
    }
    fn skip(
        &self,
        bytes_to_skip: i64,
        bytes_skipped: *mut i64,
        callback: &mut ResourceSkipCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn read(
        &self,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: &mut ResourceReadCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn read_response(
        &self,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn cancel(&self) {}

    fn into_raw(self) -> *mut _cef_resource_handler_t {
        let mut object: _cef_resource_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_resource_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resource_handler_t {
    use super::*;

    pub fn init_methods<I: ImplResourceHandler>(object: &mut _cef_resource_handler_t) {
        object.open = Some(open::<I>);
        object.process_request = Some(process_request::<I>);
        object.get_response_headers = Some(get_response_headers::<I>);
        object.skip = Some(skip::<I>);
        object.read = Some(read::<I>);
        object.read_response = Some(read_response::<I>);
        object.cancel = Some(cancel::<I>);
    }

    extern "C" fn open<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        request: *mut _cef_request_t,
        handle_request: *mut ::std::os::raw::c_int,
        callback: *mut _cef_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let callback = &mut Callback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.open(request, handle_request, callback).into()
    }

    extern "C" fn process_request<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        request: *mut _cef_request_t,
        callback: *mut _cef_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let callback = &mut Callback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface.process_request(request, callback).into()
    }

    extern "C" fn get_response_headers<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        response: *mut _cef_response_t,
        response_length: *mut i64,
        redirect_url: *mut cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        let redirect_url = &mut CefString::from(redirect_url);
        obj.interface
            .get_response_headers(response, response_length, redirect_url)
    }

    extern "C" fn skip<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        bytes_to_skip: i64,
        bytes_skipped: *mut i64,
        callback: *mut _cef_resource_skip_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut ResourceSkipCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .skip(bytes_to_skip, bytes_skipped, callback)
            .into()
    }

    extern "C" fn read<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: *mut _cef_resource_read_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut ResourceReadCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .read(data_out, bytes_to_read, bytes_read, callback)
            .into()
    }

    extern "C" fn read_response<I: ImplResourceHandler>(
        self_: *mut _cef_resource_handler_t,
        data_out: *mut ::std::os::raw::c_void,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_read: *mut ::std::os::raw::c_int,
        callback: *mut _cef_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let callback = &mut Callback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .read_response(data_out, bytes_to_read, bytes_read, callback)
            .into()
    }

    extern "C" fn cancel<I: ImplResourceHandler>(self_: *mut _cef_resource_handler_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel()
    }
}

wrapper!(
    #[doc = "See [_cef_response_filter_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResponseFilter(_cef_response_filter_t);

    pub fn init_filter(&self) -> ::std::os::raw::c_int;
    pub fn filter(
        &self,
        data_in: *mut ::std::os::raw::c_void,
        data_in_size: usize,
        data_in_read: *mut usize,
        data_out: *mut ::std::os::raw::c_void,
        data_out_size: usize,
        data_out_written: *mut usize,
    ) -> ResponseFilterStatus;
);

pub trait ImplResponseFilter: Sized {
    fn init_filter(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn filter(
        &self,
        data_in: *mut ::std::os::raw::c_void,
        data_in_size: usize,
        data_in_read: *mut usize,
        data_out: *mut ::std::os::raw::c_void,
        data_out_size: usize,
        data_out_written: *mut usize,
    ) -> ResponseFilterStatus {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_response_filter_t {
        let mut object: _cef_response_filter_t = unsafe { std::mem::zeroed() };
        impl_cef_response_filter_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_response_filter_t {
    use super::*;

    pub fn init_methods<I: ImplResponseFilter>(object: &mut _cef_response_filter_t) {
        object.init_filter = Some(init_filter::<I>);
        object.filter = Some(filter::<I>);
    }

    extern "C" fn init_filter<I: ImplResponseFilter>(
        self_: *mut _cef_response_filter_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.init_filter().into()
    }

    extern "C" fn filter<I: ImplResponseFilter>(
        self_: *mut _cef_response_filter_t,
        data_in: *mut ::std::os::raw::c_void,
        data_in_size: usize,
        data_in_read: *mut usize,
        data_out: *mut ::std::os::raw::c_void,
        data_out_size: usize,
        data_out_written: *mut usize,
    ) -> cef_response_filter_status_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .filter(
                data_in,
                data_in_size,
                data_in_read,
                data_out,
                data_out_size,
                data_out_written,
            )
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_resource_request_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResourceRequestHandler(_cef_resource_request_handler_t);

    pub fn get_cookie_access_filter(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
    ) -> CookieAccessFilter;
    pub fn on_before_resource_load(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        callback: &mut Callback,
    ) -> ReturnValue;
    pub fn get_resource_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
    ) -> ResourceHandler;
    pub fn on_resource_redirect(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        new_url: &mut CefString,
    );
    pub fn on_resource_response(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
    ) -> ::std::os::raw::c_int;
    pub fn get_resource_response_filter(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
    ) -> ResponseFilter;
    pub fn on_resource_load_complete(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        status: UrlrequestStatus,
        received_content_length: i64,
    );
    pub fn on_protocol_execution(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        allow_os_execution: *mut ::std::os::raw::c_int,
    );
);

pub trait ImplResourceRequestHandler: Sized {
    fn get_cookie_access_filter(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
    ) -> CookieAccessFilter {
        Default::default()
    }
    fn on_before_resource_load(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        callback: &mut Callback,
    ) -> ReturnValue {
        Default::default()
    }
    fn get_resource_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
    ) -> ResourceHandler {
        Default::default()
    }
    fn on_resource_redirect(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        new_url: &mut CefString,
    ) {
    }
    fn on_resource_response(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_resource_response_filter(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
    ) -> ResponseFilter {
        Default::default()
    }
    fn on_resource_load_complete(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        status: UrlrequestStatus,
        received_content_length: i64,
    ) {
    }
    fn on_protocol_execution(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        allow_os_execution: *mut ::std::os::raw::c_int,
    ) {
    }

    fn into_raw(self) -> *mut _cef_resource_request_handler_t {
        let mut object: _cef_resource_request_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_resource_request_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resource_request_handler_t {
    use super::*;

    pub fn init_methods<I: ImplResourceRequestHandler>(
        object: &mut _cef_resource_request_handler_t,
    ) {
        object.get_cookie_access_filter = Some(get_cookie_access_filter::<I>);
        object.on_before_resource_load = Some(on_before_resource_load::<I>);
        object.get_resource_handler = Some(get_resource_handler::<I>);
        object.on_resource_redirect = Some(on_resource_redirect::<I>);
        object.on_resource_response = Some(on_resource_response::<I>);
        object.get_resource_response_filter = Some(get_resource_response_filter::<I>);
        object.on_resource_load_complete = Some(on_resource_load_complete::<I>);
        object.on_protocol_execution = Some(on_protocol_execution::<I>);
    }

    extern "C" fn get_cookie_access_filter<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
    ) -> *mut _cef_cookie_access_filter_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface
            .get_cookie_access_filter(browser, frame, request)
            .into()
    }

    extern "C" fn on_before_resource_load<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        callback: *mut _cef_callback_t,
    ) -> cef_return_value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let callback = &mut Callback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_before_resource_load(browser, frame, request, callback)
            .into()
    }

    extern "C" fn get_resource_handler<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
    ) -> *mut _cef_resource_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface
            .get_resource_handler(browser, frame, request)
            .into()
    }

    extern "C" fn on_resource_redirect<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        response: *mut _cef_response_t,
        new_url: *mut cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        let new_url = &mut CefString::from(new_url);
        obj.interface
            .on_resource_redirect(browser, frame, request, response, new_url)
    }

    extern "C" fn on_resource_response<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        response: *mut _cef_response_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        obj.interface
            .on_resource_response(browser, frame, request, response)
            .into()
    }

    extern "C" fn get_resource_response_filter<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        response: *mut _cef_response_t,
    ) -> *mut _cef_response_filter_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        obj.interface
            .get_resource_response_filter(browser, frame, request, response)
            .into()
    }

    extern "C" fn on_resource_load_complete<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        response: *mut _cef_response_t,
        status: cef_urlrequest_status_t,
        received_content_length: i64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        obj.interface.on_resource_load_complete(
            browser,
            frame,
            request,
            response,
            status,
            received_content_length,
        )
    }

    extern "C" fn on_protocol_execution<I: ImplResourceRequestHandler>(
        self_: *mut _cef_resource_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        allow_os_execution: *mut ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface
            .on_protocol_execution(browser, frame, request, allow_os_execution)
    }
}

wrapper!(
    #[doc = "See [_cef_cookie_access_filter_t] for more documentation."]
    #[derive(Clone)]
    pub struct CookieAccessFilter(_cef_cookie_access_filter_t);

    pub fn can_send_cookie(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        cookie: &Cookie,
    ) -> ::std::os::raw::c_int;
    pub fn can_save_cookie(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        cookie: &Cookie,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplCookieAccessFilter: Sized {
    fn can_send_cookie(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        cookie: &Cookie,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn can_save_cookie(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        response: &mut Response,
        cookie: &Cookie,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_cookie_access_filter_t {
        let mut object: _cef_cookie_access_filter_t = unsafe { std::mem::zeroed() };
        impl_cef_cookie_access_filter_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_cookie_access_filter_t {
    use super::*;

    pub fn init_methods<I: ImplCookieAccessFilter>(object: &mut _cef_cookie_access_filter_t) {
        object.can_send_cookie = Some(can_send_cookie::<I>);
        object.can_save_cookie = Some(can_save_cookie::<I>);
    }

    extern "C" fn can_send_cookie<I: ImplCookieAccessFilter>(
        self_: *mut _cef_cookie_access_filter_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        cookie: *const _cef_cookie_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let cookie = &Cookie(unsafe { RefGuard::from_raw_add_ref(cookie) });
        obj.interface
            .can_send_cookie(browser, frame, request, cookie)
            .into()
    }

    extern "C" fn can_save_cookie<I: ImplCookieAccessFilter>(
        self_: *mut _cef_cookie_access_filter_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        response: *mut _cef_response_t,
        cookie: *const _cef_cookie_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let response = &mut Response(unsafe { RefGuard::from_raw_add_ref(response) });
        let cookie = &Cookie(unsafe { RefGuard::from_raw_add_ref(cookie) });
        obj.interface
            .can_save_cookie(browser, frame, request, response, cookie)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_sslinfo_t] for more documentation."]
    #[derive(Clone)]
    pub struct Sslinfo(_cef_sslinfo_t);

    pub fn get_cert_status(&self) -> CertStatus;
    pub fn get_x509certificate(&self) -> X509certificate;
);

pub trait ImplSslinfo: Sized {
    fn get_cert_status(&self) -> CertStatus {
        Default::default()
    }
    fn get_x509certificate(&self) -> X509certificate {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_sslinfo_t {
        let mut object: _cef_sslinfo_t = unsafe { std::mem::zeroed() };
        impl_cef_sslinfo_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_sslinfo_t {
    use super::*;

    pub fn init_methods<I: ImplSslinfo>(object: &mut _cef_sslinfo_t) {
        object.get_cert_status = Some(get_cert_status::<I>);
        object.get_x509certificate = Some(get_x509certificate::<I>);
    }

    extern "C" fn get_cert_status<I: ImplSslinfo>(self_: *mut _cef_sslinfo_t) -> cef_cert_status_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_cert_status().into()
    }

    extern "C" fn get_x509certificate<I: ImplSslinfo>(
        self_: *mut _cef_sslinfo_t,
    ) -> *mut _cef_x509certificate_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_x509certificate().into()
    }
}

wrapper!(
    #[doc = "See [_cef_unresponsive_process_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct UnresponsiveProcessCallback(_cef_unresponsive_process_callback_t);

    pub fn wait(&self);
    pub fn terminate(&self);
);

pub trait ImplUnresponsiveProcessCallback: Sized {
    fn wait(&self) {}
    fn terminate(&self) {}

    fn into_raw(self) -> *mut _cef_unresponsive_process_callback_t {
        let mut object: _cef_unresponsive_process_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_unresponsive_process_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_unresponsive_process_callback_t {
    use super::*;

    pub fn init_methods<I: ImplUnresponsiveProcessCallback>(
        object: &mut _cef_unresponsive_process_callback_t,
    ) {
        object.wait = Some(wait::<I>);
        object.terminate = Some(terminate::<I>);
    }

    extern "C" fn wait<I: ImplUnresponsiveProcessCallback>(
        self_: *mut _cef_unresponsive_process_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.wait()
    }

    extern "C" fn terminate<I: ImplUnresponsiveProcessCallback>(
        self_: *mut _cef_unresponsive_process_callback_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.terminate()
    }
}

wrapper!(
    #[doc = "See [_cef_select_client_certificate_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct SelectClientCertificateCallback(_cef_select_client_certificate_callback_t);

    pub fn select(&self, cert: &mut X509certificate);
);

pub trait ImplSelectClientCertificateCallback: Sized {
    fn select(&self, cert: &mut X509certificate) {}

    fn into_raw(self) -> *mut _cef_select_client_certificate_callback_t {
        let mut object: _cef_select_client_certificate_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_select_client_certificate_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_select_client_certificate_callback_t {
    use super::*;

    pub fn init_methods<I: ImplSelectClientCertificateCallback>(
        object: &mut _cef_select_client_certificate_callback_t,
    ) {
        object.select = Some(select::<I>);
    }

    extern "C" fn select<I: ImplSelectClientCertificateCallback>(
        self_: *mut _cef_select_client_certificate_callback_t,
        cert: *mut _cef_x509certificate_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let cert = &mut X509certificate(unsafe { RefGuard::from_raw_add_ref(cert) });
        obj.interface.select(cert)
    }
}

wrapper!(
    #[doc = "See [_cef_request_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct RequestHandler(_cef_request_handler_t);

    pub fn on_before_browse(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        user_gesture: ::std::os::raw::c_int,
        is_redirect: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_open_urlfrom_tab(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        target_url: &CefString,
        target_disposition: WindowOpenDisposition,
        user_gesture: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_resource_request_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: &CefString,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> ResourceRequestHandler;
    pub fn get_auth_credentials(
        &self,
        browser: &mut Browser,
        origin_url: &CefString,
        is_proxy: ::std::os::raw::c_int,
        host: &CefString,
        port: ::std::os::raw::c_int,
        realm: &CefString,
        scheme: &CefString,
        callback: &mut AuthCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_certificate_error(
        &self,
        browser: &mut Browser,
        cert_error: Errorcode,
        request_url: &CefString,
        ssl_info: &mut Sslinfo,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int;
    pub fn on_select_client_certificate(
        &self,
        browser: &mut Browser,
        is_proxy: ::std::os::raw::c_int,
        host: &CefString,
        port: ::std::os::raw::c_int,
        certificates_count: usize,
        certificates: *const &mut X509certificate,
        callback: &mut SelectClientCertificateCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_render_view_ready(&self, browser: &mut Browser);
    pub fn on_render_process_unresponsive(
        &self,
        browser: &mut Browser,
        callback: &mut UnresponsiveProcessCallback,
    ) -> ::std::os::raw::c_int;
    pub fn on_render_process_responsive(&self, browser: &mut Browser);
    pub fn on_render_process_terminated(
        &self,
        browser: &mut Browser,
        status: TerminationStatus,
        error_code: ::std::os::raw::c_int,
        error_string: &CefString,
    );
    pub fn on_document_available_in_main_frame(&self, browser: &mut Browser);
);

pub trait ImplRequestHandler: Sized {
    fn on_before_browse(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        user_gesture: ::std::os::raw::c_int,
        is_redirect: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_open_urlfrom_tab(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        target_url: &CefString,
        target_disposition: WindowOpenDisposition,
        user_gesture: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_resource_request_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: &CefString,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> ResourceRequestHandler {
        Default::default()
    }
    fn get_auth_credentials(
        &self,
        browser: &mut Browser,
        origin_url: &CefString,
        is_proxy: ::std::os::raw::c_int,
        host: &CefString,
        port: ::std::os::raw::c_int,
        realm: &CefString,
        scheme: &CefString,
        callback: &mut AuthCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_certificate_error(
        &self,
        browser: &mut Browser,
        cert_error: Errorcode,
        request_url: &CefString,
        ssl_info: &mut Sslinfo,
        callback: &mut Callback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_select_client_certificate(
        &self,
        browser: &mut Browser,
        is_proxy: ::std::os::raw::c_int,
        host: &CefString,
        port: ::std::os::raw::c_int,
        certificates_count: usize,
        certificates: *const &mut X509certificate,
        callback: &mut SelectClientCertificateCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_render_view_ready(&self, browser: &mut Browser) {}
    fn on_render_process_unresponsive(
        &self,
        browser: &mut Browser,
        callback: &mut UnresponsiveProcessCallback,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_render_process_responsive(&self, browser: &mut Browser) {}
    fn on_render_process_terminated(
        &self,
        browser: &mut Browser,
        status: TerminationStatus,
        error_code: ::std::os::raw::c_int,
        error_string: &CefString,
    ) {
    }
    fn on_document_available_in_main_frame(&self, browser: &mut Browser) {}

    fn into_raw(self) -> *mut _cef_request_handler_t {
        let mut object: _cef_request_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_request_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_request_handler_t {
    use super::*;

    pub fn init_methods<I: ImplRequestHandler>(object: &mut _cef_request_handler_t) {
        object.on_before_browse = Some(on_before_browse::<I>);
        object.on_open_urlfrom_tab = Some(on_open_urlfrom_tab::<I>);
        object.get_resource_request_handler = Some(get_resource_request_handler::<I>);
        object.get_auth_credentials = Some(get_auth_credentials::<I>);
        object.on_certificate_error = Some(on_certificate_error::<I>);
        object.on_select_client_certificate = Some(on_select_client_certificate::<I>);
        object.on_render_view_ready = Some(on_render_view_ready::<I>);
        object.on_render_process_unresponsive = Some(on_render_process_unresponsive::<I>);
        object.on_render_process_responsive = Some(on_render_process_responsive::<I>);
        object.on_render_process_terminated = Some(on_render_process_terminated::<I>);
        object.on_document_available_in_main_frame = Some(on_document_available_in_main_frame::<I>);
    }

    extern "C" fn on_before_browse<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        user_gesture: ::std::os::raw::c_int,
        is_redirect: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface
            .on_before_browse(browser, frame, request, user_gesture, is_redirect)
            .into()
    }

    extern "C" fn on_open_urlfrom_tab<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        target_url: *const cef_string_t,
        target_disposition: cef_window_open_disposition_t,
        user_gesture: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let target_url = &CefString::from(target_url);
        obj.interface
            .on_open_urlfrom_tab(browser, frame, target_url, target_disposition, user_gesture)
            .into()
    }

    extern "C" fn get_resource_request_handler<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_resource_request_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let request_initiator = &CefString::from(request_initiator);
        obj.interface
            .get_resource_request_handler(
                browser,
                frame,
                request,
                is_navigation,
                is_download,
                request_initiator,
                disable_default_handling,
            )
            .into()
    }

    extern "C" fn get_auth_credentials<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        origin_url: *const cef_string_t,
        is_proxy: ::std::os::raw::c_int,
        host: *const cef_string_t,
        port: ::std::os::raw::c_int,
        realm: *const cef_string_t,
        scheme: *const cef_string_t,
        callback: *mut _cef_auth_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let origin_url = &CefString::from(origin_url);
        let host = &CefString::from(host);
        let realm = &CefString::from(realm);
        let scheme = &CefString::from(scheme);
        let callback = &mut AuthCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .get_auth_credentials(
                browser, origin_url, is_proxy, host, port, realm, scheme, callback,
            )
            .into()
    }

    extern "C" fn on_certificate_error<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        cert_error: cef_errorcode_t,
        request_url: *const cef_string_t,
        ssl_info: *mut _cef_sslinfo_t,
        callback: *mut _cef_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let request_url = &CefString::from(request_url);
        let ssl_info = &mut Sslinfo(unsafe { RefGuard::from_raw_add_ref(ssl_info) });
        let callback = &mut Callback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_certificate_error(browser, cert_error, request_url, ssl_info, callback)
            .into()
    }

    extern "C" fn on_select_client_certificate<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        is_proxy: ::std::os::raw::c_int,
        host: *const cef_string_t,
        port: ::std::os::raw::c_int,
        certificates_count: usize,
        certificates: *const *mut _cef_x509certificate_t,
        callback: *mut _cef_select_client_certificate_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let host = &CefString::from(host);
        let callback =
            &mut SelectClientCertificateCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_select_client_certificate(
                browser,
                is_proxy,
                host,
                port,
                certificates_count,
                certificates,
                callback,
            )
            .into()
    }

    extern "C" fn on_render_view_ready<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_render_view_ready(browser)
    }

    extern "C" fn on_render_process_unresponsive<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        callback: *mut _cef_unresponsive_process_callback_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let callback =
            &mut UnresponsiveProcessCallback(unsafe { RefGuard::from_raw_add_ref(callback) });
        obj.interface
            .on_render_process_unresponsive(browser, callback)
            .into()
    }

    extern "C" fn on_render_process_responsive<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_render_process_responsive(browser)
    }

    extern "C" fn on_render_process_terminated<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
        status: cef_termination_status_t,
        error_code: ::std::os::raw::c_int,
        error_string: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let error_string = &CefString::from(error_string);
        obj.interface
            .on_render_process_terminated(browser, status, error_code, error_string)
    }

    extern "C" fn on_document_available_in_main_frame<I: ImplRequestHandler>(
        self_: *mut _cef_request_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_document_available_in_main_frame(browser)
    }
}

wrapper!(
    #[doc = "See [_cef_client_t] for more documentation."]
    #[derive(Clone)]
    pub struct Client(_cef_client_t);

    pub fn get_audio_handler(&self) -> AudioHandler;
    pub fn get_command_handler(&self) -> CommandHandler;
    pub fn get_context_menu_handler(&self) -> ContextMenuHandler;
    pub fn get_dialog_handler(&self) -> DialogHandler;
    pub fn get_display_handler(&self) -> DisplayHandler;
    pub fn get_download_handler(&self) -> DownloadHandler;
    pub fn get_drag_handler(&self) -> DragHandler;
    pub fn get_find_handler(&self) -> FindHandler;
    pub fn get_focus_handler(&self) -> FocusHandler;
    pub fn get_frame_handler(&self) -> FrameHandler;
    pub fn get_permission_handler(&self) -> PermissionHandler;
    pub fn get_jsdialog_handler(&self) -> JsdialogHandler;
    pub fn get_keyboard_handler(&self) -> KeyboardHandler;
    pub fn get_life_span_handler(&self) -> LifeSpanHandler;
    pub fn get_load_handler(&self) -> LoadHandler;
    pub fn get_print_handler(&self) -> PrintHandler;
    pub fn get_render_handler(&self) -> RenderHandler;
    pub fn get_request_handler(&self) -> RequestHandler;
    pub fn on_process_message_received(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        source_process: ProcessId,
        message: &mut ProcessMessage,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplClient: Sized {
    fn get_audio_handler(&self) -> AudioHandler {
        Default::default()
    }
    fn get_command_handler(&self) -> CommandHandler {
        Default::default()
    }
    fn get_context_menu_handler(&self) -> ContextMenuHandler {
        Default::default()
    }
    fn get_dialog_handler(&self) -> DialogHandler {
        Default::default()
    }
    fn get_display_handler(&self) -> DisplayHandler {
        Default::default()
    }
    fn get_download_handler(&self) -> DownloadHandler {
        Default::default()
    }
    fn get_drag_handler(&self) -> DragHandler {
        Default::default()
    }
    fn get_find_handler(&self) -> FindHandler {
        Default::default()
    }
    fn get_focus_handler(&self) -> FocusHandler {
        Default::default()
    }
    fn get_frame_handler(&self) -> FrameHandler {
        Default::default()
    }
    fn get_permission_handler(&self) -> PermissionHandler {
        Default::default()
    }
    fn get_jsdialog_handler(&self) -> JsdialogHandler {
        Default::default()
    }
    fn get_keyboard_handler(&self) -> KeyboardHandler {
        Default::default()
    }
    fn get_life_span_handler(&self) -> LifeSpanHandler {
        Default::default()
    }
    fn get_load_handler(&self) -> LoadHandler {
        Default::default()
    }
    fn get_print_handler(&self) -> PrintHandler {
        Default::default()
    }
    fn get_render_handler(&self) -> RenderHandler {
        Default::default()
    }
    fn get_request_handler(&self) -> RequestHandler {
        Default::default()
    }
    fn on_process_message_received(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        source_process: ProcessId,
        message: &mut ProcessMessage,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_client_t {
        let mut object: _cef_client_t = unsafe { std::mem::zeroed() };
        impl_cef_client_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_client_t {
    use super::*;

    pub fn init_methods<I: ImplClient>(object: &mut _cef_client_t) {
        object.get_audio_handler = Some(get_audio_handler::<I>);
        object.get_command_handler = Some(get_command_handler::<I>);
        object.get_context_menu_handler = Some(get_context_menu_handler::<I>);
        object.get_dialog_handler = Some(get_dialog_handler::<I>);
        object.get_display_handler = Some(get_display_handler::<I>);
        object.get_download_handler = Some(get_download_handler::<I>);
        object.get_drag_handler = Some(get_drag_handler::<I>);
        object.get_find_handler = Some(get_find_handler::<I>);
        object.get_focus_handler = Some(get_focus_handler::<I>);
        object.get_frame_handler = Some(get_frame_handler::<I>);
        object.get_permission_handler = Some(get_permission_handler::<I>);
        object.get_jsdialog_handler = Some(get_jsdialog_handler::<I>);
        object.get_keyboard_handler = Some(get_keyboard_handler::<I>);
        object.get_life_span_handler = Some(get_life_span_handler::<I>);
        object.get_load_handler = Some(get_load_handler::<I>);
        object.get_print_handler = Some(get_print_handler::<I>);
        object.get_render_handler = Some(get_render_handler::<I>);
        object.get_request_handler = Some(get_request_handler::<I>);
        object.on_process_message_received = Some(on_process_message_received::<I>);
    }

    extern "C" fn get_audio_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_audio_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_audio_handler().into()
    }

    extern "C" fn get_command_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_command_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_command_handler().into()
    }

    extern "C" fn get_context_menu_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_context_menu_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_context_menu_handler().into()
    }

    extern "C" fn get_dialog_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_dialog_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_dialog_handler().into()
    }

    extern "C" fn get_display_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_display_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_display_handler().into()
    }

    extern "C" fn get_download_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_download_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_download_handler().into()
    }

    extern "C" fn get_drag_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_drag_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_drag_handler().into()
    }

    extern "C" fn get_find_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_find_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_find_handler().into()
    }

    extern "C" fn get_focus_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_focus_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_focus_handler().into()
    }

    extern "C" fn get_frame_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_frame_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame_handler().into()
    }

    extern "C" fn get_permission_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_permission_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_permission_handler().into()
    }

    extern "C" fn get_jsdialog_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_jsdialog_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_jsdialog_handler().into()
    }

    extern "C" fn get_keyboard_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_keyboard_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_keyboard_handler().into()
    }

    extern "C" fn get_life_span_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_life_span_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_life_span_handler().into()
    }

    extern "C" fn get_load_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_load_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_load_handler().into()
    }

    extern "C" fn get_print_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_print_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_print_handler().into()
    }

    extern "C" fn get_render_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_render_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_render_handler().into()
    }

    extern "C" fn get_request_handler<I: ImplClient>(
        self_: *mut _cef_client_t,
    ) -> *mut _cef_request_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_request_handler().into()
    }

    extern "C" fn on_process_message_received<I: ImplClient>(
        self_: *mut _cef_client_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        source_process: cef_process_id_t,
        message: *mut _cef_process_message_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let message = &mut ProcessMessage(unsafe { RefGuard::from_raw_add_ref(message) });
        obj.interface
            .on_process_message_received(browser, frame, source_process, message)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_command_line_t] for more documentation."]
    #[derive(Clone)]
    pub struct CommandLine(_cef_command_line_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_read_only(&self) -> ::std::os::raw::c_int;
    pub fn copy(&self) -> CommandLine;
    pub fn init_from_argv(
        &self,
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    );
    pub fn init_from_string(&self, command_line: &CefString);
    pub fn reset(&self);
    pub fn get_argv(&self, argv: &mut CefStringList);
    pub fn get_command_line_string(&self) -> CefStringUserfree;
    pub fn get_program(&self) -> CefStringUserfree;
    pub fn set_program(&self, program: &CefString);
    pub fn has_switches(&self) -> ::std::os::raw::c_int;
    pub fn has_switch(&self, name: &CefString) -> ::std::os::raw::c_int;
    pub fn get_switch_value(&self, name: &CefString) -> CefStringUserfree;
    pub fn get_switches(&self, switches: &mut CefStringMap);
    pub fn append_switch(&self, name: &CefString);
    pub fn append_switch_with_value(&self, name: &CefString, value: &CefString);
    pub fn has_arguments(&self) -> ::std::os::raw::c_int;
    pub fn get_arguments(&self, arguments: &mut CefStringList);
    pub fn append_argument(&self, argument: &CefString);
    pub fn prepend_wrapper(&self, wrapper: &CefString);
);

pub trait ImplCommandLine: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_read_only(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn copy(&self) -> CommandLine {
        Default::default()
    }
    fn init_from_argv(
        &self,
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) {
    }
    fn init_from_string(&self, command_line: &CefString) {}
    fn reset(&self) {}
    fn get_argv(&self, argv: &mut CefStringList) {}
    fn get_command_line_string(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_program(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_program(&self, program: &CefString) {}
    fn has_switches(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_switch(&self, name: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_switch_value(&self, name: &CefString) -> CefStringUserfree {
        Default::default()
    }
    fn get_switches(&self, switches: &mut CefStringMap) {}
    fn append_switch(&self, name: &CefString) {}
    fn append_switch_with_value(&self, name: &CefString, value: &CefString) {}
    fn has_arguments(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_arguments(&self, arguments: &mut CefStringList) {}
    fn append_argument(&self, argument: &CefString) {}
    fn prepend_wrapper(&self, wrapper: &CefString) {}

    fn into_raw(self) -> *mut _cef_command_line_t {
        let mut object: _cef_command_line_t = unsafe { std::mem::zeroed() };
        impl_cef_command_line_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_command_line_t {
    use super::*;

    pub fn init_methods<I: ImplCommandLine>(object: &mut _cef_command_line_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_read_only = Some(is_read_only::<I>);
        object.copy = Some(copy::<I>);
        object.init_from_argv = Some(init_from_argv::<I>);
        object.init_from_string = Some(init_from_string::<I>);
        object.reset = Some(reset::<I>);
        object.get_argv = Some(get_argv::<I>);
        object.get_command_line_string = Some(get_command_line_string::<I>);
        object.get_program = Some(get_program::<I>);
        object.set_program = Some(set_program::<I>);
        object.has_switches = Some(has_switches::<I>);
        object.has_switch = Some(has_switch::<I>);
        object.get_switch_value = Some(get_switch_value::<I>);
        object.get_switches = Some(get_switches::<I>);
        object.append_switch = Some(append_switch::<I>);
        object.append_switch_with_value = Some(append_switch_with_value::<I>);
        object.has_arguments = Some(has_arguments::<I>);
        object.get_arguments = Some(get_arguments::<I>);
        object.append_argument = Some(append_argument::<I>);
        object.prepend_wrapper = Some(prepend_wrapper::<I>);
    }

    extern "C" fn is_valid<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_read_only<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_read_only().into()
    }

    extern "C" fn copy<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> *mut _cef_command_line_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.copy().into()
    }

    extern "C" fn init_from_argv<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.init_from_argv(argc, argv)
    }

    extern "C" fn init_from_string<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        command_line: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let command_line = &CefString::from(command_line);
        obj.interface.init_from_string(command_line)
    }

    extern "C" fn reset<I: ImplCommandLine>(self_: *mut _cef_command_line_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.reset()
    }

    extern "C" fn get_argv<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        argv: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let argv = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(argv) });
        obj.interface.get_argv(argv)
    }

    extern "C" fn get_command_line_string<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_command_line_string().into()
    }

    extern "C" fn get_program<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_program().into()
    }

    extern "C" fn set_program<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        program: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let program = &CefString::from(program);
        obj.interface.set_program(program)
    }

    extern "C" fn has_switches<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_switches().into()
    }

    extern "C" fn has_switch<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        name: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.has_switch(name).into()
    }

    extern "C" fn get_switch_value<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        name: *const cef_string_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.get_switch_value(name).into()
    }

    extern "C" fn get_switches<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        switches: cef_string_map_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let switches = &mut CefStringMap(unsafe { RefGuard::from_raw_add_ref(switches) });
        obj.interface.get_switches(switches)
    }

    extern "C" fn append_switch<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        name: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        obj.interface.append_switch(name)
    }

    extern "C" fn append_switch_with_value<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        name: *const cef_string_t,
        value: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let value = &CefString::from(value);
        obj.interface.append_switch_with_value(name, value)
    }

    extern "C" fn has_arguments<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_arguments().into()
    }

    extern "C" fn get_arguments<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        arguments: cef_string_list_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let arguments = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(arguments) });
        obj.interface.get_arguments(arguments)
    }

    extern "C" fn append_argument<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        argument: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let argument = &CefString::from(argument);
        obj.interface.append_argument(argument)
    }

    extern "C" fn prepend_wrapper<I: ImplCommandLine>(
        self_: *mut _cef_command_line_t,
        wrapper: *const cef_string_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let wrapper = &CefString::from(wrapper);
        obj.interface.prepend_wrapper(wrapper)
    }
}

wrapper!(
    #[doc = "See [_cef_request_context_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct RequestContextHandler(_cef_request_context_handler_t);

    pub fn on_request_context_initialized(&self, request_context: &mut RequestContext);
    pub fn get_resource_request_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: &CefString,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> ResourceRequestHandler;
);

pub trait ImplRequestContextHandler: Sized {
    fn on_request_context_initialized(&self, request_context: &mut RequestContext) {}
    fn get_resource_request_handler(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        request: &mut Request,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: &CefString,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> ResourceRequestHandler {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_request_context_handler_t {
        let mut object: _cef_request_context_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_request_context_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_request_context_handler_t {
    use super::*;

    pub fn init_methods<I: ImplRequestContextHandler>(object: &mut _cef_request_context_handler_t) {
        object.on_request_context_initialized = Some(on_request_context_initialized::<I>);
        object.get_resource_request_handler = Some(get_resource_request_handler::<I>);
    }

    extern "C" fn on_request_context_initialized<I: ImplRequestContextHandler>(
        self_: *mut _cef_request_context_handler_t,
        request_context: *mut _cef_request_context_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let request_context =
            &mut RequestContext(unsafe { RefGuard::from_raw_add_ref(request_context) });
        obj.interface
            .on_request_context_initialized(request_context)
    }

    extern "C" fn get_resource_request_handler<I: ImplRequestContextHandler>(
        self_: *mut _cef_request_context_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        request: *mut _cef_request_t,
        is_navigation: ::std::os::raw::c_int,
        is_download: ::std::os::raw::c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_resource_request_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        let request_initiator = &CefString::from(request_initiator);
        obj.interface
            .get_resource_request_handler(
                browser,
                frame,
                request,
                is_navigation,
                is_download,
                request_initiator,
                disable_default_handling,
            )
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_browser_process_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct BrowserProcessHandler(_cef_browser_process_handler_t);

    pub fn on_register_custom_preferences(
        &self,
        type_: PreferencesType,
        registrar: &mut PreferenceRegistrar,
    );
    pub fn on_context_initialized(&self);
    pub fn on_before_child_process_launch(&self, command_line: &mut CommandLine);
    pub fn on_already_running_app_relaunch(
        &self,
        command_line: &mut CommandLine,
        current_directory: &CefString,
    ) -> ::std::os::raw::c_int;
    pub fn on_schedule_message_pump_work(&self, delay_ms: i64);
    pub fn get_default_client(&self) -> Client;
    pub fn get_default_request_context_handler(&self) -> RequestContextHandler;
);

pub trait ImplBrowserProcessHandler: Sized {
    fn on_register_custom_preferences(
        &self,
        type_: PreferencesType,
        registrar: &mut PreferenceRegistrar,
    ) {
    }
    fn on_context_initialized(&self) {}
    fn on_before_child_process_launch(&self, command_line: &mut CommandLine) {}
    fn on_already_running_app_relaunch(
        &self,
        command_line: &mut CommandLine,
        current_directory: &CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_schedule_message_pump_work(&self, delay_ms: i64) {}
    fn get_default_client(&self) -> Client {
        Default::default()
    }
    fn get_default_request_context_handler(&self) -> RequestContextHandler {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_browser_process_handler_t {
        let mut object: _cef_browser_process_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_browser_process_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_browser_process_handler_t {
    use super::*;

    pub fn init_methods<I: ImplBrowserProcessHandler>(object: &mut _cef_browser_process_handler_t) {
        object.on_register_custom_preferences = Some(on_register_custom_preferences::<I>);
        object.on_context_initialized = Some(on_context_initialized::<I>);
        object.on_before_child_process_launch = Some(on_before_child_process_launch::<I>);
        object.on_already_running_app_relaunch = Some(on_already_running_app_relaunch::<I>);
        object.on_schedule_message_pump_work = Some(on_schedule_message_pump_work::<I>);
        object.get_default_client = Some(get_default_client::<I>);
        object.get_default_request_context_handler = Some(get_default_request_context_handler::<I>);
    }

    extern "C" fn on_register_custom_preferences<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
        type_: cef_preferences_type_t,
        registrar: *mut _cef_preference_registrar_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let registrar = &mut PreferenceRegistrar(unsafe { RefGuard::from_raw_add_ref(registrar) });
        obj.interface
            .on_register_custom_preferences(type_, registrar)
    }

    extern "C" fn on_context_initialized<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_context_initialized()
    }

    extern "C" fn on_before_child_process_launch<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
        command_line: *mut _cef_command_line_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let command_line = &mut CommandLine(unsafe { RefGuard::from_raw_add_ref(command_line) });
        obj.interface.on_before_child_process_launch(command_line)
    }

    extern "C" fn on_already_running_app_relaunch<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
        command_line: *mut _cef_command_line_t,
        current_directory: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let command_line = &mut CommandLine(unsafe { RefGuard::from_raw_add_ref(command_line) });
        let current_directory = &CefString::from(current_directory);
        obj.interface
            .on_already_running_app_relaunch(command_line, current_directory)
            .into()
    }

    extern "C" fn on_schedule_message_pump_work<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
        delay_ms: i64,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_schedule_message_pump_work(delay_ms)
    }

    extern "C" fn get_default_client<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
    ) -> *mut _cef_client_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_default_client().into()
    }

    extern "C" fn get_default_request_context_handler<I: ImplBrowserProcessHandler>(
        self_: *mut _cef_browser_process_handler_t,
    ) -> *mut _cef_request_context_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_default_request_context_handler().into()
    }
}

wrapper!(
    #[doc = "See [_cef_task_t] for more documentation."]
    #[derive(Clone)]
    pub struct Task(_cef_task_t);

    pub fn execute(&self);
);

pub trait ImplTask: Sized {
    fn execute(&self) {}

    fn into_raw(self) -> *mut _cef_task_t {
        let mut object: _cef_task_t = unsafe { std::mem::zeroed() };
        impl_cef_task_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_task_t {
    use super::*;

    pub fn init_methods<I: ImplTask>(object: &mut _cef_task_t) {
        object.execute = Some(execute::<I>);
    }

    extern "C" fn execute<I: ImplTask>(self_: *mut _cef_task_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.execute()
    }
}

wrapper!(
    #[doc = "See [_cef_task_runner_t] for more documentation."]
    #[derive(Clone)]
    pub struct TaskRunner(_cef_task_runner_t);

    pub fn is_same(&self, that: &mut TaskRunner) -> ::std::os::raw::c_int;
    pub fn belongs_to_current_thread(&self) -> ::std::os::raw::c_int;
    pub fn belongs_to_thread(&self, thread_id: ThreadId) -> ::std::os::raw::c_int;
    pub fn post_task(&self, task: &mut Task) -> ::std::os::raw::c_int;
    pub fn post_delayed_task(&self, task: &mut Task, delay_ms: i64) -> ::std::os::raw::c_int;
);

pub trait ImplTaskRunner: Sized {
    fn is_same(&self, that: &mut TaskRunner) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn belongs_to_current_thread(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn belongs_to_thread(&self, thread_id: ThreadId) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn post_task(&self, task: &mut Task) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn post_delayed_task(&self, task: &mut Task, delay_ms: i64) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_task_runner_t {
        let mut object: _cef_task_runner_t = unsafe { std::mem::zeroed() };
        impl_cef_task_runner_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_task_runner_t {
    use super::*;

    pub fn init_methods<I: ImplTaskRunner>(object: &mut _cef_task_runner_t) {
        object.is_same = Some(is_same::<I>);
        object.belongs_to_current_thread = Some(belongs_to_current_thread::<I>);
        object.belongs_to_thread = Some(belongs_to_thread::<I>);
        object.post_task = Some(post_task::<I>);
        object.post_delayed_task = Some(post_delayed_task::<I>);
    }

    extern "C" fn is_same<I: ImplTaskRunner>(
        self_: *mut _cef_task_runner_t,
        that: *mut _cef_task_runner_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut TaskRunner(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn belongs_to_current_thread<I: ImplTaskRunner>(
        self_: *mut _cef_task_runner_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.belongs_to_current_thread().into()
    }

    extern "C" fn belongs_to_thread<I: ImplTaskRunner>(
        self_: *mut _cef_task_runner_t,
        thread_id: cef_thread_id_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.belongs_to_thread(thread_id).into()
    }

    extern "C" fn post_task<I: ImplTaskRunner>(
        self_: *mut _cef_task_runner_t,
        task: *mut _cef_task_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let task = &mut Task(unsafe { RefGuard::from_raw_add_ref(task) });
        obj.interface.post_task(task).into()
    }

    extern "C" fn post_delayed_task<I: ImplTaskRunner>(
        self_: *mut _cef_task_runner_t,
        task: *mut _cef_task_t,
        delay_ms: i64,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let task = &mut Task(unsafe { RefGuard::from_raw_add_ref(task) });
        obj.interface.post_delayed_task(task, delay_ms).into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8context_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8context(_cef_v8context_t);

    pub fn get_task_runner(&self) -> TaskRunner;
    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn get_browser(&self) -> Browser;
    pub fn get_frame(&self) -> Frame;
    pub fn get_global(&self) -> V8value;
    pub fn enter(&self) -> ::std::os::raw::c_int;
    pub fn exit(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut V8context) -> ::std::os::raw::c_int;
    pub fn eval(
        &self,
        code: &CefString,
        script_url: &CefString,
        start_line: ::std::os::raw::c_int,
        retval: *mut &mut V8value,
        exception: *mut &mut V8exception,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplV8context: Sized {
    fn get_task_runner(&self) -> TaskRunner {
        Default::default()
    }
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_browser(&self) -> Browser {
        Default::default()
    }
    fn get_frame(&self) -> Frame {
        Default::default()
    }
    fn get_global(&self) -> V8value {
        Default::default()
    }
    fn enter(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn exit(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut V8context) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn eval(
        &self,
        code: &CefString,
        script_url: &CefString,
        start_line: ::std::os::raw::c_int,
        retval: *mut &mut V8value,
        exception: *mut &mut V8exception,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8context_t {
        let mut object: _cef_v8context_t = unsafe { std::mem::zeroed() };
        impl_cef_v8context_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8context_t {
    use super::*;

    pub fn init_methods<I: ImplV8context>(object: &mut _cef_v8context_t) {
        object.get_task_runner = Some(get_task_runner::<I>);
        object.is_valid = Some(is_valid::<I>);
        object.get_browser = Some(get_browser::<I>);
        object.get_frame = Some(get_frame::<I>);
        object.get_global = Some(get_global::<I>);
        object.enter = Some(enter::<I>);
        object.exit = Some(exit::<I>);
        object.is_same = Some(is_same::<I>);
        object.eval = Some(eval::<I>);
    }

    extern "C" fn get_task_runner<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
    ) -> *mut _cef_task_runner_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_task_runner().into()
    }

    extern "C" fn is_valid<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn get_browser<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
    ) -> *mut _cef_browser_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser().into()
    }

    extern "C" fn get_frame<I: ImplV8context>(self_: *mut _cef_v8context_t) -> *mut _cef_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame().into()
    }

    extern "C" fn get_global<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
    ) -> *mut _cef_v8value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_global().into()
    }

    extern "C" fn enter<I: ImplV8context>(self_: *mut _cef_v8context_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.enter().into()
    }

    extern "C" fn exit<I: ImplV8context>(self_: *mut _cef_v8context_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.exit().into()
    }

    extern "C" fn is_same<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
        that: *mut _cef_v8context_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut V8context(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn eval<I: ImplV8context>(
        self_: *mut _cef_v8context_t,
        code: *const cef_string_t,
        script_url: *const cef_string_t,
        start_line: ::std::os::raw::c_int,
        retval: *mut *mut _cef_v8value_t,
        exception: *mut *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let code = &CefString::from(code);
        let script_url = &CefString::from(script_url);
        obj.interface
            .eval(code, script_url, start_line, retval, exception)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8handler(_cef_v8handler_t);

    pub fn execute(
        &self,
        name: &CefString,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplV8handler: Sized {
    fn execute(
        &self,
        name: &CefString,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8handler_t {
        let mut object: _cef_v8handler_t = unsafe { std::mem::zeroed() };
        impl_cef_v8handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8handler_t {
    use super::*;

    pub fn init_methods<I: ImplV8handler>(object: &mut _cef_v8handler_t) {
        object.execute = Some(execute::<I>);
    }

    extern "C" fn execute<I: ImplV8handler>(
        self_: *mut _cef_v8handler_t,
        name: *const cef_string_t,
        object: *mut _cef_v8value_t,
        arguments_count: usize,
        arguments: *const *mut _cef_v8value_t,
        retval: *mut *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let exception = &mut CefString::from(exception);
        obj.interface
            .execute(name, object, arguments_count, arguments, retval, exception)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8accessor_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8accessor(_cef_v8accessor_t);

    pub fn get(
        &self,
        name: &CefString,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
    pub fn set(
        &self,
        name: &CefString,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplV8accessor: Sized {
    fn get(
        &self,
        name: &CefString,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set(
        &self,
        name: &CefString,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8accessor_t {
        let mut object: _cef_v8accessor_t = unsafe { std::mem::zeroed() };
        impl_cef_v8accessor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8accessor_t {
    use super::*;

    pub fn init_methods<I: ImplV8accessor>(object: &mut _cef_v8accessor_t) {
        object.get = Some(get::<I>);
        object.set = Some(set::<I>);
    }

    extern "C" fn get<I: ImplV8accessor>(
        self_: *mut _cef_v8accessor_t,
        name: *const cef_string_t,
        object: *mut _cef_v8value_t,
        retval: *mut *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let exception = &mut CefString::from(exception);
        obj.interface.get(name, object, retval, exception).into()
    }

    extern "C" fn set<I: ImplV8accessor>(
        self_: *mut _cef_v8accessor_t,
        name: *const cef_string_t,
        object: *mut _cef_v8value_t,
        value: *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let value = &mut V8value(unsafe { RefGuard::from_raw_add_ref(value) });
        let exception = &mut CefString::from(exception);
        obj.interface.set(name, object, value, exception).into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8interceptor_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8interceptor(_cef_v8interceptor_t);

    pub fn get_byname(
        &self,
        name: &CefString,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
    pub fn get_byindex(
        &self,
        index: ::std::os::raw::c_int,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
    pub fn set_byname(
        &self,
        name: &CefString,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
    pub fn set_byindex(
        &self,
        index: ::std::os::raw::c_int,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplV8interceptor: Sized {
    fn get_byname(
        &self,
        name: &CefString,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_byindex(
        &self,
        index: ::std::os::raw::c_int,
        object: &mut V8value,
        retval: *mut &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_byname(
        &self,
        name: &CefString,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_byindex(
        &self,
        index: ::std::os::raw::c_int,
        object: &mut V8value,
        value: &mut V8value,
        exception: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8interceptor_t {
        let mut object: _cef_v8interceptor_t = unsafe { std::mem::zeroed() };
        impl_cef_v8interceptor_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8interceptor_t {
    use super::*;

    pub fn init_methods<I: ImplV8interceptor>(object: &mut _cef_v8interceptor_t) {
        object.get_byname = Some(get_byname::<I>);
        object.get_byindex = Some(get_byindex::<I>);
        object.set_byname = Some(set_byname::<I>);
        object.set_byindex = Some(set_byindex::<I>);
    }

    extern "C" fn get_byname<I: ImplV8interceptor>(
        self_: *mut _cef_v8interceptor_t,
        name: *const cef_string_t,
        object: *mut _cef_v8value_t,
        retval: *mut *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let exception = &mut CefString::from(exception);
        obj.interface
            .get_byname(name, object, retval, exception)
            .into()
    }

    extern "C" fn get_byindex<I: ImplV8interceptor>(
        self_: *mut _cef_v8interceptor_t,
        index: ::std::os::raw::c_int,
        object: *mut _cef_v8value_t,
        retval: *mut *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let exception = &mut CefString::from(exception);
        obj.interface
            .get_byindex(index, object, retval, exception)
            .into()
    }

    extern "C" fn set_byname<I: ImplV8interceptor>(
        self_: *mut _cef_v8interceptor_t,
        name: *const cef_string_t,
        object: *mut _cef_v8value_t,
        value: *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let name = &CefString::from(name);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let value = &mut V8value(unsafe { RefGuard::from_raw_add_ref(value) });
        let exception = &mut CefString::from(exception);
        obj.interface
            .set_byname(name, object, value, exception)
            .into()
    }

    extern "C" fn set_byindex<I: ImplV8interceptor>(
        self_: *mut _cef_v8interceptor_t,
        index: ::std::os::raw::c_int,
        object: *mut _cef_v8value_t,
        value: *mut _cef_v8value_t,
        exception: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        let value = &mut V8value(unsafe { RefGuard::from_raw_add_ref(value) });
        let exception = &mut CefString::from(exception);
        obj.interface
            .set_byindex(index, object, value, exception)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8exception_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8exception(_cef_v8exception_t);

    pub fn get_message(&self) -> CefStringUserfree;
    pub fn get_source_line(&self) -> CefStringUserfree;
    pub fn get_script_resource_name(&self) -> CefStringUserfree;
    pub fn get_line_number(&self) -> ::std::os::raw::c_int;
    pub fn get_start_position(&self) -> ::std::os::raw::c_int;
    pub fn get_end_position(&self) -> ::std::os::raw::c_int;
    pub fn get_start_column(&self) -> ::std::os::raw::c_int;
    pub fn get_end_column(&self) -> ::std::os::raw::c_int;
);

pub trait ImplV8exception: Sized {
    fn get_message(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_source_line(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_script_resource_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_line_number(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_start_position(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_end_position(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_start_column(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_end_column(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8exception_t {
        let mut object: _cef_v8exception_t = unsafe { std::mem::zeroed() };
        impl_cef_v8exception_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8exception_t {
    use super::*;

    pub fn init_methods<I: ImplV8exception>(object: &mut _cef_v8exception_t) {
        object.get_message = Some(get_message::<I>);
        object.get_source_line = Some(get_source_line::<I>);
        object.get_script_resource_name = Some(get_script_resource_name::<I>);
        object.get_line_number = Some(get_line_number::<I>);
        object.get_start_position = Some(get_start_position::<I>);
        object.get_end_position = Some(get_end_position::<I>);
        object.get_start_column = Some(get_start_column::<I>);
        object.get_end_column = Some(get_end_column::<I>);
    }

    extern "C" fn get_message<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_message().into()
    }

    extern "C" fn get_source_line<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_source_line().into()
    }

    extern "C" fn get_script_resource_name<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_script_resource_name().into()
    }

    extern "C" fn get_line_number<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_line_number().into()
    }

    extern "C" fn get_start_position<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_start_position().into()
    }

    extern "C" fn get_end_position<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_end_position().into()
    }

    extern "C" fn get_start_column<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_start_column().into()
    }

    extern "C" fn get_end_column<I: ImplV8exception>(
        self_: *mut _cef_v8exception_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_end_column().into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8array_buffer_release_callback_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8arrayBufferReleaseCallback(_cef_v8array_buffer_release_callback_t);

    pub fn release_buffer(&self, buffer: *mut ::std::os::raw::c_void);
);

pub trait ImplV8arrayBufferReleaseCallback: Sized {
    fn release_buffer(&self, buffer: *mut ::std::os::raw::c_void) {}

    fn into_raw(self) -> *mut _cef_v8array_buffer_release_callback_t {
        let mut object: _cef_v8array_buffer_release_callback_t = unsafe { std::mem::zeroed() };
        impl_cef_v8array_buffer_release_callback_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8array_buffer_release_callback_t {
    use super::*;

    pub fn init_methods<I: ImplV8arrayBufferReleaseCallback>(
        object: &mut _cef_v8array_buffer_release_callback_t,
    ) {
        object.release_buffer = Some(release_buffer::<I>);
    }

    extern "C" fn release_buffer<I: ImplV8arrayBufferReleaseCallback>(
        self_: *mut _cef_v8array_buffer_release_callback_t,
        buffer: *mut ::std::os::raw::c_void,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.release_buffer(buffer)
    }
}

wrapper!(
    #[doc = "See [_cef_v8value_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8value(_cef_v8value_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_undefined(&self) -> ::std::os::raw::c_int;
    pub fn is_null(&self) -> ::std::os::raw::c_int;
    pub fn is_bool(&self) -> ::std::os::raw::c_int;
    pub fn is_int(&self) -> ::std::os::raw::c_int;
    pub fn is_uint(&self) -> ::std::os::raw::c_int;
    pub fn is_double(&self) -> ::std::os::raw::c_int;
    pub fn is_date(&self) -> ::std::os::raw::c_int;
    pub fn is_string(&self) -> ::std::os::raw::c_int;
    pub fn is_object(&self) -> ::std::os::raw::c_int;
    pub fn is_array(&self) -> ::std::os::raw::c_int;
    pub fn is_array_buffer(&self) -> ::std::os::raw::c_int;
    pub fn is_function(&self) -> ::std::os::raw::c_int;
    pub fn is_promise(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut V8value) -> ::std::os::raw::c_int;
    pub fn get_bool_value(&self) -> ::std::os::raw::c_int;
    pub fn get_int_value(&self) -> i32;
    pub fn get_uint_value(&self) -> u32;
    pub fn get_double_value(&self) -> f64;
    pub fn get_date_value(&self) -> Basetime;
    pub fn get_string_value(&self) -> CefStringUserfree;
    pub fn is_user_created(&self) -> ::std::os::raw::c_int;
    pub fn has_exception(&self) -> ::std::os::raw::c_int;
    pub fn get_exception(&self) -> V8exception;
    pub fn clear_exception(&self) -> ::std::os::raw::c_int;
    pub fn will_rethrow_exceptions(&self) -> ::std::os::raw::c_int;
    pub fn set_rethrow_exceptions(&self, rethrow: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn has_value_bykey(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn has_value_byindex(&self, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn delete_value_bykey(&self, key: &CefString) -> ::std::os::raw::c_int;
    pub fn delete_value_byindex(&self, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn get_value_bykey(&self, key: &CefString) -> V8value;
    pub fn get_value_byindex(&self, index: ::std::os::raw::c_int) -> V8value;
    pub fn set_value_bykey(
        &self,
        key: &CefString,
        value: &mut V8value,
        attribute: V8Propertyattribute,
    ) -> ::std::os::raw::c_int;
    pub fn set_value_byindex(
        &self,
        index: ::std::os::raw::c_int,
        value: &mut V8value,
    ) -> ::std::os::raw::c_int;
    pub fn set_value_byaccessor(
        &self,
        key: &CefString,
        attribute: V8Propertyattribute,
    ) -> ::std::os::raw::c_int;
    pub fn get_keys(&self, keys: &mut CefStringList) -> ::std::os::raw::c_int;
    pub fn set_user_data(&self, user_data: &mut BaseRefCounted) -> ::std::os::raw::c_int;
    pub fn get_user_data(&self) -> BaseRefCounted;
    pub fn get_externally_allocated_memory(&self) -> ::std::os::raw::c_int;
    pub fn adjust_externally_allocated_memory(
        &self,
        change_in_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_array_length(&self) -> ::std::os::raw::c_int;
    pub fn get_array_buffer_release_callback(&self) -> V8arrayBufferReleaseCallback;
    pub fn neuter_array_buffer(&self) -> ::std::os::raw::c_int;
    pub fn get_array_buffer_byte_length(&self) -> usize;
    pub fn get_array_buffer_data(&self) -> *mut ::std::os::raw::c_void;
    pub fn get_function_name(&self) -> CefStringUserfree;
    pub fn get_function_handler(&self) -> V8handler;
    pub fn execute_function(
        &self,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
    ) -> V8value;
    pub fn execute_function_with_context(
        &self,
        context: &mut V8context,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
    ) -> V8value;
    pub fn resolve_promise(&self, arg: &mut V8value) -> ::std::os::raw::c_int;
    pub fn reject_promise(&self, error_msg: &CefString) -> ::std::os::raw::c_int;
);

pub trait ImplV8value: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_undefined(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_null(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_bool(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_int(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_uint(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_double(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_date(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_string(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_object(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_array(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_array_buffer(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_function(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_promise(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut V8value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_bool_value(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_int_value(&self) -> i32 {
        Default::default()
    }
    fn get_uint_value(&self) -> u32 {
        Default::default()
    }
    fn get_double_value(&self) -> f64 {
        Default::default()
    }
    fn get_date_value(&self) -> Basetime {
        Default::default()
    }
    fn get_string_value(&self) -> CefStringUserfree {
        Default::default()
    }
    fn is_user_created(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_exception(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_exception(&self) -> V8exception {
        Default::default()
    }
    fn clear_exception(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn will_rethrow_exceptions(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_rethrow_exceptions(&self, rethrow: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_value_bykey(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_value_byindex(&self, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn delete_value_bykey(&self, key: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn delete_value_byindex(&self, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_value_bykey(&self, key: &CefString) -> V8value {
        Default::default()
    }
    fn get_value_byindex(&self, index: ::std::os::raw::c_int) -> V8value {
        Default::default()
    }
    fn set_value_bykey(
        &self,
        key: &CefString,
        value: &mut V8value,
        attribute: V8Propertyattribute,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_value_byindex(
        &self,
        index: ::std::os::raw::c_int,
        value: &mut V8value,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_value_byaccessor(
        &self,
        key: &CefString,
        attribute: V8Propertyattribute,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_keys(&self, keys: &mut CefStringList) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_user_data(&self, user_data: &mut BaseRefCounted) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_user_data(&self) -> BaseRefCounted {
        Default::default()
    }
    fn get_externally_allocated_memory(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn adjust_externally_allocated_memory(
        &self,
        change_in_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_array_length(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_array_buffer_release_callback(&self) -> V8arrayBufferReleaseCallback {
        Default::default()
    }
    fn neuter_array_buffer(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_array_buffer_byte_length(&self) -> usize {
        Default::default()
    }
    fn get_array_buffer_data(&self) -> *mut ::std::os::raw::c_void {
        Default::default()
    }
    fn get_function_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_function_handler(&self) -> V8handler {
        Default::default()
    }
    fn execute_function(
        &self,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
    ) -> V8value {
        Default::default()
    }
    fn execute_function_with_context(
        &self,
        context: &mut V8context,
        object: &mut V8value,
        arguments_count: usize,
        arguments: *const &mut V8value,
    ) -> V8value {
        Default::default()
    }
    fn resolve_promise(&self, arg: &mut V8value) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn reject_promise(&self, error_msg: &CefString) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8value_t {
        let mut object: _cef_v8value_t = unsafe { std::mem::zeroed() };
        impl_cef_v8value_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8value_t {
    use super::*;

    pub fn init_methods<I: ImplV8value>(object: &mut _cef_v8value_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_undefined = Some(is_undefined::<I>);
        object.is_null = Some(is_null::<I>);
        object.is_bool = Some(is_bool::<I>);
        object.is_int = Some(is_int::<I>);
        object.is_uint = Some(is_uint::<I>);
        object.is_double = Some(is_double::<I>);
        object.is_date = Some(is_date::<I>);
        object.is_string = Some(is_string::<I>);
        object.is_object = Some(is_object::<I>);
        object.is_array = Some(is_array::<I>);
        object.is_array_buffer = Some(is_array_buffer::<I>);
        object.is_function = Some(is_function::<I>);
        object.is_promise = Some(is_promise::<I>);
        object.is_same = Some(is_same::<I>);
        object.get_bool_value = Some(get_bool_value::<I>);
        object.get_int_value = Some(get_int_value::<I>);
        object.get_uint_value = Some(get_uint_value::<I>);
        object.get_double_value = Some(get_double_value::<I>);
        object.get_date_value = Some(get_date_value::<I>);
        object.get_string_value = Some(get_string_value::<I>);
        object.is_user_created = Some(is_user_created::<I>);
        object.has_exception = Some(has_exception::<I>);
        object.get_exception = Some(get_exception::<I>);
        object.clear_exception = Some(clear_exception::<I>);
        object.will_rethrow_exceptions = Some(will_rethrow_exceptions::<I>);
        object.set_rethrow_exceptions = Some(set_rethrow_exceptions::<I>);
        object.has_value_bykey = Some(has_value_bykey::<I>);
        object.has_value_byindex = Some(has_value_byindex::<I>);
        object.delete_value_bykey = Some(delete_value_bykey::<I>);
        object.delete_value_byindex = Some(delete_value_byindex::<I>);
        object.get_value_bykey = Some(get_value_bykey::<I>);
        object.get_value_byindex = Some(get_value_byindex::<I>);
        object.set_value_bykey = Some(set_value_bykey::<I>);
        object.set_value_byindex = Some(set_value_byindex::<I>);
        object.set_value_byaccessor = Some(set_value_byaccessor::<I>);
        object.get_keys = Some(get_keys::<I>);
        object.set_user_data = Some(set_user_data::<I>);
        object.get_user_data = Some(get_user_data::<I>);
        object.get_externally_allocated_memory = Some(get_externally_allocated_memory::<I>);
        object.adjust_externally_allocated_memory = Some(adjust_externally_allocated_memory::<I>);
        object.get_array_length = Some(get_array_length::<I>);
        object.get_array_buffer_release_callback = Some(get_array_buffer_release_callback::<I>);
        object.neuter_array_buffer = Some(neuter_array_buffer::<I>);
        object.get_array_buffer_byte_length = Some(get_array_buffer_byte_length::<I>);
        object.get_array_buffer_data = Some(get_array_buffer_data::<I>);
        object.get_function_name = Some(get_function_name::<I>);
        object.get_function_handler = Some(get_function_handler::<I>);
        object.execute_function = Some(execute_function::<I>);
        object.execute_function_with_context = Some(execute_function_with_context::<I>);
        object.resolve_promise = Some(resolve_promise::<I>);
        object.reject_promise = Some(reject_promise::<I>);
    }

    extern "C" fn is_valid<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_undefined<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_undefined().into()
    }

    extern "C" fn is_null<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_null().into()
    }

    extern "C" fn is_bool<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_bool().into()
    }

    extern "C" fn is_int<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_int().into()
    }

    extern "C" fn is_uint<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_uint().into()
    }

    extern "C" fn is_double<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_double().into()
    }

    extern "C" fn is_date<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_date().into()
    }

    extern "C" fn is_string<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_string().into()
    }

    extern "C" fn is_object<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_object().into()
    }

    extern "C" fn is_array<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_array().into()
    }

    extern "C" fn is_array_buffer<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_array_buffer().into()
    }

    extern "C" fn is_function<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_function().into()
    }

    extern "C" fn is_promise<I: ImplV8value>(self_: *mut _cef_v8value_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_promise().into()
    }

    extern "C" fn is_same<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        that: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut V8value(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn get_bool_value<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bool_value().into()
    }

    extern "C" fn get_int_value<I: ImplV8value>(self_: *mut _cef_v8value_t) -> i32 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_int_value().into()
    }

    extern "C" fn get_uint_value<I: ImplV8value>(self_: *mut _cef_v8value_t) -> u32 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_uint_value().into()
    }

    extern "C" fn get_double_value<I: ImplV8value>(self_: *mut _cef_v8value_t) -> f64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_double_value().into()
    }

    extern "C" fn get_date_value<I: ImplV8value>(self_: *mut _cef_v8value_t) -> cef_basetime_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_date_value().into()
    }

    extern "C" fn get_string_value<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_string_value().into()
    }

    extern "C" fn is_user_created<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_user_created().into()
    }

    extern "C" fn has_exception<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_exception().into()
    }

    extern "C" fn get_exception<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> *mut _cef_v8exception_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_exception().into()
    }

    extern "C" fn clear_exception<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.clear_exception().into()
    }

    extern "C" fn will_rethrow_exceptions<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.will_rethrow_exceptions().into()
    }

    extern "C" fn set_rethrow_exceptions<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        rethrow: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_rethrow_exceptions(rethrow).into()
    }

    extern "C" fn has_value_bykey<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.has_value_bykey(key).into()
    }

    extern "C" fn has_value_byindex<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_value_byindex(index).into()
    }

    extern "C" fn delete_value_bykey<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        key: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.delete_value_bykey(key).into()
    }

    extern "C" fn delete_value_byindex<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.delete_value_byindex(index).into()
    }

    extern "C" fn get_value_bykey<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        key: *const cef_string_t,
    ) -> *mut _cef_v8value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.get_value_bykey(key).into()
    }

    extern "C" fn get_value_byindex<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        index: ::std::os::raw::c_int,
    ) -> *mut _cef_v8value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_value_byindex(index).into()
    }

    extern "C" fn set_value_bykey<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        key: *const cef_string_t,
        value: *mut _cef_v8value_t,
        attribute: cef_v8_propertyattribute_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        let value = &mut V8value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_value_bykey(key, value, attribute).into()
    }

    extern "C" fn set_value_byindex<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        index: ::std::os::raw::c_int,
        value: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let value = &mut V8value(unsafe { RefGuard::from_raw_add_ref(value) });
        obj.interface.set_value_byindex(index, value).into()
    }

    extern "C" fn set_value_byaccessor<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        key: *const cef_string_t,
        attribute: cef_v8_propertyattribute_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let key = &CefString::from(key);
        obj.interface.set_value_byaccessor(key, attribute).into()
    }

    extern "C" fn get_keys<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        keys: cef_string_list_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let keys = &mut CefStringList(unsafe { RefGuard::from_raw_add_ref(keys) });
        obj.interface.get_keys(keys).into()
    }

    extern "C" fn set_user_data<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        user_data: *mut _cef_base_ref_counted_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let user_data = &mut BaseRefCounted(unsafe { RefGuard::from_raw_add_ref(user_data) });
        obj.interface.set_user_data(user_data).into()
    }

    extern "C" fn get_user_data<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> *mut _cef_base_ref_counted_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_user_data().into()
    }

    extern "C" fn get_externally_allocated_memory<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_externally_allocated_memory().into()
    }

    extern "C" fn adjust_externally_allocated_memory<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        change_in_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .adjust_externally_allocated_memory(change_in_bytes)
            .into()
    }

    extern "C" fn get_array_length<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_array_length().into()
    }

    extern "C" fn get_array_buffer_release_callback<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> *mut _cef_v8array_buffer_release_callback_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_array_buffer_release_callback().into()
    }

    extern "C" fn neuter_array_buffer<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.neuter_array_buffer().into()
    }

    extern "C" fn get_array_buffer_byte_length<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_array_buffer_byte_length().into()
    }

    extern "C" fn get_array_buffer_data<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> *mut ::std::os::raw::c_void {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_array_buffer_data().into()
    }

    extern "C" fn get_function_name<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_function_name().into()
    }

    extern "C" fn get_function_handler<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
    ) -> *mut _cef_v8handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_function_handler().into()
    }

    extern "C" fn execute_function<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        object: *mut _cef_v8value_t,
        arguments_count: usize,
        arguments: *const *mut _cef_v8value_t,
    ) -> *mut _cef_v8value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        obj.interface
            .execute_function(object, arguments_count, arguments)
            .into()
    }

    extern "C" fn execute_function_with_context<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        context: *mut _cef_v8context_t,
        object: *mut _cef_v8value_t,
        arguments_count: usize,
        arguments: *const *mut _cef_v8value_t,
    ) -> *mut _cef_v8value_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let context = &mut V8context(unsafe { RefGuard::from_raw_add_ref(context) });
        let object = &mut V8value(unsafe { RefGuard::from_raw_add_ref(object) });
        obj.interface
            .execute_function_with_context(context, object, arguments_count, arguments)
            .into()
    }

    extern "C" fn resolve_promise<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        arg: *mut _cef_v8value_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let arg = &mut V8value(unsafe { RefGuard::from_raw_add_ref(arg) });
        obj.interface.resolve_promise(arg).into()
    }

    extern "C" fn reject_promise<I: ImplV8value>(
        self_: *mut _cef_v8value_t,
        error_msg: *const cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let error_msg = &CefString::from(error_msg);
        obj.interface.reject_promise(error_msg).into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8stack_trace_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8stackTrace(_cef_v8stack_trace_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn get_frame_count(&self) -> ::std::os::raw::c_int;
    pub fn get_frame(&self, index: ::std::os::raw::c_int) -> V8stackFrame;
);

pub trait ImplV8stackTrace: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_frame_count(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_frame(&self, index: ::std::os::raw::c_int) -> V8stackFrame {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8stack_trace_t {
        let mut object: _cef_v8stack_trace_t = unsafe { std::mem::zeroed() };
        impl_cef_v8stack_trace_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8stack_trace_t {
    use super::*;

    pub fn init_methods<I: ImplV8stackTrace>(object: &mut _cef_v8stack_trace_t) {
        object.is_valid = Some(is_valid::<I>);
        object.get_frame_count = Some(get_frame_count::<I>);
        object.get_frame = Some(get_frame::<I>);
    }

    extern "C" fn is_valid<I: ImplV8stackTrace>(
        self_: *mut _cef_v8stack_trace_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn get_frame_count<I: ImplV8stackTrace>(
        self_: *mut _cef_v8stack_trace_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame_count().into()
    }

    extern "C" fn get_frame<I: ImplV8stackTrace>(
        self_: *mut _cef_v8stack_trace_t,
        index: ::std::os::raw::c_int,
    ) -> *mut _cef_v8stack_frame_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_frame(index).into()
    }
}

wrapper!(
    #[doc = "See [_cef_v8stack_frame_t] for more documentation."]
    #[derive(Clone)]
    pub struct V8stackFrame(_cef_v8stack_frame_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn get_script_name(&self) -> CefStringUserfree;
    pub fn get_script_name_or_source_url(&self) -> CefStringUserfree;
    pub fn get_function_name(&self) -> CefStringUserfree;
    pub fn get_line_number(&self) -> ::std::os::raw::c_int;
    pub fn get_column(&self) -> ::std::os::raw::c_int;
    pub fn is_eval(&self) -> ::std::os::raw::c_int;
    pub fn is_constructor(&self) -> ::std::os::raw::c_int;
);

pub trait ImplV8stackFrame: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_script_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_script_name_or_source_url(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_function_name(&self) -> CefStringUserfree {
        Default::default()
    }
    fn get_line_number(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_column(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_eval(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_constructor(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_v8stack_frame_t {
        let mut object: _cef_v8stack_frame_t = unsafe { std::mem::zeroed() };
        impl_cef_v8stack_frame_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_v8stack_frame_t {
    use super::*;

    pub fn init_methods<I: ImplV8stackFrame>(object: &mut _cef_v8stack_frame_t) {
        object.is_valid = Some(is_valid::<I>);
        object.get_script_name = Some(get_script_name::<I>);
        object.get_script_name_or_source_url = Some(get_script_name_or_source_url::<I>);
        object.get_function_name = Some(get_function_name::<I>);
        object.get_line_number = Some(get_line_number::<I>);
        object.get_column = Some(get_column::<I>);
        object.is_eval = Some(is_eval::<I>);
        object.is_constructor = Some(is_constructor::<I>);
    }

    extern "C" fn is_valid<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn get_script_name<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_script_name().into()
    }

    extern "C" fn get_script_name_or_source_url<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_script_name_or_source_url().into()
    }

    extern "C" fn get_function_name<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_function_name().into()
    }

    extern "C" fn get_line_number<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_line_number().into()
    }

    extern "C" fn get_column<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_column().into()
    }

    extern "C" fn is_eval<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_eval().into()
    }

    extern "C" fn is_constructor<I: ImplV8stackFrame>(
        self_: *mut _cef_v8stack_frame_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_constructor().into()
    }
}

wrapper!(
    #[doc = "See [_cef_render_process_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct RenderProcessHandler(_cef_render_process_handler_t);

    pub fn on_web_kit_initialized(&self);
    pub fn on_browser_created(&self, browser: &mut Browser, extra_info: &mut DictionaryValue);
    pub fn on_browser_destroyed(&self, browser: &mut Browser);
    pub fn get_load_handler(&self) -> LoadHandler;
    pub fn on_context_created(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
    );
    pub fn on_context_released(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
    );
    pub fn on_uncaught_exception(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
        exception: &mut V8exception,
        stack_trace: &mut V8stackTrace,
    );
    pub fn on_focused_node_changed(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        node: &mut Domnode,
    );
    pub fn on_process_message_received(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        source_process: ProcessId,
        message: &mut ProcessMessage,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplRenderProcessHandler: Sized {
    fn on_web_kit_initialized(&self) {}
    fn on_browser_created(&self, browser: &mut Browser, extra_info: &mut DictionaryValue) {}
    fn on_browser_destroyed(&self, browser: &mut Browser) {}
    fn get_load_handler(&self) -> LoadHandler {
        Default::default()
    }
    fn on_context_created(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
    ) {
    }
    fn on_context_released(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
    ) {
    }
    fn on_uncaught_exception(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        context: &mut V8context,
        exception: &mut V8exception,
        stack_trace: &mut V8stackTrace,
    ) {
    }
    fn on_focused_node_changed(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        node: &mut Domnode,
    ) {
    }
    fn on_process_message_received(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        source_process: ProcessId,
        message: &mut ProcessMessage,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_render_process_handler_t {
        let mut object: _cef_render_process_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_render_process_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_render_process_handler_t {
    use super::*;

    pub fn init_methods<I: ImplRenderProcessHandler>(object: &mut _cef_render_process_handler_t) {
        object.on_web_kit_initialized = Some(on_web_kit_initialized::<I>);
        object.on_browser_created = Some(on_browser_created::<I>);
        object.on_browser_destroyed = Some(on_browser_destroyed::<I>);
        object.get_load_handler = Some(get_load_handler::<I>);
        object.on_context_created = Some(on_context_created::<I>);
        object.on_context_released = Some(on_context_released::<I>);
        object.on_uncaught_exception = Some(on_uncaught_exception::<I>);
        object.on_focused_node_changed = Some(on_focused_node_changed::<I>);
        object.on_process_message_received = Some(on_process_message_received::<I>);
    }

    extern "C" fn on_web_kit_initialized<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.on_web_kit_initialized()
    }

    extern "C" fn on_browser_created<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        extra_info: *mut _cef_dictionary_value_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let extra_info = &mut DictionaryValue(unsafe { RefGuard::from_raw_add_ref(extra_info) });
        obj.interface.on_browser_created(browser, extra_info)
    }

    extern "C" fn on_browser_destroyed<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_browser_destroyed(browser)
    }

    extern "C" fn get_load_handler<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
    ) -> *mut _cef_load_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_load_handler().into()
    }

    extern "C" fn on_context_created<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        context: *mut _cef_v8context_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let context = &mut V8context(unsafe { RefGuard::from_raw_add_ref(context) });
        obj.interface.on_context_created(browser, frame, context)
    }

    extern "C" fn on_context_released<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        context: *mut _cef_v8context_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let context = &mut V8context(unsafe { RefGuard::from_raw_add_ref(context) });
        obj.interface.on_context_released(browser, frame, context)
    }

    extern "C" fn on_uncaught_exception<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        context: *mut _cef_v8context_t,
        exception: *mut _cef_v8exception_t,
        stack_trace: *mut _cef_v8stack_trace_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let context = &mut V8context(unsafe { RefGuard::from_raw_add_ref(context) });
        let exception = &mut V8exception(unsafe { RefGuard::from_raw_add_ref(exception) });
        let stack_trace = &mut V8stackTrace(unsafe { RefGuard::from_raw_add_ref(stack_trace) });
        obj.interface
            .on_uncaught_exception(browser, frame, context, exception, stack_trace)
    }

    extern "C" fn on_focused_node_changed<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        node: *mut _cef_domnode_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let node = &mut Domnode(unsafe { RefGuard::from_raw_add_ref(node) });
        obj.interface.on_focused_node_changed(browser, frame, node)
    }

    extern "C" fn on_process_message_received<I: ImplRenderProcessHandler>(
        self_: *mut _cef_render_process_handler_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        source_process: cef_process_id_t,
        message: *mut _cef_process_message_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let message = &mut ProcessMessage(unsafe { RefGuard::from_raw_add_ref(message) });
        obj.interface
            .on_process_message_received(browser, frame, source_process, message)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_resource_bundle_handler_t] for more documentation."]
    #[derive(Clone)]
    pub struct ResourceBundleHandler(_cef_resource_bundle_handler_t);

    pub fn get_localized_string(
        &self,
        string_id: ::std::os::raw::c_int,
        string: &mut CefString,
    ) -> ::std::os::raw::c_int;
    pub fn get_data_resource(
        &self,
        resource_id: ::std::os::raw::c_int,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int;
    pub fn get_data_resource_for_scale(
        &self,
        resource_id: ::std::os::raw::c_int,
        scale_factor: ScaleFactor,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplResourceBundleHandler: Sized {
    fn get_localized_string(
        &self,
        string_id: ::std::os::raw::c_int,
        string: &mut CefString,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_data_resource(
        &self,
        resource_id: ::std::os::raw::c_int,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_data_resource_for_scale(
        &self,
        resource_id: ::std::os::raw::c_int,
        scale_factor: ScaleFactor,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_resource_bundle_handler_t {
        let mut object: _cef_resource_bundle_handler_t = unsafe { std::mem::zeroed() };
        impl_cef_resource_bundle_handler_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_resource_bundle_handler_t {
    use super::*;

    pub fn init_methods<I: ImplResourceBundleHandler>(object: &mut _cef_resource_bundle_handler_t) {
        object.get_localized_string = Some(get_localized_string::<I>);
        object.get_data_resource = Some(get_data_resource::<I>);
        object.get_data_resource_for_scale = Some(get_data_resource_for_scale::<I>);
    }

    extern "C" fn get_localized_string<I: ImplResourceBundleHandler>(
        self_: *mut _cef_resource_bundle_handler_t,
        string_id: ::std::os::raw::c_int,
        string: *mut cef_string_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let string = &mut CefString::from(string);
        obj.interface.get_localized_string(string_id, string).into()
    }

    extern "C" fn get_data_resource<I: ImplResourceBundleHandler>(
        self_: *mut _cef_resource_bundle_handler_t,
        resource_id: ::std::os::raw::c_int,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_data_resource(resource_id, data, data_size)
            .into()
    }

    extern "C" fn get_data_resource_for_scale<I: ImplResourceBundleHandler>(
        self_: *mut _cef_resource_bundle_handler_t,
        resource_id: ::std::os::raw::c_int,
        scale_factor: cef_scale_factor_t,
        data: *mut *mut ::std::os::raw::c_void,
        data_size: *mut usize,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .get_data_resource_for_scale(resource_id, scale_factor, data, data_size)
            .into()
    }
}

/// See [_cef_scheme_registrar_t] for more documentation.
pub struct SchemeRegistrar(_cef_scheme_registrar_t);

impl From<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn from(value: _cef_scheme_registrar_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn into(self) -> _cef_scheme_registrar_t {
        self.0
    }
}

impl AsRef<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn as_ref(&self) -> &_cef_scheme_registrar_t {
        &self.0
    }
}

impl AsMut<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn as_mut(&mut self) -> &mut _cef_scheme_registrar_t {
        &mut self.0
    }
}

impl Default for SchemeRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_scheme_handler_factory_t] for more documentation."]
    #[derive(Clone)]
    pub struct SchemeHandlerFactory(_cef_scheme_handler_factory_t);

    pub fn create(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        scheme_name: &CefString,
        request: &mut Request,
    ) -> ResourceHandler;
);

pub trait ImplSchemeHandlerFactory: Sized {
    fn create(
        &self,
        browser: &mut Browser,
        frame: &mut Frame,
        scheme_name: &CefString,
        request: &mut Request,
    ) -> ResourceHandler {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_scheme_handler_factory_t {
        let mut object: _cef_scheme_handler_factory_t = unsafe { std::mem::zeroed() };
        impl_cef_scheme_handler_factory_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_scheme_handler_factory_t {
    use super::*;

    pub fn init_methods<I: ImplSchemeHandlerFactory>(object: &mut _cef_scheme_handler_factory_t) {
        object.create = Some(create::<I>);
    }

    extern "C" fn create<I: ImplSchemeHandlerFactory>(
        self_: *mut _cef_scheme_handler_factory_t,
        browser: *mut _cef_browser_t,
        frame: *mut _cef_frame_t,
        scheme_name: *const cef_string_t,
        request: *mut _cef_request_t,
    ) -> *mut _cef_resource_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        let frame = &mut Frame(unsafe { RefGuard::from_raw_add_ref(frame) });
        let scheme_name = &CefString::from(scheme_name);
        let request = &mut Request(unsafe { RefGuard::from_raw_add_ref(request) });
        obj.interface
            .create(browser, frame, scheme_name, request)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_app_t] for more documentation."]
    #[derive(Clone)]
    pub struct App(_cef_app_t);

    pub fn on_before_command_line_processing(
        &self,
        process_type: &CefString,
        command_line: &mut CommandLine,
    );
    pub fn on_register_custom_schemes(&self, registrar: &mut SchemeRegistrar);
    pub fn get_resource_bundle_handler(&self) -> ResourceBundleHandler;
    pub fn get_browser_process_handler(&self) -> BrowserProcessHandler;
    pub fn get_render_process_handler(&self) -> RenderProcessHandler;
);

pub trait ImplApp: Sized {
    fn on_before_command_line_processing(
        &self,
        process_type: &CefString,
        command_line: &mut CommandLine,
    ) {
    }
    fn on_register_custom_schemes(&self, registrar: &mut SchemeRegistrar) {}
    fn get_resource_bundle_handler(&self) -> ResourceBundleHandler {
        Default::default()
    }
    fn get_browser_process_handler(&self) -> BrowserProcessHandler {
        Default::default()
    }
    fn get_render_process_handler(&self) -> RenderProcessHandler {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_app_t {
        let mut object: _cef_app_t = unsafe { std::mem::zeroed() };
        impl_cef_app_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_app_t {
    use super::*;

    pub fn init_methods<I: ImplApp>(object: &mut _cef_app_t) {
        object.on_before_command_line_processing = Some(on_before_command_line_processing::<I>);
        object.on_register_custom_schemes = Some(on_register_custom_schemes::<I>);
        object.get_resource_bundle_handler = Some(get_resource_bundle_handler::<I>);
        object.get_browser_process_handler = Some(get_browser_process_handler::<I>);
        object.get_render_process_handler = Some(get_render_process_handler::<I>);
    }

    extern "C" fn on_before_command_line_processing<I: ImplApp>(
        self_: *mut _cef_app_t,
        process_type: *const cef_string_t,
        command_line: *mut _cef_command_line_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let process_type = &CefString::from(process_type);
        let command_line = &mut CommandLine(unsafe { RefGuard::from_raw_add_ref(command_line) });
        obj.interface
            .on_before_command_line_processing(process_type, command_line)
    }

    extern "C" fn on_register_custom_schemes<I: ImplApp>(
        self_: *mut _cef_app_t,
        registrar: *mut _cef_scheme_registrar_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let registrar = &mut SchemeRegistrar(unsafe { RefGuard::from_raw_add_ref(registrar) });
        obj.interface.on_register_custom_schemes(registrar)
    }

    extern "C" fn get_resource_bundle_handler<I: ImplApp>(
        self_: *mut _cef_app_t,
    ) -> *mut _cef_resource_bundle_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_resource_bundle_handler().into()
    }

    extern "C" fn get_browser_process_handler<I: ImplApp>(
        self_: *mut _cef_app_t,
    ) -> *mut _cef_browser_process_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser_process_handler().into()
    }

    extern "C" fn get_render_process_handler<I: ImplApp>(
        self_: *mut _cef_app_t,
    ) -> *mut _cef_render_process_handler_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_render_process_handler().into()
    }
}

wrapper!(
    #[doc = "See [_cef_view_delegate_t] for more documentation."]
    #[derive(Clone)]
    pub struct ViewDelegate(_cef_view_delegate_t);

    pub fn get_preferred_size(&self, view: &mut View) -> Size;
    pub fn get_minimum_size(&self, view: &mut View) -> Size;
    pub fn get_maximum_size(&self, view: &mut View) -> Size;
    pub fn get_height_for_width(
        &self,
        view: &mut View,
        width: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_parent_view_changed(
        &self,
        view: &mut View,
        added: ::std::os::raw::c_int,
        parent: &mut View,
    );
    pub fn on_child_view_changed(
        &self,
        view: &mut View,
        added: ::std::os::raw::c_int,
        child: &mut View,
    );
    pub fn on_window_changed(&self, view: &mut View, added: ::std::os::raw::c_int);
    pub fn on_layout_changed(&self, view: &mut View, new_bounds: &Rect);
    pub fn on_focus(&self, view: &mut View);
    pub fn on_blur(&self, view: &mut View);
    pub fn on_theme_changed(&self, view: &mut View);
);

pub trait ImplViewDelegate: Sized {
    fn get_preferred_size(&self, view: &mut View) -> Size {
        Default::default()
    }
    fn get_minimum_size(&self, view: &mut View) -> Size {
        Default::default()
    }
    fn get_maximum_size(&self, view: &mut View) -> Size {
        Default::default()
    }
    fn get_height_for_width(
        &self,
        view: &mut View,
        width: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_parent_view_changed(
        &self,
        view: &mut View,
        added: ::std::os::raw::c_int,
        parent: &mut View,
    ) {
    }
    fn on_child_view_changed(
        &self,
        view: &mut View,
        added: ::std::os::raw::c_int,
        child: &mut View,
    ) {
    }
    fn on_window_changed(&self, view: &mut View, added: ::std::os::raw::c_int) {}
    fn on_layout_changed(&self, view: &mut View, new_bounds: &Rect) {}
    fn on_focus(&self, view: &mut View) {}
    fn on_blur(&self, view: &mut View) {}
    fn on_theme_changed(&self, view: &mut View) {}

    fn into_raw(self) -> *mut _cef_view_delegate_t {
        let mut object: _cef_view_delegate_t = unsafe { std::mem::zeroed() };
        impl_cef_view_delegate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_view_delegate_t {
    use super::*;

    pub fn init_methods<I: ImplViewDelegate>(object: &mut _cef_view_delegate_t) {
        object.get_preferred_size = Some(get_preferred_size::<I>);
        object.get_minimum_size = Some(get_minimum_size::<I>);
        object.get_maximum_size = Some(get_maximum_size::<I>);
        object.get_height_for_width = Some(get_height_for_width::<I>);
        object.on_parent_view_changed = Some(on_parent_view_changed::<I>);
        object.on_child_view_changed = Some(on_child_view_changed::<I>);
        object.on_window_changed = Some(on_window_changed::<I>);
        object.on_layout_changed = Some(on_layout_changed::<I>);
        object.on_focus = Some(on_focus::<I>);
        object.on_blur = Some(on_blur::<I>);
        object.on_theme_changed = Some(on_theme_changed::<I>);
    }

    extern "C" fn get_preferred_size<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.get_preferred_size(view).into()
    }

    extern "C" fn get_minimum_size<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.get_minimum_size(view).into()
    }

    extern "C" fn get_maximum_size<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.get_maximum_size(view).into()
    }

    extern "C" fn get_height_for_width<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
        width: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.get_height_for_width(view, width).into()
    }

    extern "C" fn on_parent_view_changed<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
        added: ::std::os::raw::c_int,
        parent: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        let parent = &mut View(unsafe { RefGuard::from_raw_add_ref(parent) });
        obj.interface.on_parent_view_changed(view, added, parent)
    }

    extern "C" fn on_child_view_changed<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
        added: ::std::os::raw::c_int,
        child: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        let child = &mut View(unsafe { RefGuard::from_raw_add_ref(child) });
        obj.interface.on_child_view_changed(view, added, child)
    }

    extern "C" fn on_window_changed<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
        added: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.on_window_changed(view, added)
    }

    extern "C" fn on_layout_changed<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
        new_bounds: *const cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        let new_bounds = &Rect(unsafe { RefGuard::from_raw_add_ref(new_bounds) });
        obj.interface.on_layout_changed(view, new_bounds)
    }

    extern "C" fn on_focus<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.on_focus(view)
    }

    extern "C" fn on_blur<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.on_blur(view)
    }

    extern "C" fn on_theme_changed<I: ImplViewDelegate>(
        self_: *mut _cef_view_delegate_t,
        view: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.on_theme_changed(view)
    }
}

wrapper!(
    #[doc = "See [_cef_browser_view_delegate_t] for more documentation."]
    #[derive(Clone)]
    pub struct BrowserViewDelegate(_cef_browser_view_delegate_t);

    pub fn on_browser_created(&self, browser_view: &mut BrowserView, browser: &mut Browser);
    pub fn on_browser_destroyed(&self, browser_view: &mut BrowserView, browser: &mut Browser);
    pub fn get_delegate_for_popup_browser_view(
        &self,
        browser_view: &mut BrowserView,
        settings: &BrowserSettings,
        client: &mut Client,
        is_devtools: ::std::os::raw::c_int,
    ) -> BrowserViewDelegate;
    pub fn on_popup_browser_view_created(
        &self,
        browser_view: &mut BrowserView,
        popup_browser_view: &mut BrowserView,
        is_devtools: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn get_chrome_toolbar_type(&self, browser_view: &mut BrowserView) -> ChromeToolbarType;
    pub fn use_frameless_window_for_picture_in_picture(
        &self,
        browser_view: &mut BrowserView,
    ) -> ::std::os::raw::c_int;
    pub fn on_gesture_command(
        &self,
        browser_view: &mut BrowserView,
        gesture_command: GestureCommand,
    ) -> ::std::os::raw::c_int;
    pub fn get_browser_runtime_style(&self) -> RuntimeStyle;
);

pub trait ImplBrowserViewDelegate: ImplViewDelegate {
    fn on_browser_created(&self, browser_view: &mut BrowserView, browser: &mut Browser) {}
    fn on_browser_destroyed(&self, browser_view: &mut BrowserView, browser: &mut Browser) {}
    fn get_delegate_for_popup_browser_view(
        &self,
        browser_view: &mut BrowserView,
        settings: &BrowserSettings,
        client: &mut Client,
        is_devtools: ::std::os::raw::c_int,
    ) -> BrowserViewDelegate {
        Default::default()
    }
    fn on_popup_browser_view_created(
        &self,
        browser_view: &mut BrowserView,
        popup_browser_view: &mut BrowserView,
        is_devtools: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_chrome_toolbar_type(&self, browser_view: &mut BrowserView) -> ChromeToolbarType {
        Default::default()
    }
    fn use_frameless_window_for_picture_in_picture(
        &self,
        browser_view: &mut BrowserView,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_gesture_command(
        &self,
        browser_view: &mut BrowserView,
        gesture_command: GestureCommand,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_browser_runtime_style(&self) -> RuntimeStyle {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_browser_view_delegate_t {
        let mut object: _cef_browser_view_delegate_t = unsafe { std::mem::zeroed() };
        impl_cef_view_delegate_t::init_methods::<Self>(&mut object.base);
        impl_cef_browser_view_delegate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_browser_view_delegate_t {
    use super::*;

    pub fn init_methods<I: ImplBrowserViewDelegate>(object: &mut _cef_browser_view_delegate_t) {
        object.on_browser_created = Some(on_browser_created::<I>);
        object.on_browser_destroyed = Some(on_browser_destroyed::<I>);
        object.get_delegate_for_popup_browser_view = Some(get_delegate_for_popup_browser_view::<I>);
        object.on_popup_browser_view_created = Some(on_popup_browser_view_created::<I>);
        object.get_chrome_toolbar_type = Some(get_chrome_toolbar_type::<I>);
        object.use_frameless_window_for_picture_in_picture =
            Some(use_frameless_window_for_picture_in_picture::<I>);
        object.on_gesture_command = Some(on_gesture_command::<I>);
        object.get_browser_runtime_style = Some(get_browser_runtime_style::<I>);
    }

    extern "C" fn on_browser_created<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_browser_created(browser_view, browser)
    }

    extern "C" fn on_browser_destroyed<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
        browser: *mut _cef_browser_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        let browser = &mut Browser(unsafe { RefGuard::from_raw_add_ref(browser) });
        obj.interface.on_browser_destroyed(browser_view, browser)
    }

    extern "C" fn get_delegate_for_popup_browser_view<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
        settings: *const _cef_browser_settings_t,
        client: *mut _cef_client_t,
        is_devtools: ::std::os::raw::c_int,
    ) -> *mut _cef_browser_view_delegate_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        let settings = &BrowserSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        let client = &mut Client(unsafe { RefGuard::from_raw_add_ref(client) });
        obj.interface
            .get_delegate_for_popup_browser_view(browser_view, settings, client, is_devtools)
            .into()
    }

    extern "C" fn on_popup_browser_view_created<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
        popup_browser_view: *mut _cef_browser_view_t,
        is_devtools: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        let popup_browser_view =
            &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(popup_browser_view) });
        obj.interface
            .on_popup_browser_view_created(browser_view, popup_browser_view, is_devtools)
            .into()
    }

    extern "C" fn get_chrome_toolbar_type<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
    ) -> cef_chrome_toolbar_type_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        obj.interface.get_chrome_toolbar_type(browser_view).into()
    }

    extern "C" fn use_frameless_window_for_picture_in_picture<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        obj.interface
            .use_frameless_window_for_picture_in_picture(browser_view)
            .into()
    }

    extern "C" fn on_gesture_command<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
        browser_view: *mut _cef_browser_view_t,
        gesture_command: cef_gesture_command_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        obj.interface
            .on_gesture_command(browser_view, gesture_command)
            .into()
    }

    extern "C" fn get_browser_runtime_style<I: ImplBrowserViewDelegate>(
        self_: *mut _cef_browser_view_delegate_t,
    ) -> cef_runtime_style_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser_runtime_style().into()
    }
}

/// See [_cef_button_t] for more documentation.
pub struct Button(_cef_button_t);

impl From<_cef_button_t> for Button {
    fn from(value: _cef_button_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_button_t> for Button {
    fn into(self) -> _cef_button_t {
        self.0
    }
}

impl AsRef<_cef_button_t> for Button {
    fn as_ref(&self) -> &_cef_button_t {
        &self.0
    }
}

impl AsMut<_cef_button_t> for Button {
    fn as_mut(&mut self) -> &mut _cef_button_t {
        &mut self.0
    }
}

impl Default for Button {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_scroll_view_t] for more documentation.
pub struct ScrollView(_cef_scroll_view_t);

impl From<_cef_scroll_view_t> for ScrollView {
    fn from(value: _cef_scroll_view_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_scroll_view_t> for ScrollView {
    fn into(self) -> _cef_scroll_view_t {
        self.0
    }
}

impl AsRef<_cef_scroll_view_t> for ScrollView {
    fn as_ref(&self) -> &_cef_scroll_view_t {
        &self.0
    }
}

impl AsMut<_cef_scroll_view_t> for ScrollView {
    fn as_mut(&mut self) -> &mut _cef_scroll_view_t {
        &mut self.0
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_textfield_t] for more documentation.
pub struct Textfield(_cef_textfield_t);

impl From<_cef_textfield_t> for Textfield {
    fn from(value: _cef_textfield_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_textfield_t> for Textfield {
    fn into(self) -> _cef_textfield_t {
        self.0
    }
}

impl AsRef<_cef_textfield_t> for Textfield {
    fn as_ref(&self) -> &_cef_textfield_t {
        &self.0
    }
}

impl AsMut<_cef_textfield_t> for Textfield {
    fn as_mut(&mut self) -> &mut _cef_textfield_t {
        &mut self.0
    }
}

impl Default for Textfield {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_view_t] for more documentation."]
    #[derive(Clone)]
    pub struct View(_cef_view_t);

    pub fn as_browser_view(&self) -> BrowserView;
    pub fn as_button(&self) -> Button;
    pub fn as_panel(&self) -> Panel;
    pub fn as_scroll_view(&self) -> ScrollView;
    pub fn as_textfield(&self) -> Textfield;
    pub fn get_type_string(&self) -> CefStringUserfree;
    pub fn to_string(&self, include_children: ::std::os::raw::c_int) -> CefStringUserfree;
    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_attached(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut View) -> ::std::os::raw::c_int;
    pub fn get_delegate(&self) -> ViewDelegate;
    pub fn get_window(&self) -> Window;
    pub fn get_id(&self) -> ::std::os::raw::c_int;
    pub fn set_id(&self, id: ::std::os::raw::c_int);
    pub fn get_group_id(&self) -> ::std::os::raw::c_int;
    pub fn set_group_id(&self, group_id: ::std::os::raw::c_int);
    pub fn get_parent_view(&self) -> View;
    pub fn get_view_for_id(&self, id: ::std::os::raw::c_int) -> View;
    pub fn set_bounds(&self, bounds: &Rect);
    pub fn get_bounds(&self) -> Rect;
    pub fn get_bounds_in_screen(&self) -> Rect;
    pub fn set_size(&self, size: &Size);
    pub fn get_size(&self) -> Size;
    pub fn set_position(&self, position: &Point);
    pub fn get_position(&self) -> Point;
    pub fn set_insets(&self, insets: &Insets);
    pub fn get_insets(&self) -> Insets;
    pub fn get_preferred_size(&self) -> Size;
    pub fn size_to_preferred_size(&self);
    pub fn get_minimum_size(&self) -> Size;
    pub fn get_maximum_size(&self) -> Size;
    pub fn get_height_for_width(&self, width: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn invalidate_layout(&self);
    pub fn set_visible(&self, visible: ::std::os::raw::c_int);
    pub fn is_visible(&self) -> ::std::os::raw::c_int;
    pub fn is_drawn(&self) -> ::std::os::raw::c_int;
    pub fn set_enabled(&self, enabled: ::std::os::raw::c_int);
    pub fn is_enabled(&self) -> ::std::os::raw::c_int;
    pub fn set_focusable(&self, focusable: ::std::os::raw::c_int);
    pub fn is_focusable(&self) -> ::std::os::raw::c_int;
    pub fn is_accessibility_focusable(&self) -> ::std::os::raw::c_int;
    pub fn has_focus(&self) -> ::std::os::raw::c_int;
    pub fn request_focus(&self);
    pub fn set_background_color(&self, color: &mut Color);
    pub fn get_background_color(&self) -> Color;
    pub fn get_theme_color(&self, color_id: ::std::os::raw::c_int) -> Color;
    pub fn convert_point_to_screen(&self, point: &mut Point) -> ::std::os::raw::c_int;
    pub fn convert_point_from_screen(&self, point: &mut Point) -> ::std::os::raw::c_int;
    pub fn convert_point_to_window(&self, point: &mut Point) -> ::std::os::raw::c_int;
    pub fn convert_point_from_window(&self, point: &mut Point) -> ::std::os::raw::c_int;
    pub fn convert_point_to_view(
        &self,
        view: &mut View,
        point: &mut Point,
    ) -> ::std::os::raw::c_int;
    pub fn convert_point_from_view(
        &self,
        view: &mut View,
        point: &mut Point,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplView: Sized {
    fn as_browser_view(&self) -> BrowserView {
        Default::default()
    }
    fn as_button(&self) -> Button {
        Default::default()
    }
    fn as_panel(&self) -> Panel {
        Default::default()
    }
    fn as_scroll_view(&self) -> ScrollView {
        Default::default()
    }
    fn as_textfield(&self) -> Textfield {
        Default::default()
    }
    fn get_type_string(&self) -> CefStringUserfree {
        Default::default()
    }
    fn to_string(&self, include_children: ::std::os::raw::c_int) -> CefStringUserfree {
        Default::default()
    }
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_attached(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut View) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_delegate(&self) -> ViewDelegate {
        Default::default()
    }
    fn get_window(&self) -> Window {
        Default::default()
    }
    fn get_id(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_id(&self, id: ::std::os::raw::c_int) {}
    fn get_group_id(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_group_id(&self, group_id: ::std::os::raw::c_int) {}
    fn get_parent_view(&self) -> View {
        Default::default()
    }
    fn get_view_for_id(&self, id: ::std::os::raw::c_int) -> View {
        Default::default()
    }
    fn set_bounds(&self, bounds: &Rect) {}
    fn get_bounds(&self) -> Rect {
        Default::default()
    }
    fn get_bounds_in_screen(&self) -> Rect {
        Default::default()
    }
    fn set_size(&self, size: &Size) {}
    fn get_size(&self) -> Size {
        Default::default()
    }
    fn set_position(&self, position: &Point) {}
    fn get_position(&self) -> Point {
        Default::default()
    }
    fn set_insets(&self, insets: &Insets) {}
    fn get_insets(&self) -> Insets {
        Default::default()
    }
    fn get_preferred_size(&self) -> Size {
        Default::default()
    }
    fn size_to_preferred_size(&self) {}
    fn get_minimum_size(&self) -> Size {
        Default::default()
    }
    fn get_maximum_size(&self) -> Size {
        Default::default()
    }
    fn get_height_for_width(&self, width: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn invalidate_layout(&self) {}
    fn set_visible(&self, visible: ::std::os::raw::c_int) {}
    fn is_visible(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_drawn(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_enabled(&self, enabled: ::std::os::raw::c_int) {}
    fn is_enabled(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn set_focusable(&self, focusable: ::std::os::raw::c_int) {}
    fn is_focusable(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_accessibility_focusable(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn has_focus(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn request_focus(&self) {}
    fn set_background_color(&self, color: &mut Color) {}
    fn get_background_color(&self) -> Color {
        Default::default()
    }
    fn get_theme_color(&self, color_id: ::std::os::raw::c_int) -> Color {
        Default::default()
    }
    fn convert_point_to_screen(&self, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn convert_point_from_screen(&self, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn convert_point_to_window(&self, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn convert_point_from_window(&self, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn convert_point_to_view(&self, view: &mut View, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn convert_point_from_view(&self, view: &mut View, point: &mut Point) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_view_t {
        let mut object: _cef_view_t = unsafe { std::mem::zeroed() };
        impl_cef_view_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_view_t {
    use super::*;

    pub fn init_methods<I: ImplView>(object: &mut _cef_view_t) {
        object.as_browser_view = Some(as_browser_view::<I>);
        object.as_button = Some(as_button::<I>);
        object.as_panel = Some(as_panel::<I>);
        object.as_scroll_view = Some(as_scroll_view::<I>);
        object.as_textfield = Some(as_textfield::<I>);
        object.get_type_string = Some(get_type_string::<I>);
        object.to_string = Some(to_string::<I>);
        object.is_valid = Some(is_valid::<I>);
        object.is_attached = Some(is_attached::<I>);
        object.is_same = Some(is_same::<I>);
        object.get_delegate = Some(get_delegate::<I>);
        object.get_window = Some(get_window::<I>);
        object.get_id = Some(get_id::<I>);
        object.set_id = Some(set_id::<I>);
        object.get_group_id = Some(get_group_id::<I>);
        object.set_group_id = Some(set_group_id::<I>);
        object.get_parent_view = Some(get_parent_view::<I>);
        object.get_view_for_id = Some(get_view_for_id::<I>);
        object.set_bounds = Some(set_bounds::<I>);
        object.get_bounds = Some(get_bounds::<I>);
        object.get_bounds_in_screen = Some(get_bounds_in_screen::<I>);
        object.set_size = Some(set_size::<I>);
        object.get_size = Some(get_size::<I>);
        object.set_position = Some(set_position::<I>);
        object.get_position = Some(get_position::<I>);
        object.set_insets = Some(set_insets::<I>);
        object.get_insets = Some(get_insets::<I>);
        object.get_preferred_size = Some(get_preferred_size::<I>);
        object.size_to_preferred_size = Some(size_to_preferred_size::<I>);
        object.get_minimum_size = Some(get_minimum_size::<I>);
        object.get_maximum_size = Some(get_maximum_size::<I>);
        object.get_height_for_width = Some(get_height_for_width::<I>);
        object.invalidate_layout = Some(invalidate_layout::<I>);
        object.set_visible = Some(set_visible::<I>);
        object.is_visible = Some(is_visible::<I>);
        object.is_drawn = Some(is_drawn::<I>);
        object.set_enabled = Some(set_enabled::<I>);
        object.is_enabled = Some(is_enabled::<I>);
        object.set_focusable = Some(set_focusable::<I>);
        object.is_focusable = Some(is_focusable::<I>);
        object.is_accessibility_focusable = Some(is_accessibility_focusable::<I>);
        object.has_focus = Some(has_focus::<I>);
        object.request_focus = Some(request_focus::<I>);
        object.set_background_color = Some(set_background_color::<I>);
        object.get_background_color = Some(get_background_color::<I>);
        object.get_theme_color = Some(get_theme_color::<I>);
        object.convert_point_to_screen = Some(convert_point_to_screen::<I>);
        object.convert_point_from_screen = Some(convert_point_from_screen::<I>);
        object.convert_point_to_window = Some(convert_point_to_window::<I>);
        object.convert_point_from_window = Some(convert_point_from_window::<I>);
        object.convert_point_to_view = Some(convert_point_to_view::<I>);
        object.convert_point_from_view = Some(convert_point_from_view::<I>);
    }

    extern "C" fn as_browser_view<I: ImplView>(
        self_: *mut _cef_view_t,
    ) -> *mut _cef_browser_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_browser_view().into()
    }

    extern "C" fn as_button<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_button_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_button().into()
    }

    extern "C" fn as_panel<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_panel_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_panel().into()
    }

    extern "C" fn as_scroll_view<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_scroll_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_scroll_view().into()
    }

    extern "C" fn as_textfield<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_textfield_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_textfield().into()
    }

    extern "C" fn get_type_string<I: ImplView>(self_: *mut _cef_view_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_type_string().into()
    }

    extern "C" fn to_string<I: ImplView>(
        self_: *mut _cef_view_t,
        include_children: ::std::os::raw::c_int,
    ) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.to_string(include_children).into()
    }

    extern "C" fn is_valid<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_attached<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_attached().into()
    }

    extern "C" fn is_same<I: ImplView>(
        self_: *mut _cef_view_t,
        that: *mut _cef_view_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut View(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn get_delegate<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_view_delegate_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_delegate().into()
    }

    extern "C" fn get_window<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_window_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window().into()
    }

    extern "C" fn get_id<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn set_id<I: ImplView>(self_: *mut _cef_view_t, id: ::std::os::raw::c_int) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_id(id)
    }

    extern "C" fn get_group_id<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_group_id().into()
    }

    extern "C" fn set_group_id<I: ImplView>(
        self_: *mut _cef_view_t,
        group_id: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_group_id(group_id)
    }

    extern "C" fn get_parent_view<I: ImplView>(self_: *mut _cef_view_t) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_parent_view().into()
    }

    extern "C" fn get_view_for_id<I: ImplView>(
        self_: *mut _cef_view_t,
        id: ::std::os::raw::c_int,
    ) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_view_for_id(id).into()
    }

    extern "C" fn set_bounds<I: ImplView>(self_: *mut _cef_view_t, bounds: *const cef_rect_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let bounds = &Rect(unsafe { RefGuard::from_raw_add_ref(bounds) });
        obj.interface.set_bounds(bounds)
    }

    extern "C" fn get_bounds<I: ImplView>(self_: *mut _cef_view_t) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bounds().into()
    }

    extern "C" fn get_bounds_in_screen<I: ImplView>(self_: *mut _cef_view_t) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bounds_in_screen().into()
    }

    extern "C" fn set_size<I: ImplView>(self_: *mut _cef_view_t, size: *const cef_size_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let size = &Size(unsafe { RefGuard::from_raw_add_ref(size) });
        obj.interface.set_size(size)
    }

    extern "C" fn get_size<I: ImplView>(self_: *mut _cef_view_t) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_size().into()
    }

    extern "C" fn set_position<I: ImplView>(self_: *mut _cef_view_t, position: *const cef_point_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let position = &Point(unsafe { RefGuard::from_raw_add_ref(position) });
        obj.interface.set_position(position)
    }

    extern "C" fn get_position<I: ImplView>(self_: *mut _cef_view_t) -> cef_point_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_position().into()
    }

    extern "C" fn set_insets<I: ImplView>(self_: *mut _cef_view_t, insets: *const cef_insets_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let insets = &Insets(unsafe { RefGuard::from_raw_add_ref(insets) });
        obj.interface.set_insets(insets)
    }

    extern "C" fn get_insets<I: ImplView>(self_: *mut _cef_view_t) -> cef_insets_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_insets().into()
    }

    extern "C" fn get_preferred_size<I: ImplView>(self_: *mut _cef_view_t) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_preferred_size().into()
    }

    extern "C" fn size_to_preferred_size<I: ImplView>(self_: *mut _cef_view_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.size_to_preferred_size()
    }

    extern "C" fn get_minimum_size<I: ImplView>(self_: *mut _cef_view_t) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_minimum_size().into()
    }

    extern "C" fn get_maximum_size<I: ImplView>(self_: *mut _cef_view_t) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_maximum_size().into()
    }

    extern "C" fn get_height_for_width<I: ImplView>(
        self_: *mut _cef_view_t,
        width: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_height_for_width(width).into()
    }

    extern "C" fn invalidate_layout<I: ImplView>(self_: *mut _cef_view_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.invalidate_layout()
    }

    extern "C" fn set_visible<I: ImplView>(
        self_: *mut _cef_view_t,
        visible: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_visible(visible)
    }

    extern "C" fn is_visible<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_visible().into()
    }

    extern "C" fn is_drawn<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_drawn().into()
    }

    extern "C" fn set_enabled<I: ImplView>(
        self_: *mut _cef_view_t,
        enabled: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_enabled(enabled)
    }

    extern "C" fn is_enabled<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_enabled().into()
    }

    extern "C" fn set_focusable<I: ImplView>(
        self_: *mut _cef_view_t,
        focusable: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_focusable(focusable)
    }

    extern "C" fn is_focusable<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_focusable().into()
    }

    extern "C" fn is_accessibility_focusable<I: ImplView>(
        self_: *mut _cef_view_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_accessibility_focusable().into()
    }

    extern "C" fn has_focus<I: ImplView>(self_: *mut _cef_view_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.has_focus().into()
    }

    extern "C" fn request_focus<I: ImplView>(self_: *mut _cef_view_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.request_focus()
    }

    extern "C" fn set_background_color<I: ImplView>(self_: *mut _cef_view_t, color: cef_color_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_background_color(color)
    }

    extern "C" fn get_background_color<I: ImplView>(self_: *mut _cef_view_t) -> cef_color_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_background_color().into()
    }

    extern "C" fn get_theme_color<I: ImplView>(
        self_: *mut _cef_view_t,
        color_id: ::std::os::raw::c_int,
    ) -> cef_color_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_theme_color(color_id).into()
    }

    extern "C" fn convert_point_to_screen<I: ImplView>(
        self_: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_to_screen(point).into()
    }

    extern "C" fn convert_point_from_screen<I: ImplView>(
        self_: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_from_screen(point).into()
    }

    extern "C" fn convert_point_to_window<I: ImplView>(
        self_: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_to_window(point).into()
    }

    extern "C" fn convert_point_from_window<I: ImplView>(
        self_: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_from_window(point).into()
    }

    extern "C" fn convert_point_to_view<I: ImplView>(
        self_: *mut _cef_view_t,
        view: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_to_view(view, point).into()
    }

    extern "C" fn convert_point_from_view<I: ImplView>(
        self_: *mut _cef_view_t,
        view: *mut _cef_view_t,
        point: *mut cef_point_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_from_view(view, point).into()
    }
}

wrapper!(
    #[doc = "See [_cef_browser_view_t] for more documentation."]
    #[derive(Clone)]
    pub struct BrowserView(_cef_browser_view_t);

    pub fn get_browser(&self) -> Browser;
    pub fn get_chrome_toolbar(&self) -> View;
    pub fn set_prefer_accelerators(&self, prefer_accelerators: ::std::os::raw::c_int);
    pub fn get_runtime_style(&self) -> RuntimeStyle;
);

pub trait ImplBrowserView: ImplView {
    fn get_browser(&self) -> Browser {
        Default::default()
    }
    fn get_chrome_toolbar(&self) -> View {
        Default::default()
    }
    fn set_prefer_accelerators(&self, prefer_accelerators: ::std::os::raw::c_int) {}
    fn get_runtime_style(&self) -> RuntimeStyle {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_browser_view_t {
        let mut object: _cef_browser_view_t = unsafe { std::mem::zeroed() };
        impl_cef_view_t::init_methods::<Self>(&mut object.base);
        impl_cef_browser_view_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_browser_view_t {
    use super::*;

    pub fn init_methods<I: ImplBrowserView>(object: &mut _cef_browser_view_t) {
        object.get_browser = Some(get_browser::<I>);
        object.get_chrome_toolbar = Some(get_chrome_toolbar::<I>);
        object.set_prefer_accelerators = Some(set_prefer_accelerators::<I>);
        object.get_runtime_style = Some(get_runtime_style::<I>);
    }

    extern "C" fn get_browser<I: ImplBrowserView>(
        self_: *mut _cef_browser_view_t,
    ) -> *mut _cef_browser_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_browser().into()
    }

    extern "C" fn get_chrome_toolbar<I: ImplBrowserView>(
        self_: *mut _cef_browser_view_t,
    ) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_chrome_toolbar().into()
    }

    extern "C" fn set_prefer_accelerators<I: ImplBrowserView>(
        self_: *mut _cef_browser_view_t,
        prefer_accelerators: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_prefer_accelerators(prefer_accelerators)
    }

    extern "C" fn get_runtime_style<I: ImplBrowserView>(
        self_: *mut _cef_browser_view_t,
    ) -> cef_runtime_style_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_runtime_style().into()
    }
}

wrapper!(
    #[doc = "See [_cef_display_t] for more documentation."]
    #[derive(Clone)]
    pub struct Display(_cef_display_t);

    pub fn get_id(&self) -> i64;
    pub fn get_device_scale_factor(&self) -> f32;
    pub fn convert_point_to_pixels(&self, point: &mut Point);
    pub fn convert_point_from_pixels(&self, point: &mut Point);
    pub fn get_bounds(&self) -> Rect;
    pub fn get_work_area(&self) -> Rect;
    pub fn get_rotation(&self) -> ::std::os::raw::c_int;
);

pub trait ImplDisplay: Sized {
    fn get_id(&self) -> i64 {
        Default::default()
    }
    fn get_device_scale_factor(&self) -> f32 {
        Default::default()
    }
    fn convert_point_to_pixels(&self, point: &mut Point) {}
    fn convert_point_from_pixels(&self, point: &mut Point) {}
    fn get_bounds(&self) -> Rect {
        Default::default()
    }
    fn get_work_area(&self) -> Rect {
        Default::default()
    }
    fn get_rotation(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_display_t {
        let mut object: _cef_display_t = unsafe { std::mem::zeroed() };
        impl_cef_display_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_display_t {
    use super::*;

    pub fn init_methods<I: ImplDisplay>(object: &mut _cef_display_t) {
        object.get_id = Some(get_id::<I>);
        object.get_device_scale_factor = Some(get_device_scale_factor::<I>);
        object.convert_point_to_pixels = Some(convert_point_to_pixels::<I>);
        object.convert_point_from_pixels = Some(convert_point_from_pixels::<I>);
        object.get_bounds = Some(get_bounds::<I>);
        object.get_work_area = Some(get_work_area::<I>);
        object.get_rotation = Some(get_rotation::<I>);
    }

    extern "C" fn get_id<I: ImplDisplay>(self_: *mut _cef_display_t) -> i64 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_id().into()
    }

    extern "C" fn get_device_scale_factor<I: ImplDisplay>(self_: *mut _cef_display_t) -> f32 {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_device_scale_factor().into()
    }

    extern "C" fn convert_point_to_pixels<I: ImplDisplay>(
        self_: *mut _cef_display_t,
        point: *mut cef_point_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_to_pixels(point)
    }

    extern "C" fn convert_point_from_pixels<I: ImplDisplay>(
        self_: *mut _cef_display_t,
        point: *mut cef_point_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let point = &mut Point(unsafe { RefGuard::from_raw_add_ref(point) });
        obj.interface.convert_point_from_pixels(point)
    }

    extern "C" fn get_bounds<I: ImplDisplay>(self_: *mut _cef_display_t) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bounds().into()
    }

    extern "C" fn get_work_area<I: ImplDisplay>(self_: *mut _cef_display_t) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_work_area().into()
    }

    extern "C" fn get_rotation<I: ImplDisplay>(
        self_: *mut _cef_display_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_rotation().into()
    }
}

wrapper!(
    #[doc = "See [_cef_overlay_controller_t] for more documentation."]
    #[derive(Clone)]
    pub struct OverlayController(_cef_overlay_controller_t);

    pub fn is_valid(&self) -> ::std::os::raw::c_int;
    pub fn is_same(&self, that: &mut OverlayController) -> ::std::os::raw::c_int;
    pub fn get_contents_view(&self) -> View;
    pub fn get_window(&self) -> Window;
    pub fn get_docking_mode(&self) -> DockingMode;
    pub fn destroy(&self);
    pub fn set_bounds(&self, bounds: &Rect);
    pub fn get_bounds(&self) -> Rect;
    pub fn get_bounds_in_screen(&self) -> Rect;
    pub fn set_size(&self, size: &Size);
    pub fn get_size(&self) -> Size;
    pub fn set_position(&self, position: &Point);
    pub fn get_position(&self) -> Point;
    pub fn set_insets(&self, insets: &Insets);
    pub fn get_insets(&self) -> Insets;
    pub fn size_to_preferred_size(&self);
    pub fn set_visible(&self, visible: ::std::os::raw::c_int);
    pub fn is_visible(&self) -> ::std::os::raw::c_int;
    pub fn is_drawn(&self) -> ::std::os::raw::c_int;
);

pub trait ImplOverlayController: Sized {
    fn is_valid(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_same(&self, that: &mut OverlayController) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_contents_view(&self) -> View {
        Default::default()
    }
    fn get_window(&self) -> Window {
        Default::default()
    }
    fn get_docking_mode(&self) -> DockingMode {
        Default::default()
    }
    fn destroy(&self) {}
    fn set_bounds(&self, bounds: &Rect) {}
    fn get_bounds(&self) -> Rect {
        Default::default()
    }
    fn get_bounds_in_screen(&self) -> Rect {
        Default::default()
    }
    fn set_size(&self, size: &Size) {}
    fn get_size(&self) -> Size {
        Default::default()
    }
    fn set_position(&self, position: &Point) {}
    fn get_position(&self) -> Point {
        Default::default()
    }
    fn set_insets(&self, insets: &Insets) {}
    fn get_insets(&self) -> Insets {
        Default::default()
    }
    fn size_to_preferred_size(&self) {}
    fn set_visible(&self, visible: ::std::os::raw::c_int) {}
    fn is_visible(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_drawn(&self) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_overlay_controller_t {
        let mut object: _cef_overlay_controller_t = unsafe { std::mem::zeroed() };
        impl_cef_overlay_controller_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_overlay_controller_t {
    use super::*;

    pub fn init_methods<I: ImplOverlayController>(object: &mut _cef_overlay_controller_t) {
        object.is_valid = Some(is_valid::<I>);
        object.is_same = Some(is_same::<I>);
        object.get_contents_view = Some(get_contents_view::<I>);
        object.get_window = Some(get_window::<I>);
        object.get_docking_mode = Some(get_docking_mode::<I>);
        object.destroy = Some(destroy::<I>);
        object.set_bounds = Some(set_bounds::<I>);
        object.get_bounds = Some(get_bounds::<I>);
        object.get_bounds_in_screen = Some(get_bounds_in_screen::<I>);
        object.set_size = Some(set_size::<I>);
        object.get_size = Some(get_size::<I>);
        object.set_position = Some(set_position::<I>);
        object.get_position = Some(get_position::<I>);
        object.set_insets = Some(set_insets::<I>);
        object.get_insets = Some(get_insets::<I>);
        object.size_to_preferred_size = Some(size_to_preferred_size::<I>);
        object.set_visible = Some(set_visible::<I>);
        object.is_visible = Some(is_visible::<I>);
        object.is_drawn = Some(is_drawn::<I>);
    }

    extern "C" fn is_valid<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_valid().into()
    }

    extern "C" fn is_same<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        that: *mut _cef_overlay_controller_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let that = &mut OverlayController(unsafe { RefGuard::from_raw_add_ref(that) });
        obj.interface.is_same(that).into()
    }

    extern "C" fn get_contents_view<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_contents_view().into()
    }

    extern "C" fn get_window<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> *mut _cef_window_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window().into()
    }

    extern "C" fn get_docking_mode<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_docking_mode_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_docking_mode().into()
    }

    extern "C" fn destroy<I: ImplOverlayController>(self_: *mut _cef_overlay_controller_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.destroy()
    }

    extern "C" fn set_bounds<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        bounds: *const cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let bounds = &Rect(unsafe { RefGuard::from_raw_add_ref(bounds) });
        obj.interface.set_bounds(bounds)
    }

    extern "C" fn get_bounds<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bounds().into()
    }

    extern "C" fn get_bounds_in_screen<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_bounds_in_screen().into()
    }

    extern "C" fn set_size<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        size: *const cef_size_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let size = &Size(unsafe { RefGuard::from_raw_add_ref(size) });
        obj.interface.set_size(size)
    }

    extern "C" fn get_size<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_size_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_size().into()
    }

    extern "C" fn set_position<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        position: *const cef_point_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let position = &Point(unsafe { RefGuard::from_raw_add_ref(position) });
        obj.interface.set_position(position)
    }

    extern "C" fn get_position<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_point_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_position().into()
    }

    extern "C" fn set_insets<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        insets: *const cef_insets_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let insets = &Insets(unsafe { RefGuard::from_raw_add_ref(insets) });
        obj.interface.set_insets(insets)
    }

    extern "C" fn get_insets<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> cef_insets_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_insets().into()
    }

    extern "C" fn size_to_preferred_size<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.size_to_preferred_size()
    }

    extern "C" fn set_visible<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
        visible: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_visible(visible)
    }

    extern "C" fn is_visible<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_visible().into()
    }

    extern "C" fn is_drawn<I: ImplOverlayController>(
        self_: *mut _cef_overlay_controller_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_drawn().into()
    }
}

wrapper!(
    #[doc = "See [_cef_panel_delegate_t] for more documentation."]
    #[derive(Clone)]
    pub struct PanelDelegate(_cef_panel_delegate_t);
);

pub trait ImplPanelDelegate: ImplViewDelegate {
    fn into_raw(self) -> *mut _cef_panel_delegate_t {
        let mut object: _cef_panel_delegate_t = unsafe { std::mem::zeroed() };
        impl_cef_view_delegate_t::init_methods::<Self>(&mut object.base);
        impl_cef_panel_delegate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_panel_delegate_t {
    use super::*;

    pub fn init_methods<I: ImplPanelDelegate>(object: &mut _cef_panel_delegate_t) {}
}

/// See [_cef_box_layout_t] for more documentation.
pub struct BoxLayout(_cef_box_layout_t);

impl From<_cef_box_layout_t> for BoxLayout {
    fn from(value: _cef_box_layout_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_box_layout_t> for BoxLayout {
    fn into(self) -> _cef_box_layout_t {
        self.0
    }
}

impl AsRef<_cef_box_layout_t> for BoxLayout {
    fn as_ref(&self) -> &_cef_box_layout_t {
        &self.0
    }
}

impl AsMut<_cef_box_layout_t> for BoxLayout {
    fn as_mut(&mut self) -> &mut _cef_box_layout_t {
        &mut self.0
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_fill_layout_t] for more documentation.
pub struct FillLayout(_cef_fill_layout_t);

impl From<_cef_fill_layout_t> for FillLayout {
    fn from(value: _cef_fill_layout_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_fill_layout_t> for FillLayout {
    fn into(self) -> _cef_fill_layout_t {
        self.0
    }
}

impl AsRef<_cef_fill_layout_t> for FillLayout {
    fn as_ref(&self) -> &_cef_fill_layout_t {
        &self.0
    }
}

impl AsMut<_cef_fill_layout_t> for FillLayout {
    fn as_mut(&mut self) -> &mut _cef_fill_layout_t {
        &mut self.0
    }
}

impl Default for FillLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_layout_t] for more documentation.
pub struct Layout(_cef_layout_t);

impl From<_cef_layout_t> for Layout {
    fn from(value: _cef_layout_t) -> Self {
        Self(value)
    }
}

impl Into<_cef_layout_t> for Layout {
    fn into(self) -> _cef_layout_t {
        self.0
    }
}

impl AsRef<_cef_layout_t> for Layout {
    fn as_ref(&self) -> &_cef_layout_t {
        &self.0
    }
}

impl AsMut<_cef_layout_t> for Layout {
    fn as_mut(&mut self) -> &mut _cef_layout_t {
        &mut self.0
    }
}

impl Default for Layout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

wrapper!(
    #[doc = "See [_cef_panel_t] for more documentation."]
    #[derive(Clone)]
    pub struct Panel(_cef_panel_t);

    pub fn as_window(&self) -> Window;
    pub fn set_to_fill_layout(&self) -> FillLayout;
    pub fn set_to_box_layout(&self, settings: &BoxLayoutSettings) -> BoxLayout;
    pub fn get_layout(&self) -> Layout;
    pub fn layout(&self);
    pub fn add_child_view(&self, view: &mut View);
    pub fn add_child_view_at(&self, view: &mut View, index: ::std::os::raw::c_int);
    pub fn reorder_child_view(&self, view: &mut View, index: ::std::os::raw::c_int);
    pub fn remove_child_view(&self, view: &mut View);
    pub fn remove_all_child_views(&self);
    pub fn get_child_view_count(&self) -> usize;
    pub fn get_child_view_at(&self, index: ::std::os::raw::c_int) -> View;
);

pub trait ImplPanel: ImplView {
    fn as_window(&self) -> Window {
        Default::default()
    }
    fn set_to_fill_layout(&self) -> FillLayout {
        Default::default()
    }
    fn set_to_box_layout(&self, settings: &BoxLayoutSettings) -> BoxLayout {
        Default::default()
    }
    fn get_layout(&self) -> Layout {
        Default::default()
    }
    fn layout(&self) {}
    fn add_child_view(&self, view: &mut View) {}
    fn add_child_view_at(&self, view: &mut View, index: ::std::os::raw::c_int) {}
    fn reorder_child_view(&self, view: &mut View, index: ::std::os::raw::c_int) {}
    fn remove_child_view(&self, view: &mut View) {}
    fn remove_all_child_views(&self) {}
    fn get_child_view_count(&self) -> usize {
        Default::default()
    }
    fn get_child_view_at(&self, index: ::std::os::raw::c_int) -> View {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_panel_t {
        let mut object: _cef_panel_t = unsafe { std::mem::zeroed() };
        impl_cef_view_t::init_methods::<Self>(&mut object.base);
        impl_cef_panel_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_panel_t {
    use super::*;

    pub fn init_methods<I: ImplPanel>(object: &mut _cef_panel_t) {
        object.as_window = Some(as_window::<I>);
        object.set_to_fill_layout = Some(set_to_fill_layout::<I>);
        object.set_to_box_layout = Some(set_to_box_layout::<I>);
        object.get_layout = Some(get_layout::<I>);
        object.layout = Some(layout::<I>);
        object.add_child_view = Some(add_child_view::<I>);
        object.add_child_view_at = Some(add_child_view_at::<I>);
        object.reorder_child_view = Some(reorder_child_view::<I>);
        object.remove_child_view = Some(remove_child_view::<I>);
        object.remove_all_child_views = Some(remove_all_child_views::<I>);
        object.get_child_view_count = Some(get_child_view_count::<I>);
        object.get_child_view_at = Some(get_child_view_at::<I>);
    }

    extern "C" fn as_window<I: ImplPanel>(self_: *mut _cef_panel_t) -> *mut _cef_window_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.as_window().into()
    }

    extern "C" fn set_to_fill_layout<I: ImplPanel>(
        self_: *mut _cef_panel_t,
    ) -> *mut _cef_fill_layout_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_to_fill_layout().into()
    }

    extern "C" fn set_to_box_layout<I: ImplPanel>(
        self_: *mut _cef_panel_t,
        settings: *const cef_box_layout_settings_t,
    ) -> *mut _cef_box_layout_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let settings = &BoxLayoutSettings(unsafe { RefGuard::from_raw_add_ref(settings) });
        obj.interface.set_to_box_layout(settings).into()
    }

    extern "C" fn get_layout<I: ImplPanel>(self_: *mut _cef_panel_t) -> *mut _cef_layout_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_layout().into()
    }

    extern "C" fn layout<I: ImplPanel>(self_: *mut _cef_panel_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.layout()
    }

    extern "C" fn add_child_view<I: ImplPanel>(self_: *mut _cef_panel_t, view: *mut _cef_view_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.add_child_view(view)
    }

    extern "C" fn add_child_view_at<I: ImplPanel>(
        self_: *mut _cef_panel_t,
        view: *mut _cef_view_t,
        index: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.add_child_view_at(view, index)
    }

    extern "C" fn reorder_child_view<I: ImplPanel>(
        self_: *mut _cef_panel_t,
        view: *mut _cef_view_t,
        index: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.reorder_child_view(view, index)
    }

    extern "C" fn remove_child_view<I: ImplPanel>(
        self_: *mut _cef_panel_t,
        view: *mut _cef_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface.remove_child_view(view)
    }

    extern "C" fn remove_all_child_views<I: ImplPanel>(self_: *mut _cef_panel_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_all_child_views()
    }

    extern "C" fn get_child_view_count<I: ImplPanel>(self_: *mut _cef_panel_t) -> usize {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_child_view_count().into()
    }

    extern "C" fn get_child_view_at<I: ImplPanel>(
        self_: *mut _cef_panel_t,
        index: ::std::os::raw::c_int,
    ) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_child_view_at(index).into()
    }
}

wrapper!(
    #[doc = "See [_cef_window_delegate_t] for more documentation."]
    #[derive(Clone)]
    pub struct WindowDelegate(_cef_window_delegate_t);

    pub fn on_window_created(&self, window: &mut Window);
    pub fn on_window_closing(&self, window: &mut Window);
    pub fn on_window_destroyed(&self, window: &mut Window);
    pub fn on_window_activation_changed(&self, window: &mut Window, active: ::std::os::raw::c_int);
    pub fn on_window_bounds_changed(&self, window: &mut Window, new_bounds: &Rect);
    pub fn on_window_fullscreen_transition(
        &self,
        window: &mut Window,
        is_completed: ::std::os::raw::c_int,
    );
    pub fn get_parent_window(
        &self,
        window: &mut Window,
        is_menu: *mut ::std::os::raw::c_int,
        can_activate_menu: *mut ::std::os::raw::c_int,
    ) -> Window;
    pub fn is_window_modal_dialog(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn get_initial_bounds(&self, window: &mut Window) -> Rect;
    pub fn get_initial_show_state(&self, window: &mut Window) -> ShowState;
    pub fn is_frameless(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn with_standard_window_buttons(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn get_titlebar_height(
        &self,
        window: &mut Window,
        titlebar_height: *mut f32,
    ) -> ::std::os::raw::c_int;
    pub fn accepts_first_mouse(&self, window: &mut Window) -> State;
    pub fn can_resize(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn can_maximize(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn can_minimize(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn can_close(&self, window: &mut Window) -> ::std::os::raw::c_int;
    pub fn on_accelerator(
        &self,
        window: &mut Window,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn on_key_event(&self, window: &mut Window, event: &KeyEvent) -> ::std::os::raw::c_int;
    pub fn on_theme_colors_changed(&self, window: &mut Window, chrome_theme: ::std::os::raw::c_int);
    pub fn get_window_runtime_style(&self) -> RuntimeStyle;
    pub fn get_linux_window_properties(
        &self,
        window: &mut Window,
        properties: &mut LinuxWindowProperties,
    ) -> ::std::os::raw::c_int;
);

pub trait ImplWindowDelegate: ImplPanelDelegate {
    fn on_window_created(&self, window: &mut Window) {}
    fn on_window_closing(&self, window: &mut Window) {}
    fn on_window_destroyed(&self, window: &mut Window) {}
    fn on_window_activation_changed(&self, window: &mut Window, active: ::std::os::raw::c_int) {}
    fn on_window_bounds_changed(&self, window: &mut Window, new_bounds: &Rect) {}
    fn on_window_fullscreen_transition(
        &self,
        window: &mut Window,
        is_completed: ::std::os::raw::c_int,
    ) {
    }
    fn get_parent_window(
        &self,
        window: &mut Window,
        is_menu: *mut ::std::os::raw::c_int,
        can_activate_menu: *mut ::std::os::raw::c_int,
    ) -> Window {
        Default::default()
    }
    fn is_window_modal_dialog(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_initial_bounds(&self, window: &mut Window) -> Rect {
        Default::default()
    }
    fn get_initial_show_state(&self, window: &mut Window) -> ShowState {
        Default::default()
    }
    fn is_frameless(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn with_standard_window_buttons(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_titlebar_height(
        &self,
        window: &mut Window,
        titlebar_height: *mut f32,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn accepts_first_mouse(&self, window: &mut Window) -> State {
        Default::default()
    }
    fn can_resize(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn can_maximize(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn can_minimize(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn can_close(&self, window: &mut Window) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_accelerator(
        &self,
        window: &mut Window,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_key_event(&self, window: &mut Window, event: &KeyEvent) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn on_theme_colors_changed(&self, window: &mut Window, chrome_theme: ::std::os::raw::c_int) {}
    fn get_window_runtime_style(&self) -> RuntimeStyle {
        Default::default()
    }
    fn get_linux_window_properties(
        &self,
        window: &mut Window,
        properties: &mut LinuxWindowProperties,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_window_delegate_t {
        let mut object: _cef_window_delegate_t = unsafe { std::mem::zeroed() };
        impl_cef_view_delegate_t::init_methods::<Self>(&mut object.base.base);
        impl_cef_panel_delegate_t::init_methods::<Self>(&mut object.base);
        impl_cef_window_delegate_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_window_delegate_t {
    use super::*;

    pub fn init_methods<I: ImplWindowDelegate>(object: &mut _cef_window_delegate_t) {
        object.on_window_created = Some(on_window_created::<I>);
        object.on_window_closing = Some(on_window_closing::<I>);
        object.on_window_destroyed = Some(on_window_destroyed::<I>);
        object.on_window_activation_changed = Some(on_window_activation_changed::<I>);
        object.on_window_bounds_changed = Some(on_window_bounds_changed::<I>);
        object.on_window_fullscreen_transition = Some(on_window_fullscreen_transition::<I>);
        object.get_parent_window = Some(get_parent_window::<I>);
        object.is_window_modal_dialog = Some(is_window_modal_dialog::<I>);
        object.get_initial_bounds = Some(get_initial_bounds::<I>);
        object.get_initial_show_state = Some(get_initial_show_state::<I>);
        object.is_frameless = Some(is_frameless::<I>);
        object.with_standard_window_buttons = Some(with_standard_window_buttons::<I>);
        object.get_titlebar_height = Some(get_titlebar_height::<I>);
        object.accepts_first_mouse = Some(accepts_first_mouse::<I>);
        object.can_resize = Some(can_resize::<I>);
        object.can_maximize = Some(can_maximize::<I>);
        object.can_minimize = Some(can_minimize::<I>);
        object.can_close = Some(can_close::<I>);
        object.on_accelerator = Some(on_accelerator::<I>);
        object.on_key_event = Some(on_key_event::<I>);
        object.on_theme_colors_changed = Some(on_theme_colors_changed::<I>);
        object.get_window_runtime_style = Some(get_window_runtime_style::<I>);
        object.get_linux_window_properties = Some(get_linux_window_properties::<I>);
    }

    extern "C" fn on_window_created<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_window_created(window)
    }

    extern "C" fn on_window_closing<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_window_closing(window)
    }

    extern "C" fn on_window_destroyed<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_window_destroyed(window)
    }

    extern "C" fn on_window_activation_changed<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        active: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_window_activation_changed(window, active)
    }

    extern "C" fn on_window_bounds_changed<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        new_bounds: *const cef_rect_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        let new_bounds = &Rect(unsafe { RefGuard::from_raw_add_ref(new_bounds) });
        obj.interface.on_window_bounds_changed(window, new_bounds)
    }

    extern "C" fn on_window_fullscreen_transition<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        is_completed: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface
            .on_window_fullscreen_transition(window, is_completed)
    }

    extern "C" fn get_parent_window<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        is_menu: *mut ::std::os::raw::c_int,
        can_activate_menu: *mut ::std::os::raw::c_int,
    ) -> *mut _cef_window_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface
            .get_parent_window(window, is_menu, can_activate_menu)
            .into()
    }

    extern "C" fn is_window_modal_dialog<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.is_window_modal_dialog(window).into()
    }

    extern "C" fn get_initial_bounds<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.get_initial_bounds(window).into()
    }

    extern "C" fn get_initial_show_state<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> cef_show_state_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.get_initial_show_state(window).into()
    }

    extern "C" fn is_frameless<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.is_frameless(window).into()
    }

    extern "C" fn with_standard_window_buttons<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.with_standard_window_buttons(window).into()
    }

    extern "C" fn get_titlebar_height<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        titlebar_height: *mut f32,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface
            .get_titlebar_height(window, titlebar_height)
            .into()
    }

    extern "C" fn accepts_first_mouse<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> cef_state_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.accepts_first_mouse(window).into()
    }

    extern "C" fn can_resize<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.can_resize(window).into()
    }

    extern "C" fn can_maximize<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.can_maximize(window).into()
    }

    extern "C" fn can_minimize<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.can_minimize(window).into()
    }

    extern "C" fn can_close<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.can_close(window).into()
    }

    extern "C" fn on_accelerator<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        command_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_accelerator(window, command_id).into()
    }

    extern "C" fn on_key_event<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        event: *const cef_key_event_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        let event = &KeyEvent(unsafe { RefGuard::from_raw_add_ref(event) });
        obj.interface.on_key_event(window, event).into()
    }

    extern "C" fn on_theme_colors_changed<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        chrome_theme: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        obj.interface.on_theme_colors_changed(window, chrome_theme)
    }

    extern "C" fn get_window_runtime_style<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
    ) -> cef_runtime_style_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window_runtime_style().into()
    }

    extern "C" fn get_linux_window_properties<I: ImplWindowDelegate>(
        self_: *mut _cef_window_delegate_t,
        window: *mut _cef_window_t,
        properties: *mut _cef_linux_window_properties_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let window = &mut Window(unsafe { RefGuard::from_raw_add_ref(window) });
        let properties =
            &mut LinuxWindowProperties(unsafe { RefGuard::from_raw_add_ref(properties) });
        obj.interface
            .get_linux_window_properties(window, properties)
            .into()
    }
}

wrapper!(
    #[doc = "See [_cef_window_t] for more documentation."]
    #[derive(Clone)]
    pub struct Window(_cef_window_t);

    pub fn show(&self);
    pub fn show_as_browser_modal_dialog(&self, browser_view: &mut BrowserView);
    pub fn hide(&self);
    pub fn center_window(&self, size: &Size);
    pub fn close(&self);
    pub fn is_closed(&self) -> ::std::os::raw::c_int;
    pub fn activate(&self);
    pub fn deactivate(&self);
    pub fn is_active(&self) -> ::std::os::raw::c_int;
    pub fn bring_to_top(&self);
    pub fn set_always_on_top(&self, on_top: ::std::os::raw::c_int);
    pub fn is_always_on_top(&self) -> ::std::os::raw::c_int;
    pub fn maximize(&self);
    pub fn minimize(&self);
    pub fn restore(&self);
    pub fn set_fullscreen(&self, fullscreen: ::std::os::raw::c_int);
    pub fn is_maximized(&self) -> ::std::os::raw::c_int;
    pub fn is_minimized(&self) -> ::std::os::raw::c_int;
    pub fn is_fullscreen(&self) -> ::std::os::raw::c_int;
    pub fn get_focused_view(&self) -> View;
    pub fn set_title(&self, title: &CefString);
    pub fn get_title(&self) -> CefStringUserfree;
    pub fn set_window_icon(&self, image: &mut Image);
    pub fn get_window_icon(&self) -> Image;
    pub fn set_window_app_icon(&self, image: &mut Image);
    pub fn get_window_app_icon(&self) -> Image;
    pub fn add_overlay_view(
        &self,
        view: &mut View,
        docking_mode: DockingMode,
        can_activate: ::std::os::raw::c_int,
    ) -> OverlayController;
    pub fn show_menu(
        &self,
        menu_model: &mut MenuModel,
        screen_point: &Point,
        anchor_position: MenuAnchorPosition,
    );
    pub fn cancel_menu(&self);
    pub fn get_display(&self) -> Display;
    pub fn get_client_area_bounds_in_screen(&self) -> Rect;
    pub fn set_draggable_regions(&self, regions_count: usize, regions: &DraggableRegion);
    pub fn get_window_handle(&self) -> ::std::os::raw::c_ulong;
    pub fn send_key_press(&self, key_code: ::std::os::raw::c_int, event_flags: u32);
    pub fn send_mouse_move(&self, screen_x: ::std::os::raw::c_int, screen_y: ::std::os::raw::c_int);
    pub fn send_mouse_events(
        &self,
        button: MouseButtonType,
        mouse_down: ::std::os::raw::c_int,
        mouse_up: ::std::os::raw::c_int,
    );
    pub fn set_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
        high_priority: ::std::os::raw::c_int,
    );
    pub fn remove_accelerator(&self, command_id: ::std::os::raw::c_int);
    pub fn remove_all_accelerators(&self);
    pub fn set_theme_color(&self, color_id: ::std::os::raw::c_int, color: &mut Color);
    pub fn theme_changed(&self);
    pub fn get_runtime_style(&self) -> RuntimeStyle;
);

pub trait ImplWindow: ImplPanel {
    fn show(&self) {}
    fn show_as_browser_modal_dialog(&self, browser_view: &mut BrowserView) {}
    fn hide(&self) {}
    fn center_window(&self, size: &Size) {}
    fn close(&self) {}
    fn is_closed(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn activate(&self) {}
    fn deactivate(&self) {}
    fn is_active(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn bring_to_top(&self) {}
    fn set_always_on_top(&self, on_top: ::std::os::raw::c_int) {}
    fn is_always_on_top(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn maximize(&self) {}
    fn minimize(&self) {}
    fn restore(&self) {}
    fn set_fullscreen(&self, fullscreen: ::std::os::raw::c_int) {}
    fn is_maximized(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_minimized(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn is_fullscreen(&self) -> ::std::os::raw::c_int {
        Default::default()
    }
    fn get_focused_view(&self) -> View {
        Default::default()
    }
    fn set_title(&self, title: &CefString) {}
    fn get_title(&self) -> CefStringUserfree {
        Default::default()
    }
    fn set_window_icon(&self, image: &mut Image) {}
    fn get_window_icon(&self) -> Image {
        Default::default()
    }
    fn set_window_app_icon(&self, image: &mut Image) {}
    fn get_window_app_icon(&self) -> Image {
        Default::default()
    }
    fn add_overlay_view(
        &self,
        view: &mut View,
        docking_mode: DockingMode,
        can_activate: ::std::os::raw::c_int,
    ) -> OverlayController {
        Default::default()
    }
    fn show_menu(
        &self,
        menu_model: &mut MenuModel,
        screen_point: &Point,
        anchor_position: MenuAnchorPosition,
    ) {
    }
    fn cancel_menu(&self) {}
    fn get_display(&self) -> Display {
        Default::default()
    }
    fn get_client_area_bounds_in_screen(&self) -> Rect {
        Default::default()
    }
    fn set_draggable_regions(&self, regions_count: usize, regions: &DraggableRegion) {}
    fn get_window_handle(&self) -> ::std::os::raw::c_ulong {
        Default::default()
    }
    fn send_key_press(&self, key_code: ::std::os::raw::c_int, event_flags: u32) {}
    fn send_mouse_move(&self, screen_x: ::std::os::raw::c_int, screen_y: ::std::os::raw::c_int) {}
    fn send_mouse_events(
        &self,
        button: MouseButtonType,
        mouse_down: ::std::os::raw::c_int,
        mouse_up: ::std::os::raw::c_int,
    ) {
    }
    fn set_accelerator(
        &self,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
        high_priority: ::std::os::raw::c_int,
    ) {
    }
    fn remove_accelerator(&self, command_id: ::std::os::raw::c_int) {}
    fn remove_all_accelerators(&self) {}
    fn set_theme_color(&self, color_id: ::std::os::raw::c_int, color: &mut Color) {}
    fn theme_changed(&self) {}
    fn get_runtime_style(&self) -> RuntimeStyle {
        Default::default()
    }

    fn into_raw(self) -> *mut _cef_window_t {
        let mut object: _cef_window_t = unsafe { std::mem::zeroed() };
        impl_cef_view_t::init_methods::<Self>(&mut object.base.base);
        impl_cef_panel_t::init_methods::<Self>(&mut object.base);
        impl_cef_window_t::init_methods::<Self>(&mut object);
        RcImpl::new(object, self) as *mut _
    }
}

mod impl_cef_window_t {
    use super::*;

    pub fn init_methods<I: ImplWindow>(object: &mut _cef_window_t) {
        object.show = Some(show::<I>);
        object.show_as_browser_modal_dialog = Some(show_as_browser_modal_dialog::<I>);
        object.hide = Some(hide::<I>);
        object.center_window = Some(center_window::<I>);
        object.close = Some(close::<I>);
        object.is_closed = Some(is_closed::<I>);
        object.activate = Some(activate::<I>);
        object.deactivate = Some(deactivate::<I>);
        object.is_active = Some(is_active::<I>);
        object.bring_to_top = Some(bring_to_top::<I>);
        object.set_always_on_top = Some(set_always_on_top::<I>);
        object.is_always_on_top = Some(is_always_on_top::<I>);
        object.maximize = Some(maximize::<I>);
        object.minimize = Some(minimize::<I>);
        object.restore = Some(restore::<I>);
        object.set_fullscreen = Some(set_fullscreen::<I>);
        object.is_maximized = Some(is_maximized::<I>);
        object.is_minimized = Some(is_minimized::<I>);
        object.is_fullscreen = Some(is_fullscreen::<I>);
        object.get_focused_view = Some(get_focused_view::<I>);
        object.set_title = Some(set_title::<I>);
        object.get_title = Some(get_title::<I>);
        object.set_window_icon = Some(set_window_icon::<I>);
        object.get_window_icon = Some(get_window_icon::<I>);
        object.set_window_app_icon = Some(set_window_app_icon::<I>);
        object.get_window_app_icon = Some(get_window_app_icon::<I>);
        object.add_overlay_view = Some(add_overlay_view::<I>);
        object.show_menu = Some(show_menu::<I>);
        object.cancel_menu = Some(cancel_menu::<I>);
        object.get_display = Some(get_display::<I>);
        object.get_client_area_bounds_in_screen = Some(get_client_area_bounds_in_screen::<I>);
        object.set_draggable_regions = Some(set_draggable_regions::<I>);
        object.get_window_handle = Some(get_window_handle::<I>);
        object.send_key_press = Some(send_key_press::<I>);
        object.send_mouse_move = Some(send_mouse_move::<I>);
        object.send_mouse_events = Some(send_mouse_events::<I>);
        object.set_accelerator = Some(set_accelerator::<I>);
        object.remove_accelerator = Some(remove_accelerator::<I>);
        object.remove_all_accelerators = Some(remove_all_accelerators::<I>);
        object.set_theme_color = Some(set_theme_color::<I>);
        object.theme_changed = Some(theme_changed::<I>);
        object.get_runtime_style = Some(get_runtime_style::<I>);
    }

    extern "C" fn show<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.show()
    }

    extern "C" fn show_as_browser_modal_dialog<I: ImplWindow>(
        self_: *mut _cef_window_t,
        browser_view: *mut _cef_browser_view_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let browser_view = &mut BrowserView(unsafe { RefGuard::from_raw_add_ref(browser_view) });
        obj.interface.show_as_browser_modal_dialog(browser_view)
    }

    extern "C" fn hide<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.hide()
    }

    extern "C" fn center_window<I: ImplWindow>(self_: *mut _cef_window_t, size: *const cef_size_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let size = &Size(unsafe { RefGuard::from_raw_add_ref(size) });
        obj.interface.center_window(size)
    }

    extern "C" fn close<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.close()
    }

    extern "C" fn is_closed<I: ImplWindow>(self_: *mut _cef_window_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_closed().into()
    }

    extern "C" fn activate<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.activate()
    }

    extern "C" fn deactivate<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.deactivate()
    }

    extern "C" fn is_active<I: ImplWindow>(self_: *mut _cef_window_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_active().into()
    }

    extern "C" fn bring_to_top<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.bring_to_top()
    }

    extern "C" fn set_always_on_top<I: ImplWindow>(
        self_: *mut _cef_window_t,
        on_top: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_always_on_top(on_top)
    }

    extern "C" fn is_always_on_top<I: ImplWindow>(
        self_: *mut _cef_window_t,
    ) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_always_on_top().into()
    }

    extern "C" fn maximize<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.maximize()
    }

    extern "C" fn minimize<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.minimize()
    }

    extern "C" fn restore<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.restore()
    }

    extern "C" fn set_fullscreen<I: ImplWindow>(
        self_: *mut _cef_window_t,
        fullscreen: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_fullscreen(fullscreen)
    }

    extern "C" fn is_maximized<I: ImplWindow>(self_: *mut _cef_window_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_maximized().into()
    }

    extern "C" fn is_minimized<I: ImplWindow>(self_: *mut _cef_window_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_minimized().into()
    }

    extern "C" fn is_fullscreen<I: ImplWindow>(self_: *mut _cef_window_t) -> ::std::os::raw::c_int {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.is_fullscreen().into()
    }

    extern "C" fn get_focused_view<I: ImplWindow>(self_: *mut _cef_window_t) -> *mut _cef_view_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_focused_view().into()
    }

    extern "C" fn set_title<I: ImplWindow>(self_: *mut _cef_window_t, title: *const cef_string_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let title = &CefString::from(title);
        obj.interface.set_title(title)
    }

    extern "C" fn get_title<I: ImplWindow>(self_: *mut _cef_window_t) -> cef_string_userfree_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_title().into()
    }

    extern "C" fn set_window_icon<I: ImplWindow>(
        self_: *mut _cef_window_t,
        image: *mut _cef_image_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let image = &mut Image(unsafe { RefGuard::from_raw_add_ref(image) });
        obj.interface.set_window_icon(image)
    }

    extern "C" fn get_window_icon<I: ImplWindow>(self_: *mut _cef_window_t) -> *mut _cef_image_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window_icon().into()
    }

    extern "C" fn set_window_app_icon<I: ImplWindow>(
        self_: *mut _cef_window_t,
        image: *mut _cef_image_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let image = &mut Image(unsafe { RefGuard::from_raw_add_ref(image) });
        obj.interface.set_window_app_icon(image)
    }

    extern "C" fn get_window_app_icon<I: ImplWindow>(
        self_: *mut _cef_window_t,
    ) -> *mut _cef_image_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window_app_icon().into()
    }

    extern "C" fn add_overlay_view<I: ImplWindow>(
        self_: *mut _cef_window_t,
        view: *mut _cef_view_t,
        docking_mode: cef_docking_mode_t,
        can_activate: ::std::os::raw::c_int,
    ) -> *mut _cef_overlay_controller_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let view = &mut View(unsafe { RefGuard::from_raw_add_ref(view) });
        obj.interface
            .add_overlay_view(view, docking_mode, can_activate)
            .into()
    }

    extern "C" fn show_menu<I: ImplWindow>(
        self_: *mut _cef_window_t,
        menu_model: *mut _cef_menu_model_t,
        screen_point: *const cef_point_t,
        anchor_position: cef_menu_anchor_position_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let menu_model = &mut MenuModel(unsafe { RefGuard::from_raw_add_ref(menu_model) });
        let screen_point = &Point(unsafe { RefGuard::from_raw_add_ref(screen_point) });
        obj.interface
            .show_menu(menu_model, screen_point, anchor_position)
    }

    extern "C" fn cancel_menu<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.cancel_menu()
    }

    extern "C" fn get_display<I: ImplWindow>(self_: *mut _cef_window_t) -> *mut _cef_display_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_display().into()
    }

    extern "C" fn get_client_area_bounds_in_screen<I: ImplWindow>(
        self_: *mut _cef_window_t,
    ) -> cef_rect_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_client_area_bounds_in_screen().into()
    }

    extern "C" fn set_draggable_regions<I: ImplWindow>(
        self_: *mut _cef_window_t,
        regions_count: usize,
        regions: *const cef_draggable_region_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        let regions = &DraggableRegion(unsafe { RefGuard::from_raw_add_ref(regions) });
        obj.interface.set_draggable_regions(regions_count, regions)
    }

    extern "C" fn get_window_handle<I: ImplWindow>(
        self_: *mut _cef_window_t,
    ) -> ::std::os::raw::c_ulong {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_window_handle().into()
    }

    extern "C" fn send_key_press<I: ImplWindow>(
        self_: *mut _cef_window_t,
        key_code: ::std::os::raw::c_int,
        event_flags: u32,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.send_key_press(key_code, event_flags)
    }

    extern "C" fn send_mouse_move<I: ImplWindow>(
        self_: *mut _cef_window_t,
        screen_x: ::std::os::raw::c_int,
        screen_y: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.send_mouse_move(screen_x, screen_y)
    }

    extern "C" fn send_mouse_events<I: ImplWindow>(
        self_: *mut _cef_window_t,
        button: cef_mouse_button_type_t,
        mouse_down: ::std::os::raw::c_int,
        mouse_up: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface
            .send_mouse_events(button, mouse_down, mouse_up)
    }

    extern "C" fn set_accelerator<I: ImplWindow>(
        self_: *mut _cef_window_t,
        command_id: ::std::os::raw::c_int,
        key_code: ::std::os::raw::c_int,
        shift_pressed: ::std::os::raw::c_int,
        ctrl_pressed: ::std::os::raw::c_int,
        alt_pressed: ::std::os::raw::c_int,
        high_priority: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_accelerator(
            command_id,
            key_code,
            shift_pressed,
            ctrl_pressed,
            alt_pressed,
            high_priority,
        )
    }

    extern "C" fn remove_accelerator<I: ImplWindow>(
        self_: *mut _cef_window_t,
        command_id: ::std::os::raw::c_int,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_accelerator(command_id)
    }

    extern "C" fn remove_all_accelerators<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.remove_all_accelerators()
    }

    extern "C" fn set_theme_color<I: ImplWindow>(
        self_: *mut _cef_window_t,
        color_id: ::std::os::raw::c_int,
        color: cef_color_t,
    ) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.set_theme_color(color_id, color)
    }

    extern "C" fn theme_changed<I: ImplWindow>(self_: *mut _cef_window_t) {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.theme_changed()
    }

    extern "C" fn get_runtime_style<I: ImplWindow>(
        self_: *mut _cef_window_t,
    ) -> cef_runtime_style_t {
        let obj: &RcImpl<_, I> = RcImpl::get(self_);
        obj.interface.get_runtime_style().into()
    }
}

// Enum aliases

/// See [cef_content_setting_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContentSettingTypes(cef_content_setting_types_t);

impl AsRef<cef_content_setting_types_t> for ContentSettingTypes {
    fn as_ref(&self) -> &cef_content_setting_types_t {
        &self.0
    }
}

impl AsMut<cef_content_setting_types_t> for ContentSettingTypes {
    fn as_mut(&mut self) -> &mut cef_content_setting_types_t {
        &mut self.0
    }
}

impl From<cef_content_setting_types_t> for ContentSettingTypes {
    fn from(value: cef_content_setting_types_t) -> Self {
        Self(value)
    }
}

impl Into<cef_content_setting_types_t> for ContentSettingTypes {
    fn into(self) -> cef_content_setting_types_t {
        self.0
    }
}

impl Default for ContentSettingTypes {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_content_setting_values_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContentSettingValues(cef_content_setting_values_t);

impl AsRef<cef_content_setting_values_t> for ContentSettingValues {
    fn as_ref(&self) -> &cef_content_setting_values_t {
        &self.0
    }
}

impl AsMut<cef_content_setting_values_t> for ContentSettingValues {
    fn as_mut(&mut self) -> &mut cef_content_setting_values_t {
        &mut self.0
    }
}

impl From<cef_content_setting_values_t> for ContentSettingValues {
    fn from(value: cef_content_setting_values_t) -> Self {
        Self(value)
    }
}

impl Into<cef_content_setting_values_t> for ContentSettingValues {
    fn into(self) -> cef_content_setting_values_t {
        self.0
    }
}

impl Default for ContentSettingValues {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_color_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorType(cef_color_type_t);

impl AsRef<cef_color_type_t> for ColorType {
    fn as_ref(&self) -> &cef_color_type_t {
        &self.0
    }
}

impl AsMut<cef_color_type_t> for ColorType {
    fn as_mut(&mut self) -> &mut cef_color_type_t {
        &mut self.0
    }
}

impl From<cef_color_type_t> for ColorType {
    fn from(value: cef_color_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_color_type_t> for ColorType {
    fn into(self) -> cef_color_type_t {
        self.0
    }
}

impl Default for ColorType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_runtime_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RuntimeStyle(cef_runtime_style_t);

impl AsRef<cef_runtime_style_t> for RuntimeStyle {
    fn as_ref(&self) -> &cef_runtime_style_t {
        &self.0
    }
}

impl AsMut<cef_runtime_style_t> for RuntimeStyle {
    fn as_mut(&mut self) -> &mut cef_runtime_style_t {
        &mut self.0
    }
}

impl From<cef_runtime_style_t> for RuntimeStyle {
    fn from(value: cef_runtime_style_t) -> Self {
        Self(value)
    }
}

impl Into<cef_runtime_style_t> for RuntimeStyle {
    fn into(self) -> cef_runtime_style_t {
        self.0
    }
}

impl Default for RuntimeStyle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_log_severity_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogSeverity(cef_log_severity_t);

impl AsRef<cef_log_severity_t> for LogSeverity {
    fn as_ref(&self) -> &cef_log_severity_t {
        &self.0
    }
}

impl AsMut<cef_log_severity_t> for LogSeverity {
    fn as_mut(&mut self) -> &mut cef_log_severity_t {
        &mut self.0
    }
}

impl From<cef_log_severity_t> for LogSeverity {
    fn from(value: cef_log_severity_t) -> Self {
        Self(value)
    }
}

impl Into<cef_log_severity_t> for LogSeverity {
    fn into(self) -> cef_log_severity_t {
        self.0
    }
}

impl Default for LogSeverity {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_log_items_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogItems(cef_log_items_t);

impl AsRef<cef_log_items_t> for LogItems {
    fn as_ref(&self) -> &cef_log_items_t {
        &self.0
    }
}

impl AsMut<cef_log_items_t> for LogItems {
    fn as_mut(&mut self) -> &mut cef_log_items_t {
        &mut self.0
    }
}

impl From<cef_log_items_t> for LogItems {
    fn from(value: cef_log_items_t) -> Self {
        Self(value)
    }
}

impl Into<cef_log_items_t> for LogItems {
    fn into(self) -> cef_log_items_t {
        self.0
    }
}

impl Default for LogItems {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct State(cef_state_t);

impl AsRef<cef_state_t> for State {
    fn as_ref(&self) -> &cef_state_t {
        &self.0
    }
}

impl AsMut<cef_state_t> for State {
    fn as_mut(&mut self) -> &mut cef_state_t {
        &mut self.0
    }
}

impl From<cef_state_t> for State {
    fn from(value: cef_state_t) -> Self {
        Self(value)
    }
}

impl Into<cef_state_t> for State {
    fn into(self) -> cef_state_t {
        self.0
    }
}

impl Default for State {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_return_value_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ReturnValue(cef_return_value_t);

impl AsRef<cef_return_value_t> for ReturnValue {
    fn as_ref(&self) -> &cef_return_value_t {
        &self.0
    }
}

impl AsMut<cef_return_value_t> for ReturnValue {
    fn as_mut(&mut self) -> &mut cef_return_value_t {
        &mut self.0
    }
}

impl From<cef_return_value_t> for ReturnValue {
    fn from(value: cef_return_value_t) -> Self {
        Self(value)
    }
}

impl Into<cef_return_value_t> for ReturnValue {
    fn into(self) -> cef_return_value_t {
        self.0
    }
}

impl Default for ReturnValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_cookie_priority_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CookiePriority(cef_cookie_priority_t);

impl AsRef<cef_cookie_priority_t> for CookiePriority {
    fn as_ref(&self) -> &cef_cookie_priority_t {
        &self.0
    }
}

impl AsMut<cef_cookie_priority_t> for CookiePriority {
    fn as_mut(&mut self) -> &mut cef_cookie_priority_t {
        &mut self.0
    }
}

impl From<cef_cookie_priority_t> for CookiePriority {
    fn from(value: cef_cookie_priority_t) -> Self {
        Self(value)
    }
}

impl Into<cef_cookie_priority_t> for CookiePriority {
    fn into(self) -> cef_cookie_priority_t {
        self.0
    }
}

impl Default for CookiePriority {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_cookie_same_site_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CookieSameSite(cef_cookie_same_site_t);

impl AsRef<cef_cookie_same_site_t> for CookieSameSite {
    fn as_ref(&self) -> &cef_cookie_same_site_t {
        &self.0
    }
}

impl AsMut<cef_cookie_same_site_t> for CookieSameSite {
    fn as_mut(&mut self) -> &mut cef_cookie_same_site_t {
        &mut self.0
    }
}

impl From<cef_cookie_same_site_t> for CookieSameSite {
    fn from(value: cef_cookie_same_site_t) -> Self {
        Self(value)
    }
}

impl Into<cef_cookie_same_site_t> for CookieSameSite {
    fn into(self) -> cef_cookie_same_site_t {
        self.0
    }
}

impl Default for CookieSameSite {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_termination_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TerminationStatus(cef_termination_status_t);

impl AsRef<cef_termination_status_t> for TerminationStatus {
    fn as_ref(&self) -> &cef_termination_status_t {
        &self.0
    }
}

impl AsMut<cef_termination_status_t> for TerminationStatus {
    fn as_mut(&mut self) -> &mut cef_termination_status_t {
        &mut self.0
    }
}

impl From<cef_termination_status_t> for TerminationStatus {
    fn from(value: cef_termination_status_t) -> Self {
        Self(value)
    }
}

impl Into<cef_termination_status_t> for TerminationStatus {
    fn into(self) -> cef_termination_status_t {
        self.0
    }
}

impl Default for TerminationStatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_path_key_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PathKey(cef_path_key_t);

impl AsRef<cef_path_key_t> for PathKey {
    fn as_ref(&self) -> &cef_path_key_t {
        &self.0
    }
}

impl AsMut<cef_path_key_t> for PathKey {
    fn as_mut(&mut self) -> &mut cef_path_key_t {
        &mut self.0
    }
}

impl From<cef_path_key_t> for PathKey {
    fn from(value: cef_path_key_t) -> Self {
        Self(value)
    }
}

impl Into<cef_path_key_t> for PathKey {
    fn into(self) -> cef_path_key_t {
        self.0
    }
}

impl Default for PathKey {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_storage_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct StorageType(cef_storage_type_t);

impl AsRef<cef_storage_type_t> for StorageType {
    fn as_ref(&self) -> &cef_storage_type_t {
        &self.0
    }
}

impl AsMut<cef_storage_type_t> for StorageType {
    fn as_mut(&mut self) -> &mut cef_storage_type_t {
        &mut self.0
    }
}

impl From<cef_storage_type_t> for StorageType {
    fn from(value: cef_storage_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_storage_type_t> for StorageType {
    fn into(self) -> cef_storage_type_t {
        self.0
    }
}

impl Default for StorageType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_errorcode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Errorcode(cef_errorcode_t);

impl AsRef<cef_errorcode_t> for Errorcode {
    fn as_ref(&self) -> &cef_errorcode_t {
        &self.0
    }
}

impl AsMut<cef_errorcode_t> for Errorcode {
    fn as_mut(&mut self) -> &mut cef_errorcode_t {
        &mut self.0
    }
}

impl From<cef_errorcode_t> for Errorcode {
    fn from(value: cef_errorcode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_errorcode_t> for Errorcode {
    fn into(self) -> cef_errorcode_t {
        self.0
    }
}

impl Default for Errorcode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_cert_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CertStatus(cef_cert_status_t);

impl AsRef<cef_cert_status_t> for CertStatus {
    fn as_ref(&self) -> &cef_cert_status_t {
        &self.0
    }
}

impl AsMut<cef_cert_status_t> for CertStatus {
    fn as_mut(&mut self) -> &mut cef_cert_status_t {
        &mut self.0
    }
}

impl From<cef_cert_status_t> for CertStatus {
    fn from(value: cef_cert_status_t) -> Self {
        Self(value)
    }
}

impl Into<cef_cert_status_t> for CertStatus {
    fn into(self) -> cef_cert_status_t {
        self.0
    }
}

impl Default for CertStatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_resultcode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Resultcode(cef_resultcode_t);

impl AsRef<cef_resultcode_t> for Resultcode {
    fn as_ref(&self) -> &cef_resultcode_t {
        &self.0
    }
}

impl AsMut<cef_resultcode_t> for Resultcode {
    fn as_mut(&mut self) -> &mut cef_resultcode_t {
        &mut self.0
    }
}

impl From<cef_resultcode_t> for Resultcode {
    fn from(value: cef_resultcode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_resultcode_t> for Resultcode {
    fn into(self) -> cef_resultcode_t {
        self.0
    }
}

impl Default for Resultcode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_window_open_disposition_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WindowOpenDisposition(cef_window_open_disposition_t);

impl AsRef<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn as_ref(&self) -> &cef_window_open_disposition_t {
        &self.0
    }
}

impl AsMut<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn as_mut(&mut self) -> &mut cef_window_open_disposition_t {
        &mut self.0
    }
}

impl From<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn from(value: cef_window_open_disposition_t) -> Self {
        Self(value)
    }
}

impl Into<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn into(self) -> cef_window_open_disposition_t {
        self.0
    }
}

impl Default for WindowOpenDisposition {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_drag_operations_mask_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DragOperationsMask(cef_drag_operations_mask_t);

impl AsRef<cef_drag_operations_mask_t> for DragOperationsMask {
    fn as_ref(&self) -> &cef_drag_operations_mask_t {
        &self.0
    }
}

impl AsMut<cef_drag_operations_mask_t> for DragOperationsMask {
    fn as_mut(&mut self) -> &mut cef_drag_operations_mask_t {
        &mut self.0
    }
}

impl From<cef_drag_operations_mask_t> for DragOperationsMask {
    fn from(value: cef_drag_operations_mask_t) -> Self {
        Self(value)
    }
}

impl Into<cef_drag_operations_mask_t> for DragOperationsMask {
    fn into(self) -> cef_drag_operations_mask_t {
        self.0
    }
}

impl Default for DragOperationsMask {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_text_input_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextInputMode(cef_text_input_mode_t);

impl AsRef<cef_text_input_mode_t> for TextInputMode {
    fn as_ref(&self) -> &cef_text_input_mode_t {
        &self.0
    }
}

impl AsMut<cef_text_input_mode_t> for TextInputMode {
    fn as_mut(&mut self) -> &mut cef_text_input_mode_t {
        &mut self.0
    }
}

impl From<cef_text_input_mode_t> for TextInputMode {
    fn from(value: cef_text_input_mode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_text_input_mode_t> for TextInputMode {
    fn into(self) -> cef_text_input_mode_t {
        self.0
    }
}

impl Default for TextInputMode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_v8_propertyattribute_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct V8Propertyattribute(cef_v8_propertyattribute_t);

impl AsRef<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn as_ref(&self) -> &cef_v8_propertyattribute_t {
        &self.0
    }
}

impl AsMut<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn as_mut(&mut self) -> &mut cef_v8_propertyattribute_t {
        &mut self.0
    }
}

impl From<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn from(value: cef_v8_propertyattribute_t) -> Self {
        Self(value)
    }
}

impl Into<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn into(self) -> cef_v8_propertyattribute_t {
        self.0
    }
}

impl Default for V8Propertyattribute {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_postdataelement_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PostdataelementType(cef_postdataelement_type_t);

impl AsRef<cef_postdataelement_type_t> for PostdataelementType {
    fn as_ref(&self) -> &cef_postdataelement_type_t {
        &self.0
    }
}

impl AsMut<cef_postdataelement_type_t> for PostdataelementType {
    fn as_mut(&mut self) -> &mut cef_postdataelement_type_t {
        &mut self.0
    }
}

impl From<cef_postdataelement_type_t> for PostdataelementType {
    fn from(value: cef_postdataelement_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_postdataelement_type_t> for PostdataelementType {
    fn into(self) -> cef_postdataelement_type_t {
        self.0
    }
}

impl Default for PostdataelementType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_resource_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResourceType(cef_resource_type_t);

impl AsRef<cef_resource_type_t> for ResourceType {
    fn as_ref(&self) -> &cef_resource_type_t {
        &self.0
    }
}

impl AsMut<cef_resource_type_t> for ResourceType {
    fn as_mut(&mut self) -> &mut cef_resource_type_t {
        &mut self.0
    }
}

impl From<cef_resource_type_t> for ResourceType {
    fn from(value: cef_resource_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_resource_type_t> for ResourceType {
    fn into(self) -> cef_resource_type_t {
        self.0
    }
}

impl Default for ResourceType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_transition_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TransitionType(cef_transition_type_t);

impl AsRef<cef_transition_type_t> for TransitionType {
    fn as_ref(&self) -> &cef_transition_type_t {
        &self.0
    }
}

impl AsMut<cef_transition_type_t> for TransitionType {
    fn as_mut(&mut self) -> &mut cef_transition_type_t {
        &mut self.0
    }
}

impl From<cef_transition_type_t> for TransitionType {
    fn from(value: cef_transition_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_transition_type_t> for TransitionType {
    fn into(self) -> cef_transition_type_t {
        self.0
    }
}

impl Default for TransitionType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_urlrequest_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UrlrequestFlags(cef_urlrequest_flags_t);

impl AsRef<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn as_ref(&self) -> &cef_urlrequest_flags_t {
        &self.0
    }
}

impl AsMut<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn as_mut(&mut self) -> &mut cef_urlrequest_flags_t {
        &mut self.0
    }
}

impl From<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn from(value: cef_urlrequest_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn into(self) -> cef_urlrequest_flags_t {
        self.0
    }
}

impl Default for UrlrequestFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_urlrequest_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UrlrequestStatus(cef_urlrequest_status_t);

impl AsRef<cef_urlrequest_status_t> for UrlrequestStatus {
    fn as_ref(&self) -> &cef_urlrequest_status_t {
        &self.0
    }
}

impl AsMut<cef_urlrequest_status_t> for UrlrequestStatus {
    fn as_mut(&mut self) -> &mut cef_urlrequest_status_t {
        &mut self.0
    }
}

impl From<cef_urlrequest_status_t> for UrlrequestStatus {
    fn from(value: cef_urlrequest_status_t) -> Self {
        Self(value)
    }
}

impl Into<cef_urlrequest_status_t> for UrlrequestStatus {
    fn into(self) -> cef_urlrequest_status_t {
        self.0
    }
}

impl Default for UrlrequestStatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_process_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ProcessId(cef_process_id_t);

impl AsRef<cef_process_id_t> for ProcessId {
    fn as_ref(&self) -> &cef_process_id_t {
        &self.0
    }
}

impl AsMut<cef_process_id_t> for ProcessId {
    fn as_mut(&mut self) -> &mut cef_process_id_t {
        &mut self.0
    }
}

impl From<cef_process_id_t> for ProcessId {
    fn from(value: cef_process_id_t) -> Self {
        Self(value)
    }
}

impl Into<cef_process_id_t> for ProcessId {
    fn into(self) -> cef_process_id_t {
        self.0
    }
}

impl Default for ProcessId {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_thread_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ThreadId(cef_thread_id_t);

impl AsRef<cef_thread_id_t> for ThreadId {
    fn as_ref(&self) -> &cef_thread_id_t {
        &self.0
    }
}

impl AsMut<cef_thread_id_t> for ThreadId {
    fn as_mut(&mut self) -> &mut cef_thread_id_t {
        &mut self.0
    }
}

impl From<cef_thread_id_t> for ThreadId {
    fn from(value: cef_thread_id_t) -> Self {
        Self(value)
    }
}

impl Into<cef_thread_id_t> for ThreadId {
    fn into(self) -> cef_thread_id_t {
        self.0
    }
}

impl Default for ThreadId {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_thread_priority_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ThreadPriority(cef_thread_priority_t);

impl AsRef<cef_thread_priority_t> for ThreadPriority {
    fn as_ref(&self) -> &cef_thread_priority_t {
        &self.0
    }
}

impl AsMut<cef_thread_priority_t> for ThreadPriority {
    fn as_mut(&mut self) -> &mut cef_thread_priority_t {
        &mut self.0
    }
}

impl From<cef_thread_priority_t> for ThreadPriority {
    fn from(value: cef_thread_priority_t) -> Self {
        Self(value)
    }
}

impl Into<cef_thread_priority_t> for ThreadPriority {
    fn into(self) -> cef_thread_priority_t {
        self.0
    }
}

impl Default for ThreadPriority {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_message_loop_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageLoopType(cef_message_loop_type_t);

impl AsRef<cef_message_loop_type_t> for MessageLoopType {
    fn as_ref(&self) -> &cef_message_loop_type_t {
        &self.0
    }
}

impl AsMut<cef_message_loop_type_t> for MessageLoopType {
    fn as_mut(&mut self) -> &mut cef_message_loop_type_t {
        &mut self.0
    }
}

impl From<cef_message_loop_type_t> for MessageLoopType {
    fn from(value: cef_message_loop_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_message_loop_type_t> for MessageLoopType {
    fn into(self) -> cef_message_loop_type_t {
        self.0
    }
}

impl Default for MessageLoopType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_com_init_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ComInitMode(cef_com_init_mode_t);

impl AsRef<cef_com_init_mode_t> for ComInitMode {
    fn as_ref(&self) -> &cef_com_init_mode_t {
        &self.0
    }
}

impl AsMut<cef_com_init_mode_t> for ComInitMode {
    fn as_mut(&mut self) -> &mut cef_com_init_mode_t {
        &mut self.0
    }
}

impl From<cef_com_init_mode_t> for ComInitMode {
    fn from(value: cef_com_init_mode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_com_init_mode_t> for ComInitMode {
    fn into(self) -> cef_com_init_mode_t {
        self.0
    }
}

impl Default for ComInitMode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_value_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ValueType(cef_value_type_t);

impl AsRef<cef_value_type_t> for ValueType {
    fn as_ref(&self) -> &cef_value_type_t {
        &self.0
    }
}

impl AsMut<cef_value_type_t> for ValueType {
    fn as_mut(&mut self) -> &mut cef_value_type_t {
        &mut self.0
    }
}

impl From<cef_value_type_t> for ValueType {
    fn from(value: cef_value_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_value_type_t> for ValueType {
    fn into(self) -> cef_value_type_t {
        self.0
    }
}

impl Default for ValueType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_jsdialog_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsdialogType(cef_jsdialog_type_t);

impl AsRef<cef_jsdialog_type_t> for JsdialogType {
    fn as_ref(&self) -> &cef_jsdialog_type_t {
        &self.0
    }
}

impl AsMut<cef_jsdialog_type_t> for JsdialogType {
    fn as_mut(&mut self) -> &mut cef_jsdialog_type_t {
        &mut self.0
    }
}

impl From<cef_jsdialog_type_t> for JsdialogType {
    fn from(value: cef_jsdialog_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_jsdialog_type_t> for JsdialogType {
    fn into(self) -> cef_jsdialog_type_t {
        self.0
    }
}

impl Default for JsdialogType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_menu_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuId(cef_menu_id_t);

impl AsRef<cef_menu_id_t> for MenuId {
    fn as_ref(&self) -> &cef_menu_id_t {
        &self.0
    }
}

impl AsMut<cef_menu_id_t> for MenuId {
    fn as_mut(&mut self) -> &mut cef_menu_id_t {
        &mut self.0
    }
}

impl From<cef_menu_id_t> for MenuId {
    fn from(value: cef_menu_id_t) -> Self {
        Self(value)
    }
}

impl Into<cef_menu_id_t> for MenuId {
    fn into(self) -> cef_menu_id_t {
        self.0
    }
}

impl Default for MenuId {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_mouse_button_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MouseButtonType(cef_mouse_button_type_t);

impl AsRef<cef_mouse_button_type_t> for MouseButtonType {
    fn as_ref(&self) -> &cef_mouse_button_type_t {
        &self.0
    }
}

impl AsMut<cef_mouse_button_type_t> for MouseButtonType {
    fn as_mut(&mut self) -> &mut cef_mouse_button_type_t {
        &mut self.0
    }
}

impl From<cef_mouse_button_type_t> for MouseButtonType {
    fn from(value: cef_mouse_button_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_mouse_button_type_t> for MouseButtonType {
    fn into(self) -> cef_mouse_button_type_t {
        self.0
    }
}

impl Default for MouseButtonType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_touch_event_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TouchEventType(cef_touch_event_type_t);

impl AsRef<cef_touch_event_type_t> for TouchEventType {
    fn as_ref(&self) -> &cef_touch_event_type_t {
        &self.0
    }
}

impl AsMut<cef_touch_event_type_t> for TouchEventType {
    fn as_mut(&mut self) -> &mut cef_touch_event_type_t {
        &mut self.0
    }
}

impl From<cef_touch_event_type_t> for TouchEventType {
    fn from(value: cef_touch_event_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_touch_event_type_t> for TouchEventType {
    fn into(self) -> cef_touch_event_type_t {
        self.0
    }
}

impl Default for TouchEventType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_pointer_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PointerType(cef_pointer_type_t);

impl AsRef<cef_pointer_type_t> for PointerType {
    fn as_ref(&self) -> &cef_pointer_type_t {
        &self.0
    }
}

impl AsMut<cef_pointer_type_t> for PointerType {
    fn as_mut(&mut self) -> &mut cef_pointer_type_t {
        &mut self.0
    }
}

impl From<cef_pointer_type_t> for PointerType {
    fn from(value: cef_pointer_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_pointer_type_t> for PointerType {
    fn into(self) -> cef_pointer_type_t {
        self.0
    }
}

impl Default for PointerType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_paint_element_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PaintElementType(cef_paint_element_type_t);

impl AsRef<cef_paint_element_type_t> for PaintElementType {
    fn as_ref(&self) -> &cef_paint_element_type_t {
        &self.0
    }
}

impl AsMut<cef_paint_element_type_t> for PaintElementType {
    fn as_mut(&mut self) -> &mut cef_paint_element_type_t {
        &mut self.0
    }
}

impl From<cef_paint_element_type_t> for PaintElementType {
    fn from(value: cef_paint_element_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_paint_element_type_t> for PaintElementType {
    fn into(self) -> cef_paint_element_type_t {
        self.0
    }
}

impl Default for PaintElementType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_event_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct EventFlags(cef_event_flags_t);

impl AsRef<cef_event_flags_t> for EventFlags {
    fn as_ref(&self) -> &cef_event_flags_t {
        &self.0
    }
}

impl AsMut<cef_event_flags_t> for EventFlags {
    fn as_mut(&mut self) -> &mut cef_event_flags_t {
        &mut self.0
    }
}

impl From<cef_event_flags_t> for EventFlags {
    fn from(value: cef_event_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_event_flags_t> for EventFlags {
    fn into(self) -> cef_event_flags_t {
        self.0
    }
}

impl Default for EventFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_menu_item_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuItemType(cef_menu_item_type_t);

impl AsRef<cef_menu_item_type_t> for MenuItemType {
    fn as_ref(&self) -> &cef_menu_item_type_t {
        &self.0
    }
}

impl AsMut<cef_menu_item_type_t> for MenuItemType {
    fn as_mut(&mut self) -> &mut cef_menu_item_type_t {
        &mut self.0
    }
}

impl From<cef_menu_item_type_t> for MenuItemType {
    fn from(value: cef_menu_item_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_menu_item_type_t> for MenuItemType {
    fn into(self) -> cef_menu_item_type_t {
        self.0
    }
}

impl Default for MenuItemType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_context_menu_type_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuTypeFlags(cef_context_menu_type_flags_t);

impl AsRef<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn as_ref(&self) -> &cef_context_menu_type_flags_t {
        &self.0
    }
}

impl AsMut<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_type_flags_t {
        &mut self.0
    }
}

impl From<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn from(value: cef_context_menu_type_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn into(self) -> cef_context_menu_type_flags_t {
        self.0
    }
}

impl Default for ContextMenuTypeFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_context_menu_media_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuMediaType(cef_context_menu_media_type_t);

impl AsRef<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn as_ref(&self) -> &cef_context_menu_media_type_t {
        &self.0
    }
}

impl AsMut<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn as_mut(&mut self) -> &mut cef_context_menu_media_type_t {
        &mut self.0
    }
}

impl From<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn from(value: cef_context_menu_media_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn into(self) -> cef_context_menu_media_type_t {
        self.0
    }
}

impl Default for ContextMenuMediaType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_context_menu_media_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuMediaStateFlags(cef_context_menu_media_state_flags_t);

impl AsRef<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn as_ref(&self) -> &cef_context_menu_media_state_flags_t {
        &self.0
    }
}

impl AsMut<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_media_state_flags_t {
        &mut self.0
    }
}

impl From<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn from(value: cef_context_menu_media_state_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn into(self) -> cef_context_menu_media_state_flags_t {
        self.0
    }
}

impl Default for ContextMenuMediaStateFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_context_menu_edit_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuEditStateFlags(cef_context_menu_edit_state_flags_t);

impl AsRef<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn as_ref(&self) -> &cef_context_menu_edit_state_flags_t {
        &self.0
    }
}

impl AsMut<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_edit_state_flags_t {
        &mut self.0
    }
}

impl From<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn from(value: cef_context_menu_edit_state_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn into(self) -> cef_context_menu_edit_state_flags_t {
        self.0
    }
}

impl Default for ContextMenuEditStateFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_quick_menu_edit_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct QuickMenuEditStateFlags(cef_quick_menu_edit_state_flags_t);

impl AsRef<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn as_ref(&self) -> &cef_quick_menu_edit_state_flags_t {
        &self.0
    }
}

impl AsMut<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn as_mut(&mut self) -> &mut cef_quick_menu_edit_state_flags_t {
        &mut self.0
    }
}

impl From<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn from(value: cef_quick_menu_edit_state_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn into(self) -> cef_quick_menu_edit_state_flags_t {
        self.0
    }
}

impl Default for QuickMenuEditStateFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_key_event_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct KeyEventType(cef_key_event_type_t);

impl AsRef<cef_key_event_type_t> for KeyEventType {
    fn as_ref(&self) -> &cef_key_event_type_t {
        &self.0
    }
}

impl AsMut<cef_key_event_type_t> for KeyEventType {
    fn as_mut(&mut self) -> &mut cef_key_event_type_t {
        &mut self.0
    }
}

impl From<cef_key_event_type_t> for KeyEventType {
    fn from(value: cef_key_event_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_key_event_type_t> for KeyEventType {
    fn into(self) -> cef_key_event_type_t {
        self.0
    }
}

impl Default for KeyEventType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_focus_source_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FocusSource(cef_focus_source_t);

impl AsRef<cef_focus_source_t> for FocusSource {
    fn as_ref(&self) -> &cef_focus_source_t {
        &self.0
    }
}

impl AsMut<cef_focus_source_t> for FocusSource {
    fn as_mut(&mut self) -> &mut cef_focus_source_t {
        &mut self.0
    }
}

impl From<cef_focus_source_t> for FocusSource {
    fn from(value: cef_focus_source_t) -> Self {
        Self(value)
    }
}

impl Into<cef_focus_source_t> for FocusSource {
    fn into(self) -> cef_focus_source_t {
        self.0
    }
}

impl Default for FocusSource {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_navigation_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NavigationType(cef_navigation_type_t);

impl AsRef<cef_navigation_type_t> for NavigationType {
    fn as_ref(&self) -> &cef_navigation_type_t {
        &self.0
    }
}

impl AsMut<cef_navigation_type_t> for NavigationType {
    fn as_mut(&mut self) -> &mut cef_navigation_type_t {
        &mut self.0
    }
}

impl From<cef_navigation_type_t> for NavigationType {
    fn from(value: cef_navigation_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_navigation_type_t> for NavigationType {
    fn into(self) -> cef_navigation_type_t {
        self.0
    }
}

impl Default for NavigationType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_xml_encoding_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct XmlEncodingType(cef_xml_encoding_type_t);

impl AsRef<cef_xml_encoding_type_t> for XmlEncodingType {
    fn as_ref(&self) -> &cef_xml_encoding_type_t {
        &self.0
    }
}

impl AsMut<cef_xml_encoding_type_t> for XmlEncodingType {
    fn as_mut(&mut self) -> &mut cef_xml_encoding_type_t {
        &mut self.0
    }
}

impl From<cef_xml_encoding_type_t> for XmlEncodingType {
    fn from(value: cef_xml_encoding_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_xml_encoding_type_t> for XmlEncodingType {
    fn into(self) -> cef_xml_encoding_type_t {
        self.0
    }
}

impl Default for XmlEncodingType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_xml_node_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct XmlNodeType(cef_xml_node_type_t);

impl AsRef<cef_xml_node_type_t> for XmlNodeType {
    fn as_ref(&self) -> &cef_xml_node_type_t {
        &self.0
    }
}

impl AsMut<cef_xml_node_type_t> for XmlNodeType {
    fn as_mut(&mut self) -> &mut cef_xml_node_type_t {
        &mut self.0
    }
}

impl From<cef_xml_node_type_t> for XmlNodeType {
    fn from(value: cef_xml_node_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_xml_node_type_t> for XmlNodeType {
    fn into(self) -> cef_xml_node_type_t {
        self.0
    }
}

impl Default for XmlNodeType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_dom_document_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomDocumentType(cef_dom_document_type_t);

impl AsRef<cef_dom_document_type_t> for DomDocumentType {
    fn as_ref(&self) -> &cef_dom_document_type_t {
        &self.0
    }
}

impl AsMut<cef_dom_document_type_t> for DomDocumentType {
    fn as_mut(&mut self) -> &mut cef_dom_document_type_t {
        &mut self.0
    }
}

impl From<cef_dom_document_type_t> for DomDocumentType {
    fn from(value: cef_dom_document_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_dom_document_type_t> for DomDocumentType {
    fn into(self) -> cef_dom_document_type_t {
        self.0
    }
}

impl Default for DomDocumentType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_dom_event_category_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomEventCategory(cef_dom_event_category_t);

impl AsRef<cef_dom_event_category_t> for DomEventCategory {
    fn as_ref(&self) -> &cef_dom_event_category_t {
        &self.0
    }
}

impl AsMut<cef_dom_event_category_t> for DomEventCategory {
    fn as_mut(&mut self) -> &mut cef_dom_event_category_t {
        &mut self.0
    }
}

impl From<cef_dom_event_category_t> for DomEventCategory {
    fn from(value: cef_dom_event_category_t) -> Self {
        Self(value)
    }
}

impl Into<cef_dom_event_category_t> for DomEventCategory {
    fn into(self) -> cef_dom_event_category_t {
        self.0
    }
}

impl Default for DomEventCategory {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_dom_event_phase_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomEventPhase(cef_dom_event_phase_t);

impl AsRef<cef_dom_event_phase_t> for DomEventPhase {
    fn as_ref(&self) -> &cef_dom_event_phase_t {
        &self.0
    }
}

impl AsMut<cef_dom_event_phase_t> for DomEventPhase {
    fn as_mut(&mut self) -> &mut cef_dom_event_phase_t {
        &mut self.0
    }
}

impl From<cef_dom_event_phase_t> for DomEventPhase {
    fn from(value: cef_dom_event_phase_t) -> Self {
        Self(value)
    }
}

impl Into<cef_dom_event_phase_t> for DomEventPhase {
    fn into(self) -> cef_dom_event_phase_t {
        self.0
    }
}

impl Default for DomEventPhase {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_dom_node_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomNodeType(cef_dom_node_type_t);

impl AsRef<cef_dom_node_type_t> for DomNodeType {
    fn as_ref(&self) -> &cef_dom_node_type_t {
        &self.0
    }
}

impl AsMut<cef_dom_node_type_t> for DomNodeType {
    fn as_mut(&mut self) -> &mut cef_dom_node_type_t {
        &mut self.0
    }
}

impl From<cef_dom_node_type_t> for DomNodeType {
    fn from(value: cef_dom_node_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_dom_node_type_t> for DomNodeType {
    fn into(self) -> cef_dom_node_type_t {
        self.0
    }
}

impl Default for DomNodeType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_dom_form_control_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomFormControlType(cef_dom_form_control_type_t);

impl AsRef<cef_dom_form_control_type_t> for DomFormControlType {
    fn as_ref(&self) -> &cef_dom_form_control_type_t {
        &self.0
    }
}

impl AsMut<cef_dom_form_control_type_t> for DomFormControlType {
    fn as_mut(&mut self) -> &mut cef_dom_form_control_type_t {
        &mut self.0
    }
}

impl From<cef_dom_form_control_type_t> for DomFormControlType {
    fn from(value: cef_dom_form_control_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_dom_form_control_type_t> for DomFormControlType {
    fn into(self) -> cef_dom_form_control_type_t {
        self.0
    }
}

impl Default for DomFormControlType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_file_dialog_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FileDialogMode(cef_file_dialog_mode_t);

impl AsRef<cef_file_dialog_mode_t> for FileDialogMode {
    fn as_ref(&self) -> &cef_file_dialog_mode_t {
        &self.0
    }
}

impl AsMut<cef_file_dialog_mode_t> for FileDialogMode {
    fn as_mut(&mut self) -> &mut cef_file_dialog_mode_t {
        &mut self.0
    }
}

impl From<cef_file_dialog_mode_t> for FileDialogMode {
    fn from(value: cef_file_dialog_mode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_file_dialog_mode_t> for FileDialogMode {
    fn into(self) -> cef_file_dialog_mode_t {
        self.0
    }
}

impl Default for FileDialogMode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_color_model_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorModel(cef_color_model_t);

impl AsRef<cef_color_model_t> for ColorModel {
    fn as_ref(&self) -> &cef_color_model_t {
        &self.0
    }
}

impl AsMut<cef_color_model_t> for ColorModel {
    fn as_mut(&mut self) -> &mut cef_color_model_t {
        &mut self.0
    }
}

impl From<cef_color_model_t> for ColorModel {
    fn from(value: cef_color_model_t) -> Self {
        Self(value)
    }
}

impl Into<cef_color_model_t> for ColorModel {
    fn into(self) -> cef_color_model_t {
        self.0
    }
}

impl Default for ColorModel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_duplex_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DuplexMode(cef_duplex_mode_t);

impl AsRef<cef_duplex_mode_t> for DuplexMode {
    fn as_ref(&self) -> &cef_duplex_mode_t {
        &self.0
    }
}

impl AsMut<cef_duplex_mode_t> for DuplexMode {
    fn as_mut(&mut self) -> &mut cef_duplex_mode_t {
        &mut self.0
    }
}

impl From<cef_duplex_mode_t> for DuplexMode {
    fn from(value: cef_duplex_mode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_duplex_mode_t> for DuplexMode {
    fn into(self) -> cef_duplex_mode_t {
        self.0
    }
}

impl Default for DuplexMode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_cursor_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CursorType(cef_cursor_type_t);

impl AsRef<cef_cursor_type_t> for CursorType {
    fn as_ref(&self) -> &cef_cursor_type_t {
        &self.0
    }
}

impl AsMut<cef_cursor_type_t> for CursorType {
    fn as_mut(&mut self) -> &mut cef_cursor_type_t {
        &mut self.0
    }
}

impl From<cef_cursor_type_t> for CursorType {
    fn from(value: cef_cursor_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_cursor_type_t> for CursorType {
    fn into(self) -> cef_cursor_type_t {
        self.0
    }
}

impl Default for CursorType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_uri_unescape_rule_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UriUnescapeRule(cef_uri_unescape_rule_t);

impl AsRef<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn as_ref(&self) -> &cef_uri_unescape_rule_t {
        &self.0
    }
}

impl AsMut<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn as_mut(&mut self) -> &mut cef_uri_unescape_rule_t {
        &mut self.0
    }
}

impl From<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn from(value: cef_uri_unescape_rule_t) -> Self {
        Self(value)
    }
}

impl Into<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn into(self) -> cef_uri_unescape_rule_t {
        self.0
    }
}

impl Default for UriUnescapeRule {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_json_parser_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsonParserOptions(cef_json_parser_options_t);

impl AsRef<cef_json_parser_options_t> for JsonParserOptions {
    fn as_ref(&self) -> &cef_json_parser_options_t {
        &self.0
    }
}

impl AsMut<cef_json_parser_options_t> for JsonParserOptions {
    fn as_mut(&mut self) -> &mut cef_json_parser_options_t {
        &mut self.0
    }
}

impl From<cef_json_parser_options_t> for JsonParserOptions {
    fn from(value: cef_json_parser_options_t) -> Self {
        Self(value)
    }
}

impl Into<cef_json_parser_options_t> for JsonParserOptions {
    fn into(self) -> cef_json_parser_options_t {
        self.0
    }
}

impl Default for JsonParserOptions {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_json_writer_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsonWriterOptions(cef_json_writer_options_t);

impl AsRef<cef_json_writer_options_t> for JsonWriterOptions {
    fn as_ref(&self) -> &cef_json_writer_options_t {
        &self.0
    }
}

impl AsMut<cef_json_writer_options_t> for JsonWriterOptions {
    fn as_mut(&mut self) -> &mut cef_json_writer_options_t {
        &mut self.0
    }
}

impl From<cef_json_writer_options_t> for JsonWriterOptions {
    fn from(value: cef_json_writer_options_t) -> Self {
        Self(value)
    }
}

impl Into<cef_json_writer_options_t> for JsonWriterOptions {
    fn into(self) -> cef_json_writer_options_t {
        self.0
    }
}

impl Default for JsonWriterOptions {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_pdf_print_margin_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PdfPrintMarginType(cef_pdf_print_margin_type_t);

impl AsRef<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn as_ref(&self) -> &cef_pdf_print_margin_type_t {
        &self.0
    }
}

impl AsMut<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn as_mut(&mut self) -> &mut cef_pdf_print_margin_type_t {
        &mut self.0
    }
}

impl From<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn from(value: cef_pdf_print_margin_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn into(self) -> cef_pdf_print_margin_type_t {
        self.0
    }
}

impl Default for PdfPrintMarginType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_scale_factor_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ScaleFactor(cef_scale_factor_t);

impl AsRef<cef_scale_factor_t> for ScaleFactor {
    fn as_ref(&self) -> &cef_scale_factor_t {
        &self.0
    }
}

impl AsMut<cef_scale_factor_t> for ScaleFactor {
    fn as_mut(&mut self) -> &mut cef_scale_factor_t {
        &mut self.0
    }
}

impl From<cef_scale_factor_t> for ScaleFactor {
    fn from(value: cef_scale_factor_t) -> Self {
        Self(value)
    }
}

impl Into<cef_scale_factor_t> for ScaleFactor {
    fn into(self) -> cef_scale_factor_t {
        self.0
    }
}

impl Default for ScaleFactor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_referrer_policy_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ReferrerPolicy(cef_referrer_policy_t);

impl AsRef<cef_referrer_policy_t> for ReferrerPolicy {
    fn as_ref(&self) -> &cef_referrer_policy_t {
        &self.0
    }
}

impl AsMut<cef_referrer_policy_t> for ReferrerPolicy {
    fn as_mut(&mut self) -> &mut cef_referrer_policy_t {
        &mut self.0
    }
}

impl From<cef_referrer_policy_t> for ReferrerPolicy {
    fn from(value: cef_referrer_policy_t) -> Self {
        Self(value)
    }
}

impl Into<cef_referrer_policy_t> for ReferrerPolicy {
    fn into(self) -> cef_referrer_policy_t {
        self.0
    }
}

impl Default for ReferrerPolicy {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_response_filter_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResponseFilterStatus(cef_response_filter_status_t);

impl AsRef<cef_response_filter_status_t> for ResponseFilterStatus {
    fn as_ref(&self) -> &cef_response_filter_status_t {
        &self.0
    }
}

impl AsMut<cef_response_filter_status_t> for ResponseFilterStatus {
    fn as_mut(&mut self) -> &mut cef_response_filter_status_t {
        &mut self.0
    }
}

impl From<cef_response_filter_status_t> for ResponseFilterStatus {
    fn from(value: cef_response_filter_status_t) -> Self {
        Self(value)
    }
}

impl Into<cef_response_filter_status_t> for ResponseFilterStatus {
    fn into(self) -> cef_response_filter_status_t {
        self.0
    }
}

impl Default for ResponseFilterStatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_alpha_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AlphaType(cef_alpha_type_t);

impl AsRef<cef_alpha_type_t> for AlphaType {
    fn as_ref(&self) -> &cef_alpha_type_t {
        &self.0
    }
}

impl AsMut<cef_alpha_type_t> for AlphaType {
    fn as_mut(&mut self) -> &mut cef_alpha_type_t {
        &mut self.0
    }
}

impl From<cef_alpha_type_t> for AlphaType {
    fn from(value: cef_alpha_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_alpha_type_t> for AlphaType {
    fn into(self) -> cef_alpha_type_t {
        self.0
    }
}

impl Default for AlphaType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_text_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextStyle(cef_text_style_t);

impl AsRef<cef_text_style_t> for TextStyle {
    fn as_ref(&self) -> &cef_text_style_t {
        &self.0
    }
}

impl AsMut<cef_text_style_t> for TextStyle {
    fn as_mut(&mut self) -> &mut cef_text_style_t {
        &mut self.0
    }
}

impl From<cef_text_style_t> for TextStyle {
    fn from(value: cef_text_style_t) -> Self {
        Self(value)
    }
}

impl Into<cef_text_style_t> for TextStyle {
    fn into(self) -> cef_text_style_t {
        self.0
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_axis_alignment_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AxisAlignment(cef_axis_alignment_t);

impl AsRef<cef_axis_alignment_t> for AxisAlignment {
    fn as_ref(&self) -> &cef_axis_alignment_t {
        &self.0
    }
}

impl AsMut<cef_axis_alignment_t> for AxisAlignment {
    fn as_mut(&mut self) -> &mut cef_axis_alignment_t {
        &mut self.0
    }
}

impl From<cef_axis_alignment_t> for AxisAlignment {
    fn from(value: cef_axis_alignment_t) -> Self {
        Self(value)
    }
}

impl Into<cef_axis_alignment_t> for AxisAlignment {
    fn into(self) -> cef_axis_alignment_t {
        self.0
    }
}

impl Default for AxisAlignment {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_button_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ButtonState(cef_button_state_t);

impl AsRef<cef_button_state_t> for ButtonState {
    fn as_ref(&self) -> &cef_button_state_t {
        &self.0
    }
}

impl AsMut<cef_button_state_t> for ButtonState {
    fn as_mut(&mut self) -> &mut cef_button_state_t {
        &mut self.0
    }
}

impl From<cef_button_state_t> for ButtonState {
    fn from(value: cef_button_state_t) -> Self {
        Self(value)
    }
}

impl Into<cef_button_state_t> for ButtonState {
    fn into(self) -> cef_button_state_t {
        self.0
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_horizontal_alignment_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HorizontalAlignment(cef_horizontal_alignment_t);

impl AsRef<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn as_ref(&self) -> &cef_horizontal_alignment_t {
        &self.0
    }
}

impl AsMut<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn as_mut(&mut self) -> &mut cef_horizontal_alignment_t {
        &mut self.0
    }
}

impl From<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn from(value: cef_horizontal_alignment_t) -> Self {
        Self(value)
    }
}

impl Into<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn into(self) -> cef_horizontal_alignment_t {
        self.0
    }
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_menu_anchor_position_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuAnchorPosition(cef_menu_anchor_position_t);

impl AsRef<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn as_ref(&self) -> &cef_menu_anchor_position_t {
        &self.0
    }
}

impl AsMut<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn as_mut(&mut self) -> &mut cef_menu_anchor_position_t {
        &mut self.0
    }
}

impl From<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn from(value: cef_menu_anchor_position_t) -> Self {
        Self(value)
    }
}

impl Into<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn into(self) -> cef_menu_anchor_position_t {
        self.0
    }
}

impl Default for MenuAnchorPosition {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_menu_color_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuColorType(cef_menu_color_type_t);

impl AsRef<cef_menu_color_type_t> for MenuColorType {
    fn as_ref(&self) -> &cef_menu_color_type_t {
        &self.0
    }
}

impl AsMut<cef_menu_color_type_t> for MenuColorType {
    fn as_mut(&mut self) -> &mut cef_menu_color_type_t {
        &mut self.0
    }
}

impl From<cef_menu_color_type_t> for MenuColorType {
    fn from(value: cef_menu_color_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_menu_color_type_t> for MenuColorType {
    fn into(self) -> cef_menu_color_type_t {
        self.0
    }
}

impl Default for MenuColorType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_ssl_version_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SslVersion(cef_ssl_version_t);

impl AsRef<cef_ssl_version_t> for SslVersion {
    fn as_ref(&self) -> &cef_ssl_version_t {
        &self.0
    }
}

impl AsMut<cef_ssl_version_t> for SslVersion {
    fn as_mut(&mut self) -> &mut cef_ssl_version_t {
        &mut self.0
    }
}

impl From<cef_ssl_version_t> for SslVersion {
    fn from(value: cef_ssl_version_t) -> Self {
        Self(value)
    }
}

impl Into<cef_ssl_version_t> for SslVersion {
    fn into(self) -> cef_ssl_version_t {
        self.0
    }
}

impl Default for SslVersion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_ssl_content_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SslContentStatus(cef_ssl_content_status_t);

impl AsRef<cef_ssl_content_status_t> for SslContentStatus {
    fn as_ref(&self) -> &cef_ssl_content_status_t {
        &self.0
    }
}

impl AsMut<cef_ssl_content_status_t> for SslContentStatus {
    fn as_mut(&mut self) -> &mut cef_ssl_content_status_t {
        &mut self.0
    }
}

impl From<cef_ssl_content_status_t> for SslContentStatus {
    fn from(value: cef_ssl_content_status_t) -> Self {
        Self(value)
    }
}

impl Into<cef_ssl_content_status_t> for SslContentStatus {
    fn into(self) -> cef_ssl_content_status_t {
        self.0
    }
}

impl Default for SslContentStatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_scheme_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SchemeOptions(cef_scheme_options_t);

impl AsRef<cef_scheme_options_t> for SchemeOptions {
    fn as_ref(&self) -> &cef_scheme_options_t {
        &self.0
    }
}

impl AsMut<cef_scheme_options_t> for SchemeOptions {
    fn as_mut(&mut self) -> &mut cef_scheme_options_t {
        &mut self.0
    }
}

impl From<cef_scheme_options_t> for SchemeOptions {
    fn from(value: cef_scheme_options_t) -> Self {
        Self(value)
    }
}

impl Into<cef_scheme_options_t> for SchemeOptions {
    fn into(self) -> cef_scheme_options_t {
        self.0
    }
}

impl Default for SchemeOptions {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_composition_underline_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CompositionUnderlineStyle(cef_composition_underline_style_t);

impl AsRef<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn as_ref(&self) -> &cef_composition_underline_style_t {
        &self.0
    }
}

impl AsMut<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn as_mut(&mut self) -> &mut cef_composition_underline_style_t {
        &mut self.0
    }
}

impl From<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn from(value: cef_composition_underline_style_t) -> Self {
        Self(value)
    }
}

impl Into<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn into(self) -> cef_composition_underline_style_t {
        self.0
    }
}

impl Default for CompositionUnderlineStyle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_channel_layout_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChannelLayout(cef_channel_layout_t);

impl AsRef<cef_channel_layout_t> for ChannelLayout {
    fn as_ref(&self) -> &cef_channel_layout_t {
        &self.0
    }
}

impl AsMut<cef_channel_layout_t> for ChannelLayout {
    fn as_mut(&mut self) -> &mut cef_channel_layout_t {
        &mut self.0
    }
}

impl From<cef_channel_layout_t> for ChannelLayout {
    fn from(value: cef_channel_layout_t) -> Self {
        Self(value)
    }
}

impl Into<cef_channel_layout_t> for ChannelLayout {
    fn into(self) -> cef_channel_layout_t {
        self.0
    }
}

impl Default for ChannelLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_media_route_create_result_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaRouteCreateResult(cef_media_route_create_result_t);

impl AsRef<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn as_ref(&self) -> &cef_media_route_create_result_t {
        &self.0
    }
}

impl AsMut<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn as_mut(&mut self) -> &mut cef_media_route_create_result_t {
        &mut self.0
    }
}

impl From<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn from(value: cef_media_route_create_result_t) -> Self {
        Self(value)
    }
}

impl Into<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn into(self) -> cef_media_route_create_result_t {
        self.0
    }
}

impl Default for MediaRouteCreateResult {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_media_route_connection_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaRouteConnectionState(cef_media_route_connection_state_t);

impl AsRef<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn as_ref(&self) -> &cef_media_route_connection_state_t {
        &self.0
    }
}

impl AsMut<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn as_mut(&mut self) -> &mut cef_media_route_connection_state_t {
        &mut self.0
    }
}

impl From<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn from(value: cef_media_route_connection_state_t) -> Self {
        Self(value)
    }
}

impl Into<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn into(self) -> cef_media_route_connection_state_t {
        self.0
    }
}

impl Default for MediaRouteConnectionState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_media_sink_icon_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaSinkIconType(cef_media_sink_icon_type_t);

impl AsRef<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn as_ref(&self) -> &cef_media_sink_icon_type_t {
        &self.0
    }
}

impl AsMut<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn as_mut(&mut self) -> &mut cef_media_sink_icon_type_t {
        &mut self.0
    }
}

impl From<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn from(value: cef_media_sink_icon_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn into(self) -> cef_media_sink_icon_type_t {
        self.0
    }
}

impl Default for MediaSinkIconType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_text_field_commands_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextFieldCommands(cef_text_field_commands_t);

impl AsRef<cef_text_field_commands_t> for TextFieldCommands {
    fn as_ref(&self) -> &cef_text_field_commands_t {
        &self.0
    }
}

impl AsMut<cef_text_field_commands_t> for TextFieldCommands {
    fn as_mut(&mut self) -> &mut cef_text_field_commands_t {
        &mut self.0
    }
}

impl From<cef_text_field_commands_t> for TextFieldCommands {
    fn from(value: cef_text_field_commands_t) -> Self {
        Self(value)
    }
}

impl Into<cef_text_field_commands_t> for TextFieldCommands {
    fn into(self) -> cef_text_field_commands_t {
        self.0
    }
}

impl Default for TextFieldCommands {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_chrome_toolbar_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromeToolbarType(cef_chrome_toolbar_type_t);

impl AsRef<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn as_ref(&self) -> &cef_chrome_toolbar_type_t {
        &self.0
    }
}

impl AsMut<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn as_mut(&mut self) -> &mut cef_chrome_toolbar_type_t {
        &mut self.0
    }
}

impl From<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn from(value: cef_chrome_toolbar_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn into(self) -> cef_chrome_toolbar_type_t {
        self.0
    }
}

impl Default for ChromeToolbarType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_chrome_page_action_icon_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromePageActionIconType(cef_chrome_page_action_icon_type_t);

impl AsRef<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn as_ref(&self) -> &cef_chrome_page_action_icon_type_t {
        &self.0
    }
}

impl AsMut<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn as_mut(&mut self) -> &mut cef_chrome_page_action_icon_type_t {
        &mut self.0
    }
}

impl From<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn from(value: cef_chrome_page_action_icon_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn into(self) -> cef_chrome_page_action_icon_type_t {
        self.0
    }
}

impl Default for ChromePageActionIconType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_chrome_toolbar_button_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromeToolbarButtonType(cef_chrome_toolbar_button_type_t);

impl AsRef<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn as_ref(&self) -> &cef_chrome_toolbar_button_type_t {
        &self.0
    }
}

impl AsMut<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn as_mut(&mut self) -> &mut cef_chrome_toolbar_button_type_t {
        &mut self.0
    }
}

impl From<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn from(value: cef_chrome_toolbar_button_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn into(self) -> cef_chrome_toolbar_button_type_t {
        self.0
    }
}

impl Default for ChromeToolbarButtonType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_docking_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DockingMode(cef_docking_mode_t);

impl AsRef<cef_docking_mode_t> for DockingMode {
    fn as_ref(&self) -> &cef_docking_mode_t {
        &self.0
    }
}

impl AsMut<cef_docking_mode_t> for DockingMode {
    fn as_mut(&mut self) -> &mut cef_docking_mode_t {
        &mut self.0
    }
}

impl From<cef_docking_mode_t> for DockingMode {
    fn from(value: cef_docking_mode_t) -> Self {
        Self(value)
    }
}

impl Into<cef_docking_mode_t> for DockingMode {
    fn into(self) -> cef_docking_mode_t {
        self.0
    }
}

impl Default for DockingMode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_show_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ShowState(cef_show_state_t);

impl AsRef<cef_show_state_t> for ShowState {
    fn as_ref(&self) -> &cef_show_state_t {
        &self.0
    }
}

impl AsMut<cef_show_state_t> for ShowState {
    fn as_mut(&mut self) -> &mut cef_show_state_t {
        &mut self.0
    }
}

impl From<cef_show_state_t> for ShowState {
    fn from(value: cef_show_state_t) -> Self {
        Self(value)
    }
}

impl Into<cef_show_state_t> for ShowState {
    fn into(self) -> cef_show_state_t {
        self.0
    }
}

impl Default for ShowState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_touch_handle_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TouchHandleStateFlags(cef_touch_handle_state_flags_t);

impl AsRef<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn as_ref(&self) -> &cef_touch_handle_state_flags_t {
        &self.0
    }
}

impl AsMut<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn as_mut(&mut self) -> &mut cef_touch_handle_state_flags_t {
        &mut self.0
    }
}

impl From<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn from(value: cef_touch_handle_state_flags_t) -> Self {
        Self(value)
    }
}

impl Into<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn into(self) -> cef_touch_handle_state_flags_t {
        self.0
    }
}

impl Default for TouchHandleStateFlags {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_media_access_permission_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaAccessPermissionTypes(cef_media_access_permission_types_t);

impl AsRef<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn as_ref(&self) -> &cef_media_access_permission_types_t {
        &self.0
    }
}

impl AsMut<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn as_mut(&mut self) -> &mut cef_media_access_permission_types_t {
        &mut self.0
    }
}

impl From<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn from(value: cef_media_access_permission_types_t) -> Self {
        Self(value)
    }
}

impl Into<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn into(self) -> cef_media_access_permission_types_t {
        self.0
    }
}

impl Default for MediaAccessPermissionTypes {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_permission_request_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PermissionRequestTypes(cef_permission_request_types_t);

impl AsRef<cef_permission_request_types_t> for PermissionRequestTypes {
    fn as_ref(&self) -> &cef_permission_request_types_t {
        &self.0
    }
}

impl AsMut<cef_permission_request_types_t> for PermissionRequestTypes {
    fn as_mut(&mut self) -> &mut cef_permission_request_types_t {
        &mut self.0
    }
}

impl From<cef_permission_request_types_t> for PermissionRequestTypes {
    fn from(value: cef_permission_request_types_t) -> Self {
        Self(value)
    }
}

impl Into<cef_permission_request_types_t> for PermissionRequestTypes {
    fn into(self) -> cef_permission_request_types_t {
        self.0
    }
}

impl Default for PermissionRequestTypes {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_permission_request_result_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PermissionRequestResult(cef_permission_request_result_t);

impl AsRef<cef_permission_request_result_t> for PermissionRequestResult {
    fn as_ref(&self) -> &cef_permission_request_result_t {
        &self.0
    }
}

impl AsMut<cef_permission_request_result_t> for PermissionRequestResult {
    fn as_mut(&mut self) -> &mut cef_permission_request_result_t {
        &mut self.0
    }
}

impl From<cef_permission_request_result_t> for PermissionRequestResult {
    fn from(value: cef_permission_request_result_t) -> Self {
        Self(value)
    }
}

impl Into<cef_permission_request_result_t> for PermissionRequestResult {
    fn into(self) -> cef_permission_request_result_t {
        self.0
    }
}

impl Default for PermissionRequestResult {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_test_cert_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestCertType(cef_test_cert_type_t);

impl AsRef<cef_test_cert_type_t> for TestCertType {
    fn as_ref(&self) -> &cef_test_cert_type_t {
        &self.0
    }
}

impl AsMut<cef_test_cert_type_t> for TestCertType {
    fn as_mut(&mut self) -> &mut cef_test_cert_type_t {
        &mut self.0
    }
}

impl From<cef_test_cert_type_t> for TestCertType {
    fn from(value: cef_test_cert_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_test_cert_type_t> for TestCertType {
    fn into(self) -> cef_test_cert_type_t {
        self.0
    }
}

impl Default for TestCertType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_preferences_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PreferencesType(cef_preferences_type_t);

impl AsRef<cef_preferences_type_t> for PreferencesType {
    fn as_ref(&self) -> &cef_preferences_type_t {
        &self.0
    }
}

impl AsMut<cef_preferences_type_t> for PreferencesType {
    fn as_mut(&mut self) -> &mut cef_preferences_type_t {
        &mut self.0
    }
}

impl From<cef_preferences_type_t> for PreferencesType {
    fn from(value: cef_preferences_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_preferences_type_t> for PreferencesType {
    fn into(self) -> cef_preferences_type_t {
        self.0
    }
}

impl Default for PreferencesType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_download_interrupt_reason_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DownloadInterruptReason(cef_download_interrupt_reason_t);

impl AsRef<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn as_ref(&self) -> &cef_download_interrupt_reason_t {
        &self.0
    }
}

impl AsMut<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn as_mut(&mut self) -> &mut cef_download_interrupt_reason_t {
        &mut self.0
    }
}

impl From<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn from(value: cef_download_interrupt_reason_t) -> Self {
        Self(value)
    }
}

impl Into<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn into(self) -> cef_download_interrupt_reason_t {
        self.0
    }
}

impl Default for DownloadInterruptReason {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_gesture_command_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GestureCommand(cef_gesture_command_t);

impl AsRef<cef_gesture_command_t> for GestureCommand {
    fn as_ref(&self) -> &cef_gesture_command_t {
        &self.0
    }
}

impl AsMut<cef_gesture_command_t> for GestureCommand {
    fn as_mut(&mut self) -> &mut cef_gesture_command_t {
        &mut self.0
    }
}

impl From<cef_gesture_command_t> for GestureCommand {
    fn from(value: cef_gesture_command_t) -> Self {
        Self(value)
    }
}

impl Into<cef_gesture_command_t> for GestureCommand {
    fn into(self) -> cef_gesture_command_t {
        self.0
    }
}

impl Default for GestureCommand {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_zoom_command_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ZoomCommand(cef_zoom_command_t);

impl AsRef<cef_zoom_command_t> for ZoomCommand {
    fn as_ref(&self) -> &cef_zoom_command_t {
        &self.0
    }
}

impl AsMut<cef_zoom_command_t> for ZoomCommand {
    fn as_mut(&mut self) -> &mut cef_zoom_command_t {
        &mut self.0
    }
}

impl From<cef_zoom_command_t> for ZoomCommand {
    fn from(value: cef_zoom_command_t) -> Self {
        Self(value)
    }
}

impl Into<cef_zoom_command_t> for ZoomCommand {
    fn into(self) -> cef_zoom_command_t {
        self.0
    }
}

impl Default for ZoomCommand {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_color_variant_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorVariant(cef_color_variant_t);

impl AsRef<cef_color_variant_t> for ColorVariant {
    fn as_ref(&self) -> &cef_color_variant_t {
        &self.0
    }
}

impl AsMut<cef_color_variant_t> for ColorVariant {
    fn as_mut(&mut self) -> &mut cef_color_variant_t {
        &mut self.0
    }
}

impl From<cef_color_variant_t> for ColorVariant {
    fn from(value: cef_color_variant_t) -> Self {
        Self(value)
    }
}

impl Into<cef_color_variant_t> for ColorVariant {
    fn into(self) -> cef_color_variant_t {
        self.0
    }
}

impl Default for ColorVariant {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_task_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TaskType(cef_task_type_t);

impl AsRef<cef_task_type_t> for TaskType {
    fn as_ref(&self) -> &cef_task_type_t {
        &self.0
    }
}

impl AsMut<cef_task_type_t> for TaskType {
    fn as_mut(&mut self) -> &mut cef_task_type_t {
        &mut self.0
    }
}

impl From<cef_task_type_t> for TaskType {
    fn from(value: cef_task_type_t) -> Self {
        Self(value)
    }
}

impl Into<cef_task_type_t> for TaskType {
    fn into(self) -> cef_task_type_t {
        self.0
    }
}

impl Default for TaskType {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// Global function wrappers

pub fn string_wide_set(
    src: *const wchar_t,
    src_len: usize,
    output: &mut CefStringWide,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_wide_set(
            src.as_raw(),
            src_len.as_raw(),
            output.as_raw(),
            copy.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn string_utf8_set(
    src: *const ::std::os::raw::c_char,
    src_len: usize,
    output: &mut CefStringUtf8,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf8_set(
            src.as_raw(),
            src_len.as_raw(),
            output.as_raw(),
            copy.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn string_utf16_set(
    src: *const char16_t,
    src_len: usize,
    output: &mut CefStringUtf16,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf16_set(
            src.as_raw(),
            src_len.as_raw(),
            output.as_raw(),
            copy.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn string_wide_clear(str_: &mut CefStringWide) {
    unsafe { cef_string_wide_clear(str_.as_raw()) }
}

pub fn string_utf8_clear(str_: &mut CefStringUtf8) {
    unsafe { cef_string_utf8_clear(str_.as_raw()) }
}

pub fn string_utf16_clear(str_: &mut CefStringUtf16) {
    unsafe { cef_string_utf16_clear(str_.as_raw()) }
}

pub fn string_wide_cmp(str_1: &CefStringWide, str_2: &CefStringWide) -> ::std::os::raw::c_int {
    unsafe { cef_string_wide_cmp(str_1.as_raw(), str_2.as_raw()).as_wrapper() }
}

pub fn string_utf8_cmp(str_1: &CefStringUtf8, str_2: &CefStringUtf8) -> ::std::os::raw::c_int {
    unsafe { cef_string_utf8_cmp(str_1.as_raw(), str_2.as_raw()).as_wrapper() }
}

pub fn string_utf16_cmp(str_1: &CefStringUtf16, str_2: &CefStringUtf16) -> ::std::os::raw::c_int {
    unsafe { cef_string_utf16_cmp(str_1.as_raw(), str_2.as_raw()).as_wrapper() }
}

pub fn string_wide_to_utf8(
    src: *const wchar_t,
    src_len: usize,
    output: &mut CefStringUtf8,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_wide_to_utf8(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper() }
}

pub fn string_utf8_to_wide(
    src: *const ::std::os::raw::c_char,
    src_len: usize,
    output: &mut CefStringWide,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_utf8_to_wide(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper() }
}

pub fn string_wide_to_utf16(
    src: *const wchar_t,
    src_len: usize,
    output: &mut CefStringUtf16,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_wide_to_utf16(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_utf16_to_wide(
    src: *const char16_t,
    src_len: usize,
    output: &mut CefStringWide,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf16_to_wide(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_utf8_to_utf16(
    src: *const ::std::os::raw::c_char,
    src_len: usize,
    output: &mut CefStringUtf16,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf8_to_utf16(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_utf16_to_utf8(
    src: *const char16_t,
    src_len: usize,
    output: &mut CefStringUtf8,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf16_to_utf8(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_ascii_to_wide(
    src: *const ::std::os::raw::c_char,
    src_len: usize,
    output: &mut CefStringWide,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_ascii_to_wide(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_ascii_to_utf16(
    src: *const ::std::os::raw::c_char,
    src_len: usize,
    output: &mut CefStringUtf16,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_ascii_to_utf16(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_userfree_wide_alloc() -> CefStringUserfreeWide {
    unsafe { cef_string_userfree_wide_alloc().as_wrapper() }
}

pub fn string_userfree_utf8_alloc() -> CefStringUserfreeUtf8 {
    unsafe { cef_string_userfree_utf8_alloc().as_wrapper() }
}

pub fn string_userfree_utf16_alloc() -> CefStringUserfreeUtf16 {
    unsafe { cef_string_userfree_utf16_alloc().as_wrapper() }
}

pub fn string_userfree_wide_free(str_: &mut CefStringUserfreeWide) {
    unsafe { cef_string_userfree_wide_free(str_.as_raw()) }
}

pub fn string_userfree_utf8_free(str_: &mut CefStringUserfreeUtf8) {
    unsafe { cef_string_userfree_utf8_free(str_.as_raw()) }
}

pub fn string_userfree_utf16_free(str_: &mut CefStringUserfreeUtf16) {
    unsafe { cef_string_userfree_utf16_free(str_.as_raw()) }
}

pub fn string_utf16_to_lower(
    src: *const char16_t,
    src_len: usize,
    output: &mut CefStringUtf16,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf16_to_lower(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_utf16_to_upper(
    src: *const char16_t,
    src_len: usize,
    output: &mut CefStringUtf16,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_utf16_to_upper(src.as_raw(), src_len.as_raw(), output.as_raw()).as_wrapper()
    }
}

pub fn string_list_alloc() -> CefStringList {
    unsafe { cef_string_list_alloc().as_wrapper() }
}

pub fn string_list_size(list: &mut CefStringList) -> usize {
    unsafe { cef_string_list_size(list.as_raw()).as_wrapper() }
}

pub fn string_list_value(
    list: &mut CefStringList,
    index: usize,
    value: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_list_value(list.as_raw(), index.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_list_append(list: &mut CefStringList, value: &CefString) {
    unsafe { cef_string_list_append(list.as_raw(), value.as_raw()) }
}

pub fn string_list_clear(list: &mut CefStringList) {
    unsafe { cef_string_list_clear(list.as_raw()) }
}

pub fn string_list_free(list: &mut CefStringList) {
    unsafe { cef_string_list_free(list.as_raw()) }
}

pub fn string_list_copy(list: &mut CefStringList) -> CefStringList {
    unsafe { cef_string_list_copy(list.as_raw()).as_wrapper() }
}

pub fn string_map_alloc() -> CefStringMap {
    unsafe { cef_string_map_alloc().as_wrapper() }
}

pub fn string_map_size(map: &mut CefStringMap) -> usize {
    unsafe { cef_string_map_size(map.as_raw()).as_wrapper() }
}

pub fn string_map_find(
    map: &mut CefStringMap,
    key: &CefString,
    value: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_map_find(map.as_raw(), key.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_map_key(
    map: &mut CefStringMap,
    index: usize,
    key: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_map_key(map.as_raw(), index.as_raw(), key.as_raw()).as_wrapper() }
}

pub fn string_map_value(
    map: &mut CefStringMap,
    index: usize,
    value: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_map_value(map.as_raw(), index.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_map_append(
    map: &mut CefStringMap,
    key: &CefString,
    value: &CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_map_append(map.as_raw(), key.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_map_clear(map: &mut CefStringMap) {
    unsafe { cef_string_map_clear(map.as_raw()) }
}

pub fn string_map_free(map: &mut CefStringMap) {
    unsafe { cef_string_map_free(map.as_raw()) }
}

pub fn string_multimap_alloc() -> CefStringMultimap {
    unsafe { cef_string_multimap_alloc().as_wrapper() }
}

pub fn string_multimap_size(map: &mut CefStringMultimap) -> usize {
    unsafe { cef_string_multimap_size(map.as_raw()).as_wrapper() }
}

pub fn string_multimap_find_count(map: &mut CefStringMultimap, key: &CefString) -> usize {
    unsafe { cef_string_multimap_find_count(map.as_raw(), key.as_raw()).as_wrapper() }
}

pub fn string_multimap_enumerate(
    map: &mut CefStringMultimap,
    key: &CefString,
    value_index: usize,
    value: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_string_multimap_enumerate(
            map.as_raw(),
            key.as_raw(),
            value_index.as_raw(),
            value.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn string_multimap_key(
    map: &mut CefStringMultimap,
    index: usize,
    key: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_multimap_key(map.as_raw(), index.as_raw(), key.as_raw()).as_wrapper() }
}

pub fn string_multimap_value(
    map: &mut CefStringMultimap,
    index: usize,
    value: &mut CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_multimap_value(map.as_raw(), index.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_multimap_append(
    map: &mut CefStringMultimap,
    key: &CefString,
    value: &CefString,
) -> ::std::os::raw::c_int {
    unsafe { cef_string_multimap_append(map.as_raw(), key.as_raw(), value.as_raw()).as_wrapper() }
}

pub fn string_multimap_clear(map: &mut CefStringMultimap) {
    unsafe { cef_string_multimap_clear(map.as_raw()) }
}

pub fn string_multimap_free(map: &mut CefStringMultimap) {
    unsafe { cef_string_multimap_free(map.as_raw()) }
}

pub fn time_to_timet(cef_time: &Time, time: *mut time_t) -> ::std::os::raw::c_int {
    unsafe { cef_time_to_timet(cef_time.as_raw(), time.as_raw()).as_wrapper() }
}

pub fn time_from_timet(time: time_t, cef_time: &mut Time) -> ::std::os::raw::c_int {
    unsafe { cef_time_from_timet(time.as_raw(), cef_time.as_raw()).as_wrapper() }
}

pub fn time_to_doublet(cef_time: &Time, time: *mut f64) -> ::std::os::raw::c_int {
    unsafe { cef_time_to_doublet(cef_time.as_raw(), time.as_raw()).as_wrapper() }
}

pub fn time_from_doublet(time: f64, cef_time: &mut Time) -> ::std::os::raw::c_int {
    unsafe { cef_time_from_doublet(time.as_raw(), cef_time.as_raw()).as_wrapper() }
}

pub fn time_now(cef_time: &mut Time) -> ::std::os::raw::c_int {
    unsafe { cef_time_now(cef_time.as_raw()).as_wrapper() }
}

pub fn basetime_now() -> Basetime {
    unsafe { cef_basetime_now().as_wrapper() }
}

pub fn time_delta(
    cef_time_1: &Time,
    cef_time_2: &Time,
    delta: *mut ::std::os::raw::c_longlong,
) -> ::std::os::raw::c_int {
    unsafe { cef_time_delta(cef_time_1.as_raw(), cef_time_2.as_raw(), delta.as_raw()).as_wrapper() }
}

pub fn time_to_basetime(from: &Time, to: &mut Basetime) -> ::std::os::raw::c_int {
    unsafe { cef_time_to_basetime(from.as_raw(), to.as_raw()).as_wrapper() }
}

pub fn time_from_basetime(from: &mut Basetime, to: &mut Time) -> ::std::os::raw::c_int {
    unsafe { cef_time_from_basetime(from.as_raw(), to.as_raw()).as_wrapper() }
}

pub fn get_xdisplay() -> *mut XDisplay {
    unsafe { cef_get_xdisplay().as_wrapper() }
}

pub fn value_create() -> Value {
    unsafe { cef_value_create().as_wrapper() }
}

pub fn binary_value_create(data: *const ::std::os::raw::c_void, data_size: usize) -> BinaryValue {
    unsafe { cef_binary_value_create(data.as_raw(), data_size.as_raw()).as_wrapper() }
}

pub fn dictionary_value_create() -> DictionaryValue {
    unsafe { cef_dictionary_value_create().as_wrapper() }
}

pub fn list_value_create() -> ListValue {
    unsafe { cef_list_value_create().as_wrapper() }
}

pub fn image_create() -> Image {
    unsafe { cef_image_create().as_wrapper() }
}

pub fn stream_reader_create_for_file(file_name: &CefString) -> StreamReader {
    unsafe { cef_stream_reader_create_for_file(file_name.as_raw()).as_wrapper() }
}

pub fn stream_reader_create_for_data(
    data: *mut ::std::os::raw::c_void,
    size: usize,
) -> StreamReader {
    unsafe { cef_stream_reader_create_for_data(data.as_raw(), size.as_raw()).as_wrapper() }
}

pub fn stream_reader_create_for_handler(handler: &mut ReadHandler) -> StreamReader {
    unsafe { cef_stream_reader_create_for_handler(handler.as_raw()).as_wrapper() }
}

pub fn stream_writer_create_for_file(file_name: &CefString) -> StreamWriter {
    unsafe { cef_stream_writer_create_for_file(file_name.as_raw()).as_wrapper() }
}

pub fn stream_writer_create_for_handler(handler: &mut WriteHandler) -> StreamWriter {
    unsafe { cef_stream_writer_create_for_handler(handler.as_raw()).as_wrapper() }
}

pub fn drag_data_create() -> DragData {
    unsafe { cef_drag_data_create().as_wrapper() }
}

pub fn process_message_create(name: &CefString) -> ProcessMessage {
    unsafe { cef_process_message_create(name.as_raw()).as_wrapper() }
}

pub fn request_create() -> Request {
    unsafe { cef_request_create().as_wrapper() }
}

pub fn post_data_create() -> PostData {
    unsafe { cef_post_data_create().as_wrapper() }
}

pub fn post_data_element_create() -> PostDataElement {
    unsafe { cef_post_data_element_create().as_wrapper() }
}

pub fn cookie_manager_get_global_manager(callback: &mut CompletionCallback) -> CookieManager {
    unsafe { cef_cookie_manager_get_global_manager(callback.as_raw()).as_wrapper() }
}

pub fn media_router_get_global(callback: &mut CompletionCallback) -> MediaRouter {
    unsafe { cef_media_router_get_global(callback.as_raw()).as_wrapper() }
}

pub fn preference_manager_get_global() -> PreferenceManager {
    unsafe { cef_preference_manager_get_global().as_wrapper() }
}

pub fn request_context_get_global_context() -> RequestContext {
    unsafe { cef_request_context_get_global_context().as_wrapper() }
}

pub fn request_context_create_context(
    settings: &RequestContextSettings,
    handler: &mut RequestContextHandler,
) -> RequestContext {
    unsafe { cef_request_context_create_context(settings.as_raw(), handler.as_raw()).as_wrapper() }
}

pub fn create_context_shared(
    other: &mut RequestContext,
    handler: &mut RequestContextHandler,
) -> RequestContext {
    unsafe { cef_create_context_shared(other.as_raw(), handler.as_raw()).as_wrapper() }
}

pub fn browser_host_create_browser(
    window_info: &WindowInfo,
    client: &mut Client,
    url: &CefString,
    settings: &BrowserSettings,
    extra_info: &mut DictionaryValue,
    request_context: &mut RequestContext,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_browser_host_create_browser(
            window_info.as_raw(),
            client.as_raw(),
            url.as_raw(),
            settings.as_raw(),
            extra_info.as_raw(),
            request_context.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn browser_host_create_browser_sync(
    window_info: &WindowInfo,
    client: &mut Client,
    url: &CefString,
    settings: &BrowserSettings,
    extra_info: &mut DictionaryValue,
    request_context: &mut RequestContext,
) -> Browser {
    unsafe {
        cef_browser_host_create_browser_sync(
            window_info.as_raw(),
            client.as_raw(),
            url.as_raw(),
            settings.as_raw(),
            extra_info.as_raw(),
            request_context.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn browser_host_get_browser_by_identifier(browser_id: ::std::os::raw::c_int) -> Browser {
    unsafe { cef_browser_host_get_browser_by_identifier(browser_id.as_raw()).as_wrapper() }
}

pub fn menu_model_create(delegate: &mut MenuModelDelegate) -> MenuModel {
    unsafe { cef_menu_model_create(delegate.as_raw()).as_wrapper() }
}

pub fn print_settings_create() -> PrintSettings {
    unsafe { cef_print_settings_create().as_wrapper() }
}

pub fn response_create() -> Response {
    unsafe { cef_response_create().as_wrapper() }
}

pub fn is_cert_status_error(status: &mut CertStatus) -> ::std::os::raw::c_int {
    unsafe { cef_is_cert_status_error(status.as_raw()).as_wrapper() }
}

pub fn command_line_create() -> CommandLine {
    unsafe { cef_command_line_create().as_wrapper() }
}

pub fn command_line_get_global() -> CommandLine {
    unsafe { cef_command_line_get_global().as_wrapper() }
}

pub fn task_runner_get_for_current_thread() -> TaskRunner {
    unsafe { cef_task_runner_get_for_current_thread().as_wrapper() }
}

pub fn task_runner_get_for_thread(thread_id: &mut ThreadId) -> TaskRunner {
    unsafe { cef_task_runner_get_for_thread(thread_id.as_raw()).as_wrapper() }
}

pub fn currently_on(thread_id: &mut ThreadId) -> ::std::os::raw::c_int {
    unsafe { cef_currently_on(thread_id.as_raw()).as_wrapper() }
}

pub fn post_task(thread_id: &mut ThreadId, task: &mut Task) -> ::std::os::raw::c_int {
    unsafe { cef_post_task(thread_id.as_raw(), task.as_raw()).as_wrapper() }
}

pub fn post_delayed_task(
    thread_id: &mut ThreadId,
    task: &mut Task,
    delay_ms: i64,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_post_delayed_task(thread_id.as_raw(), task.as_raw(), delay_ms.as_raw()).as_wrapper()
    }
}

pub fn v8context_get_current_context() -> V8context {
    unsafe { cef_v8context_get_current_context().as_wrapper() }
}

pub fn v8context_get_entered_context() -> V8context {
    unsafe { cef_v8context_get_entered_context().as_wrapper() }
}

pub fn v8context_in_context() -> ::std::os::raw::c_int {
    unsafe { cef_v8context_in_context().as_wrapper() }
}

pub fn v8value_create_undefined() -> V8value {
    unsafe { cef_v8value_create_undefined().as_wrapper() }
}

pub fn v8value_create_null() -> V8value {
    unsafe { cef_v8value_create_null().as_wrapper() }
}

pub fn v8value_create_bool(value: ::std::os::raw::c_int) -> V8value {
    unsafe { cef_v8value_create_bool(value.as_raw()).as_wrapper() }
}

pub fn v8value_create_int(value: i32) -> V8value {
    unsafe { cef_v8value_create_int(value.as_raw()).as_wrapper() }
}

pub fn v8value_create_uint(value: u32) -> V8value {
    unsafe { cef_v8value_create_uint(value.as_raw()).as_wrapper() }
}

pub fn v8value_create_double(value: f64) -> V8value {
    unsafe { cef_v8value_create_double(value.as_raw()).as_wrapper() }
}

pub fn v8value_create_date(date: &mut Basetime) -> V8value {
    unsafe { cef_v8value_create_date(date.as_raw()).as_wrapper() }
}

pub fn v8value_create_string(value: &CefString) -> V8value {
    unsafe { cef_v8value_create_string(value.as_raw()).as_wrapper() }
}

pub fn v8value_create_object(
    accessor: &mut V8accessor,
    interceptor: &mut V8interceptor,
) -> V8value {
    unsafe { cef_v8value_create_object(accessor.as_raw(), interceptor.as_raw()).as_wrapper() }
}

pub fn v8value_create_array(length: ::std::os::raw::c_int) -> V8value {
    unsafe { cef_v8value_create_array(length.as_raw()).as_wrapper() }
}

pub fn v8value_create_array_buffer(
    buffer: *mut ::std::os::raw::c_void,
    length: usize,
    release_callback: &mut V8arrayBufferReleaseCallback,
) -> V8value {
    unsafe {
        cef_v8value_create_array_buffer(buffer.as_raw(), length.as_raw(), release_callback.as_raw())
            .as_wrapper()
    }
}

pub fn v8value_create_array_buffer_with_copy(
    buffer: *mut ::std::os::raw::c_void,
    length: usize,
) -> V8value {
    unsafe {
        cef_v8value_create_array_buffer_with_copy(buffer.as_raw(), length.as_raw()).as_wrapper()
    }
}

pub fn v8value_create_function(name: &CefString, handler: &mut V8handler) -> V8value {
    unsafe { cef_v8value_create_function(name.as_raw(), handler.as_raw()).as_wrapper() }
}

pub fn v8value_create_promise() -> V8value {
    unsafe { cef_v8value_create_promise().as_wrapper() }
}

pub fn v8stack_trace_get_current(frame_limit: ::std::os::raw::c_int) -> V8stackTrace {
    unsafe { cef_v8stack_trace_get_current(frame_limit.as_raw()).as_wrapper() }
}

pub fn register_extension(
    extension_name: &CefString,
    javascript_code: &CefString,
    handler: &mut V8handler,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_register_extension(
            extension_name.as_raw(),
            javascript_code.as_raw(),
            handler.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn register_scheme_handler_factory(
    scheme_name: &CefString,
    domain_name: &CefString,
    factory: &mut SchemeHandlerFactory,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_register_scheme_handler_factory(
            scheme_name.as_raw(),
            domain_name.as_raw(),
            factory.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn clear_scheme_handler_factories() -> ::std::os::raw::c_int {
    unsafe { cef_clear_scheme_handler_factories().as_wrapper() }
}

pub fn execute_process(
    args: &MainArgs,
    application: &mut App,
    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_execute_process(
            args.as_raw(),
            application.as_raw(),
            windows_sandbox_info.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn initialize(
    args: &MainArgs,
    settings: &Settings,
    application: &mut App,
    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        cef_initialize(
            args.as_raw(),
            settings.as_raw(),
            application.as_raw(),
            windows_sandbox_info.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn get_exit_code() -> ::std::os::raw::c_int {
    unsafe { cef_get_exit_code().as_wrapper() }
}

pub fn shutdown() {
    unsafe { cef_shutdown() }
}

pub fn do_message_loop_work() {
    unsafe { cef_do_message_loop_work() }
}

pub fn run_message_loop() {
    unsafe { cef_run_message_loop() }
}

pub fn quit_message_loop() {
    unsafe { cef_quit_message_loop() }
}

pub fn browser_view_create(
    client: &mut Client,
    url: &CefString,
    settings: &BrowserSettings,
    extra_info: &mut DictionaryValue,
    request_context: &mut RequestContext,
    delegate: &mut BrowserViewDelegate,
) -> BrowserView {
    unsafe {
        cef_browser_view_create(
            client.as_raw(),
            url.as_raw(),
            settings.as_raw(),
            extra_info.as_raw(),
            request_context.as_raw(),
            delegate.as_raw(),
        )
        .as_wrapper()
    }
}

pub fn browser_view_get_for_browser(browser: &mut Browser) -> BrowserView {
    unsafe { cef_browser_view_get_for_browser(browser.as_raw()).as_wrapper() }
}

pub fn display_get_primary() -> Display {
    unsafe { cef_display_get_primary().as_wrapper() }
}

pub fn display_get_nearest_point(
    point: &Point,
    input_pixel_coords: ::std::os::raw::c_int,
) -> Display {
    unsafe {
        cef_display_get_nearest_point(point.as_raw(), input_pixel_coords.as_raw()).as_wrapper()
    }
}

pub fn display_get_matching_bounds(
    bounds: &Rect,
    input_pixel_coords: ::std::os::raw::c_int,
) -> Display {
    unsafe {
        cef_display_get_matching_bounds(bounds.as_raw(), input_pixel_coords.as_raw()).as_wrapper()
    }
}

pub fn display_get_count() -> usize {
    unsafe { cef_display_get_count().as_wrapper() }
}

pub fn display_get_alls(displays_count: *mut usize, displays: *mut &mut Display) {
    unsafe { cef_display_get_alls(displays_count.as_raw(), displays.as_raw()) }
}

pub fn display_convert_screen_point_to_pixels(point: &Point) -> Point {
    unsafe { cef_display_convert_screen_point_to_pixels(point.as_raw()).as_wrapper() }
}

pub fn display_convert_screen_point_from_pixels(point: &Point) -> Point {
    unsafe { cef_display_convert_screen_point_from_pixels(point.as_raw()).as_wrapper() }
}

pub fn display_convert_screen_rect_to_pixels(rect: &Rect) -> Rect {
    unsafe { cef_display_convert_screen_rect_to_pixels(rect.as_raw()).as_wrapper() }
}

pub fn display_convert_screen_rect_from_pixels(rect: &Rect) -> Rect {
    unsafe { cef_display_convert_screen_rect_from_pixels(rect.as_raw()).as_wrapper() }
}

pub fn panel_create(delegate: &mut PanelDelegate) -> Panel {
    unsafe { cef_panel_create(delegate.as_raw()).as_wrapper() }
}

pub fn window_create_top_level(delegate: &mut WindowDelegate) -> Window {
    unsafe { cef_window_create_top_level(delegate.as_raw()).as_wrapper() }
}
