//! String module

use cef_sys::{
    _cef_string_list_t, _cef_string_map_t, _cef_string_multimap_t, _cef_string_utf16_t,
    _cef_string_utf8_t, _cef_string_wide_t,
};
use std::{
    fmt::{self, Display, Formatter},
    mem, ptr, slice,
};

use crate::CefString;

enum CefStringData<T> {
    Borrowed(Option<T>),
    Clear(T),
    UserFree(*mut T),
}

impl<T> Clone for CefStringData<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        let data: Option<&T> = self.into();
        let data = data.map(ptr::from_ref).unwrap_or(ptr::null());
        data.into()
    }
}

impl<T> Default for CefStringData<T> {
    fn default() -> Self {
        Self::Borrowed(None)
    }
}

impl<T> From<*mut T> for CefStringData<T> {
    fn from(value: *mut T) -> Self {
        Self::UserFree(value)
    }
}

impl<T> From<*const T> for CefStringData<T>
where
    T: Copy,
{
    fn from(value: *const T) -> Self {
        Self::Borrowed(unsafe { value.as_ref() }.copied())
    }
}

impl<T> From<T> for CefStringData<T>
where
    T: Copy,
{
    fn from(value: T) -> Self {
        Self::Clear(value)
    }
}

impl<'a, T> Into<Option<&'a T>> for &'a CefStringData<T> {
    fn into(self) -> Option<&'a T> {
        match self {
            CefStringData::Borrowed(value) => value.as_ref(),
            CefStringData::Clear(value) => Some(value),
            CefStringData::UserFree(value) => unsafe { value.as_ref() },
        }
    }
}

impl<'a, T> Into<Option<&'a mut T>> for &'a mut CefStringData<T> {
    fn into(self) -> Option<&'a mut T> {
        match self {
            CefStringData::Borrowed(value) => value.as_mut(),
            CefStringData::Clear(value) => Some(value),
            CefStringData::UserFree(value) => unsafe { value.as_mut() },
        }
    }
}

/// See [_cef_string_utf8_t] for more documentation.
#[derive(Clone, Default)]
pub struct CefStringUtf8(CefStringData<_cef_string_utf8_t>);

impl Drop for CefStringUtf8 {
    fn drop(&mut self) {
        unsafe {
            match &mut self.0 {
                CefStringData::UserFree(value) => {
                    value
                        .as_mut()
                        .map(|value| cef_sys::cef_string_userfree_utf8_free(value));
                }
                CefStringData::Clear(value) => {
                    cef_sys::cef_string_utf8_clear(value);
                }
                _ => {}
            }
        }
    }
}

impl From<&str> for CefStringUtf8 {
    fn from(value: &str) -> Self {
        Self(
            unsafe {
                let cef_string = cef_sys::cef_string_userfree_utf8_alloc();
                if !cef_string.is_null() {
                    cef_sys::cef_string_utf8_set(
                        value.as_bytes().as_ptr() as *const _,
                        value.as_bytes().len(),
                        cef_string,
                        1,
                    );
                }
                cef_string
            }
            .into(),
        )
    }
}

