//! # WASMI binding
//!
//! This module binds the kernel's runtime state to the WASMI execution engine
//! (an interpreter for Wasm code).  Note that this binding process is fairly
//! specific to WASMI, and if we were to implement another binding for e.g.
//! Wasmtime, we'd need another module to handle it.
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

pub mod runtime_state;
mod runtime_trap;
mod system_call_numbers;
mod system_interface_types;
mod type_checking;
