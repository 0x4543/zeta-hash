use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ZetaError {
    Io(io::Error),
    Internal(String),
}

impl fmt::Display for ZetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZetaError::Io(e) => write!(f, "IO Error: {}", e),
            ZetaError::Internal(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl From<io::Error> for ZetaError {
    fn from(err: io::Error) -> ZetaError {
        ZetaError::Io(err)
    }
}

impl std::error::Error for ZetaError {}