use cef_sys::cef_command_line_t;

use crate::rc::RefGuard;

/// Structure used to create and/or parse command line arguments. Arguments with
/// "--", "-" and, on Windows, "/" prefixes are considered switches. Switches
/// will always precede any arguments without switch prefixes. Switches can
/// optionally have a value specified using the "=" delimiter (e.g.
/// "-switch=value"). An argument of "--" will terminate switch parsing with all
/// subsequent tokens, regardless of prefix, being interpreted as non-switch
/// arguments. Switch names should be lowercase ASCII and will be converted to
/// such if necessary. Switch values will retain the original case and UTF8
/// encoding. This structure can be used before cef_initialize() is called.
pub struct CommandLine(RefGuard<cef_command_line_t>);

impl CommandLine {
    pub fn from_raw(raw: *mut cef_command_line_t) -> Option<CommandLine> {
        if raw.is_null() {
            None
        } else {
            Some(CommandLine(RefGuard::from_raw(raw)))
        }
    }

    // TODO draw the rest owl
}
