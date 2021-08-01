//! # Error codes
//!
//! In most LCF-style proof assistants, errors are signalled via exceptions.  We
//! cannot use exceptions in Supervisionary, so use error codes instead.  Note
//! that the contents of this file must also be mirror in prover-space, as it
//! forms part of the ABI contract between kernel and prover.
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
//! [Dominic Mulligan]: https://dominic-mulligan.co.uk
//! [Arm Research]: http://www.arm.com/research

use std::{
    convert::TryFrom,
    fmt::{Display, Error as DisplayError, Formatter},
};

/// The upper limit (exclusive) on the encoding space of the `ErrorCode` type.
pub const ERRORCODE_ENCODING_UPPER_BOUND: usize = 29;

/// Error codes, used for passing back information on why a kernel operation
/// failed to prover-space.  These codes are intra-convertible between the `i32`
/// type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorCode {
    /* ABI errors. */
    /// The operation completed successfully.
    Success,
    /// The type-signature of an ABI function was not as expected.
    SignatureFailure,
    /// The WASM guest program tried to call a host function that does not
    /// exist.
    NoSuchFunction,
    /* Dangling objects. */
    /// A handle was supplied that did not reference a registered constant.
    NoSuchConstantRegistered,
    /// A handle was supplied that did not reference a registered term.
    NoSuchTermRegistered,
    /// A handle was supplied that did not reference a registered theorem.
    NoSuchTheoremRegistered,
    /// A handle was supplied that did not reference a registered type-former.
    NoSuchTypeFormerRegistered,
    /* Type-former related errors. */
    /// A type-former was applied to the wrong number of arguments.
    MismatchedArity,
    /* -- Type related errors. */
    /// A term with functional type was applied to an argument that had a
    /// different type to the domain type of the function.
    DomainTypeMismatch,
    /// A handle was supplied that did not reference a registered type.
    NoSuchTypeRegistered,
    /// A type was expected to be a functional type, but it was not.
    NotAFunctionType,
    /// A type was expected to be a type-combination, but it was not.
    NotATypeCombination,
    /// A type was expected to be a type-variable, but it was not.
    NotATypeVariable,
    /// A type passed to a function as an argument was not well-formed.
    TypeNotWellformed,
    /* -- Constant related errors. */
    /* -- Term related errors. */
    NotAConjunction,
    /// A term passed to a function was expected to be a constant but it was
    /// not.
    NotAConstant,
    NotAForall,
    NotADisjunction,
    /// A term passed to a function was expected to be a lambda-abstraction but
    /// it was not.
    NotALambda,
    /// A term passed to a function was expected to be an application but it was
    /// not.
    NotAnApplication,
    NotAnEquality,
    NotAnExists,
    NotAnImplication,
    /// A term passed to a function was expected to be a negation but it was
    /// not.
    NotANegation,
    /// A term passed to a function as an argument did not have propositional
    /// type.
    NotAProposition,
    /// A term passed to a function was expected to be a variable but it was
    /// not.
    NotAVariable,
    /// A term passed to a function as an argument was not well-formed.
    TermNotWellformed,
    /* -- Theorem related errors. */
    /// An inference rule expected its hypotheses to be in a certain shape, but
    /// they were not.
    ShapeMismatch,
    /// A theorem passed to a function as an argument was not well-formed.
    TheoremNotWellformed,
}

