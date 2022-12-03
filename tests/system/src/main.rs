//! # Tests for the Supervisionary system ABI
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

use libsupervisionary::raw::system::*;

fn main() {
    system_io_write("Hello, world!");

    system_io_write_error("Error!  Error!");
}
