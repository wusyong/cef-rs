use std::ffi::{c_char, CString};

use crate::MainArgs;

#[derive(Clone, Default)]
pub struct Args {
    _source: Vec<CString>,
    _argv: Vec<*const c_char>,
    main_args: MainArgs,
}

impl Args {
    pub fn new<T: IntoIterator<Item = String>>(args: T) -> Self {
        let _source = args
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();
        let _argv = _source
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();
        let main_args = MainArgs {
            argc: _argv.len() as i32,
            argv: _argv.as_ptr() as *mut *mut _,
        };

        Self {
            _source,
            _argv,
            main_args,
        }
    }

    pub fn as_raw(&self) -> &MainArgs {
        &self.main_args
    }
}
