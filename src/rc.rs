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
//! `get_raw` that can create the raw cef type.
//!
//! ## If raw cef type has [`cef_base_ref_counted_t`]...
//!
//! ## ...and it's a delegate type we should create in Rust and pass to C API
//!
//! For example like [`cef_window_delegate_t`], Define a trait like [`WindowDelegate`] with trait bound of [`Sized`].
//! We define a trampoline function with the same signature, and then define a trait method like
//! [`WindowDelegate::on_window_created`]. Finally, define a trait method [`into_raw`] that can
//! create raw cef type with reference counted. In the implementation of [`into_raw`], create the raw
//! cef type by `unsafe { std::mem::zeroed }` first. And then fill each field by adding the
//! trampoline function. Return the value by calling [`RcImpl::new`]. This is the wrapper to add
//! [`cef_base_ref_counted_t`] to the type, so the trampoline function can call [`RcImpl::get`] to
//! retreive rust type and use it.
//!
//! ### ... and if it's a type we sould get from C API
//! For example like [`cef_window_t`], it should implement [`Rc`] trait fisrt.
//! There are some private macros `impl_rc` in this module for you to implement it.
//! And then define a new type like [`Window`] to wrap the raw type with [`RefGuard`].
//! Finally, define a method called `from_raw`. For more implementation details, please see the
//! documentation of [`RefGuard`].
//!
//! [`cef_settings_t`]: cef_sys::cef_settings_t
//! [`cef_window_delegate_t`]: cef_sys::cef_window_delegate_t
//! [`Settings`]: crate::Settings
//! [`WindowDelegate`]: crate::WindowDelegate
//! [`WindowDelegate::on_window_created`]: crate::WindowDelegate::on_window_created
//! [`into_raw`]: crate::WindowDelegate::into_raw
//! [`cef_window_t`]: cef_sys::cef_window_t
//! [`Window`]: crate::Window

