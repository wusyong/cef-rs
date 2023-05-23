use std::ffi::{c_char, CString};

use cef_sys::cef_main_args_t;

pub struct Args(pub Vec<CString>);

impl Args {
    pub fn new() -> Self {
        Self(
            std::env::args()
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<CString>>(),
        )
    }

    pub fn to_raw(&self) -> cef_main_args_t {
        // convert the strings to raw pointers
        let mut c_args = self
            .0
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        cef_main_args_t {
            argc: c_args.len() as i32,
            argv: c_args.as_mut_ptr() as *mut *mut _,
        }
    }
}