////////////////////////////////////////////////////////////////////////////////
// Trait implementations.
////////////////////////////////////////////////////////////////////////////////

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), DisplayError> {
        match self {
            ErrorCode::Success => write!(f, "Success"),
            ErrorCode::SignatureFailure => write!(f, "SignatureFailure"),
            ErrorCode::NoSuchFunction => write!(f, "NoSuchFunction"),
            ErrorCode::NoSuchConstantRegistered => {
                write!(f, "NoSuchConstantRegistered")
            }
            ErrorCode::NoSuchTermRegistered => {
                write!(f, "NoSuchTermRegistered")
            }
            ErrorCode::NoSuchTheoremRegistered => {
                write!(f, "NoSuchTheoremRegistered")
            }
            ErrorCode::NoSuchTypeFormerRegistered => {
                write!(f, "NoSuchTypeFormerRegistered")
            }
            ErrorCode::MismatchedArity => write!(f, "MismatchedArity"),
            ErrorCode::DomainTypeMismatch => write!(f, "DomainTypeMismatch"),
            ErrorCode::NoSuchTypeRegistered => {
                write!(f, "NoSuchTypeRegistered")
            }
            ErrorCode::NotAFunctionType => write!(f, "NotAFunctionType"),
            ErrorCode::NotATypeCombination => write!(f, "NotATypeCombination"),
            ErrorCode::NotATypeVariable => write!(f, "NotATypeVariable"),
            ErrorCode::TypeNotWellformed => write!(f, "TypeNotWellformed"),
            ErrorCode::NotAConjunction => write!(f, "NotAConjunction"),
            ErrorCode::NotAConstant => write!(f, "NotAConstant"),
            ErrorCode::NotAForall => write!(f, "NotAForall"),
            ErrorCode::NotADisjunction => write!(f, "NotADisjunction"),
            ErrorCode::NotALambda => write!(f, "NotALambda"),
            ErrorCode::NotAnApplication => write!(f, "NotAnApplication"),
            ErrorCode::NotAnEquality => write!(f, "NotAnEquality"),
            ErrorCode::NotAnExists => write!(f, "NotAnExists"),
            ErrorCode::NotAnImplication => write!(f, "NotAnImplication"),
            ErrorCode::NotANegation => write!(f, "NotANegation"),
            ErrorCode::NotAProposition => write!(f, "NotAProposition"),
            ErrorCode::NotAVariable => write!(f, "NotAVariable"),
            ErrorCode::TermNotWellformed => write!(f, "TermNotWellformed"),
            ErrorCode::ShapeMismatch => write!(f, "ShapeMismatch"),
            ErrorCode::TheoremNotWellformed => {
                write!(f, "TheoremNotWellformed")
            }
        }
    }
}

/// Conversion into an `i32` type for ABI transport.
impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        match self {
            ErrorCode::Success => 0,
            ErrorCode::SignatureFailure => 1,
            ErrorCode::NoSuchFunction => 2,
            ErrorCode::NoSuchConstantRegistered => 3,
            ErrorCode::NoSuchTermRegistered => 4,
            ErrorCode::NoSuchTheoremRegistered => 5,
            ErrorCode::NoSuchTypeFormerRegistered => 6,
            ErrorCode::MismatchedArity => 7,
            ErrorCode::DomainTypeMismatch => 8,
            ErrorCode::NoSuchTypeRegistered => 9,
            ErrorCode::NotAFunctionType => 10,
            ErrorCode::NotATypeCombination => 11,
            ErrorCode::NotATypeVariable => 12,
            ErrorCode::TypeNotWellformed => 13,
            ErrorCode::NotAConjunction => 14,
            ErrorCode::NotAConstant => 15,
            ErrorCode::NotAForall => 16,
            ErrorCode::NotADisjunction => 17,
            ErrorCode::NotALambda => 18,
            ErrorCode::NotAnApplication => 19,
            ErrorCode::NotAnEquality => 20,
            ErrorCode::NotAnExists => 21,
            ErrorCode::NotAnImplication => 22,
            ErrorCode::NotANegation => 23,
            ErrorCode::NotAProposition => 24,
            ErrorCode::NotAVariable => 25,
            ErrorCode::TermNotWellformed => 26,
            ErrorCode::ShapeMismatch => 27,
            ErrorCode::TheoremNotWellformed => 28,
        }
    }
}

