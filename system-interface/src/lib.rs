#![feature(cstr_from_bytes_until_nul)]
//! # The Supervisionary system interface
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

use std::convert::TryInto;

pub mod pass_through;

////////////////////////////////////////////////////////////////////////////////
// File handles.
////////////////////////////////////////////////////////////////////////////////

/// The type of file handles, merely 64-bit machine words.
pub type FileHandle = u64;

/// The distinguished `stdout` file handle.
pub const STDOUT_FILEHANDLE: FileHandle = 0;
/// The distinguished `stdin` file handle.
pub const STDIN_FILEHANDLE: FileHandle = 1;
/// The distinguished `stderr` file handle.
pub const STDERR_FILEHANDLE: FileHandle = 2;

////////////////////////////////////////////////////////////////////////////////
// File mode.
////////////////////////////////////////////////////////////////////////////////

/// File modes indicating whether a file should be opened as a readable,
/// writeable or both, and so on.
#[repr(u32)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FileMode {
    /// The file should be opened in a read-only mode.
    Read = 0x0,
    /// The file should be opened in a write-only mode.
    Write = 0x1,
    /// The file should be opened in a readable and writable mode.
    ReadWrite = 0x2,
    /// The file should be opened in an appendable mode.
    Append = 0x3,
}

impl FileMode {
    /// Converts a `FileMode` into a string encoding suitable for use with
    /// `libc`-type functions that expect mode strings.
    #[inline]
    pub fn unix_file_mode(&self) -> &str {
        match self {
            FileMode::Read => "r",
            FileMode::Write => "w",
            FileMode::ReadWrite => "rw",
            FileMode::Append => "a",
        }
    }
}

impl From<FileMode> for u32 {
    #[inline]
    fn from(value: FileMode) -> u32 {
        match value {
            FileMode::Read => 0x0,
            FileMode::Write => 0x1,
            FileMode::ReadWrite => 0x2,
            FileMode::Append => 0x3,
        }
    }
}

impl TryInto<FileMode> for u32 {
    type Error = ();

    fn try_into(self) -> Result<FileMode, Self::Error> {
        if self == 0x0 {
            Ok(FileMode::Read)
        } else if self == 0x1 {
            Ok(FileMode::Write)
        } else if self == 0x2 {
            Ok(FileMode::ReadWrite)
        } else if self == 0x3 {
            Ok(FileMode::Append)
        } else {
            Err(())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// File permissions.
////////////////////////////////////////////////////////////////////////////////

/// File permissions indicating whether a file handle points-to a file that is
/// readable, writeable, or both.  Note we do not handle executable files (yet).
#[repr(u32)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilePermissions {
    /// The file read permission bit.
    Read = 0x0,
    /// The file write permission bit.
    Write = 0x1,
}

impl From<FilePermissions> for u32 {
    #[inline]
    fn from(value: FilePermissions) -> u32 {
        match value {
            FilePermissions::Read => 0x0,
            FilePermissions::Write => 0x1,
        }
    }
}

impl TryInto<FilePermissions> for u32 {
    type Error = ();

    fn try_into(self) -> Result<FilePermissions, Self::Error> {
        if self == 0x0 {
            Ok(FilePermissions::Read)
        } else if self == 0x1 {
            Ok(FilePermissions::Write)
        } else {
            Err(())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// File types.
////////////////////////////////////////////////////////////////////////////////

/// File permissions indicating whether a file handle points-to a file that is
/// readable, writeable, or both.  Note we do not handle executable files (yet).
#[repr(u32)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FileType {
    /// A normal file, for example a text file inside the system's filesystem.
    Normal = 0x0,
    /// A special file, for example a file under kernel-management akin to the
    /// Linux `/dev/` or `/proc/` filesystems.
    Special = 0x1,
}

impl From<FileType> for u32 {
    #[inline]
    fn from(value: FileType) -> u32 {
        match value {
            FileType::Normal => 0x0,
            FileType::Special => 0x1,
        }
    }
}

impl TryInto<FileType> for u32 {
    type Error = ();

    fn try_into(self) -> Result<FileType, Self::Error> {
        if self == 0x0 {
            Ok(FileType::Normal)
        } else if self == 0x1 {
            Ok(FileType::Special)
        } else {
            Err(())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// System interface error codes.
////////////////////////////////////////////////////////////////////////////////

/// System error codes, which indicate that something went wrong when trying to
/// modify or query the system, and what.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemErrorCode {
    /// A file or directory was unknown.
    UnknownFile,
    /// A filemode was invalid.
    InvalidFileMode,
    /// A file or directory had the wrong permissions.
    PermissionError,
    /// A file was not open, but somebody tried to close it.
    FileNotOpen,
    /// A path was not a valid UTF-8 filepath.
    MalformedPath,
    /// A file handle was not valid, and did not point-to an open file.
    InvalidHandle,
}

////////////////////////////////////////////////////////////////////////////////
// The generic system interface trait.
////////////////////////////////////////////////////////////////////////////////

/// A generic interface used to query and manipulate the system.  Note that this
/// allows us to decouple the rest of the Supervisionary codebase from the
/// decision over whether we are actually querying the real, underlying system
/// (e.g., the MacOS or Linux filesystem), or whether Supervisionary is
/// maintaining its own synthetic, in-memory filesystem.
pub trait SystemInterface {
    /// Opens a file at a given path.  Note that the `path` is assumed to be
    /// encoded as UTF-8 as a `/`-separated path.  If the path exists, the
    /// function returns a file handle which can be used henceforth to refer to
    /// the file.
    fn fopen(
        &mut self,
        path: Vec<u8>,
        mode: FileMode,
    ) -> Result<FileHandle, SystemErrorCode>;

    /// Closes a file pointed-to by a filehandle.
    fn fclose(&mut self, handle: FileHandle) -> Result<(), SystemErrorCode>;

    /// Writes a buffer of bytes to a file pointed-to by the filehandle.
    fn fwrite(
        &mut self,
        handle: FileHandle,
        buffer: Vec<u8>,
        count: usize,
    ) -> Result<usize, SystemErrorCode>;

    /// Reads a buffer of bytes from a file pointed-to by the filehandle.
    fn fread(
        &mut self,
        handle: FileHandle,
        buffer: &mut [u8],
        count: usize,
    ) -> Result<usize, SystemErrorCode>;
}
