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

use libsupervisionary::raw::{system::system_io_write, theorem::*, ErrorCode};

fn main() -> Result<(), ErrorCode> {
    system_io_write(format!(
        "PREALLOCATED_HANDLE_THEOREM_TRUTH_INTRODUCTION: {PREALLOCATED_HANDLE_THEOREM_TRUTH_INTRODUCTION:?}\n",
    ))?;

    Ok(())
}
