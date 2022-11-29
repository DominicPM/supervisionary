//! # Bindings to Supervisionary's term ABI
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
// Pre-allocated term-related handles.
////////////////////////////////////////////////////////////////////////////////

/// A pre-allocated handle used to refer to the truth term, the truth constant
/// lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_TRUE: Handle<tags::Term> =
    Handle::new(19usize, PhantomData);
/// A pre-allocated handle used to refer to the falsity term, the falsity
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_FALSE: Handle<tags::Term> =
    Handle::new(20usize, PhantomData);
/// A pre-allocated handle used to refer to the negation term, the negation
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_NEGATION: Handle<tags::Term> =
    Handle::new(21usize, PhantomData);
/// A pre-allocated handle used to refer to the conjunction term, the
/// conjunction constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_CONJUNCTION: Handle<tags::Term> =
    Handle::new(22usize, PhantomData);
/// A pre-allocated handle used to refer to the disjunction term, the
/// disjunction constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_DISJUNCTION: Handle<tags::Term> =
    Handle::new(23usize, PhantomData);
/// A pre-allocated handle used to refer to the implication term, the
/// implication constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_IMPLICATION: Handle<tags::Term> =
    Handle::new(24usize, PhantomData);
/// A pre-allocated handle used to refer to the equality term, the equality
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_EQUALITY: Handle<tags::Term> =
    Handle::new(25usize, PhantomData);
/// A pre-allocated handle used to refer to the universal quantifier term, the
/// universal quantifier constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_FORALL: Handle<tags::Term> =
    Handle::new(26usize, PhantomData);
/// A pre-allocated handle used to refer to the existential quantifier term, the
/// existential quantifier constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_EXISTS: Handle<tags::Term> =
    Handle::new(27usize, PhantomData);

////////////////////////////////////////////////////////////////////////////////
// ABI bindings.
////////////////////////////////////////////////////////////////////////////////

