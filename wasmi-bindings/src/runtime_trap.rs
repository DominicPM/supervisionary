//! # Runtime traps (or: kernel panics)
//!
//! Defines various types of runtime traps/kernel panics that are specific to
//! the WASMI execution engine binding.  Note that these represent unrecoverable
//! errors that cause a system abort.
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

use kernel::error_code::ErrorCode;
use std::fmt::{Display, Error as DisplayError, Formatter};
use wasmi::{Error as WasmiError, HostError, Trap, TrapKind};

/// Runtime traps are unrecoverable errors raised by the WASM program host.
/// These are equivalent, essentially, to kernel panics in a typical operating
/// system.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuntimeTrap {
    /// The WASM guest's memory was not registered with the runtime state.
    MemoryNotRegistered,
    /// An attempted read from the WASM guest's heap failed.
    MemoryReadFailed,
    /// An attempted write to the WASM guest's heap failed.
    MemoryWriteFailed,
    /// The WASM guest program tried to call a function that does not exist.
    NoSuchFunction(usize),
    /// A type-signature check on a host-function failed.
    SignatureFailure,
}

/// Pretty-printing for `RuntimeTrap` values.
impl Display for RuntimeTrap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), DisplayError> {
        match self {
            RuntimeTrap::NoSuchFunction(opcode) => {
                write!(f, "NoSuchFunction: {opcode}")
            }
            RuntimeTrap::SignatureFailure => write!(f, "SignatureFailure"),
            RuntimeTrap::MemoryNotRegistered => {
                write!(f, "MemoryNotRegistered")
            }
            RuntimeTrap::MemoryReadFailed => write!(f, "MemoryReadFailed"),
            RuntimeTrap::MemoryWriteFailed => write!(f, "MemoryWriteFailed"),
        }
    }
}

impl HostError for RuntimeTrap {}

/// Lifts a kernel error into an error that can be passed back to the WASM
/// program.
#[inline]
pub fn host_error(code: ErrorCode) -> WasmiError {
    WasmiError::Host(Box::new(code))
}

/// Creates a WASMI `Trap` type from a `RuntimeTrap`.
#[inline]
pub fn host_trap(trap: RuntimeTrap) -> Trap {
    Trap::new(TrapKind::Host(Box::new(trap)))
}
