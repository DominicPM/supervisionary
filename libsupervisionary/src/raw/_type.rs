//! # Bindings to Supervisionary's type interface
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

use crate::raw::{tags, ErrorCode, Handle, Name, RawHandle};
use std::{
    collections::HashSet, convert::TryFrom, iter::FromIterator,
    marker::PhantomData,
};

////////////////////////////////////////////////////////////////////////////////
// Pre-allocated type-related handles.
////////////////////////////////////////////////////////////////////////////////

/// A pre-allocated handle used to refer to the type-variable `A`.
pub const PREALLOCATED_HANDLE_TYPE_ALPHA: Handle<tags::Type> =
    Handle::new(2usize, PhantomData);
/// A pre-allocated handle used to refer to the type-variable `B`.
pub const PREALLOCATED_HANDLE_TYPE_BETA: Handle<tags::Type> =
    Handle::new(3usize, PhantomData);
/// A pre-allocated handle used to refer to the `Prop` type.
pub const PREALLOCATED_HANDLE_TYPE_PROP: Handle<tags::Type> =
    Handle::new(4usize, PhantomData);
/// A pre-allocated handle used to refer to the type of unary predicates.
pub const PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE: Handle<tags::Type> =
    Handle::new(5usize, PhantomData);
/// A pre-allocated handle used to refer to the type of binary predicates.
pub const PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE: Handle<tags::Type> =
    Handle::new(6usize, PhantomData);
/// A pre-allocated handle used to refer to the type of unary connectives.
pub const PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE: Handle<tags::Type> =
    Handle::new(7usize, PhantomData);
/// A pre-allocated handle used to refer to the type of binary connectives.
pub const PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE: Handle<tags::Type> =
    Handle::new(8usize, PhantomData);
/// A pre-allocated handle used to refer to the type of polymorphic quantifiers.
pub const PREALLOCATED_HANDLE_TYPE_QUANTIFIER: Handle<tags::Type> =
    Handle::new(9usize, PhantomData);

////////////////////////////////////////////////////////////////////////////////
// ABI bindings.
////////////////////////////////////////////////////////////////////////////////

