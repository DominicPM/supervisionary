//! # Pass through: access to the underlying system
//!
//! The following implements a "pass through" system interface, which simply
//! calls down to the underlying system, e.g., the MacOS or Linux, or whatever
//! OS Supervisionary is running under, filesytem.
//!
//! # Authors
//!
//! [Dominic Mulligan]
//!
//! # Copyright
//!
//! Please see the `LICENSE.markdown` file in the *Supervisionary* root
//! directory for licensing information.
//!
//! [Dominic Mulligan]: https://dominicpm.github.io

use super::SystemInterface;
use std::io::{stderr, stdout, Write};

////////////////////////////////////////////////////////////////////////////////
// The dummy type.
////////////////////////////////////////////////////////////////////////////////

/// This is "just" a dummy type which maintains no internal state, and is merely
/// introduced to have a type to implement a trait on.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PassThroughSystemInterface;

impl Default for PassThroughSystemInterface {
    #[inline]
    fn default() -> Self {
        PassThroughSystemInterface
    }
}

impl PassThroughSystemInterface {
    /// Creates a new, pass through system interface.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Instantiating the SystemInterface trait.
////////////////////////////////////////////////////////////////////////////////

impl SystemInterface for PassThroughSystemInterface {
    /// Writes a string, `s`, to the system's `stdout` file handle.
    #[inline]
    fn write(&self, s: String) {
        stdout().write(s.as_bytes()).unwrap();
    }

    /// Writes a string, `s`, to the system's `stderr` file handle.
    #[inline]
    fn write_error(&self, s: String) {
        stderr().write(s.as_bytes()).unwrap();
    }
}
