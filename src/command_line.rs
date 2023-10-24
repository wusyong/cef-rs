use cef_sys::cef_command_line_t;

use crate::{impl_rc, rc::RefGuard};

/// See [cef_command_line_t] for more documentation.
pub struct CommandLine(RefGuard<cef_command_line_t>);
impl_rc!(CommandLine, cef_command_line_t);

impl CommandLine {
    pub fn from_raw(raw: *mut cef_command_line_t) -> Option<CommandLine> {
        if raw.is_null() {
            None
        } else {
            Some(CommandLine(unsafe { RefGuard::from_raw(raw) }))
        }
    }

    // TODO draw the rest owl
}
