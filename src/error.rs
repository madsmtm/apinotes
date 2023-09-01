use core::fmt;

/// The error type used in this crate.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Yaml(serde_yaml::Error),
}

impl Error {
    pub(crate) fn from_yaml(err: serde_yaml::Error) -> Self {
        Self {
            kind: ErrorKind::Yaml(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Yaml(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
