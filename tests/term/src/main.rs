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
//! [Dominic Mulligan]<https://dominicpm.github.io>

use libsupervisionary::raw::{
    _type::{
        PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE,
        PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE,
        PREALLOCATED_HANDLE_TYPE_PROP, PREALLOCATED_HANDLE_TYPE_QUANTIFIER,
        PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE,
    },
    constant::{
        PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION,
        PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION,
        PREALLOCATED_HANDLE_CONSTANT_EQUALITY,
        PREALLOCATED_HANDLE_CONSTANT_EXISTS,
        PREALLOCATED_HANDLE_CONSTANT_FALSE,
        PREALLOCATED_HANDLE_CONSTANT_FORALL,
        PREALLOCATED_HANDLE_CONSTANT_IMPLICATION,
        PREALLOCATED_HANDLE_CONSTANT_NEGATION,
        PREALLOCATED_HANDLE_CONSTANT_TRUE,
    },
    term::*,
};
use std::collections::HashSet;

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

    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok(true)
    );
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok(true)
    );
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok(true)
    );
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok(true)
    );
    assert_eq!(term_test_constant(PREALLOCATED_HANDLE_TERM_FALSE), Ok(true));
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok(true)
    );
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok(true)
    );
    assert_eq!(
        term_test_constant(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok(true)
    );
    assert_eq!(term_test_constant(PREALLOCATED_HANDLE_TERM_TRUE), Ok(true));

    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_CONJUNCTION,
            PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_DISJUNCTION,
            PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_EQUALITY,
            PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_EXISTS,
            PREALLOCATED_HANDLE_TYPE_QUANTIFIER
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_FALSE),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_FALSE,
            PREALLOCATED_HANDLE_TYPE_PROP
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_FORALL,
            PREALLOCATED_HANDLE_TYPE_QUANTIFIER
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_IMPLICATION,
            PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_NEGATION,
            PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE
        ))
    );
    assert_eq!(
        term_split_constant(PREALLOCATED_HANDLE_TERM_TRUE),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_TRUE,
            PREALLOCATED_HANDLE_TYPE_PROP
        ))
    );

    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_CONJUNCTION), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_DISJUNCTION), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_EQUALITY), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_EXISTS), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_FALSE), Ok(2usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_FORALL), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_IMPLICATION), Ok(6usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_NEGATION), Ok(4usize));
    assert_eq!(term_size(PREALLOCATED_HANDLE_TERM_TRUE), Ok(2usize));

    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_FALSE),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_variables(PREALLOCATED_HANDLE_TERM_TRUE),
        Ok(HashSet::new())
    );

    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok(HashSet::from([0]))
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok(HashSet::from([0]))
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_FALSE),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok(HashSet::from([0]))
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok(HashSet::new())
    );
    assert_eq!(
        term_free_type_variables(PREALLOCATED_HANDLE_TERM_TRUE),
        Ok(HashSet::new())
    );

    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok(PREALLOCATED_HANDLE_TYPE_QUANTIFIER)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_FALSE),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok(PREALLOCATED_HANDLE_TYPE_QUANTIFIER)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE)
    );
    assert_eq!(
        term_type_infer(PREALLOCATED_HANDLE_TERM_TRUE),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );

    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_CONJUNCTION),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_DISJUNCTION),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_EQUALITY),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_EXISTS),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_FALSE),
        Ok(true)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_FORALL),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_IMPLICATION),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_NEGATION),
        Ok(false)
    );
    assert_eq!(
        term_type_is_proposition(PREALLOCATED_HANDLE_TERM_TRUE),
        Ok(true)
    );

    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_CONJUNCTION,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_CONJUNCTION)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_DISJUNCTION,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_DISJUNCTION)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_EQUALITY,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        )
        .and_then(term_split_constant),
        Ok((
            PREALLOCATED_HANDLE_CONSTANT_EQUALITY,
            PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE
        ))
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_EXISTS,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        )
        .and_then(term_split_constant)
        .map(|t| t.0),
        Ok(PREALLOCATED_HANDLE_CONSTANT_EXISTS)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_FALSE,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_FALSE)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_FORALL,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        )
        .and_then(term_split_constant)
        .map(|t| t.0),
        Ok(PREALLOCATED_HANDLE_CONSTANT_FORALL)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_IMPLICATION,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_IMPLICATION)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_NEGATION,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_NEGATION)
    );
    assert_eq!(
        term_type_substitute(
            PREALLOCATED_HANDLE_TERM_TRUE,
            vec![(0u32, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_TRUE)
    );

    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_CONJUNCTION,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_CONJUNCTION)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_DISJUNCTION,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_DISJUNCTION)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_EQUALITY,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_EQUALITY)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_EXISTS,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_EXISTS)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_FALSE,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_FALSE)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_FORALL,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_FORALL)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_IMPLICATION,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_IMPLICATION)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_NEGATION,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_NEGATION)
    );
    assert_eq!(
        term_substitute(
            PREALLOCATED_HANDLE_TERM_TRUE,
            vec![(
                (0u32, PREALLOCATED_HANDLE_TYPE_PROP),
                PREALLOCATED_HANDLE_TERM_CONJUNCTION
            )]
        ),
        Ok(PREALLOCATED_HANDLE_TERM_TRUE)
    );
}
