//! # Bindings to Supervisionary's type-former ABI
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

use crate::raw::{tags, Arity, ErrorCode, Handle, RawHandle};
use std::{convert::TryFrom, marker::PhantomData};

////////////////////////////////////////////////////////////////////////////////
// Pre-allocated type-former handles.
////////////////////////////////////////////////////////////////////////////////

/// A pre-allocated handle used to refer to the `Prop` type-former.
pub const PREALLOCATED_HANDLE_TYPE_FORMER_PROP: Handle<tags::TypeFormer> =
    Handle::new(0usize, PhantomData);
/// A pre-allocated handle used to refer to the function-space type-former.
pub const PREALLOCATED_HANDLE_TYPE_FORMER_ARROW: Handle<tags::TypeFormer> =
    Handle::new(1usize, PhantomData);

////////////////////////////////////////////////////////////////////////////////
// ABI bindings.
////////////////////////////////////////////////////////////////////////////////

extern "C" {
    /// Raw ABI binding to the `TypeFormer.Register` function.
    fn __type_former_register(handle: RawHandle) -> u64;
    /// Raw ABI binding to the `TypeFormer.IsRegistered` function.
    fn __type_former_is_registered(handle: RawHandle) -> bool;
    /// Raw ABI binding to the `TypeFormer.Resolve` function.
    fn __type_former_resolve(handle: RawHandle, out: *mut u64) -> i32;
}

/// Registers a new type-former with a given `arity`.  Returns the handle to the
/// new type-former.  Note that this function is generative, in the sense that
/// registering two type-formers with the same arity results in two different
/// type-formers.
#[inline]
pub fn type_former_register<T>(arity: T) -> Handle<tags::TypeFormer>
where
    T: Into<Arity>,
{
    let handle = unsafe { __type_former_register(arity.into()) };

    Handle::new(handle as usize, PhantomData)
}

/// Returns `true` iff `handle` points-to a registered type-former in the
/// kernel's heap.
#[inline]
pub fn type_former_is_registered<H>(handle: H) -> bool
where
    H: AsRef<Handle<tags::TypeFormer>>,
{
    unsafe { __type_former_is_registered(*handle.as_ref().clone() as u64) }
}

/// Returns the arity of the type-former pointed-to by `handle` in the kernel's
/// heap, if any.
pub fn type_former_resolve<H>(handle: H) -> Result<Arity, ErrorCode>
where
    H: AsRef<Handle<tags::TypeFormer>>,
{
    let mut arity: Arity = 0u64;

    let result = unsafe {
        __type_former_resolve(
            *handle.as_ref().clone() as u64,
            &mut arity as *mut u64,
        )
    };

    if result == 0 {
        Ok(arity)
    } else {
        Err(ErrorCode::try_from(result).unwrap())
    }
}
