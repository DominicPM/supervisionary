//! # Kernel object handles
//!
//! Kernel objects are manipulated only by the kernel, so untrusted
//! "prover-space" code needs some way of naming the object that should be
//! manipulated by the kernel.  In Supervisionary, we use *handles* for this
//! purpose, which are simply machine words suitable for passing across the
//! kernel/prover-space system call boundary.
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

use std::{
    fmt,
    fmt::{Display, Formatter},
    marker::PhantomData,
    ops::Deref,
};

////////////////////////////////////////////////////////////////////////////////
// Handle tags.
////////////////////////////////////////////////////////////////////////////////

/// This module contains dummy types that are used as type-parameters to the
/// parameterized `Handle` struct, defined below, which allow us to distinguish
/// between handles used for different purposes within the kernel.  This, though
/// handles are really just represented as machine words, allow us to statically
/// avoid mixing up handles that are assumed to point to e.g. a HOL type, with
/// those assumed to point to a HOL theorem.
pub mod tags {
    /// The handle tag for type-formers.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TypeFormer;

    /// The handle tag for types.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Type;

    /// The handle tag for constants.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Constant;

    /// The handle tag for terms.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Term;

    /// The handle tag for theorems.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Theorem;

    /// This is a dummy trait which will allow us to assert that a particular
    /// type parameter may indeed be instantiated exclusively with a handle tag.
    pub trait IsTag {}

    impl IsTag for TypeFormer {}

    impl IsTag for Type {}

    impl IsTag for Constant {}

    impl IsTag for Term {}

    impl IsTag for Theorem {}
}

////////////////////////////////////////////////////////////////////////////////
// Tagged handles.
////////////////////////////////////////////////////////////////////////////////

/// Kernel handles consist of a machine word, which acts as the handle-proper,
/// along with some phantom data which binds the `T` type-parameter to the
/// machine word, and which is used to tag the handle with, using some instance
/// of the `IsTag` trait.  This allows us to statically distinguish between
/// handles that e.g. are assumed to point to HOL terms from those that are e.g.
/// assumed to point to theorems, within the kernel.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Handle<T>
where
    T: tags::IsTag,
{
    /// We use the Rust `usize` type as our handle type.  Note that on modern 64-bit
    /// systems this is implemented as a 64-bit unsigned integer.
    handle: usize,
    /// The phantom data binding the tag type, `T`.
    marker: PhantomData<T>,
}

/// The upper-bound (exclusive) of the preallocated handles.
pub const PREALLOCATED_HANDLE_UPPER_BOUND: usize = 28;

/// Returns `true` iff the handle is a pre-allocated handle built into the
/// kernel.
#[inline]
pub fn is_preallocated<T>(handle: Handle<T>) -> bool
where
    T: tags::IsTag,
{
    *handle < PREALLOCATED_HANDLE_UPPER_BOUND
}

////////////////////////////////////////////////////////////////////////////////
// Pre-allocated handles for kernel objects.
////////////////////////////////////////////////////////////////////////////////

