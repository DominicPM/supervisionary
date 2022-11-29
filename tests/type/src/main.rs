//! # Tests for the Supervisionary type ABI
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
//! [Dominic Mulligan]: https://dominicpm.github.io
//! [Nick Spinale]: https://nickspinale.com
//! [Arm Research]: http://www.arm.com/research

use libsupervisionary::{
    _type::*,
    type_former::{
        PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
        PREALLOCATED_HANDLE_TYPE_FORMER_PROP,
    },
};

use std::{collections::HashSet, iter::FromIterator};

fn main() {
    assert!(type_is_registered(PREALLOCATED_HANDLE_TYPE_ALPHA));
    assert!(type_is_registered(PREALLOCATED_HANDLE_TYPE_BETA));
    assert!(type_is_registered(
        PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE
    ));
    assert!(type_is_registered(
        PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE
    ));
    assert!(type_is_registered(PREALLOCATED_HANDLE_TYPE_PROP));
    assert!(type_is_registered(PREALLOCATED_HANDLE_TYPE_QUANTIFIER));
    assert!(type_is_registered(
        PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE
    ));
    assert!(type_is_registered(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE));

    assert_eq!(type_test_variable(PREALLOCATED_HANDLE_TYPE_ALPHA), Ok(true));
    assert_eq!(type_test_variable(PREALLOCATED_HANDLE_TYPE_BETA), Ok(true));

    assert_eq!(
        type_test_function(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE),
        Ok(true)
    );
    assert_eq!(
        type_test_function(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE),
        Ok(true)
    );
    assert_eq!(
        type_test_function(PREALLOCATED_HANDLE_TYPE_QUANTIFIER),
        Ok(true)
    );
    assert_eq!(
        type_test_function(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE),
        Ok(true)
    );
    assert_eq!(
        type_test_function(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE),
        Ok(true)
    );

    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_PROP),
        Ok(true)
    );
    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE),
        Ok(true)
    );
    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE),
        Ok(true)
    );
    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_QUANTIFIER),
        Ok(true)
    );
    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE),
        Ok(true)
    );
    assert_eq!(
        type_test_combination(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE),
        Ok(true)
    );

    assert_eq!(type_split_variable(PREALLOCATED_HANDLE_TYPE_ALPHA), Ok(0));
    assert_eq!(type_split_variable(PREALLOCATED_HANDLE_TYPE_BETA), Ok(1));

    assert_eq!(
        type_split_function(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_PROP,
            PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE
        ))
    );
    assert_eq!(
        type_split_function(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE
        ))
    );
    assert_eq!(
        type_split_function(PREALLOCATED_HANDLE_TYPE_QUANTIFIER),
        Ok((
            PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE,
            PREALLOCATED_HANDLE_TYPE_PROP
        ))
    );
    assert_eq!(
        type_split_function(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            PREALLOCATED_HANDLE_TYPE_PROP
        ))
    );
    assert_eq!(
        type_split_function(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE),
        Ok((PREALLOCATED_HANDLE_TYPE_PROP, PREALLOCATED_HANDLE_TYPE_PROP))
    );

    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_ALPHA), Ok(1));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_BETA), Ok(1));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_PROP), Ok(1));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE), Ok(5));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE), Ok(5));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_QUANTIFIER), Ok(5));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE), Ok(3));
    assert_eq!(type_size(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE), Ok(3));

    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_PROP),
        Ok((PREALLOCATED_HANDLE_TYPE_FORMER_PROP, Vec::new()))
    );
    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
            vec![
                PREALLOCATED_HANDLE_TYPE_ALPHA,
                PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE
            ]
        ))
    );
    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
            vec![
                PREALLOCATED_HANDLE_TYPE_PROP,
                PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE
            ]
        ))
    );
    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_QUANTIFIER),
        Ok((
            PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
            vec![
                PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE,
                PREALLOCATED_HANDLE_TYPE_PROP
            ]
        ))
    );
    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
            vec![PREALLOCATED_HANDLE_TYPE_PROP, PREALLOCATED_HANDLE_TYPE_PROP]
        ))
    );
    assert_eq!(
        type_split_combination(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE),
        Ok((
            PREALLOCATED_HANDLE_TYPE_FORMER_ARROW,
            vec![
                PREALLOCATED_HANDLE_TYPE_ALPHA,
                PREALLOCATED_HANDLE_TYPE_PROP
            ]
        ))
    );

    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_ALPHA),
        Ok(HashSet::from_iter(vec![0]))
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_BETA),
        Ok(HashSet::from_iter(vec![1]))
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_PROP),
        Ok(HashSet::new())
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE),
        Ok(HashSet::from_iter(vec![0]))
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE),
        Ok(HashSet::from_iter(vec![0]))
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_QUANTIFIER),
        Ok(HashSet::from_iter(vec![0]))
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE),
        Ok(HashSet::new())
    );
    assert_eq!(
        type_variables(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE),
        Ok(HashSet::new())
    );

    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_ALPHA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_BETA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BETA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_ALPHA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_ALPHA,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );

    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BETA,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BETA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BETA,
            vec![(1u64, PREALLOCATED_HANDLE_TYPE_BETA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BETA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BETA,
            vec![(1u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_ALPHA)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BETA,
            vec![(3u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BETA)
    );

    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_PROP,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_PROP)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_ALPHA)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );

    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_CONNECTIVE)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE,
            vec![(0u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_UNARY_CONNECTIVE)
    );

    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE,
            vec![(1u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_BINARY_PREDICATE)
    );
    assert_eq!(
        type_substitute(
            PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE,
            vec![(1u64, PREALLOCATED_HANDLE_TYPE_PROP)]
        ),
        Ok(PREALLOCATED_HANDLE_TYPE_UNARY_PREDICATE)
    );
}
