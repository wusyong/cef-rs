use cef_sys::{cef_app_t, cef_command_line_t, cef_string_t};

use crate::{
    args::Args, command_line::CommandLine, rc::RcImpl, settings::Settings, string::CefString,
};

/// Implement this trait to provide handler implementations. Methods will be
/// called by the process and/or thread indicated.
pub trait App: Clone {
    /// Provides an opportunity to view and/or modify command-line arguments
    /// before processing by CEF and Chromium. The `process_type` value will be
    /// `None` for the browser process. Do not keep a reference to the
    /// `cef_command_line_t` object passed to this function. The
    /// `cef_settings_t.command_line_args_disabled` value can be used to start with
    /// an NULL command-line object. Any values specified in CefSettings that
    /// equate to command-line arguments will be set before this function is
    /// called. Be cautious when using this function to modify command-line
    /// arguments for non-browser processes as this may result in undefined
    /// behavior including crashes.
    fn on_before_command_line_processing(
        &self,
        _process_type: Option<CefString>,
        _command_line: Option<CommandLine>,
    ) {
    }

    /// Create cef raw types for internal usafe. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide to wrap your own type with `Arc` or
    /// perform deep clone.
    fn to_raw(&self) -> *mut cef_app_t {
        let mut object: cef_app_t = unsafe { std::mem::zeroed() };

        object.on_before_command_line_processing = Some(on_before_command_line_processing::<Self>);

        RcImpl::new(object, self.clone()) as *mut _
    }
}

/// This function should be called from the application entry point function to
/// execute a secondary process. It can be used to run secondary processes from
/// the browser client executable (default behavior) or from a separate
/// executable specified by the `cef_settings_t.browser_subprocess_path` value. If
/// called for the browser process (identified by no "type" command-line value)
/// it will return immediately with a value of -1. If called for a recognized
/// secondary process it will block until the process should exit and then
/// return the process exit code. The `application` parameter may be `None`.
pub fn execute_process<T: App>(args: &Args, app: Option<T>) -> i32 {
    let args = args.to_raw();
    let app = app.map(|app| app.to_raw()).unwrap_or(std::ptr::null_mut());

    unsafe { cef_sys::cef_execute_process(&args, app, std::ptr::null_mut()) }
}

/// This function should be called on the main application thread to initialize
/// the CEF browser process. The |application| parameter may be NULL. A return
/// value of true (1) indicates that it succeeded and false (0) indicates that
/// it failed.
pub fn initialize<T: App>(args: &Args, settings: Settings, app: Option<T>) -> i32 {
    let args = args.to_raw();
    let settings = &settings.into_raw() as *const _;
    let app = app.map(|app| app.to_raw()).unwrap_or(std::ptr::null_mut());

    unsafe { cef_sys::cef_initialize(&args, settings, app, std::ptr::null_mut()) }
}

extern "C" fn on_before_command_line_processing<I: App>(
    this: *mut cef_app_t,
    process_type: *const cef_string_t,
    command_line: *mut cef_command_line_t,
) {
    let obj: &mut RcImpl<_, I> = RcImpl::get(this);
    let process_type = CefString::from_raw(process_type);
    let cmd = CommandLine::from_raw(command_line);

    obj.interface
        .on_before_command_line_processing(process_type, cmd);
}
