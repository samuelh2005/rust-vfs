use core::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    NotDir,
    IsDir,
    AlreadyExists,
    InvalidInput,
    IoError,
    NotSupported,
    BadDescriptor,
    PermissionDenied,
    TooManyFiles,
    Unknown,
}

pub type Result<T> = core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_: core::fmt::Error) -> Self {
        Error::IoError
    }
}
