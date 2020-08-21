use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

/// A common error variant returned by this library.
#[derive(Debug)]
pub enum Error {
    /// Returned when an unknown pixel is attempted to be processed.
    UnknownPixel(String),
    /// Returned when decompression of a file fails.
    DecompressionError(String),
    /// Returned when an IO operation fails.
    IoError(String),
    /// Returned when a non-specific, miscellaneous error occurs.
    ///
    /// It is also returned when a string is used to create an `Error` directly.
    Other(String),
}

impl Error {
    /// Returns a reference to the inner string of the error.
    pub fn inner(&self) -> &String {
        match self {
            Self::UnknownPixel(e) => e,
            Self::DecompressionError(e) => e,
            Self::IoError(e) => e,
            Self::Other(e) => e,
        }
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.inner())
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self::Other(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::IoError(error.to_string())
    }
}
