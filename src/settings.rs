use std::ffi::c_int;

use cef_sys::{cef_color_t, cef_settings_t};

use crate::{string::CefString, LogItems, LogSeverity};

/// See [cef_settings_t] for more documentation.
#[derive(Debug, Default, Clone)]
pub struct Settings {
    pub no_sandbox: bool,

    pub browser_subprocess_path: CefString,

    pub framework_dir_path: CefString,

    pub main_bundle_path: CefString,

    pub chrome_runtime: bool,

    pub multi_threaded_message_loop: bool,

    pub external_message_pump: bool,

    pub windowless_rendering_enabled: bool,

    pub command_line_args_disabled: bool,

    pub cache_path: CefString,

    pub root_cache_path: CefString,

    pub user_data_path: CefString,

    pub persist_session_cookies: bool,

    pub persist_user_preferences: bool,

    pub user_agent: CefString,

    pub user_agent_product: CefString,

    pub locale: CefString,

    pub log_file: CefString,

    pub log_severity: LogSeverity,

    pub log_items: LogItems,

    pub javascript_flags: CefString,

    pub resources_dir_path: CefString,

    pub locales_dir_path: CefString,

    pub pack_loading_disabled: bool,

    pub remote_debugging_port: u32,

    pub uncaught_exception_stack_size: u32,

    pub background_color: u32,

    pub accept_language_list: CefString,

    pub cookieable_schemes_list: CefString,

    pub cookieable_schemes_exclude_defaults: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            // no_sandbox: true,
            remote_debugging_port: 5566,
            // log_severity: LogSeverity::LOGSEVERITY_VERBOSE,
            // log_items: LogItems::LOG_ITEMS_DEFAULT,
            ..Default::default()
        }
    }

    pub fn get_raw(&self) -> cef_settings_t {
        cef_settings_t {
            size: std::mem::size_of::<cef_settings_t>(),
            no_sandbox: self.no_sandbox as c_int,
            browser_subprocess_path: self.browser_subprocess_path.get_raw(),
            framework_dir_path: self.framework_dir_path.get_raw(),
            main_bundle_path: self.main_bundle_path.get_raw(),
            chrome_runtime: self.chrome_runtime as c_int,
            multi_threaded_message_loop: self.multi_threaded_message_loop as c_int,
            external_message_pump: self.external_message_pump as c_int,
            windowless_rendering_enabled: self.windowless_rendering_enabled as c_int,
            command_line_args_disabled: self.command_line_args_disabled as c_int,
            cache_path: self.cache_path.get_raw(),
            root_cache_path: self.root_cache_path.get_raw(),
            persist_session_cookies: self.persist_session_cookies as c_int,
            persist_user_preferences: self.persist_user_preferences as c_int,
            user_agent: self.user_agent.get_raw(),
            user_agent_product: self.user_agent_product.get_raw(),
            locale: self.locale.get_raw(),
            log_file: self.log_file.get_raw(),
            log_severity: self.log_severity,
            log_items: self.log_items,
            javascript_flags: self.javascript_flags.get_raw(),
            resources_dir_path: self.resources_dir_path.get_raw(),
            locales_dir_path: self.locales_dir_path.get_raw(),
            pack_loading_disabled: self.pack_loading_disabled as c_int,
            remote_debugging_port: self.remote_debugging_port as c_int,
            uncaught_exception_stack_size: self.uncaught_exception_stack_size as c_int,
            background_color: self.background_color as cef_color_t,
            accept_language_list: self.accept_language_list.get_raw(),
            cookieable_schemes_list: self.cookieable_schemes_list.get_raw(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults as c_int,
        }
    }
}
