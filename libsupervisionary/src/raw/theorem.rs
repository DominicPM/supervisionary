//! # Bindings to Supervisionary's theorem ABI
//!
//! # Authors
//!
//! [Dominic Mulligan], Systems Research Group, [Arm Research] Cambridge.
//!
//! # Copyright
//!
//! Copyright (c) Arm Limited, 2021.  All rights reserved (r).  Please see the
//! `LICENSE.markdown` file in the *Supervisionary* root directory for licensing
//! information.
//!
//! [Dominic Mulligan]<https://dominicpm.github.io>

use crate::raw::{tags, ErrorCode, Handle, Name, RawHandle};
use std::{convert::TryFrom, marker::PhantomData};

////////////////////////////////////////////////////////////////////////////////
// ABI bindings.
////////////////////////////////////////////////////////////////////////////////

/* TODO: add bindings for existential inference rules */
extern "C" {
    /// Raw ABI binding to the `Theorem.IsRegistered` function.
    fn __theorem_is_registered(theorem_handle: RawHandle) -> bool;
    /// Raw ABI binding to the `Theorem.Size` function.
    fn __theorem_size(theorem_handle: RawHandle, result: *mut u64) -> i32;
    /// Raw ABI binding to the `Theorem.Split.Conclusion` function.
    fn __theorem_split_conclusion(
        theorem_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Split.Hypotheses` function.
    fn __theorem_split_hypotheses(
        theorem_handle: RawHandle,
        hypotheses_base: *mut RawHandle,
        hypotheses_length: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Assumption` function.
    fn __theorem_register_assumption(
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Weaken` function.
    fn __theorem_register_weaken(
        term_handle: RawHandle,
        theorem_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Reflexivity` function.
    fn __theorem_register_reflexivity(
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Symmetry` function.
    fn __theorem_register_symmetry(
        theorem_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Transitivity` function.
    fn __theorem_register_transitivity(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Application` function.
    fn __theorem_register_application(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Lambda` function.
    fn __theorem_register_lambda(
        name: Name,
        type_handle: RawHandle,
        body_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Beta` function.
    fn __theorem_register_beta(
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Eta` function.
    fn __theorem_register_eta(
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Substitute` function.
    fn __theorem_register_substitute(
        theorem_handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        type_base: *const Name,
        type_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.TypeSubstitute` function.
    fn __theorem_register_type_substitute(
        theorem_handle: RawHandle,
        domain_base: *const Name,
        domain_length: u64,
        range_base: *const RawHandle,
        range_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Truth.Introduction` function.
    fn __theorem_register_truth_introduction(
        hypotheses_base: *const RawHandle,
        hypotheses_length: u64,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Falsity.Elimination` function.
    fn __theorem_register_falsity_elimination(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Conjunction.Introduction` function.
    fn __theorem_register_conjunction_introduction(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Conjunction.LeftElimination` function.
    fn __theorem_register_conjunction_left_elimination(
        left_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Conjunction.RightElimination` function.
    fn __theorem_register_conjunction_right_elimination(
        left_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Disjunction.LeftIntroduction` function.
    fn __theorem_register_disjunction_left_introduction(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Disjunction.RightIntroduction` function.
    fn __theorem_register_disjunction_right_introduction(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Disjunction.Elimination` function.
    fn __theorem_register_disjunction_elimination(
        left_handle: RawHandle,
        mid_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Negation.Introduction` function.
    fn __theorem_register_negation_introduction(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Negation.Elimination` function.
    fn __theorem_register_negation_elimination(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Implication.Elimination` function.
    fn __theorem_register_implication_introduction(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Implication.Elimination` function.
    fn __theorem_register_implication_elimination(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Iff.Introduction` function.
    fn __theorem_register_iff_introduction(
        left_handle: RawHandle,
        right_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Iff.LeftElimination` function.
    fn __theorem_register_iff_left_elimination(
        theorem_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Forall.Introduction` function.
    fn __theorem_register_forall_introduction(
        theorem_handle: RawHandle,
        name: Name,
        type_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
    /// Raw ABI binding to the `Theorem.Register.Forall.Elimination` function.
    fn __theorem_register_forall_elimination(
        theorem_handle: RawHandle,
        term_handle: RawHandle,
        result: *mut RawHandle,
    ) -> i32;
}

#[inline]
pub fn theorem_is_registered<T>(theorem_handle: T) -> bool
where
    T: AsRef<Handle<tags::Theorem>>,
{
    unsafe { __theorem_is_registered(*theorem_handle.as_ref().clone() as u64) }
}

pub fn theorem_size<T>(theorem_handle: T) -> Result<usize, ErrorCode>
where
    T: AsRef<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_size(
            *theorem_handle.as_ref().clone() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(result as usize)
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_split_conclusion<T>(
    theorem_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_split_conclusion(
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_split_hypotheses<T>(
    theorem_handle: T,
) -> Result<Vec<Handle<tags::Term>>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let theorem_handle = theorem_handle.into();
    let size = theorem_size(&theorem_handle)?;
    let mut hypotheses = vec![0u64; size];
    let mut hypothesis_count: u64 = 0;

    let status = unsafe {
        __theorem_split_hypotheses(
            *theorem_handle as u64,
            hypotheses.as_mut_ptr() as *mut u64,
            &mut hypothesis_count as *mut u64,
        )
    };

    if status == 0 {
        hypotheses.truncate(hypothesis_count as usize);

        Ok(hypotheses
            .iter()
            .map(|h| Handle::new(*h as usize, PhantomData))
            .collect())
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_assumption<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let term_handle = *term_handle.into() as u64;
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_assumption(term_handle, &mut result as *mut u64)
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_weaken<T, U>(
    term_handle: T,
    theorem_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
    U: Into<Handle<tags::Theorem>>,
{
    let term_handle = *term_handle.into() as u64;
    let theorem_handle = *theorem_handle.into() as u64;
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_weaken(
            term_handle,
            theorem_handle,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_reflexivity<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let term_handle = *term_handle.into() as u64;
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_reflexivity(term_handle, &mut result as *mut u64)
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_symmetry<T>(
    theorem_handle: T,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_symmetry(
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_transitivity<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_transitivity(
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

pub fn theorem_register_application<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_application(
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

pub fn theorem_register_lambda<N, T, U>(
    name: N,
    type_handle: T,
    theorem_handle: U,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_lambda(
            name.into(),
            *type_handle.into() as u64,
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_beta<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let term_handle = *term_handle.into() as u64;
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_beta(term_handle, &mut result as *mut u64)
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_eta<T>(
    term_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>>,
{
    let term_handle = *term_handle.into() as u64;
    let mut result: u64 = 0;

    let status =
        unsafe { __theorem_register_eta(term_handle, &mut result as *mut u64) };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_substitute<T, N, U, V>(
    theorem_handle: T,
    substitution: Vec<((N, U), V)>,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    N: Into<Name> + Clone,
    U: Into<Handle<tags::Type>> + Clone,
    V: Into<Handle<tags::Term>> + Clone,
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
        __theorem_register_substitute(
            *theorem_handle.into() as u64,
            domain.as_ptr(),
            domain.len() as u64,
            types.as_ptr(),
            types.len() as u64,
            range.as_ptr(),
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

pub fn theorem_register_type_substitute<T, N, U>(
    theorem_handle: T,
    substitution: Vec<(N, U)>,
) -> Result<Handle<tags::Theorem>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    N: Into<Name> + Clone,
    U: Into<Handle<tags::Type>> + Clone,
{
    let mut result: u64 = 0;

    let (domains, ranges): (Vec<_>, Vec<_>) = substitution
        .iter()
        .cloned()
        .map(|(d, r)| (d.into(), *r.into() as u64))
        .unzip();

    let status = unsafe {
        __theorem_register_type_substitute(
            *theorem_handle.into() as u64,
            domains.as_ptr(),
            domains.len() as u64,
            ranges.as_ptr(),
            ranges.len() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_truth_introduction<T>(
    hypotheses: Vec<T>,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Term>> + Clone,
{
    let mut result: u64 = 0;
    let hypotheses: Vec<u64> = hypotheses
        .iter()
        .cloned()
        .map(|h| *(h.into()) as u64)
        .collect();

    let status = unsafe {
        __theorem_register_truth_introduction(
            hypotheses.as_ptr(),
            hypotheses.len() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_falsity_elimination<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_falsity_elimination(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_conjunction_introduction<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_conjunction_introduction(
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

pub fn theorem_register_conjunction_left_elimination<T>(
    theorem_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_conjunction_left_elimination(
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_conjunction_right_elimination<T>(
    theorem_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_conjunction_right_elimination(
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_disjunction_left_introduction<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_disjunction_left_introduction(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_disjunction_right_introduction<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_disjunction_right_introduction(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_disjunction_elimination<T, U, V>(
    left_handle: T,
    mid_handle: U,
    right_handle: V,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
    V: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_disjunction_elimination(
            *left_handle.into() as u64,
            *mid_handle.into() as u64,
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

pub fn theorem_negation_introduction<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_negation_introduction(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_negation_elimination<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_negation_elimination(
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

pub fn theorem_implication_introduction<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_implication_introduction(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_register_implication_elimination<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_implication_elimination(
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

pub fn theorem_register_iff_introduction<T, U>(
    left_handle: T,
    right_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_iff_introduction(
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

pub fn theorem_register_iff_left_elimination<T>(
    theorem_handle: T,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_iff_left_elimination(
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_forall_introduction<N, T, U>(
    name_handle: N,
    type_handle: T,
    theorem_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    N: Into<Name>,
    T: Into<Handle<tags::Type>>,
    U: Into<Handle<tags::Theorem>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_forall_introduction(
            name_handle.into(),
            *type_handle.into() as u64,
            *theorem_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn theorem_forall_elimination<T, U>(
    theorem_handle: T,
    term_handle: U,
) -> Result<Handle<tags::Term>, ErrorCode>
where
    T: Into<Handle<tags::Theorem>>,
    U: Into<Handle<tags::Term>>,
{
    let mut result: u64 = 0;

    let status = unsafe {
        __theorem_register_forall_elimination(
            *theorem_handle.into() as u64,
            *term_handle.into() as u64,
            &mut result as *mut u64,
        )
    };

    if status == 0 {
        Ok(Handle::new(result as usize, PhantomData))
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}
