use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Git { err: git2::Error },
    Io { err: io::Error },
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git { err }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io { err }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: ")?;
        match self {
            Error::Git { err } => write!(f, "{}", err),
            Error::Io { err } => write!(f, "{}", err),
        }
    }
}
