use std::ffi::{c_char, CString};

use crate::MainArgs;

#[derive(Debug, Clone)]
pub struct Args {
    _source: Vec<CString>,
    argv: Vec<*const c_char>,
}

impl Args {
    pub fn new<T: IntoIterator<Item = String>>(args: T) -> Self {
        let _source = args
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();
        let argv = _source
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();
        Self { _source, argv }
    }

    pub fn to_raw(&self) -> MainArgs {
        MainArgs {
            argc: self.argv.len() as i32,
            argv: self.argv.as_ptr() as *mut *mut _,
        }
    }
}
