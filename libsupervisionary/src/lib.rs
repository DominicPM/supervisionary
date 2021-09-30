//! # libsupervisionary
//!
//! Rust language support for interacting with the Supervisionary kernel.  This
//! library provides a binding to the kernel's host-call interface, with some
//! minor abstractions built on top.
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

#![feature(const_fn_trait_bound)]

/// Raw bindings to the Supervisionary kernel ABI.
pub mod raw;
