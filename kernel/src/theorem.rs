//! # HOL theorems
//!
//! Theorems are recursively-defined trees, constructed per the axioms and
//! inference rules of HOL.  Note that in Supervisionary, like most HOL
//! implementations, we do not store "full proofs" of a theorem.  Instead, we
//! ensure that theorems are correctly-constructed via checks in the functions
//! that implement our axioms and inference rules, and ensure that *only* those
//! functions can actually modify the kernel's theorem heap.  This means we do
//! not need to store "back pointers" to other theorem objects in our
//! representation of theorems, nor do we need any way of ascertaining whether a
//! theorem was constructed by using e.g. a conjunction introduction rules, as a
//! last step.  This simplifies the representation of theorems, somewhat, and
//! allows us to simply record a conclusion and a set of premisses, rather than
//! having a constructor for each primitive axiom/inference rule in the type.
//!
//! Note, however, that some operations on theorem do need to make reference to
//! the runtime heap of theorems, in the kernel's runtime state.  As a result,
//! most of the complex functionality around theorem object is elsewhere,
//! leaving only basic construction and manipulation functionality, here.
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

use crate::handle::{tags, Handle};

////////////////////////////////////////////////////////////////////////////////
// Theorems, proper.
////////////////////////////////////////////////////////////////////////////////

/// Theorem objects consist of a list of premisses, each of which is assumed to
/// be a formula, and a single conclusion, again assumed to be a formula.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Theorem {
    /// The premisses of the theorem, i.e. the set of propositions that must
    /// hold for the conclusion to also hold.  All elements of this list should
    /// be handles pointing-to propositions in the runtime state's term-table.
    /// Handles should be stored in ascending sorted order.
    premisses: Vec<Handle<tags::Term>>,
    /// The conclusion of the theorem, which must be a handle pointing-to a
    /// proposition in the runtime state's term-table.
    conclusion: Handle<tags::Term>,
}

impl Theorem {
    /// Creates a new theorem from a collection of hypotheses and a handle to a
    /// conclusion.  Hypotheses are sorted before constructing the theorem
    /// object and are checked to make sure they all point-to propositions.
    /// Similarly, it is assumed that `conclusion` also points-to a proposition.
    pub fn new<T, U>(premisses: Vec<T>, conclusion: U) -> Self
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>>,
    {
        let mut premisses: Vec<Handle<tags::Term>> =
            premisses.iter().cloned().map(|h| h.into()).collect();

        premisses.sort();
        premisses.dedup();

        Self {
            premisses,
            conclusion: conclusion.into(),
        }
    }

    /// Returns the handle to the theorem's conclusion.
    #[inline]
    pub fn conclusion(&self) -> &Handle<tags::Term> {
        &self.conclusion
    }

    /// Returns the set of premisses of the theorem.
    #[inline]
    pub fn premisses(&self) -> &Vec<Handle<tags::Term>> {
        &self.premisses
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests.
////////////////////////////////////////////////////////////////////////////////

/// Tests for theorem-related functionality.
#[cfg(test)]
mod test {
    use crate::{
        handle::{tags, Handle, PREALLOCATED_HANDLE_TERM_TRUE},
        theorem::Theorem,
    };

    /// Tests that constructing then deconstructing a theorem object gets you
    /// back to where you started.
    #[test]
    pub fn theorem_test0() {
        let empty: Vec<Handle<tags::Term>> = Vec::new();
        let t = Theorem::new(empty, PREALLOCATED_HANDLE_TERM_TRUE);

        assert_eq!(t.premisses(), &Vec::new());
        assert_eq!(t.conclusion(), &PREALLOCATED_HANDLE_TERM_TRUE);
    }
}
