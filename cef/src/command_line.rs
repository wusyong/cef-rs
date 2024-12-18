use cef_sys::cef_command_line_t;

use crate::{rc::RefGuard, wrapper};

wrapper!(
    #[doc = "See [cef_command_line_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct CommandLine(cef_command_line_t);
);
