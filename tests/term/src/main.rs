//! # Tests for the Supervisionary term ABI
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

use libsupervisionary::raw::term::*;

fn main() {
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_CONJUNCTION));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_DISJUNCTION));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_EQUALITY));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_EXISTS));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_FALSE));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_FORALL));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_IMPLICATION));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_NEGATION));
    assert!(term_is_registered(PREALLOCATED_HANDLE_TERM_TRUE));
}