impl From<_cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl From<*const _cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: *const _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: *mut _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl Into<*const _cef_string_utf8_t> for &CefStringUtf8 {
    fn into(self) -> *const _cef_string_utf8_t {
        let data: Option<&_cef_string_utf8_t> = (&self.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&CefStringUtf16> for CefStringUtf8 {
    fn from(value: &CefStringUtf16) -> Self {
        Self(
            unsafe {
                let cef_string = ptr::null_mut();
                let value: *const _cef_string_utf16_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_utf16_to_utf8(str_, length, cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl From<&CefStringWide> for CefStringUtf8 {
    fn from(value: &CefStringWide) -> Self {
        Self(
            unsafe {
                let cef_string = ptr::null_mut();
                let value: *const _cef_string_wide_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_wide_to_utf8(str_, length, cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl Into<_cef_string_utf8_t> for CefStringUtf8 {
    fn into(self) -> _cef_string_utf8_t {
        match self.0 {
            CefStringData::Borrowed(Some(value)) => value,
            _ => unsafe { mem::zeroed() },
        }
    }
}

impl Into<*mut _cef_string_utf8_t> for CefStringUtf8 {
    fn into(self) -> *mut _cef_string_utf8_t {
        let value = match self.0 {
            CefStringData::UserFree(value) => value,
            _ => ptr::null_mut(),
        };
        mem::forget(self);
        value
    }
}

impl Into<*mut _cef_string_utf8_t> for &mut CefStringUtf8 {
    fn into(self) -> *mut _cef_string_utf8_t {
        mem::take(self).into()
    }
}

impl CefStringUtf8 {
    fn as_str(&self) -> Option<&str> {
        let data: Option<&_cef_string_utf8_t> = (&self.0).into();
        let (str_, length) = data.map(|value| (value.str_, value.length))?;
        Some(unsafe {
            let slice = slice::from_raw_parts(str_ as *const _, length);
            std::str::from_utf8_unchecked(slice)
        })
    }
}

impl Display for CefStringUtf8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(value) = self.as_str() {
            write!(f, "{value}")
        } else {
            Ok(())
        }
    }
}

/// See [_cef_string_utf16_t] for more documentation.
#[derive(Clone, Default)]
pub struct CefStringUtf16(CefStringData<_cef_string_utf16_t>);

impl Drop for CefStringUtf16 {
    fn drop(&mut self) {
        unsafe {
            match &mut self.0 {
                CefStringData::UserFree(value) => {
                    value
                        .as_mut()
                        .map(|value| cef_sys::cef_string_userfree_utf16_free(value));
                }
                CefStringData::Clear(value) => {
                    cef_sys::cef_string_utf16_clear(value);
                }
                _ => {}
            }
        }
    }
}

impl From<_cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl From<*const _cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: *const _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: *mut _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl Into<*const _cef_string_utf16_t> for &CefStringUtf16 {
    fn into(self) -> *const _cef_string_utf16_t {
        let data: Option<&_cef_string_utf16_t> = (&self.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&CefStringUtf8> for CefStringUtf16 {
    fn from(value: &CefStringUtf8) -> Self {
        Self(
            unsafe {
                let mut cef_string = mem::zeroed();
                let value: *const _cef_string_utf8_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_utf8_to_utf16(str_, length, &mut cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl From<&CefStringWide> for CefStringUtf16 {
    fn from(value: &CefStringWide) -> Self {
        Self(
            unsafe {
                let mut cef_string = mem::zeroed();
                let value: *const _cef_string_wide_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_wide_to_utf16(str_, length, &mut cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl Into<_cef_string_utf16_t> for CefStringUtf16 {
    fn into(self) -> _cef_string_utf16_t {
        match self.0 {
            CefStringData::Borrowed(Some(value)) => value,
            _ => unsafe { mem::zeroed() },
        }
    }
}

impl Into<*mut _cef_string_utf16_t> for CefStringUtf16 {
    fn into(self) -> *mut _cef_string_utf16_t {
        let value = match self.0 {
            CefStringData::UserFree(value) => value,
            _ => ptr::null_mut(),
        };
        mem::forget(self);
        value
    }
}

impl Into<*mut _cef_string_utf16_t> for &mut CefStringUtf16 {
    fn into(self) -> *mut _cef_string_utf16_t {
        mem::take(self).into()
    }
}

/// See [_cef_string_wide_t] for more documentation.
#[derive(Clone, Default)]
pub struct CefStringWide(CefStringData<_cef_string_wide_t>);

impl Drop for CefStringWide {
    fn drop(&mut self) {
        unsafe {
            match &mut self.0 {
                CefStringData::UserFree(value) => {
                    value
                        .as_mut()
                        .map(|value| cef_sys::cef_string_userfree_wide_free(value));
                }
                CefStringData::Clear(value) => {
                    cef_sys::cef_string_wide_clear(value);
                }
                _ => {}
            }
        }
    }
}

impl From<_cef_string_wide_t> for CefStringWide {
    fn from(value: _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl From<*const _cef_string_wide_t> for CefStringWide {
    fn from(value: *const _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_wide_t> for CefStringWide {
    fn from(value: *mut _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl Into<*const _cef_string_wide_t> for &CefStringWide {
    fn into(self) -> *const _cef_string_wide_t {
        let data: Option<&_cef_string_wide_t> = (&self.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&CefStringUtf8> for CefStringWide {
    fn from(value: &CefStringUtf8) -> Self {
        Self(
            unsafe {
                let mut cef_string = mem::zeroed();
                let value: *const _cef_string_utf8_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_utf8_to_wide(str_, length, &mut cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl From<&CefStringUtf16> for CefStringWide {
    fn from(value: &CefStringUtf16) -> Self {
        Self(
            unsafe {
                let mut cef_string = mem::zeroed();
                let value: *const _cef_string_utf16_t = value.into();
                if let Some((str_, length)) = value.as_ref().map(|value| (value.str_, value.length))
                {
                    cef_sys::cef_string_utf16_to_wide(str_, length, &mut cef_string);
                }
                cef_string
            }
            .into(),
        )
    }
}

impl Into<_cef_string_wide_t> for CefStringWide {
    fn into(self) -> _cef_string_wide_t {
        match self.0 {
            CefStringData::Borrowed(Some(value)) => value,
            _ => unsafe { mem::zeroed() },
        }
    }
}

impl Into<*mut _cef_string_wide_t> for CefStringWide {
    fn into(self) -> *mut _cef_string_wide_t {
        let value = match self.0 {
            CefStringData::UserFree(value) => value,
            _ => ptr::null_mut(),
        };
        mem::forget(self);
        value
    }
}

impl Into<*mut _cef_string_wide_t> for &mut CefStringWide {
    fn into(self) -> *mut _cef_string_wide_t {
        mem::take(self).into()
    }
}

/// See [_cef_string_list_t] for more documentation.
pub struct CefStringList(*mut _cef_string_list_t);

impl Drop for CefStringList {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_sys::cef_string_list_free(value));
        }
    }
}

impl From<*mut _cef_string_list_t> for CefStringList {
    fn from(value: *mut _cef_string_list_t) -> Self {
        Self(value)
    }
}

impl Into<*mut _cef_string_list_t> for &mut CefStringList {
    fn into(self) -> *mut _cef_string_list_t {
        self.0
    }
}

impl IntoIterator for CefStringList {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let list = unsafe { self.0.as_mut() };
        list.map(|list| {
            let count = unsafe { cef_sys::cef_string_list_size(list) };
            (0..count)
                .filter_map(|i| unsafe {
                    let mut value = mem::zeroed();
                    (cef_sys::cef_string_list_value(list, i, &mut value) > 0).then_some(value)
                })
                .map(|value| {
                    CefStringUtf8::from(&CefString::from(ptr::from_ref(&value))).to_string()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
    }
}

/// See [_cef_string_map_t] for more documentation.
pub struct CefStringMap(*mut _cef_string_map_t);

impl Drop for CefStringMap {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_sys::cef_string_map_free(value));
        }
    }
}

impl From<*mut _cef_string_map_t> for CefStringMap {
    fn from(value: *mut _cef_string_map_t) -> Self {
        Self(value)
    }
}

impl Into<*mut _cef_string_map_t> for &mut CefStringMap {
    fn into(self) -> *mut _cef_string_map_t {
        self.0
    }
}

impl IntoIterator for CefStringMap {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let map = unsafe { self.0.as_mut() };
        map.map(|map| {
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
                        CefStringUtf8::from(&CefString::from(ptr::from_ref(&key))).to_string(),
                        CefStringUtf8::from(&CefString::from(ptr::from_ref(&value))).to_string(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
    }
}

/// See [_cef_string_multimap_t] for more documentation.
pub struct CefStringMultimap(*mut _cef_string_multimap_t);

impl Drop for CefStringMultimap {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_sys::cef_string_multimap_free(value));
        }
    }
}

impl From<*mut _cef_string_multimap_t> for CefStringMultimap {
    fn from(value: *mut _cef_string_multimap_t) -> Self {
        Self(value)
    }
}

impl Into<*mut _cef_string_multimap_t> for &mut CefStringMultimap {
    fn into(self) -> *mut _cef_string_multimap_t {
        self.0
    }
}
