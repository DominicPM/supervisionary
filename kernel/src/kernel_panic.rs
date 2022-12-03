//! # Kernel panic messages
//!
//! The Supervisionary kernel has a number of failure modes: ordinarily a system
//! call will produce a defined error code in response to some recoverable
//! error.  For example, if passed a kernel handle that "dangles", and does not
//! point to any registered kernel object, the system call will abort and return
//! an appropriate error code back to untrusted "prover-space" code which called
//! it, diagnosing the issue.
//!
//! Unfortunately, however, there may be situations where some internal
//! invariant within the Supervisionary kernel fails: for example if any of our
//! internal heaps fail to be *inductive*, and contain an object which itself
//! points-to another object which does not exist in another kernel heap.  In
//! these cases, we have hit an internal kernel error, which is unrecoverable,
//! and must abort at runtime with a *kernel panic*.
//!
//! The messages in this module contain user-facing error messages that are
//! raised by the kernel when a kernel panic is encountered.
//!
//! # Authors
//!
//! [Dominic Mulligan], Systems Research Group, [Arm Research] Cambridge.
//! [Nick Spinale]: Systems Research Group, [Arm Research] Cambridge.
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

////////////////////////////////////////////////////////////////////////////////
// Kernel panic messages.
////////////////////////////////////////////////////////////////////////////////

/// Error message produced during a kernel panic due to the kernel running out
/// of fresh handles.
pub const HANDLE_EXHAUST_ERROR: &str =
    "Kernel invariant failed: handles have been exhausted";

/// Error message produced when the kernel failed to generate a fresh name, for
/// e.g. capture-avoiding substitution.
pub const FRESH_NAME_GENERATION_FAILED: &str =
    "Exhausted fresh name generation.";

/// Error message produced during a kernel panic due to the kernel encountering
/// a registered kernel-object with a dangling handle.
pub const DANGLING_HANDLE_ERROR: &str =
    "Kernel invariant failed: dangling handle.";

/// Error message produced during a kernel panic due to the kernel failing to
/// build a kernel primitive.
pub const PRIMITIVE_CONSTRUCTION_ERROR: &str =
    "Kernel invariant failed: failed to construct a kernel primitive.";
