//! Reference counted module
//!
//! Many cef types are reference counted, this module is the building block to create them. Users
//! typically don't need to uses these types, but anyone who want to add feaures and
//! implementations to this crate will need to understand them.
//!
//! In order to create a new Rust type for a raw cef type, simply create a module for it first. And
//! then work on the implementations based on following conditions:
//!  
//! ## If raw cef type is a simple struct with basic fields
//! For example like [`cef_settings_t`], just create a struct like [`Settings`] and define a method
//! `into_raw` that can convert to raw cef type.
//!
//! ## If raw cef type has [`cef_base_ref_counted_t`]
//! For example like [`cef_app_t`], it should implement [`Rc`] trait.
//! There's a private macro `impl_rc` in this module for you to implement it. And for the Rust
//! type...
//!
//! ### if it's a type we should create in Rust and pass to C API
//!
//! Define a trait with [`Clone`] trait bound like [`App`] for it.
//! We need [`Clone`] trait because users will define a reference counted type based on it.
//! Each field of this kind of cef type usually is a callback like `Option<unsafe extern "C" fn(...)>`.
//! We define a trampoline function with  the same signature, and then define a trait method like
//! [`App::on_before_command_line_processing`]. Finally, define a trait method [`to_raw`] that can
//! create raw cef type with reference counted. In the implementation of [`to_raw`], create the raw
//! cef type by `unsafe { std::mem::zeroed }` first. And then fill each field by adding the
//! trampoline function. Return the value by calling [`RcImpl::new`]. This is the wrapper to add
//! [`cef_base_ref_counted_t`] to the type, so the trampoline function can call [`RcImpl::get`] to
//! retreive rust type and use it.
//!
//! ### if it's type we sould get from C API
//!
//! Define a new type like [`CommandLine`] to wrap the raw type with [`RefGuard`], and then define
//! a method called `from_raw`.
//!
//! [`cef_settings_t`]: cef_sys::cef_settings_t
//! [`cef_app_t`]: cef_sys::cef_app_t
//! [`Settings`]: crate::Settings
//! [`App`]: crate::App
//! [`App::on_before_command_line_processing`]: crate::App::on_before_command_line_processing
//! [`to_raw`]: crate::App::to_raw
//! [`CommandLine`]: crate::CommandLine

use std::{
    ops::Deref,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

use cef_sys::cef_base_ref_counted_t;

/// Reference counted trait for types has [`cef_base_ref_counted_t`].
pub trait Rc {
    fn add_ref(&self);
    fn has_one_ref(&self) -> bool;
    fn has_at_least_one_ref(&self) -> bool;
    fn release(&self) -> bool;
    fn as_base(&self) -> &cef_base_ref_counted_t;
}

impl Rc for cef_base_ref_counted_t {
    fn add_ref(&self) {
        if let Some(add_ref) = self.add_ref {
            unsafe {
                add_ref(self as *const _ as *mut _);
            }
        }
    }

    fn has_one_ref(&self) -> bool {
        if let Some(has_one_ref) = self.has_one_ref {
            let result = unsafe { has_one_ref(self as *const _ as *mut _) };

            return result == 1;
        }

        false
    }

    fn has_at_least_one_ref(&self) -> bool {
        if let Some(has_at_least_one_ref) = self.has_at_least_one_ref {
            let result = unsafe { has_at_least_one_ref(self as *const _ as *mut _) };

            return result == 1;
        }

        false
    }

    fn release(&self) -> bool {
        if let Some(release) = self.release {
            let result = unsafe { release(self as *const _ as *mut _) };

            return result == 1;
        }

        false
    }

    fn as_base(&self) -> &Self {
        self
    }
}

macro_rules! impl_rc {
    ($name:ident) => {
        impl Rc for cef_sys::$name {
            fn add_ref(&self) {
                self.as_base().add_ref();
            }

            fn has_one_ref(&self) -> bool {
                self.as_base().has_one_ref()
            }

            fn has_at_least_one_ref(&self) -> bool {
                self.as_base().has_at_least_one_ref()
            }

            fn release(&self) -> bool {
                self.as_base().release()
            }

            fn as_base(&self) -> &cef_base_ref_counted_t {
                &self.base
            }
        }
    };
}

impl_rc!(cef_browser_t);
impl_rc!(cef_command_line_t);

impl_rc!(cef_frame_t);
impl_rc!(cef_browser_host_t);
impl_rc!(cef_v8context_t);
impl_rc!(cef_v8value_t);
impl_rc!(cef_process_message_t);
impl_rc!(cef_list_value_t);
impl_rc!(cef_task_t);
impl_rc!(cef_task_runner_t);
impl_rc!(cef_context_menu_params_t);
impl_rc!(cef_menu_model_t);

/// A smart pointer for types from cef library.
pub struct RefGuard<T: Rc> {
    object: *mut T,
}

impl<T: Rc> RefGuard<T> {
    pub fn from_raw(ptr: *mut T) -> RefGuard<T> {
        RefGuard { object: ptr }
    }

    pub fn from_raw_add_ref(ptr: *mut T) -> RefGuard<T> {
        let guard = RefGuard { object: ptr };

        guard.add_ref();

        guard
    }

    /// clone value
    pub fn into_raw(self) -> *mut T {
        let ptr = unsafe { self.get_raw() };
        std::mem::forget(self);
        ptr
    }

    pub unsafe fn get_raw(&self) -> *mut T {
        self.object
    }
}

unsafe impl<T: Rc> Send for RefGuard<T> {}

impl<T: Rc> Clone for RefGuard<T> {
    fn clone(&self) -> RefGuard<T> {
        self.add_ref();

        RefGuard {
            object: self.object,
        }
    }
}

impl<T: Rc> Deref for RefGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.object }
    }
}

