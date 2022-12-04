//! # System call type-checking
//!
//! Type-checks the arguments to system calls and makes sure that they are
//! well-formed.
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
//! [Dominic Mulligan]<https://dominicpm.github.io>
//! [Nick Spinale]<https://nickspinale.com>
//! [Arm Research]<http://www.arm.com/research>

use crate::system_interface_types::AbiType;
use wasmi::Signature;

/// Returns `true` iff the semantic function signature type described by
/// `params` and `ret` is implemented by the WASM type described by
/// `signature`.
fn check_signature(
    signature: &Signature,
    params: &[AbiType],
    ret: &Option<AbiType>,
) -> bool {
    let params = signature
        .params()
        .iter()
        .zip(params)
        .all(|(a, w)| w.implemented_by(a));

    match (ret, signature.return_type()) {
        (None, None) => params,
        (Some(a), Some(w)) => a.implemented_by(&w) && params,
        _otherwise => false,
    }
}

////////////////////////////////////////////////////////////////////////////////
// Type-checking proof-related system calls.
////////////////////////////////////////////////////////////////////////////////

/// Checks the signature of the `TypeFormer.Resolve` ABI function.
#[inline]
pub(crate) fn check_type_former_resolve_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `TypeFormer.Register` ABI function.
#[inline]
pub(crate) fn check_type_former_register_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Arity], &Some(AbiType::Handle))
}

/// Checks the signature of the `TypeFormer.IsRegistered` ABI function.
#[inline]
pub(crate) fn check_type_former_is_registered_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Handle], &Some(AbiType::Boolean))
}

/// Checks the signature of the `Type.Register.Variable` ABI function.
#[inline]
pub(crate) fn check_type_register_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Name], &Some(AbiType::Handle))
}

/// Checks the signature of the `Type.Register.Combination` ABI function.
#[inline]
pub(crate) fn check_type_register_combination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Register.Function` ABI function.
#[inline]
pub(crate) fn check_type_register_function_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.IsRegistered` ABI function.
#[inline]
pub(crate) fn check_type_is_registered_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Handle], &Some(AbiType::Boolean))
}

/// Checks the signature of the `Type.Split.Variable` ABI function.
#[inline]
pub(crate) fn check_type_split_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Split.Combination` ABI function.
#[inline]
pub(crate) fn check_type_split_combination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Split.Function` ABI function.
#[inline]
pub(crate) fn check_type_split_function_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Test.Variable` ABI function.
#[inline]
pub(crate) fn check_type_test_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Test.Combination` ABI function.
#[inline]
pub(crate) fn check_type_test_combination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Test.Function` ABI function.
#[inline]
pub(crate) fn check_type_test_function_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Size` ABI function.
#[inline]
pub(crate) fn check_type_size_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Variables` ABI function.
#[inline]
pub(crate) fn check_type_variables_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Type.Substitute` ABI function.
#[inline]
pub(crate) fn check_type_substitute_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Constant.Register` ABI function.
#[inline]
pub(crate) fn check_constant_register_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Constant.Resolve` ABI function.
#[inline]
pub(crate) fn check_constant_resolve_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Constant.IsRegistered` ABI function.
#[inline]
pub(crate) fn check_constant_is_registered_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Handle], &Some(AbiType::Boolean))
}

/// Checks the signature of the `Term.IsRegistered` ABI function.
#[inline]
pub(crate) fn check_term_is_registered_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Handle], &Some(AbiType::Boolean))
}

/// Checks the signature of the `Term.Register.Variable` ABI function.
#[inline]
pub(crate) fn check_term_register_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Name, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Constant` ABI function.
#[inline]
pub(crate) fn check_term_register_constant_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Application` ABI function.
#[inline]
pub(crate) fn check_term_register_application_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Lambda` ABI function.
#[inline]
pub(crate) fn check_term_register_lambda_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Name,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Negation` ABI function.
#[inline]
pub(crate) fn check_term_register_negation_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Conjunction` ABI function.
#[inline]
pub(crate) fn check_term_register_conjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Disjunction` ABI function.
#[inline]
pub(crate) fn check_term_register_disjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Implication` ABI function.
#[inline]
pub(crate) fn check_term_register_implication_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Equality` ABI function.
#[inline]
pub(crate) fn check_term_register_equality_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Forall` ABI function.
#[inline]
pub(crate) fn check_term_register_forall_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Name,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Register.Exists` ABI function.
#[inline]
pub(crate) fn check_term_register_exists_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Name,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Variable` ABI function.
#[inline]
pub(crate) fn check_term_split_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Constant` ABI function.
#[inline]
pub(crate) fn check_term_split_constant_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Application` ABI function.
#[inline]
pub(crate) fn check_term_split_application_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Lambda` ABI function.
#[inline]
pub(crate) fn check_term_split_lambda_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Negation` ABI function.
#[inline]
pub(crate) fn check_term_split_negation_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Conjunction` ABI function.
#[inline]
pub(crate) fn check_term_split_conjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Disjunction` ABI function.
#[inline]
pub(crate) fn check_term_split_disjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Implication` ABI function.
#[inline]
pub(crate) fn check_term_split_implication_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Equality` ABI function.
#[inline]
pub(crate) fn check_term_split_equality_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Forall` ABI function.
#[inline]
pub(crate) fn check_term_split_forall_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Split.Exists` ABI function.
#[inline]
pub(crate) fn check_term_split_exists_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Variable` ABI function.
#[inline]
pub(crate) fn check_term_test_variable_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Constant` ABI function.
#[inline]
pub(crate) fn check_term_test_constant_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Application` ABI function.
#[inline]
pub(crate) fn check_term_test_application_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Lambda` ABI function.
#[inline]
pub(crate) fn check_term_test_lambda_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Negation` ABI function.
#[inline]
pub(crate) fn check_term_test_negation_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Conjunction` ABI function.
#[inline]
pub(crate) fn check_term_test_conjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Disjunction` ABI function.
#[inline]
pub(crate) fn check_term_test_disjunction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Implication` ABI function.
#[inline]
pub(crate) fn check_term_test_implication_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Equality` ABI function.
#[inline]
pub(crate) fn check_term_test_equality_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Forall` ABI function.
#[inline]
pub(crate) fn check_term_test_forall_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Test.Exists` ABI function.
#[inline]
pub(crate) fn check_term_test_exists_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Size` ABI function.
#[inline]
pub(crate) fn check_term_size_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.FreeVariables` ABI function.
#[inline]
pub(crate) fn check_term_fv_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Substitute` ABI function.
#[inline]
pub(crate) fn check_term_substitute_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Type.Variables` ABI function.
#[inline]
pub(crate) fn check_term_type_variables_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Type.Substitute` ABI function.
#[inline]
pub(crate) fn check_term_type_substitute_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Type.Infer` ABI function.
#[inline]
pub(crate) fn check_term_type_infer_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Term.Type.IsProposition` ABI function.
#[inline]
pub(crate) fn check_term_type_is_proposition_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.IsRegistered` ABI function.
#[inline]
pub(crate) fn check_theorem_is_registered_signature(
    signature: &Signature,
) -> bool {
    check_signature(signature, &[AbiType::Handle], &Some(AbiType::ErrorCode))
}

/// Checks the signature of the `Theorem.Register.Assumption` ABI function.
#[inline]
pub(crate) fn check_theorem_register_assumption_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Weaken` ABI function.
#[inline]
pub(crate) fn check_theorem_register_weaken_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Reflexivity` ABI function.
#[inline]
pub(crate) fn check_theorem_register_reflexivity_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Symmetry` ABI function.
#[inline]
pub(crate) fn check_theorem_register_symmetry_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Transitivity` ABI function.
#[inline]
pub(crate) fn check_theorem_register_transitivity_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Substitute` ABI function.
#[inline]
pub(crate) fn check_theorem_register_substitute_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.TypeSubstitute` ABI function.
#[inline]
pub(crate) fn check_theorem_register_type_substitute_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
            AbiType::Size,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Application` ABI function.