/// A pre-allocated handle used to refer to the `Prop` type-former.
pub const PREALLOCATED_HANDLE_TYPE_FORMER_PROP: Handle<tags::TypeFormer> =
    Handle {
        handle: 0,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the function-space type-former.
pub const PREALLOCATED_HANDLE_TYPE_FORMER_ARROW: Handle<tags::TypeFormer> =
    Handle {
        handle: 1,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the type-variable `⍺`.
pub const PREALLOCATED_HANDLE_TYPE_ALPHA: Handle<tags::Type> = Handle {
    handle: 2,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the type-variable `β`.
pub const PREALLOCATED_HANDLE_TYPE_BETA: Handle<tags::Type> = Handle {
    handle: 3,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the `Prop` type.
pub const PREALLOCATED_HANDLE_TYPE_PROP: Handle<tags::Type> = Handle {
    handle: 4,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the type of unary predicates.
pub const PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE: Handle<tags::Type> =
    Handle {
        handle: 5,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the type of binary predicates.
pub const PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE: Handle<tags::Type> =
    Handle {
        handle: 6,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the type of unary connectives.
pub const PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE: Handle<tags::Type> =
    Handle {
        handle: 7,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the type of binary connectives.
pub const PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE: Handle<tags::Type> =
    Handle {
        handle: 8,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the type of polymorphic quantifiers.
pub const PREALLOCATED_HANDLE_TYPE_QUANTIFIER: Handle<tags::Type> = Handle {
    handle: 9,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the truth constant.
pub const PREALLOCATED_HANDLE_CONSTANT_TRUE: Handle<tags::Constant> = Handle {
    handle: 10,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the falsity constant.
pub const PREALLOCATED_HANDLE_CONSTANT_FALSE: Handle<tags::Constant> = Handle {
    handle: 11,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the negation constant.
pub const PREALLOCATED_HANDLE_CONSTANT_NEGATION: Handle<tags::Constant> =
    Handle {
        handle: 12,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the binary conjunction connective.
pub const PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION: Handle<tags::Constant> =
    Handle {
        handle: 13,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the binary disjunction connective.
pub const PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION: Handle<tags::Constant> =
    Handle {
        handle: 14,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the binary implication connective.
pub const PREALLOCATED_HANDLE_CONSTANT_IMPLICATION: Handle<tags::Constant> =
    Handle {
        handle: 15,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the universal quantifier constant.
pub const PREALLOCATED_HANDLE_CONSTANT_FORALL: Handle<tags::Constant> =
    Handle {
        handle: 16,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the existential quantifier constant.
pub const PREALLOCATED_HANDLE_CONSTANT_EXISTS: Handle<tags::Constant> =
    Handle {
        handle: 17,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the equality constant.
pub const PREALLOCATED_HANDLE_CONSTANT_EQUALITY: Handle<tags::Constant> =
    Handle {
        handle: 18,
        marker: PhantomData,
    };
/// A pre-allocated handle used to refer to the truth term, the truth constant
/// lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_TRUE: Handle<tags::Term> = Handle {
    handle: 19,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the falsity term, the falsity
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_FALSE: Handle<tags::Term> = Handle {
    handle: 20,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the negation term, the negation
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_NEGATION: Handle<tags::Term> = Handle {
    handle: 21,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the conjunction term, the
/// conjunction constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_CONJUNCTION: Handle<tags::Term> = Handle {
    handle: 22,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the disjunction term, the
/// disjunction constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_DISJUNCTION: Handle<tags::Term> = Handle {
    handle: 23,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the implication term, the
/// implication constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_IMPLICATION: Handle<tags::Term> = Handle {
    handle: 24,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the equality term, the equality
/// constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_EQUALITY: Handle<tags::Term> = Handle {
    handle: 25,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the universal quantifier term, the
/// universal quantifier constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_FORALL: Handle<tags::Term> = Handle {
    handle: 26,
    marker: PhantomData,
};
/// A pre-allocated handle used to refer to the existential quantifier term, the
/// existential quantifier constant lifted into a term.
pub const PREALLOCATED_HANDLE_TERM_EXISTS: Handle<tags::Term> = Handle {
    handle: 27,
    marker: PhantomData,
};

////////////////////////////////////////////////////////////////////////////////
// Trait implementations.
////////////////////////////////////////////////////////////////////////////////

/// Dereferencing a `Handle` simply returns its associated machine word.
impl<T> Deref for Handle<T>
where
    T: tags::IsTag,
{
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

/// Injection from machine words into the `Handle` type.
impl<T> From<usize> for Handle<T>
where
    T: tags::IsTag,
{
    #[inline]
    fn from(handle: usize) -> Self {
        Handle {
            handle,
            marker: PhantomData,
        }
    }
}

/// Pretty-printing for term handles.
impl Display for Handle<tags::Term> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (term handle)", self.handle)
    }
}

/// Pretty-printing for constant handles.
impl Display for Handle<tags::Constant> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (constant handle)", self.handle)
    }
}

/// Pretty-printing for type-former handles.
impl Display for Handle<tags::TypeFormer> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (type-former handle)", self.handle)
    }
}

/// Pretty-printing for type handles.
impl Display for Handle<tags::Type> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (type handle)", self.handle)
    }
}

/// Pretty-printing for theorem handles.
impl Display for Handle<tags::Theorem> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} (theorem handle)", self.handle)
    }
}
