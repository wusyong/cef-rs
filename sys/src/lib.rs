#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
mod bindings;

use std::ptr::null_mut;

pub use bindings::*;

impl Default for _cef_string_utf16_t {
    fn default() -> Self {
        Self {
            str_: null_mut(),
            length: 0,
            dtor: None,
        }
    }
}

impl Default for cef_log_severity_t {
    fn default() -> Self {
        Self::LOGSEVERITY_DEFAULT
    }
}

impl Default for cef_log_items_t {
    fn default() -> Self {
        Self::LOG_ITEMS_DEFAULT
    }
}

#[test]
fn test_init() {
    use std::ptr::*;

    unsafe {
        assert_eq!(cef_initialize(null(), null(), null_mut(), null_mut()), 0);
    };
}
