//! # Bindings to Supervisionary's constant ABI
//!
//! # Authors
//!
//! [Dominic Mulligan], Systems Research Group, [Arm Research] Cambridge.
//! [Nick Spinale], Systems Research Group, [Arm Research] Cambridge.
//!
//! # Copyright
//!
//! Copyright (c) Arm Limited, 2021.  All rights reserved (r).  Please see the
//! `LICENSE.markdown` file in the *Supervisionary* root directory for licensing
//! information.
//!
//! [Dominic Mulligan]: https://dominicpm.github.io
//! [Nick Spinale]: https://nickspinale.com
//! [Arm Research]: http://www.arm.com/research

use crate::raw::{tags, ErrorCode, Handle, RawHandle};
use std::{convert::TryFrom, marker::PhantomData};

////////////////////////////////////////////////////////////////////////////////
// Pre-allocated constant-related handles.
////////////////////////////////////////////////////////////////////////////////

/// A pre-allocated handle used to refer to the truth constant.
pub const PREALLOCATED_HANDLE_CONSTANT_TRUE: Handle<tags::Constant> =
    Handle::new(10usize, PhantomData);
/// A pre-allocated handle used to refer to the falsity constant.
pub const PREALLOCATED_HANDLE_CONSTANT_FALSE: Handle<tags::Constant> =
    Handle::new(11usize, PhantomData);
/// A pre-allocated handle used to refer to the negation constant.
pub const PREALLOCATED_HANDLE_CONSTANT_NEGATION: Handle<tags::Constant> =
    Handle::new(12usize, PhantomData);
/// A pre-allocated handle used to refer to the binary conjunction connective.
pub const PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION: Handle<tags::Constant> =
    Handle::new(13usize, PhantomData);
/// A pre-allocated handle used to refer to the binary disjunction connective.
pub const PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION: Handle<tags::Constant> =
    Handle::new(14usize, PhantomData);
/// A pre-allocated handle used to refer to the binary implication connective.
pub const PREALLOCATED_HANDLE_CONSTANT_IMPLICATION: Handle<tags::Constant> =
    Handle::new(15usize, PhantomData);
/// A pre-allocated handle used to refer to the universal quantifier constant.
pub const PREALLOCATED_HANDLE_CONSTANT_FORALL: Handle<tags::Constant> =
    Handle::new(16usize, PhantomData);
/// A pre-allocated handle used to refer to the existential quantifier constant.
pub const PREALLOCATED_HANDLE_CONSTANT_EXISTS: Handle<tags::Constant> =
    Handle::new(17usize, PhantomData);
/// A pre-allocated handle used to refer to the equality constant.
pub const PREALLOCATED_HANDLE_CONSTANT_EQUALITY: Handle<tags::Constant> =
    Handle::new(18usize, PhantomData);

////////////////////////////////////////////////////////////////////////////////
// ABI bindings.
////////////////////////////////////////////////////////////////////////////////

extern "C" {
    /// Raw ABI binding to the `__constant_is_registered` function.
    fn __constant_is_registered(handle: RawHandle) -> i32;
    /// Raw ABI binding to the `__constant_resolve` function.
    fn __constant_resolve(handle: RawHandle, result: *mut RawHandle) -> i32;
    /// Raw ABI binding to the `__constant_register` function.
    fn __constant_register(
        type_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
}

/// Returns `true` iff `handle` points-to an allocated constant in the kernel's
/// heaps.
#[inline]
pub fn constant_is_registered<H>(handle: H) -> bool
where
    H: AsRef<Handle<tags::Constant>>,
{
    let result =
        unsafe { __constant_is_registered(*handle.as_ref().clone() as u64) };

    result == 0
}

/// Returns the registered type of the constant pointed-to by `handle`, if any,
/// in the kernel's heaps.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchConstantRegistered` if `handle` does not point-to
/// any allocated constant in the kernel's heaps.
pub fn constant_resolve<H>(handle: H) -> Result<Handle<tags::Type>, ErrorCode>
where
    H: AsRef<Handle<tags::Constant>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __constant_resolve(
            *handle.as_ref().clone() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Allocates a new constant in the kernel's heap with a registered type
/// pointed-to by `type_handle`.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `type_handle` does not point-to
/// an allocated type in the kernel's heaps.
pub fn constant_register<H>(
    type_handle: H,
) -> Result<Handle<tags::Constant>, ErrorCode>
where
    H: Into<Handle<tags::Type>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __constant_register(*type_handle.into() as u64, &mut result as *mut u64)
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}
