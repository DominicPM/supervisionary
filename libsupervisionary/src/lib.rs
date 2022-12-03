//! # libsupervisionary
//!
//! Rust language support for interacting with the Supervisionary kernel.  This
//! library provides a binding to the kernel's host-call interface, with some
//! minor abstractions built on top.
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

pub mod build;
pub mod raw;
