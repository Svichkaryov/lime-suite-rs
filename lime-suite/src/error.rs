mod lms_rv;
mod lms_rv_ns;

pub use lms_rv::*;
pub use lms_rv_ns::*;

#[derive(Debug)]
pub enum Error {
    Lms(LmsRetVal),

    InvalidValue,

    InvalidInput,

    NotSupported,

    CantDispose,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lms(e) => write!(f, "LMS error: {e}"),
            Error::InvalidValue => write!(f, "Invalid value"),
            Error::InvalidInput => write!(f, "Invalid input"),
            Error::NotSupported => write!(f, "Feature not supported"),
            Error::CantDispose => write!(f, "Can't dispose"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
impl From<LmsRetVal> for Error {
    fn from(lms_rv_error: LmsRetVal) -> Self {
        Error::Lms(lms_rv_error)
    }
}

// Main Result type
// pub type Result<T> = core::result::Result<T, Error>;
