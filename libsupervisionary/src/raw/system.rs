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
//! [Dominic Mulligan]: https://dominicpm.github.io

use crate::raw::ErrorCode;
use std::convert::TryFrom;

extern "C" {
    /// Raw ABI binding to the `System.IO.Write` function.
    fn __system_io_write(bytes: *const u8, length: u64) -> i32;
    /// Raw ABI binding to the `System.IO.WriteError` function.
    fn __system_io_write_error(bytes: *const u8, length: u64) -> i32;
}

pub fn system_io_write<S>(s: S) -> Result<(), ErrorCode>
where
    S: Into<String>,
{
    let s = s.into();
    let status = unsafe { __system_io_write(s.as_ptr(), s.len() as u64) };

    if status == 0 {
        Ok(())
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}

pub fn system_io_write_error<S>(s: S) -> Result<(), ErrorCode>
where
    S: Into<String>,
{
    let s = s.into();
    let status = unsafe { __system_io_write_error(s.as_ptr(), s.len() as u64) };

    if status == 0 {
        Ok(())
    } else {
        Err(ErrorCode::try_from(status).unwrap())
    }
}
