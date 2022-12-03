//! # System interface types
//!
//! Type aliases and other related functionality for describing values passed
//! across the system interface boundary.
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

use wasmi::ValueType;

/// Type-synonyms for declaratively describing the intended purpose of WASM
/// types passed across the ABI boundary.
pub(crate) mod semantic_types {
    /// A WASM address, used for reading-from and writing-to the guest WASM
    /// program heap, assuming the `wasm32-abi`.
    pub type Pointer = u32;
    /// An arity of a type-former.
    pub type Arity = u64;
    /// A handle to a kernel object.
    pub type Handle = u64;
    /// A name of a variable (e.g. a lambda-abstracted variable, or
    /// type-variable).
    pub type Name = u64;
    /// A size of a buffer, or other heap-allocated structure, used for
    /// reading-from and writing-to the guest WASM program heap, assuming the
    /// `wasm32-abi`.
    pub type Size = u64;
}

/// A type capturing semantic types of the ABI, more descriptive than the base
/// types of WASM.  Note that the constructors of this type are intended to shadow
/// the type-synyonyms defined in the `semantic_types` module.
pub(crate) enum AbiType {
    /// A handle pointing-to a kernel object.
    Handle,
    /// A name (e.g. of a lambda-abstracted variable, or similar).
    Name,
    /// An arity for a type-former.
    Arity,
    /// A pointer into the host WASM program's heap.
    Pointer,
    /// A size (or length) of an object appearing in the WASM program's heap.
    Size,
    /// A Boolean value.
    Boolean,
    /// An error code returned from an ABI function.
    ErrorCode,
}

impl AbiType {
    /// Returns `true` iff the current `AbiType` is implemented by the WASM
    /// value type, `tau`.
    pub(crate) fn implemented_by(&self, tau: &ValueType) -> bool {
        match self {
            AbiType::Boolean => tau == &ValueType::I32,
            AbiType::Handle => tau == &ValueType::I64,
            AbiType::Arity => tau == &ValueType::I64,
            AbiType::Name => tau == &ValueType::I64,
            AbiType::Pointer => tau == &ValueType::I32,
            AbiType::Size => tau == &ValueType::I64,
            AbiType::ErrorCode => tau == &ValueType::I32,
        }
    }
}
