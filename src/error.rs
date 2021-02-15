use std::{fmt, io, path};

#[derive(Debug)]
pub enum Error {
    Git(git2::Error),
    Io(io::Error),
    Path(path::StripPrefixError),
    Internal(String),
}

macro_rules! internal {
    ($msg: literal) => {
        $crate::error::Error::Internal(String::from($msg))
    };
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Error {
        Error::Path(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: ")?;
        match self {
            Error::Git(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::Path(err) => write!(f, "{}", err),
            Error::Internal(msg) => write!(f, "{}", msg),
        }
    }
}