extern "C" {
    /// Raw ABI binding to the `Type.IsRegistered` function.
    fn __type_is_registered(handle: RawHandle) -> bool;
    /// Raw ABI binding to the `Type.Register.Variable` function.
    fn __type_register_variable(name: Name) -> u64;
    /// Raw ABI binding to the `Type.Register.Combination` function.
    fn __type_register_combination(
        type_former_handle: RawHandle,
        argument_base: *const RawHandle,
        argument_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Type.Register.Function` function.
    fn __type_register_function(
        domain_handle: RawHandle,
        range_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Type.Split.Variable` function.
    fn __type_split_variable(handle: RawHandle, result: *mut Name) -> i32;
    /// Raw ABI binding to the `Type.Split.Combination` function.
    fn __type_split_combination(
        handle: RawHandle,
        type_former: *mut RawHandle,
        argument_base: *mut RawHandle,
        argument_length: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `Type.Split.Function` function.
    fn __type_split_function(
        handle: RawHandle,
        domain_handle: *mut RawHandle,
        range_handle: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Type.Test.Function` function.
    fn __type_test_variable(handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Type.Test.Combination` function.
    fn __type_test_combination(handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Type.Test.Function` function.
    fn __type_test_function(handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Type.Size` function.
    fn __type_size(handle: RawHandle, result: *mut u64) -> i32;
    /// Raw ABI binding to the `Type.Variables` function.
    fn __type_variables(
        handle: RawHandle,
        result_base: *mut Name,
        result_length: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `Type.Substitute` function.
    fn __type_substitute(
        handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
}

/// Returns `true` iff `handle` points-to a registered type in the kernel's
/// heap.
#[inline]
pub fn type_is_registered<H>(handle: H) -> bool
where
    H: AsRef<Handle<tags::Type>>,
{
    unsafe { __type_is_registered(*handle.as_ref().clone() as u64) }
}

/// Allocates a new type-variable with a given `name`.  Note that this function
/// enforces maximal sharing in the kernel: allocating a second type-variable
/// with the same name as a previously-allocated variable returns the handle of
/// the previously-allocated variable.
#[inline]
pub fn type_register_variable<N>(name: N) -> Handle<tags::Type>
where
    N: Into<Name>,
{
    let raw_handle: u64 = unsafe { __type_register_variable(name.into()) };

    Handle::new(raw_handle as usize, PhantomData)
}

/// Allocates a new type combination, wherein a type-former `type_former` is
/// applied to a list of argument types, `arguments`.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeFormerRegistered` if `type_former` does not
/// point-to an allocated type-former in the kernel's heaps.
///
/// Returns `ErrorCode::MismatchedArity` if the length of `arguments` does not
/// match the registered arity of `type_former`.
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if any of the handles in
/// `arguments` does not point-to an allocated type in the kernel's heaps.
pub fn type_register_combination<T, A>(
    type_former: T,
    arguments: Vec<A>,
) -> Result<Handle<tags::Type>, ErrorCode>
where
    T: Into<Handle<tags::TypeFormer>>,
    A: Into<Handle<tags::Type>> + Clone,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __type_register_combination(
            *type_former.into() as RawHandle,
            arguments
                .iter()
                .cloned()
                .map(|e| *e.into() as u64)
                .collect::<Vec<_>>()
                .as_ptr(),
            arguments.len() as u64,
            &mut result as *mut RawHandle,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Allocates a new function type from a domain type, pointed-to by
/// `domain_handle`, and a range type, pointed-to by `range_handle`.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if either of the handles
/// `domain_handle` or `range_handle` do not point-to allocated types in the
/// kernel's heaps.
pub fn type_register_function<D, R>(
    domain_handle: D,
    range_handle: R,
) -> Result<Handle<tags::Type>, ErrorCode>
where
    D: Into<Handle<tags::Type>>,
    R: Into<Handle<tags::Type>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __type_register_function(
            *domain_handle.into() as RawHandle,
            *range_handle.into() as RawHandle,
            &mut result as *mut RawHandle,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns the "size" of the type pointed-to by handle, if any.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
pub fn type_size<H>(handle: H) -> Result<usize, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
{
    let mut size: u64 = 0;

    let status = unsafe {
        __type_size(*handle.as_ref().clone() as u64, &mut size as *mut u64)
    };

    if status == 0 {
        Ok(size as usize)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns the name of the type-variable pointed-to by `handle`, if any.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
///
/// Returns `ErrorCode::NotATypeVariable` if `handle` does not point-to a
/// type-variable in the kernel's heaps.
pub fn type_split_variable<H>(handle: H) -> Result<Name, ErrorCode>
where
    H: Into<Handle<tags::Type>>,
{
    let mut result: Name = 0;

    let status = unsafe {
        __type_split_variable(
            *handle.into() as RawHandle,
            &mut result as *mut Name,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns the type-former handle, and the list of argument handles, of the
/// type combination pointed-to by `handle`, if any.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
///
/// Returns `ErrorCode::NotATypeCombination` if `handle` does not point-to a
/// type-combination in the kernel's heaps.
pub fn type_split_combination<H>(
    handle: H,
) -> Result<(Handle<tags::TypeFormer>, Vec<Handle<tags::Type>>), ErrorCode>
where
    H: Into<Handle<tags::Type>>,
{
    let handle = handle.into();
    let size = type_size(&handle)?;

    let mut type_former: u64 = 0;
    let mut arguments = vec![0u64; size];
    let mut argument_length: u64 = 0;

    let status = unsafe {
        __type_split_combination(
            *handle as RawHandle,
            &mut type_former as *mut RawHandle,
            arguments.as_mut_ptr() as *mut u64,
            &mut argument_length as *mut u64,
        )
    };

    if status == 0 {
        arguments.truncate(argument_length as usize);

        let arguments = arguments
            .iter()
            .map(|h| Handle::new(*h as usize, PhantomData))
            .collect();

        Ok((Handle::new(type_former as usize, PhantomData), arguments))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns the handles of the domain and range types of the type pointed-to by
/// `handle`, if any.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
///
/// Returns `ErrorCode::NotAFunctionType` if `handle` does not point to a
/// function type in the kernel's heaps.
pub fn type_split_function<H>(
    handle: H,
) -> Result<(Handle<tags::Type>, Handle<tags::Type>), ErrorCode>
where
    H: Into<Handle<tags::Type>>,
{
    let mut domain_handle: u64 = 0;
    let mut range_handle: u64 = 0;

    let status = unsafe {
        __type_split_function(
            *handle.into() as u64,
            &mut domain_handle as *mut u64,
            &mut range_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(domain_handle as usize, PhantomData),
            Handle::new(range_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns `Ok(true)` iff `handle` points to a type-variable in the kernel's
/// heaps.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
pub fn type_test_variable<H>(handle: H) -> Result<bool, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
{
    let mut result = false;

    let status = unsafe {
        __type_test_variable(
            *handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns `Ok(true)` iff `handle` points to a type combination in the kernel's
/// heaps.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
pub fn type_test_combination<H>(handle: H) -> Result<bool, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
{
    let mut result = false;

    let status = unsafe {
        __type_test_combination(
            *handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns `Ok(true)` iff `handle` points to a function type in the kernel's
/// heaps.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
pub fn type_test_function<H>(handle: H) -> Result<bool, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
{
    let mut result = false;

    let status = unsafe {
        __type_test_function(
            *handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Returns the set of type-variables of the type pointed-to by `handle`, if any.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle` does not point-to an
/// allocated type in the kernel's heaps.
pub fn type_variables<H>(handle: H) -> Result<HashSet<Name>, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
{
    let size = type_size(&handle)?;

    let mut variables = vec![0u64; size];
    let mut variables_length: u64 = 0;

    let status = unsafe {
        __type_variables(
            *handle.as_ref().clone() as u64,
            variables.as_mut_ptr() as *mut u64,
            &mut variables_length as *mut u64,
        )
    };

    if status == 0 {
        variables.truncate(variables_length as usize);

        Ok(HashSet::from_iter(variables))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

/// Performs a substitution of the variables in the type pointed-to by `handle`
/// with `substitution`.
///
/// # Errors
///
/// Returns `ErrorCode::NoSuchTypeRegistered` if `handle`, or any of the types
/// appearing in the range of `substitution`, do not point-to an allocated type
/// in the kernel's heaps.
pub fn type_substitute<H, N, T>(
    handle: H,
    substitution: Vec<(N, T)>,
) -> Result<Handle<tags::Type>, ErrorCode>
where
    H: AsRef<Handle<tags::Type>>,
    N: Into<Name> + Clone,
    T: Into<Handle<tags::Type>> + Clone,
{
    let mut result: u64 = 0;
    let (domain, range): (Vec<_>, Vec<_>) = substitution
        .iter()
        .cloned()
        .map(|(d, r)| (d.into(), *r.into() as u64))
        .unzip();

    let status = unsafe {
        __type_substitute(
            *handle.as_ref().clone() as u64,
            domain.as_ptr() as *const u64,
            domain.len() as u64,
            range.as_ptr() as *const u64,
            range.len() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}
