//! String module

use std::{
    fmt::{self, Display, Formatter},
    mem, slice,
};

use crate::{
    rc::ConvertParam, CefString, CefStringList, CefStringMap, CefStringMultimap, CefStringUtf16,
    CefStringUtf8, CefStringWide,
};

impl Drop for CefStringUtf8 {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_utf8_clear(self.as_mut());
        }
    }
}

impl From<&str> for CefStringUtf8 {
    fn from(value: &str) -> Self {
        Self::from(unsafe {
            let mut cef_string = mem::zeroed();
            cef_sys::cef_string_utf8_set(
                value.as_bytes().as_ptr() as *const _,
                value.as_bytes().len(),
                &mut cef_string,
                1,
            );
            cef_string
        })
    }
}

impl ConvertParam<*const cef_sys::_cef_string_utf8_t> for &CefStringUtf8 {
    fn as_raw(self) -> *const cef_sys::_cef_string_utf8_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_utf8_t> for &mut CefStringUtf8 {
    fn as_raw(self) -> *mut cef_sys::_cef_string_utf8_t {
        self.as_mut() as *mut _
    }
}

impl Into<cef_sys::cef_string_userfree_utf8_t> for CefStringUtf8 {
    fn into(self) -> cef_sys::cef_string_userfree_utf8_t {
        unsafe {
            let cef_string = cef_sys::cef_string_userfree_utf8_alloc();
            if let Some(cef_string) = cef_string.as_mut() {
                let mut this = self;
                mem::swap(this.as_mut(), cef_string);
            }
            cef_string
        }
    }
}

impl From<cef_sys::cef_string_userfree_utf8_t> for CefStringUtf8 {
    fn from(value: cef_sys::cef_string_userfree_utf8_t) -> Self {
        unsafe {
            value.as_mut().map(|value| {
                let mut cef_string = mem::zeroed();
                mem::swap(value, &mut cef_string);
                cef_sys::cef_string_userfree_utf8_free(value);
                cef_string
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<*const cef_sys::cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: *const cef_sys::cef_string_utf8_t) -> Self {
        unsafe {
            value.as_ref().map(|value| cef_sys::cef_string_utf8_t {
                dtor: None,
                ..*value
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<&CefStringUtf16> for CefStringUtf8 {
    fn from(value: &CefStringUtf16) -> Self {
        Self::from(unsafe {
            let mut cef_string = mem::zeroed();
            cef_sys::cef_string_utf16_to_utf8(
                value.as_ref().str_ as *const _,
                value.as_ref().length,
                &mut cef_string,
            );
            cef_string
        })
    }
}

impl From<&CefStringWide> for CefStringUtf8 {
    fn from(value: &CefStringWide) -> Self {
        Self::from(unsafe {
            let mut cef_string = mem::zeroed();
            cef_sys::cef_string_wide_to_utf8(
                value.as_ref().str_ as *const _,
                value.as_ref().length,
                &mut cef_string,
            );
            cef_string
        })
    }
}

impl CefStringUtf8 {
    fn as_str(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts(self.as_ref().str_ as *const _, self.as_ref().length);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl Display for CefStringUtf8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Drop for CefStringUtf16 {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_utf16_clear(self.as_mut());
        }
    }
}

impl ConvertParam<*const cef_sys::_cef_string_utf16_t> for &CefStringUtf16 {
    fn as_raw(self) -> *const cef_sys::_cef_string_utf16_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_utf16_t> for &mut CefStringUtf16 {
    fn as_raw(self) -> *mut cef_sys::_cef_string_utf16_t {
        self.as_mut() as *mut _
    }
}

impl Into<cef_sys::cef_string_userfree_utf16_t> for CefStringUtf16 {
    fn into(self) -> cef_sys::cef_string_userfree_utf16_t {
        unsafe {
            let cef_string = cef_sys::cef_string_userfree_utf16_alloc();
            if let Some(cef_string) = cef_string.as_mut() {
                let mut this = self;
                mem::swap(this.as_mut(), cef_string);
            }
            cef_string
        }
    }
}

impl From<cef_sys::cef_string_userfree_utf16_t> for CefStringUtf16 {
    fn from(value: cef_sys::cef_string_userfree_utf16_t) -> Self {
        unsafe {
            value.as_mut().map(|value| {
                let mut cef_string = mem::zeroed();
                mem::swap(value, &mut cef_string);
                cef_sys::cef_string_userfree_utf16_free(value);
                cef_string
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<*const cef_sys::cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: *const cef_sys::cef_string_utf16_t) -> Self {
        unsafe {
            value.as_ref().map(|value| cef_sys::cef_string_utf16_t {
                dtor: None,
                ..*value
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<&CefStringUtf8> for CefStringUtf16 {
    fn from(value: &CefStringUtf8) -> Self {
        Self::from(unsafe {
            let mut cef_string = mem::zeroed();
            cef_sys::cef_string_utf8_to_utf16(
                value.as_ref().str_ as *const _,
                value.as_ref().length,
                &mut cef_string,
            );
            cef_string
        })
    }
}

impl Drop for CefStringWide {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_wide_clear(self.as_mut());
        }
    }
}

impl ConvertParam<*const cef_sys::_cef_string_wide_t> for &CefStringWide {
    fn as_raw(self) -> *const cef_sys::_cef_string_wide_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_wide_t> for &mut CefStringWide {
    fn as_raw(self) -> *mut cef_sys::_cef_string_wide_t {
        self.as_mut() as *mut _
    }
}

impl Into<cef_sys::cef_string_userfree_wide_t> for CefStringWide {
    fn into(self) -> cef_sys::cef_string_userfree_wide_t {
        unsafe {
            let cef_string = cef_sys::cef_string_userfree_wide_alloc();
            if let Some(cef_string) = cef_string.as_mut() {
                let mut this = self;
                mem::swap(this.as_mut(), cef_string);
            }
            cef_string
        }
    }
}

impl From<cef_sys::cef_string_userfree_wide_t> for CefStringWide {
    fn from(value: cef_sys::cef_string_userfree_wide_t) -> Self {
        unsafe {
            value.as_mut().map(|value| {
                let mut cef_string = mem::zeroed();
                mem::swap(value, &mut cef_string);
                cef_sys::cef_string_userfree_wide_free(value);
                cef_string
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<*const cef_sys::cef_string_wide_t> for CefStringWide {
    fn from(value: *const cef_sys::cef_string_wide_t) -> Self {
        unsafe {
            value.as_ref().map(|value| cef_sys::cef_string_wide_t {
                dtor: None,
                ..*value
            })
        }
        .map(Self::from)
        .unwrap_or_default()
    }
}

impl From<&CefStringUtf8> for CefStringWide {
    fn from(value: &CefStringUtf8) -> Self {
        Self::from(unsafe {
            let mut cef_string = mem::zeroed();
            cef_sys::cef_string_utf8_to_wide(
                value.as_ref().str_ as *const _,
                value.as_ref().length,
                &mut cef_string,
            );
            cef_string
        })
    }
}

impl Drop for CefStringList {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_list_clear(self.as_mut() as *mut _);
        }
    }
}

impl ConvertParam<*const cef_sys::_cef_string_list_t> for &CefStringList {
    fn as_raw(self) -> *const cef_sys::_cef_string_list_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_list_t> for &mut CefStringList {
    fn as_raw(self) -> *mut cef_sys::_cef_string_list_t {
        self.as_mut() as *mut _
    }
}

impl IntoIterator for CefStringList {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut list = self;
        let list = list.as_mut() as *mut _;
        let count = unsafe { cef_sys::cef_string_list_size(list) };
        (0..count)
            .filter_map(|i| unsafe {
                let mut value = mem::zeroed();
                (cef_sys::cef_string_list_value(list, i, &mut value) > 0).then_some(value)
            })
            .map(|value| CefStringUtf8::from(&CefString::from(value)).to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Drop for CefStringMap {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_map_clear(self.as_mut() as *mut _);
        }
    }
}

impl ConvertParam<*const cef_sys::_cef_string_map_t> for &CefStringMap {
    fn as_raw(self) -> *const cef_sys::_cef_string_map_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_map_t> for &mut CefStringMap {
    fn as_raw(self) -> *mut cef_sys::_cef_string_map_t {
        self.as_mut() as *mut _
    }
}

impl IntoIterator for CefStringMap {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut map = self;
        let map = map.as_mut() as *mut _;
        let count = unsafe { cef_sys::cef_string_map_size(map) };
        (0..count)
            .filter_map(|i| unsafe {
                let mut key = mem::zeroed();
                let mut value = mem::zeroed();
                (cef_sys::cef_string_map_key(map, i, &mut key) > 0
                    && cef_sys::cef_string_map_value(map, i, &mut value) > 0)
                    .then_some((key, value))
            })
            .map(|(key, value)| {
                (
                    CefStringUtf8::from(&CefString::from(key)).to_string(),
                    CefStringUtf8::from(&CefString::from(value)).to_string(),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Drop for CefStringMultimap {
    fn drop(&mut self) {
        unsafe {
            cef_sys::cef_string_multimap_clear(self.as_mut() as *mut _);
        }
    }
}

impl ConvertParam<*const cef_sys::_cef_string_multimap_t> for &CefStringMultimap {
    fn as_raw(self) -> *const cef_sys::_cef_string_multimap_t {
        self.as_ref() as *const _
    }
}

impl ConvertParam<*mut cef_sys::_cef_string_multimap_t> for &mut CefStringMultimap {
    fn as_raw(self) -> *mut cef_sys::_cef_string_multimap_t {
        self.as_mut() as *mut _
    }
}
