use cef_sys::{
    cef_string_list_t, cef_string_map_t, cef_string_userfree_utf16_t, cef_string_utf16_t,
};
use std::collections::HashMap;
use std::ptr::null_mut;
use widestring::U16CString;

/// Helper type to deal with Cef string. It's essentially an Utf-16 C string.
#[derive(Debug, Default, Clone)]
pub struct CefString(pub U16CString);

impl CefString {
    pub fn new(s: &str) -> Self {
        Self(U16CString::from_str(s).expect("Failed to create CefString from str."))
    }

    /// Create a `CefString` from raw `cef_string_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns `None`.
    pub fn from_raw(ptr: *const cef_string_utf16_t) -> Option<CefString> {
        if ptr.is_null() {
            return None;
        } else {
            // It's a smart pointer, so cef retains ownership and will call the dtor
            unsafe {
                U16CString::from_ptr((*ptr).str_, (*ptr).length)
                    .ok()
                    .map(|x| CefString(x))
            }
        }
    }

    /// Create a `CefString` from raw `cef_string_userfree_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns `None`.
    pub fn from_userfree_cef(ptr: cef_string_userfree_utf16_t) -> Option<CefString> {
        let res = Self::from_raw(ptr);
        unsafe {
            cef_sys::cef_string_userfree_utf16_free(ptr);
        }
        res
    }

    pub fn into_raw(self) -> cef_string_utf16_t {
        extern "C" fn free_str(ptr: *mut u16) {
            if !ptr.is_null() {
                unsafe {
                    // Restore and drop
                    let _ = U16CString::from_raw(ptr);
                }
            }
        }

        cef_string_utf16_t {
            length: self.0.len(),
            str_: self.0.into_raw(),
            dtor: Some(free_str),
        }
    }
}

impl ToString for CefString {
    fn to_string(&self) -> String {
        self.0.to_string_lossy()
    }
}

pub fn str_to_cef(s: &str) -> cef_string_utf16_t {
    CefString::new(s).into_raw()
}

pub unsafe fn parse_string_list(ptr: cef_string_list_t) -> Vec<String> {
    let count = cef_sys::cef_string_list_size(ptr);
    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        let value = null_mut();
        if cef_sys::cef_string_list_value(ptr, i, value) > 0 {
            CefString::from_raw(value).map(|v| res.push(v.to_string()));
        }
    }
    res
}

pub unsafe fn parse_string_map(ptr: cef_string_map_t) -> HashMap<String, String> {
    let count = cef_sys::cef_string_map_size(ptr);
    let mut res = HashMap::with_capacity(count);
    for i in 0..count {
        let key = null_mut();
        let value = null_mut();
        cef_sys::cef_string_map_key(ptr, i, key);
        cef_sys::cef_string_map_value(ptr, i, value);

        CefString::from_raw(key)
            .map(|k| CefString::from_raw(value).map(|v| res.insert(k.to_string(), v.to_string())));
    }
    res
}
