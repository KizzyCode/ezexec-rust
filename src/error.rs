//! Implements the crate's error types

use ebacktrace::define_error;
use std::{
    result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::new($kind, format!($($arg)*)) })
}
/// Creates a new `ErrorImpl::PathError` kind
#[macro_export] macro_rules! epath {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::PathError, $($arg)*) });
}
/// Creates a new `ErrorImpl::ExecError` kind
#[macro_export] macro_rules! eexec {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::ExecError, $($arg)*) });
}
/// Creates a new `ErrorImpl::ChildError` kind
#[macro_export] macro_rules! echild {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::ChildError, $($arg)*) });
}


/// The error kind
#[derive(Debug)]
pub enum ErrorKind {
    /// Failed to find the requested binary
    PathError,
    /// Failed to execute the child
    ExecError,
    /// The child exited with a non-zero error code
    ChildError
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::PathError => write!(f, "Failed to find the requested binary"),
            Self::ExecError => write!(f, "Failed to execute the child"),
            Self::ChildError => write!(f, "The child exited with a non-zero error code")
        }
    }
}


// Define our custom error type
define_error!(ErrorImpl);


/// A nice typealias for our custom error
pub type Error = ErrorImpl<ErrorKind>;
/// A nice typealias for a `Result` with our custom error
pub type Result<T = ()> = result::Result<T, ErrorImpl<ErrorKind>>;
