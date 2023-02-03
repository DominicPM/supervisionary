//! # Bindings to Supervisionary's system access ABI
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

use crate::raw::ErrorCode;

extern "C" {
    /// Raw ABI binding to the `System.IO.File.Open` function.
    fn __system_io_fopen(
        path: *const u8,
        length: u32,
        mode: u32,
        handle: *mut u64,
    ) -> i32;
    /// Raw ABI binding to the `System.IO.File.Close` function.
    fn __system_io_fclose(handle: u64) -> i32;
}
