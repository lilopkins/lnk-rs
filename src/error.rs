use std::io::ErrorKind;

use thiserror::Error;

/// The error type for shell link parsing errors.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("The parsed file isn't a shell link.")]
    NotAShellLinkError,

    #[error("Unexpected End-of-File while expecting a '{0}' instead")]
    UnexpectedEof(&'static str),

    #[error("Error while parsing {0}: {1}")]
    BinReadError(&'static str, binrw::Error),

    #[error("Error while writing {0}: {1}")]
    BinWriteError(&'static str, binrw::Error),
}

impl Error {
    /// creates an [`Error::BinReadError`] instance which wraps a [`binrw::Error`]
    /// together with some context information which describes where the error
    /// has occurred.
    pub fn while_parsing(context: &'static str, be: binrw::Error) -> Self {
        if let binrw::Error::Io(ref why) = be {
            if why.kind() == ErrorKind::UnexpectedEof {
                return Self::UnexpectedEof(context);
            }
        }
        Self::BinReadError(context, be)
    }

    /// creates an [`Error::BinWriteError`] instance which wraps a [`binrw::Error`]
    /// together with some context information which describes where the error
    /// has occurred.
    pub fn while_writing(context: &'static str, be: binrw::Error) -> Self {
        Self::BinWriteError(context, be)
    }
}
