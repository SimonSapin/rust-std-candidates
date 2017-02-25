//! The `Ref` and `RefMut` types in `std::cell` each have a `map` method that create
//! a new `Ref` (`RefMut`) that borrows something (a sub-component) inside of a `RefCell`.
//!
//! When that component may or may not be there,
//! you may find yourself checking for its precense twice:
//!
//! ```
//! # use std::cell::{RefCell, Ref};
//! # use std::collections::HashMap;
//! fn borrow_get<'a>(hashmap: &'a RefCell<HashMap<String, String>>, key: &str)
//!                   -> Option<Ref<'a, String>> {
//!     let hashmap = hashmap.borrow();
//!     if hashmap.contains_key(key) {  // Duplicated hash table lookup.
//!         Some(Ref::map(hashmap, |hashmap| {
//!             &hashmap[key]  // panic!() for missing key unlikely to be optimized away
//!         }))
//!     } else {
//!         None
//!     }
//! }
//! ```
//!
//! This crate define `ref_filter_map` and `ref_mut_filter_map` functions
//! that are a lot like `Ref::map` and `RefMut::map`,
//! but return `Option` and take closures that return `Option`.
//!
//! Internally they use a raw pointer and some `unsafe` code,
//! but the API they provide is believed to be safe.
//!
//! This was once part of `std::cell` but has been deprecated there since it makes `Option`
//! too much of a special case.
//!
//! https://github.com/rust-lang/rust/pull/25747
//! https://github.com/rust-lang/rust/issues/27746

use std::cell::{Ref, RefMut};

/// Make a new `Ref` for a optional component of the borrowed data, e.g. an enum variant.
///
/// The `RefCell` is already immutably borrowed, so this cannot fail.
///
/// This is an associated function that needs to be used as `Ref::filter_map(...)`.
/// A method would interfere with methods of the same name on the contents of a `RefCell`
/// used through `Deref`.
///
/// # Example
///
/// ```
/// use std::cell::{RefCell, Ref};
/// use ref_filter_map::ref_filter_map;
///
/// let c = RefCell::new(Ok(5));
/// let b1: Ref<Result<u32, ()>> = c.borrow();
/// let b2: Ref<u32> = ref_filter_map(b1, |o| o.as_ref().ok()).unwrap();
/// assert_eq!(*b2, 5)
/// ```
pub fn ref_filter_map<
    T: ?Sized,
    U: ?Sized,
    F: FnOnce(&T) -> Option<&U>
>(orig: Ref<T>, f: F) -> Option<Ref<U>> {
    f(&orig)
        .map(|new| new as *const U)
        .map(|raw| Ref::map(orig, |_| unsafe { &*raw }))
}

/// Make a new `RefMut` for a optional component of the borrowed data, e.g. an enum variant.
///
/// The `RefCell` is already mutably borrowed, so this cannot fail.
///
/// This is an associated function that needs to be used as `RefMut::filter_map(...)`.
/// A method would interfere with methods of the same name on the contents of a `RefCell`
/// used through `Deref`.
///
/// # Example
///
/// ```
/// use std::cell::{RefCell, RefMut};
/// use ref_filter_map::ref_mut_filter_map;
///
/// let c = RefCell::new(Ok(5));
/// {
///     let b1: RefMut<Result<u32, ()>> = c.borrow_mut();
///     let mut b2: RefMut<u32> = ref_mut_filter_map(b1, |o| o.as_mut().ok()).unwrap();
///     assert_eq!(*b2, 5);
///     *b2 = 42;
/// }
/// assert_eq!(*c.borrow(), Ok(42));
/// ```
pub fn ref_mut_filter_map<
    T: ?Sized,
    U: ?Sized,
    F: FnOnce(&mut T) -> Option<&mut U>
>(mut orig: RefMut<T>, f: F) -> Option<RefMut<U>> {
    f(&mut orig)
        .map(|new| new as *mut U)
        .map(|raw| RefMut::map(orig, |_| unsafe { &mut *raw }))
}