impl<T: Rc> Drop for RefGuard<T> {
    fn drop(&mut self) {
        self.release();
    }
}

/// There are some types require users to implement one their own in Rust and then create a raw type around it to
/// pass to sys level api. This is the wrapper type for it.
#[repr(C)]
pub struct RcImpl<T, I> {
    /// Raw cef types
    pub cef_object: T,
    /// Rust interface of such type
    pub interface: I,
    ref_count: AtomicUsize,
}

impl<T, I> RcImpl<T, I> {
    pub fn new(mut cef_object: T, interface: I) -> *mut RcImpl<T, I> {
        let base = unsafe { &mut *(&mut cef_object as *mut T as *mut cef_base_ref_counted_t) };

        base.size = std::mem::size_of::<T>();
        base.add_ref = Some(add_ref::<T, I>);
        base.has_one_ref = Some(has_one_ref::<T, I>);
        base.has_at_least_one_ref = Some(has_at_least_one_ref::<T, I>);
        base.release = Some(release::<T, I>);

        Box::into_raw(Box::new(RcImpl {
            cef_object,
            interface,
            ref_count: AtomicUsize::new(1),
        }))
    }

    pub fn get<'a>(ptr: *mut T) -> &'a mut RcImpl<T, I> {
        unsafe { &mut *(ptr as *mut RcImpl<T, I>) }
    }
}

extern "C" fn add_ref<T, I>(this: *mut cef_base_ref_counted_t) {
    let obj = RcImpl::<T, I>::get(this as *mut T);

    obj.ref_count.fetch_add(1, Ordering::Relaxed);
}

extern "C" fn has_one_ref<T, I>(this: *mut cef_base_ref_counted_t) -> i32 {
    let obj = RcImpl::<T, I>::get(this as *mut T);

    if obj.ref_count.load(Ordering::Relaxed) == 1 {
        1
    } else {
        0
    }
}

extern "C" fn has_at_least_one_ref<T, I>(this: *mut cef_base_ref_counted_t) -> i32 {
    let obj = RcImpl::<T, I>::get(this as *mut T);

    if obj.ref_count.load(Ordering::Relaxed) >= 1 {
        1
    } else {
        0
    }
}

pub extern "C" fn release<T, I>(this: *mut cef_base_ref_counted_t) -> i32 {
    let obj = RcImpl::<T, I>::get(this as *mut T);

    if obj.ref_count.fetch_sub(1, Ordering::Release) != 1 {
        0
    } else {
        fence(Ordering::Acquire);
        let _ = unsafe { Box::from_raw(this as *mut RcImpl<T, I>) };
        1
    }
}
