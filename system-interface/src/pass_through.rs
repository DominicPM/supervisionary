//! # Pass through: access to the underlying system
//!
//! The following implements a "pass through" system interface, which simply
//! calls down to the underlying system, e.g., the MacOS or Linux, or whatever
//! OS Supervisionary is running under, filesytem.
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

use crate::{FileHandle, FileMode, SystemErrorCode, SystemInterface};
use log::info;
use nix::libc::{fclose, fopen, fread, fwrite, FILE};
use std::ffi::{c_void, CString};

////////////////////////////////////////////////////////////////////////////////
// Constants.
////////////////////////////////////////////////////////////////////////////////

/// Runtime abort message produced if the file mode conversion fails to produce
/// a valid C-style string.
const FILE_MODE_CONVERSION_ERROR_MESSAGE: &str =
    "Internal invariant failed: file mode conversion failed.";

////////////////////////////////////////////////////////////////////////////////
// The dummy type.
////////////////////////////////////////////////////////////////////////////////

/// This is "just" a dummy type which maintains no internal state, and is merely
/// introduced to have a type to implement a trait on.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PassThroughSystemInterface;

impl Default for PassThroughSystemInterface {
    #[inline]
    fn default() -> Self {
        PassThroughSystemInterface
    }
}

impl PassThroughSystemInterface {
    /// Creates a new, pass through system interface.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Instantiating the SystemInterface trait.
////////////////////////////////////////////////////////////////////////////////

impl SystemInterface for PassThroughSystemInterface {
    /// "Passthrough" implementation of `fopen` which merely calls through to
    /// the underlying `libc` implementation of `fopen` on the host's machine.
    fn fopen(
        &mut self,
        path: Vec<u8>,
        mode: FileMode,
    ) -> Result<FileHandle, SystemErrorCode> {
        info!("Called `fopen` with {path:?} in {mode:?}.");

        let path =
            CString::new(path).map_err(|_| SystemErrorCode::MalformedPath)?;
        let mode = CString::new(mode.unix_file_mode().as_bytes())
            .expect(FILE_MODE_CONVERSION_ERROR_MESSAGE);

        /* NB: we should not return the pointer back to user-space but introduce
          an indirection, via some mapping between `FileHandle` and `*mut File`
          types...
        */
        let file = unsafe { fopen(path.as_ptr(), mode.as_ptr()) };

        if file.is_null() {
            Err(SystemErrorCode::UnknownFile)
        } else {
            Ok(file as u64)
        }
    }

    /// "Passthrough" implementation of `fclose` which merely calls through to
    /// the underlying `libc` implementation of `fclose` on the host's machine.
    fn fclose(&mut self, handle: FileHandle) -> Result<(), SystemErrorCode> {
        info!("Called `fclose` with {handle}.");

        let handle = handle as *mut FILE;

        let ret = unsafe { fclose(handle) };

        if ret != 0 {
            Err(SystemErrorCode::FileNotOpen)
        } else {
            Ok(())
        }
    }

    /// "Passthrough" implementation of `fwrite` which merely calls through to
    /// the underlying `libc` implementation of `fwrite` on the host's machine.
    fn fwrite(
        &mut self,
        handle: FileHandle,
        buffer: Vec<u8>,
        count: usize,
    ) -> Result<usize, SystemErrorCode> {
        info!("Called `fwrite` with {handle} and {buffer:?}.");

        let handle = handle as *mut FILE;

        let ret = unsafe {
            fwrite(
                buffer.as_ptr() as *const c_void,
                buffer.len(),
                count,
                handle,
            )
        };

        Ok(ret)
    }

    fn fread(
        &mut self,
        handle: FileHandle,
        buffer: &mut [u8],
        count: usize,
    ) -> Result<usize, SystemErrorCode> {
        info!("Called `fread` with {handle} and {buffer:?}.");

        let handle = handle as *mut FILE;

        let ret = unsafe {
            fread(buffer.as_ptr() as *mut c_void, buffer.len(), count, handle)
        };

        Ok(ret)
    }
}