use std::{
    ops::Deref,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

use cef_sys::cef_base_ref_counted_t;

/// Reference counted trait for types has [`cef_base_ref_counted_t`].
pub trait Rc {
    /// Increase the reference count by 1.
    ///
    /// # Safety
    ///
    /// Calling this method when you need to manually handle the reference count.
    /// Otherwise, these methods shouldn't be called externally in most cases.
    unsafe fn add_ref(&self) {
        self.as_base().add_ref();
    }

    /// Decrease reference count by 1 and release the value if the count meets 0.
    /// Reuturn `True` if it is released.
    unsafe fn release(&self) -> bool {
        self.as_base().release()
    }

    /// `True` if the reference count is exactly 1.
    fn has_one_ref(&self) -> bool {
        self.as_base().has_one_ref()
    }

    /// `True` if the reference count is larger than 0.
    fn has_at_least_one_ref(&self) -> bool {
        self.as_base().has_at_least_one_ref()
    }

    /// Get the reference of [cef_base_ref_counted_t].
    fn as_base(&self) -> &cef_base_ref_counted_t;
}

impl Rc for cef_base_ref_counted_t {
    unsafe fn add_ref(&self) {
        if let Some(add_ref) = self.add_ref {
            add_ref(self as *const _ as *mut _);
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

    unsafe fn release(&self) -> bool {
        if let Some(release) = self.release {
            return release(self as *const _ as *mut _) == 1;
        }

        false
    }

    fn as_base(&self) -> &Self {
        self
    }
}

#[macro_export]
macro_rules! gen_fn {
    ($visibility:vis fn $method:ident(
        $($arg:ident: $t:ty)*)
    $(-> $($n:ident)?$value:ty)?) => {
        $visibility fn $method(&self $(,$a: $t)*) $(-> $value)? {
            unsafe {
                let _result = self.0.$method.map(|f|
                    f(self.0.get_raw() $(,crate::gen_fn!($c $arg))*)
                );

                $(crate::gen_fn!(return $($n)? _result))?
            }
        }
    };
    (into $arg:ident) => {
        $arg.0.into_raw()
    };
    (return $result:ident) => {
        $result
            .filter(|p| p.is_null())
            .map(|p| BrowserView(RefGuard::from_raw(p)))
    }
}

#[macro_export]
macro_rules! wrapper {
    (
    $(#[$attr:meta])*
    pub struct $name:ident($sys:ident);
    $($visibility:vis fn $method:ident(
        &self
        $(,$arg:ident: [$ref:ident] $type:ty)*)
    $(->$value:ty)?;)*
    ) => {
        $(#[$attr])*
        pub struct $name(pub(crate) crate::rc::RefGuard<$sys>);

        impl crate::rc::Rc for $sys {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                &self.base.as_base()
            }
        }

        impl crate::rc::Rc for $name {
            fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
                self.0.as_base()
            }
        }

        impl $name {
            pub unsafe fn from_raw(ptr: *mut $sys) -> Self {
                Self(RefGuard::from_raw(ptr))
            }

            pub unsafe fn into_raw(self) -> *mut $sys {
                self.0.into_raw()
            }

            $(crate::gen_fn!($visibility fn $method(
                $($arg: $ref $type)*
            )$(-> $value)?);)*
        }
    };
}
pub use gen_fn;
pub use wrapper;

/// A smart pointer for types from cef library.
#[derive(Debug)]
pub struct RefGuard<T: Rc> {
    object: *mut T,
}

impl<T: Rc> RefGuard<T> {
    /// Create [RefGuard] from a raw C pointer.
    ///
    /// # Safety
    ///
    /// This should be used to get the **return value** of the FFI function. This means we get the
    /// ownership of the value. The reference count of the return value is already increased when
    /// you get it. So we don't need to increase it again manually. Using this method elsewhere may
    /// cause incorrect reference count and memory safty issues.
    pub unsafe fn from_raw(ptr: *mut T) -> RefGuard<T> {
        RefGuard { object: ptr }
    }

    /// Create [RefGuard] from a raw C pointer and increase a reference count. This should be used
    /// when you want to copy the value and create another wrapper type.
    ///
    /// # Safety
    ///
    /// THis should be used when you want to manually increase the reference count upon getting the
    /// raw pointer. Using this method elsewehre may cause incorrect reference count and memory
    /// safety issues.
    pub unsafe fn from_raw_add_ref(ptr: *mut T) -> RefGuard<T> {
        let guard = RefGuard { object: ptr };

        guard.add_ref();

        guard
    }

    // Get the raw pointer of [RefGuard].
    //
    /// # Safety
    ///
    /// This should be used when you need to pass wrapper type to the FFI function as **parameter**, and it **is**
    /// the `self` type (usually the first parameter). This means we pass the ownership of the
    /// value to the function call. Using this method elsewehre may cause incorrect reference count
    /// and memory safety issues.
    pub unsafe fn get_raw(&self) -> *mut T {
        self.object
    }

    /// Consume the [RefGuard] and return the raw pointer without decrease the reference count.
    ///
    /// # Safety
    ///
    /// This should be used when you need to pass wrapper type to the FFI function as **parameter**, and it is **not**
    /// the `self` type (usually the first parameter). This means we pass the ownership of the
    /// value to the function call. Using this method elsewehre may cause incorrect reference count
    /// and memory safety issues.
    pub unsafe fn into_raw(self) -> *mut T {
        let ptr = unsafe { self.get_raw() };
        std::mem::forget(self);
        ptr
    }

    /// Convert the value to another value that is also reference counted.
    ///
    /// # Safety
    ///
    /// This should be used when the type has type `U` as its base type. Using this method
    /// elsewhere may cause memory safety issues.
    pub unsafe fn convert<U: Rc>(&self) -> RefGuard<U> {
        RefGuard::from_raw_add_ref(self.get_raw() as *mut _)
    }
}

unsafe impl<T: Rc> Send for RefGuard<T> {}
unsafe impl<T: Rc> Sync for RefGuard<T> {}

impl<T: Rc> Clone for RefGuard<T> {
    fn clone(&self) -> RefGuard<T> {
        unsafe { self.add_ref() };

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
        unsafe { self.release() };
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
