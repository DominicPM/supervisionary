//! # Tests for the Supervisionary theorem ABI
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

use libsupervisionary::raw::{theorem::*, ErrorCode};

fn main() -> Result<(), ErrorCode> {
    assert!(theorem_is_registered(
        PREALLOCATED_HANDLE_THEOREM_TRUTH_INTRODUCTION
    ));

    Ok(())
}
