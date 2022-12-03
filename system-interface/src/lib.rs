//! # The Supervisionary system interface
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
//! [Dominic Mulligan]<https://dominicpm.github.io>

pub mod pass_through;

////////////////////////////////////////////////////////////////////////////////
// The generic system interface trait.
////////////////////////////////////////////////////////////////////////////////

/// A generic interface used to query and manipulate the system.  Note that this
/// allows us to decouple the rest of the Supervisionary codebase from the
/// decision over whether we are actually querying the real, underlying system
/// (e.g., the MacOS or Linux filesystem), or whether Supervisionary is
/// maintaining its own synthetic, in-memory filesystem.
pub trait SystemInterface {
    /// Writes a string to `stdout` in the system interface.
    fn write(&self, s: String);

    /// Writes a string to `stderr` in the system interface.
    fn write_error(&self, s: String);
}
