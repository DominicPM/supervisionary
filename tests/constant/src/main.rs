//! # Tests for the Supervisionary constant ABI
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

use libsupervisionary::raw::{
    _type::{
        PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE,
        PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE,
        PREALLOCATED_HANDLE_TYPE_PROP, PREALLOCATED_HANDLE_TYPE_QUANTIFIER,
        PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE,
    },
    constant::*,
};

fn main() {
    assert!(constant_is_registered(
        PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION
    ));
    assert!(constant_is_registered(
        PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION
    ));
    assert!(constant_is_registered(
        PREALLOCATED_HANDLE_CONSTANT_EQUALITY
    ));
    assert!(constant_is_registered(PREALLOCATED_HANDLE_CONSTANT_EXISTS));
    assert!(constant_is_registered(PREALLOCATED_HANDLE_CONSTANT_FALSE));
    assert!(constant_is_registered(PREALLOCATED_HANDLE_CONSTANT_FORALL));
    assert!(constant_is_registered(
        PREALLOCATED_HANDLE_CONSTANT_IMPLICATION
    ));
    assert!(constant_is_registered(
        PREALLOCATED_HANDLE_CONSTANT_NEGATION
    ));
    assert!(constant_is_registered(PREALLOCATED_HANDLE_CONSTANT_TRUE));

    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_EQUALITY),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_EXISTS),
        Ok(PREALLOCATED_HANDLE_TYPE_QUANTIFIER)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_FALSE),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_FORALL),
        Ok(PREALLOCATED_HANDLE_TYPE_QUANTIFIER)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_IMPLICATION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_NEGATION),
        Ok(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE)
    );
    assert_eq!(
        constant_resolve(PREALLOCATED_HANDLE_CONSTANT_TRUE),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );
}