extern "C" {
    /// Raw ABI binding to the `Term.IsRegistered` function.
    fn __term_is_registered(handle: RawHandle) -> bool;
    /// Raw ABI binding to the `Term.Register.Variable` function.
    fn __term_register_variable(
        name: Name,
        type_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Constant` function.
    fn __term_register_constant(
        constant_handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Application` function.
    fn __term_register_application(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Lambda` function.
    fn __term_register_lambda(
        bound_name: Name,
        type_handle: RawHandle,
        body_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Negation` function.
    fn __term_register_negation(
        body_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Conjunction` function.
    fn __term_register_conjunction(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Disjunction` function.
    fn __term_register_disjunction(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Implication` function.
    fn __term_register_implication(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Equality` function.
    fn __term_register_equality(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Forall` function.
    fn __term_register_forall(
        bound_name: Name,
        type_handle: RawHandle,
        body_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Register.Exists` function.
    fn __term_register_exists(
        bound_name: Name,
        type_handle: RawHandle,
        body_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Variable` function.
    fn __term_split_variable(
        term_handle: RawHandle,
        result_name: *mut Name,
        result_type: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Constant` function.
    fn __term_split_constant(
        term_handle: RawHandle,
        constant_handle: *mut RawHandle,
        type_handle: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Application` function.
    fn __term_split_application(
        term_handle: RawHandle,
        result_left: *mut RawHandle,
        result_right: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Lambda` function.
    fn __term_split_lambda(
        term_handle: RawHandle,
        result_name: *mut Name,
        result_type: *mut RawHandle,
        result_body: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Negation` function.
    fn __term_split_negation(
        term_handle: RawHandle,
        result_body: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Conjunction` function.
    fn __term_split_conjunction(
        term_handle: RawHandle,
        result_left: *mut RawHandle,
        result_right: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Disjunction` function.
    fn __term_split_disjunction(
        term_handle: RawHandle,
        result_left: *mut RawHandle,
        result_right: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Implication` function.
    fn __term_split_implication(
        term_handle: RawHandle,
        result_left: *mut RawHandle,
        result_right: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Equality` function.
    fn __term_split_equality(
        term_handle: RawHandle,
        result_left: *mut RawHandle,
        result_right: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Forall` function.
    fn __term_split_forall(
        term_handle: RawHandle,
        result_name: *mut Name,
        result_type: *mut RawHandle,
        result_body: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Split.Exists` function.
    fn __term_split_exists(
        term_handle: RawHandle,
        result_name: *mut Name,
        result_type: *mut RawHandle,
        result_body: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Test.Variable` function.
    fn __term_test_variable(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Constant` function.
    fn __term_test_constant(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Application` function.
    fn __term_test_application(
        term_handle: RawHandle,
        result: *mut bool,
    ) -> i32;
    /// Raw ABI binding to the `Term.Test.Lambda` function.
    fn __term_test_lambda(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Negation` function.
    fn __term_test_negation(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Conjunction` function.
    fn __term_test_conjunction(
        term_handle: RawHandle,
        result: *mut bool,
    ) -> i32;
    /// Raw ABI binding to the `Term.Test.Disjunction` function.
    fn __term_test_disjunction(
        term_handle: RawHandle,
        result: *mut bool,
    ) -> i32;
    /// Raw ABI binding to the `Term.Test.Implication` function.
    fn __term_test_implication(
        term_handle: RawHandle,
        result: *mut bool,
    ) -> i32;
    /// Raw ABI binding to the `Term.Test.Equality` function.
    fn __term_test_equality(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Forall` function.
    fn __term_test_forall(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Test.Exists` function.
    fn __term_test_exists(term_handle: RawHandle, result: *mut bool) -> i32;
    /// Raw ABI binding to the `Term.Size` function.
    fn __term_size(handle: RawHandle, result: *mut u64) -> i32;
    /// Raw ABI binding to the `Term.FreeVariables` function.
    fn __term_free_variables(
        term_handle: RawHandle,
        result_name_base: *mut Name,
        result_name_length: *mut u64,
        result_type_base: *mut RawHandle,
        result_type_length: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `Term.Substitution` function.
    fn __term_substitution(
        term_handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        type_base: *const Name,
        type_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Type.Variables` function.
    fn __term_free_type_variables(
        term_handle: RawHandle,
        result_type_base: *mut RawHandle,
        result_type_length: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `Term.Type.Substitution` function.
    fn __term_type_substitution(
        term_handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Term.Type.Infer` function.
    fn __term_type_infer(term_handle: RawHandle, result: *mut RawHandle)
        -> i32;
    /// Raw ABI binding to the `Term.Type.IsProposition` function.
    fn __term_type_is_proposition(
        term_handle: RawHandle,
        result: *mut bool,
    ) -> i32;
}

#[inline]
pub fn term_is_registered<T>(handle: T) -> bool
where
    T: AsRef<Handle<tags::Term>>,
{
    unsafe { __term_is_registered(*handle.as_ref().clone() as u64) }
}

pub fn term_register_variable<N, T>(
    name: N,
    type_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_variable(
            name.into(),
            *type_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_constant<C, N, T>(
    constant_handle: C,
    substitution: Vec<(N, T)>,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    C: Into<Handle<tags::Constant>>,
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
        __term_register_constant(
            *constant_handle.into() as u64,
            domain.as_ptr(),
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

pub fn term_register_application<L, R>(
    left_handle: L,
    right_handle: R,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    L: Into<Handle<tags::Term>>,
    R: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_application(
            *left_handle.into() as u64,
            *right_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_lambda<N, T, B>(
    name: N,
    type_handle: T,
    body_handle: B,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
    B: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_lambda(
            name.into(),
            *type_handle.into() as u64,
            *body_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_negation<B>(
    body_handle: B,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    B: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_negation(
            *body_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_conjunction<L, R>(
    left_handle: L,
    right_handle: R,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    L: Into<Handle<tags::Term>>,
    R: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_conjunction(
            *left_handle.into() as u64,
            *right_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_disjunction<L, R>(
    left_handle: L,
    right_handle: R,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    L: Into<Handle<tags::Term>>,
    R: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_disjunction(
            *left_handle.into() as u64,
            *right_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_implication<L, R>(
    left_handle: L,
    right_handle: R,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    L: Into<Handle<tags::Term>>,
    R: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_implication(
            *left_handle.into() as u64,
            *right_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_equality<L, R>(
    left_handle: L,
    right_handle: R,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    L: Into<Handle<tags::Term>>,
    R: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_equality(
            *left_handle.into() as u64,
            *right_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_forall<N, T, B>(
    name: N,
    type_handle: T,
    body_handle: B,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
    B: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_forall(
            name.into(),
            *type_handle.into() as u64,
            *body_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_register_exists<N, T, B>(
    name: N,
    type_handle: T,
    body_handle: B,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
    B: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_register_exists(
            name.into(),
            *type_handle.into() as u64,
            *body_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_variable<T>(
    term_handle: T,
) -> Result<(Name, Handle<tags::Type>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_name: u64 = 0;
    let mut result_type_handle: u64 = 0;

    let status = unsafe {
        __term_split_variable(
            *term_handle.into() as u64,
            &mut result_name as *mut u64,
            &mut result_type_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            result_name,
            Handle::new(result_type_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_constant<T>(
    term_handle: T,
) -> Result<(Handle<tags::Constant>, Handle<tags::Type>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_constant_handle: u64 = 0;
    let mut result_type_handle: u64 = 0;

    let status = unsafe {
        __term_split_constant(
            *term_handle.into() as u64,
            &mut result_constant_handle as *mut u64,
            &mut result_type_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_constant_handle as usize, PhantomData),
            Handle::new(result_type_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_application<T>(
    term_handle: T,
) -> Result<(Handle<tags::Term>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_left_handle: u64 = 0;
    let mut result_right_handle: u64 = 0;

    let status = unsafe {
        __term_split_application(
            *term_handle.into() as u64,
            &mut result_left_handle as *mut u64,
            &mut result_right_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_left_handle as usize, PhantomData),
            Handle::new(result_right_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_lambda<T>(
    term_handle: T,
) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_name: u64 = 0;
    let mut result_type_handle: u64 = 0;
    let mut result_body_handle: u64 = 0;

    let status = unsafe {
        __term_split_lambda(
            *term_handle.into() as u64,
            &mut result_name as *mut u64,
            &mut result_type_handle as *mut u64,
            &mut result_body_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            result_name,
            Handle::new(result_type_handle as usize, PhantomData),
            Handle::new(result_body_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_negation<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_body_handle: u64 = 0;

    let status = unsafe {
        __term_split_negation(
            *term_handle.into() as u64,
            &mut result_body_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result_body_handle as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_conjunction<T>(
    term_handle: T,
) -> Result<(Handle<tags::Term>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_left_handle: u64 = 0;
    let mut result_right_handle: u64 = 0;

    let status = unsafe {
        __term_split_conjunction(
            *term_handle.into() as u64,
            &mut result_left_handle as *mut u64,
            &mut result_right_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_left_handle as usize, PhantomData),
            Handle::new(result_right_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_disjunction<T>(
    term_handle: T,
) -> Result<(Handle<tags::Term>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_left_handle: u64 = 0;
    let mut result_right_handle: u64 = 0;

    let status = unsafe {
        __term_split_disjunction(
            *term_handle.into() as u64,
            &mut result_left_handle as *mut u64,
            &mut result_right_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_left_handle as usize, PhantomData),
            Handle::new(result_right_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_implication<T>(
    term_handle: T,
) -> Result<(Handle<tags::Term>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_left_handle: u64 = 0;
    let mut result_right_handle: u64 = 0;

    let status = unsafe {
        __term_split_implication(
            *term_handle.into() as u64,
            &mut result_left_handle as *mut u64,
            &mut result_right_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_left_handle as usize, PhantomData),
            Handle::new(result_right_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_equality<T>(
    term_handle: T,
) -> Result<(Handle<tags::Term>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_left_handle: u64 = 0;
    let mut result_right_handle: u64 = 0;

    let status = unsafe {
        __term_split_equality(
            *term_handle.into() as u64,
            &mut result_left_handle as *mut u64,
            &mut result_right_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            Handle::new(result_left_handle as usize, PhantomData),
            Handle::new(result_right_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_exists<T>(
    term_handle: T,
) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_name: u64 = 0;
    let mut result_type_handle: u64 = 0;
    let mut result_body_handle: u64 = 0;

    let status = unsafe {
        __term_split_exists(
            *term_handle.into() as u64,
            &mut result_name as *mut u64,
            &mut result_type_handle as *mut u64,
            &mut result_body_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            result_name,
            Handle::new(result_type_handle as usize, PhantomData),
            Handle::new(result_body_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_split_forall<T>(
    term_handle: T,
) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let mut result_name: u64 = 0;
    let mut result_type_handle: u64 = 0;
    let mut result_body_handle: u64 = 0;

    let status = unsafe {
        __term_split_forall(
            *term_handle.into() as u64,
            &mut result_name as *mut u64,
            &mut result_type_handle as *mut u64,
            &mut result_body_handle as *mut u64,
        )
    };

    if status == 0 {
        Ok((
            result_name,
            Handle::new(result_type_handle as usize, PhantomData),
            Handle::new(result_body_handle as usize, PhantomData),
        ))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_variable<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_variable(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_constant<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_constant(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_application<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_application(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_lambda<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_lambda(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_negation<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_negation(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_conjunction<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_conjunction(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_disjunction<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_disjunction(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_implication<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_implication(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_equality<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_equality(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_forall<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_forall(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_test_exists<T>(term_handle: T) -> Result<bool, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_test_exists(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(result)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_size<T>(term_handle: T) -> Result<usize, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_size(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(result as usize)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_free_variables<T>(
    term_handle: T,
) -> Result<HashSet<(Name, Handle<tags::Type>)>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let size = term_size(&term_handle)?;

    let mut result_domain = vec![0u64; size];
    let mut result_range = vec![0u64; size];

    let mut result_domain_size: u64 = 0;
    let mut result_range_size: u64 = 0;

    let status = unsafe {
        __term_free_variables(
            *term_handle.as_ref().clone() as u64,
            result_domain.as_mut_ptr() as *mut u64,
            &mut result_domain_size as *mut u64,
            result_range.as_mut_ptr() as *mut u64,
            &mut result_range_size as *mut u64,
        )
    };

    if status == 0 {
        assert_eq!(result_domain_size, result_range_size);

        result_domain.truncate(result_domain_size as usize);
        result_range.truncate(result_range_size as usize);

        let substitute = result_domain
            .iter()
            .zip(result_range)
            .map(|(d, r)| (*d, Handle::new(r as usize, PhantomData)))
            .collect();

        Ok(substitute)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_free_type_variables<T>(
    term_handle: T,
) -> Result<HashSet<Name>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let size = term_size(&term_handle)?;

    let mut result = vec![0u64; size];
    let mut result_size: u64 = 0;

    let status = unsafe {
        __term_free_type_variables(
            *term_handle.as_ref().clone() as u64,
            result.as_mut_ptr() as *mut u64,
            &mut result_size as *mut u64,
        )
    };

    if status == 0 {
        result.truncate(result_size as usize);

        Ok(HashSet::from_iter(result))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_type_substitute<T, N, U>(
    term_handle: T,
    substitution: Vec<(N, U)>,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
    N: Into<Name> + Clone,
    U: Into<Handle<tags::Type>> + Clone,
{
    let mut result: u64 = 0;
    let (domain, range): (Vec<_>, Vec<_>) = substitution
        .iter()
        .cloned()
        .map(|(d, r)| (d.into(), *r.into() as u64))
        .unzip();

    let status = unsafe {
        __term_type_substitution(
            *term_handle.as_ref().clone() as u64,
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

pub fn term_substitute<T, N, S, U>(
    term_handle: T,
    substitution: Vec<((N, S), U)>,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
    N: Into<Name> + Clone,
    S: Into<Handle<tags::Type>> + Clone,
    U: Into<Handle<tags::Term>> + Clone,
{
    let mut result: u64 = 0;

    let mut domain: Vec<u64> = vec![];
    let mut types: Vec<u64> = vec![];
    let mut range: Vec<u64> = vec![];

    for ((d, t), r) in substitution.iter() {
        domain.push(d.clone().into());
        types.push(*t.clone().into() as u64);
        range.push(*r.clone().into() as u64);
    }

    let status = unsafe {
        __term_substitution(
            *term_handle.as_ref().clone() as u64,
            domain.as_ptr() as *const u64,
            domain.len() as u64,
            types.as_ptr() as *const u64,
            types.len() as u64,
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

pub fn term_type_infer<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __term_type_infer(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn term_type_is_proposition<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: AsRef<Handle<tags::Term>>,
{
    let mut result: bool = false;

    let status = unsafe {
        __term_type_is_proposition(
            *term_handle.as_ref().clone() as u64,
            &mut result as *mut bool,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}
