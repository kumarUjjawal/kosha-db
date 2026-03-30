use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum DbError {
    EmptyKey,
    Io(io::Error),
    Corruption(&'static str),
}

pub type Result<T> = std::result::Result<T, DbError>;

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => write!(f, "key must not be empty"),
            Self::Io(err) => write!(f, "io error: {err}"),
            Self::Corruption(message) => write!(f, "message: {message}"),
        }
    }
}

impl Error for DbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::EmptyKey | Self::Corruption(_) => None,
        }
    }
}

impl From<io::Error> for DbError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
