use std::{fmt, io};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Stretch(io::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (comment, error) = match self {
            Error::Stretch(e) => ("Failed to determine terminal width: ", e),
            Error::Io(e) => ("", e),
        };

        write!(f, "{comment}{error}")
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        crate::Error::Io(value)
    }
}