#[inline]
pub(crate) fn check_theorem_register_application_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Lambda` ABI function.
#[inline]
pub(crate) fn check_theorem_register_lambda_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Name,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Beta` ABI function.
#[inline]
pub(crate) fn check_theorem_register_beta_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.Eta` ABI function.
#[inline]
pub(crate) fn check_theorem_register_eta_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.FalsityElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_falsity_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ConjunctionIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_conjunction_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ConjunctionLeftElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_conjunction_left_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ConjunctionRightElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_conjunction_right_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.DisjunctionElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_disjunction_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.DisjunctionLeftIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_disjunction_left_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.DisjunctionRightIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_disjunction_right_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ImplicationIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_implication_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ImplicationElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_implication_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.IffIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_iff_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.IffLeftElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_iff_left_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.NegationIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_negation_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.NegationElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_negation_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ForallIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_forall_introduction_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[
            AbiType::Name,
            AbiType::Handle,
            AbiType::Handle,
            AbiType::Pointer,
        ],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ForallElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_forall_elimination_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Register.ExistsIntroduction` ABI function.
#[inline]
pub(crate) fn check_theorem_register_exists_introduction_signature(
    _signature: &Signature,
) -> bool {
    unimplemented!()
}

/// Checks the signature of the `Theorem.Register.ExistsElimination` ABI function.
#[inline]
pub(crate) fn check_theorem_register_exists_elimination_signature(
    _signature: &Signature,
) -> bool {
    unimplemented!()
}

/// Checks the signature of the `Theorem.Split.Conclusion` ABI function.
#[inline]
pub(crate) fn check_theorem_split_conclusion_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Split.Hypotheses` ABI function.
#[inline]
pub(crate) fn check_theorem_split_hypotheses_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `Theorem.Size` ABI function.
#[inline]
pub(crate) fn check_theorem_size_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Handle, AbiType::Pointer],
        &Some(AbiType::ErrorCode),
    )
}

////////////////////////////////////////////////////////////////////////////////
// Type-checking system access system calls.
////////////////////////////////////////////////////////////////////////////////

/// Checks the signature of the `System.IO.Write` ABI function.
#[inline]
pub(crate) fn check_system_io_write_signature(signature: &Signature) -> bool {
    check_signature(
        signature,
        &[AbiType::Pointer, AbiType::Size],
        &Some(AbiType::ErrorCode),
    )
}

/// Checks the signature of the `System.IO.WriteError` ABI function.
#[inline]
pub(crate) fn check_system_io_write_error_signature(
    signature: &Signature,
) -> bool {
    check_signature(
        signature,
        &[AbiType::Pointer, AbiType::Size],
        &Some(AbiType::ErrorCode),
    )
}
