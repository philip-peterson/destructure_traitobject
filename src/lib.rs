#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![allow(dyn_drop)]

//! # traitobject
//!
//! Unsafe helpers for working with raw TraitObjects.

/// Get the data pointer from this trait object.
///
/// Highly unsafe, as there is no information about the type of the data.
pub unsafe fn data<T: ?Sized>(val: *const T) -> *const () {
    val as *const ()
}

/// Get the data pointer from this trait object, mutably.
///
/// Highly unsafe, as there is no information about the type of the data.
pub unsafe fn data_mut<T: ?Sized>(val: *mut T) -> *mut () {
    val as *mut ()
}

#[test]
fn test_simple() {
    let x = &7 as &dyn Send;
    unsafe { assert!(&7 == std::mem::transmute::<_, &i32>(data(x))) };
}

#[test]
fn test_mut() {
    let x = &mut 7 as &mut dyn Send;
    unsafe { assert!(&mut 7 == std::mem::transmute::<_, &i32>(data_mut(x))) };
}

/// A trait implemented for all trait objects.
///
/// Implementations for all traits in std are provided.
pub unsafe trait Trait {}

mod impls;

