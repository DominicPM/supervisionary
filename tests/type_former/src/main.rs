//! # Tests for the Supervisionary type-former ABI
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

use libsupervisionary::raw::type_former::*;

fn main() {
    assert!(type_former_is_registered(
        PREALLOCATED_HANDLE_TYPE_FORMER_PROP
    ));

    assert!(type_former_is_registered(
        PREALLOCATED_HANDLE_TYPE_FORMER_ARROW
    ));

    assert_eq!(
        type_former_resolve(PREALLOCATED_HANDLE_TYPE_FORMER_PROP),
        Ok(0u64)
    );

    assert_eq!(
        type_former_resolve(PREALLOCATED_HANDLE_TYPE_FORMER_ARROW),
        Ok(2u64)
    );

    let handle = type_former_register(5u64);

    assert_eq!(type_former_resolve(&handle), Ok(5u64));
    assert!(type_former_is_registered(&handle));
}