impl TryFrom<i32> for ErrorCode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ErrorCode::Success),
            1 => Ok(ErrorCode::SignatureFailure),
            2 => Ok(ErrorCode::NoSuchFunction),
            3 => Ok(ErrorCode::NoSuchConstantRegistered),
            4 => Ok(ErrorCode::NoSuchTermRegistered),
            5 => Ok(ErrorCode::NoSuchTheoremRegistered),
            6 => Ok(ErrorCode::NoSuchTypeFormerRegistered),
            7 => Ok(ErrorCode::MismatchedArity),
            8 => Ok(ErrorCode::DomainTypeMismatch),
            9 => Ok(ErrorCode::NoSuchTypeRegistered),
            10 => Ok(ErrorCode::NotAFunctionType),
            11 => Ok(ErrorCode::NotATypeCombination),
            12 => Ok(ErrorCode::NotATypeVariable),
            13 => Ok(ErrorCode::TypeNotWellformed),
            14 => Ok(ErrorCode::NotAConjunction),
            15 => Ok(ErrorCode::NotAConstant),
            16 => Ok(ErrorCode::NotAForall),
            17 => Ok(ErrorCode::NotADisjunction),
            18 => Ok(ErrorCode::NotALambda),
            19 => Ok(ErrorCode::NotAnApplication),
            20 => Ok(ErrorCode::NotAnEquality),
            21 => Ok(ErrorCode::NotAnExists),
            22 => Ok(ErrorCode::NotAnImplication),
            23 => Ok(ErrorCode::NotANegation),
            24 => Ok(ErrorCode::NotAProposition),
            25 => Ok(ErrorCode::NotAVariable),
            26 => Ok(ErrorCode::TermNotWellformed),
            27 => Ok(ErrorCode::ShapeMismatch),
            28 => Ok(ErrorCode::TheoremNotWellformed),
            _otherwise => Err(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests.
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::kernel::error_code::{
        ErrorCode, ERRORCODE_ENCODING_UPPER_BOUND,
    };
    use std::convert::{TryFrom, TryInto};

    /// Tests conversion from an `i32` and back again gets you back to where you
    /// started.
    #[test]
    pub fn errorcode_test0() {
        for i in 0..29 {
            assert_eq!(ErrorCode::try_from(i).unwrap().try_into(), Ok(i));
        }
    }

    /// Tests that the upper bound on the encoding space really is the upper
    /// bound.
    #[test]
    pub fn errorcode_test1() {
        assert!(
            ErrorCode::try_from(ERRORCODE_ENCODING_UPPER_BOUND as i32).is_err()
        );
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test2() {
        let i: i32 = ErrorCode::into(ErrorCode::Success);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::Success);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test3() {
        let i: i32 = ErrorCode::into(ErrorCode::SignatureFailure);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::SignatureFailure);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test4() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchFunction);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchFunction);
    }

    #[test]
    pub fn errorcode_test5() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchConstantRegistered);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchConstantRegistered);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test6() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchTermRegistered);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchTermRegistered);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test7() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchTheoremRegistered);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchTheoremRegistered);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test8() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchTypeFormerRegistered);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchTypeFormerRegistered);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test9() {
        let i: i32 = ErrorCode::into(ErrorCode::MismatchedArity);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::MismatchedArity);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test10() {
        let i: i32 = ErrorCode::into(ErrorCode::DomainTypeMismatch);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::DomainTypeMismatch);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test11() {
        let i: i32 = ErrorCode::into(ErrorCode::NoSuchTypeRegistered);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NoSuchTypeRegistered);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test12() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAFunctionType);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAFunctionType);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test13() {
        let i: i32 = ErrorCode::into(ErrorCode::NotATypeCombination);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotATypeCombination);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test14() {
        let i: i32 = ErrorCode::into(ErrorCode::NotATypeVariable);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotATypeVariable);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test15() {
        let i: i32 = ErrorCode::into(ErrorCode::TypeNotWellformed);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::TypeNotWellformed);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test16() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAConjunction);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAConjunction);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test17() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAConstant);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAConstant);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test18() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAForall);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAForall);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test19() {
        let i: i32 = ErrorCode::into(ErrorCode::NotADisjunction);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotADisjunction);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test20() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAConjunction);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAConjunction);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test21() {
        let i: i32 = ErrorCode::into(ErrorCode::NotALambda);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotALambda);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test22() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAnApplication);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAnApplication);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test23() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAnEquality);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAnEquality);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test24() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAnExists);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAnExists);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test25() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAnImplication);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAnImplication);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test26() {
        let i: i32 = ErrorCode::into(ErrorCode::NotANegation);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotANegation);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test27() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAProposition);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAProposition);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test28() {
        let i: i32 = ErrorCode::into(ErrorCode::NotAVariable);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::NotAVariable);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test29() {
        let i: i32 = ErrorCode::into(ErrorCode::TermNotWellformed);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::TermNotWellformed);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test30() {
        let i: i32 = ErrorCode::into(ErrorCode::ShapeMismatch);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::ShapeMismatch);
    }

    /// Pointwise test that conversion to an `i32` and back again gets you back
    /// to where you started.
    #[test]
    pub fn errorcode_test31() {
        let i: i32 = ErrorCode::into(ErrorCode::TheoremNotWellformed);
        let e: ErrorCode = ErrorCode::try_from(i).unwrap();
        assert_eq!(e, ErrorCode::TheoremNotWellformed);
    }
}
