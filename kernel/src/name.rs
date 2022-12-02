//! # Fresh name generation
//!
//! Supervisionary uses an explicit name-carrying syntax for its implementation
//! of the simply-typed λ-calculus, in a similar vein to HOL Light.  (An
//! alternative would have been to use De Bruijn indices, which is the strategy
//! used by Isabelle.)  One consequence of this design decision is the need to
//! sometimes generate a "fresh" name, distinct from a set of existing names,
//! for example when performing a capture-avoiding substitution.  This module
//! implements that functionality.
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

////////////////////////////////////////////////////////////////////////////////
// Miscellaneous material.
////////////////////////////////////////////////////////////////////////////////

use crate::kernel_panic::FRESH_NAME_GENERATION_FAILED;
use log::info;

////////////////////////////////////////////////////////////////////////////////
// Names and related material.
////////////////////////////////////////////////////////////////////////////////

/// We use `u64` values to represent variable names.
pub type Name = u64;

/// Fresh name generation, for e.g. implementing the capture-avoiding
/// substitution action.  Finds a name that is not contained in the `avoid` set
/// of names.
pub fn fresh<T>(mut avoid: T) -> Name
where
    T: Iterator<Item = Name>,
{
    let mut counter = 0u64;

    loop {
        if avoid.any(|x| x == counter) {
            if let Some(next) = counter.checked_add(1) {
                counter = next;
            } else {
                panic!("{}", FRESH_NAME_GENERATION_FAILED);
            }
        } else {
            info!("Fresh name generated: {}.", counter);
            return counter;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests.
////////////////////////////////////////////////////////////////////////////////

/// Tests for fresh name generation-related functionality.
#[cfg(test)]
mod test {
    use crate::name::fresh;

    /// Tests that fresh-name generation is indeed fresh.
    #[test]
    pub fn name_test0() {
        let n = fresh(0..100);

        assert!(!(0..100).contains(&n));
    }
}
