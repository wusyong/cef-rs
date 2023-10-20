use cef_sys::{cef_app_t, cef_command_line_t, cef_execute_process, cef_initialize, cef_string_t, cef_shutdown, cef_run_message_loop};

use crate::{
    args::Args, command_line::CommandLine, rc::RcImpl, settings::Settings, string::CefString,
};

/// See [cef_app_t] for more documentation.
pub trait App: Clone + Send + Sync {
    fn on_before_command_line_processing(
        &self,
        _process_type: Option<CefString>,
        _command_line: Option<CommandLine>,
    ) {
    }

    /// Create cef raw types for internal usage. The reason for `Clone` requirement is because
    /// these types have ref counted object. User can decide to wrap your own type with `Arc` or
    /// perform deep clone.
    fn to_raw(&self) -> *mut cef_app_t {
        let mut object: cef_app_t = unsafe { std::mem::zeroed() };

        object.on_before_command_line_processing = Some(on_before_command_line_processing::<Self>);

        RcImpl::new(object, self.clone()) as *mut _
    }
}

/// See [cef_execute_process] for more documentation.
pub fn execute_process<T: App>(args: &Args, app: Option<T>) -> i32 {
    let args = args.to_raw();
    let app = app.map(|app| app.to_raw()).unwrap_or(std::ptr::null_mut());

    unsafe { cef_execute_process(&args, app, std::ptr::null_mut()) }
}

/// See [cef_initialize] for more documentation.
pub fn initialize<T: App>(args: &Args, settings: Settings, app: Option<T>) -> bool {
    let args = args.to_raw();
    let settings = &settings.into_raw() as *const _;
    let app = app.map(|app| app.to_raw()).unwrap_or(std::ptr::null_mut());

    unsafe { cef_initialize(&args, settings, app, std::ptr::null_mut()) == 1 }
}

/// See [cef_run_message_loop] for more documentation.
pub fn run_message_loop() {
    unsafe { cef_run_message_loop() }
}

/// See [cef_shutdown] for more documentation.
pub fn shutdown() {
    unsafe { cef_shutdown() }
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
