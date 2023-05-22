use cef_sys::{cef_string_list_t, cef_string_map_t, cef_string_utf16_t};
use std::collections::HashMap;
use std::ptr::null_mut;
use widestring::U16CString;

/// Helper type to deal with Cef string. It's essentially an Utf-16 C string.
#[derive(Debug, Clone)]
pub struct CefString(pub U16CString);

impl CefString {
    pub fn from_str(s: &str) -> Self {
        Self(U16CString::from_str(s).expect("Failed to create CefString from str."))
    }

    pub fn from_raw(ptr: *const cef_string_utf16_t) -> CefString {
        // It's a pointer, so CEF retains ownership and will call the dtor
        unsafe {
            CefString(
                U16CString::from_ptr((*ptr).str_, (*ptr).length)
                    .expect("Failed to create CefString from cef_string_utf16_t"),
            )
        }
    }

    pub fn from_userfree_cef(ptr: *mut cef_string_utf16_t) -> CefString {
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
    CefString::from_str(s).into_raw()
}

pub unsafe fn parse_string_list(ptr: cef_string_list_t) -> Vec<String> {
    let count = cef_sys::cef_string_list_size(ptr);
    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        let value = null_mut();
        if cef_sys::cef_string_list_value(ptr, i, value) > 0 {
            res.push(CefString::from_raw(value).to_string());
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

        res.insert(
            CefString::from_raw(key).to_string(),
            CefString::from_raw(value).to_string(),
        );
    }
    res
}
