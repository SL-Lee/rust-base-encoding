use std::{error, fmt};

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::MalformedInput => {
                write!(f, "cannot decode malformed input stream")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::MalformedInput => {
                write!(f, "cannot decode malformed input stream")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    MalformedInput,
}
