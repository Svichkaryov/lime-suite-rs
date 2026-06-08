use lime_suite_sys::bindings::LMS_SUCCESS;

use super::Error;

#[allow(non_camel_case_types)]
pub type LMS_RV = core::ffi::c_int;

/// General error for all functions.
pub const LMS_GENERAL_ERROR: core::ffi::c_int = -1;

/// LMS function return values.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LmsRetVal {
    Success,
    GeneralError,
}

impl std::fmt::Display for LmsRetVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LmsRetVal::Success => {
                write!(f, "Identifier: Success. Description: The function executed successfully.")
            }
            LmsRetVal::GeneralError => {
                write!(f, "Identifier: GeneralError. Description: Some horrible error has occurred.")
            }
        }
    }
}

impl From<LMS_RV> for LmsRetVal {
    fn from(lms_rv: LMS_RV) -> Self {
        match lms_rv {
            LMS_SUCCESS => LmsRetVal::Success,
            LMS_GENERAL_ERROR => LmsRetVal::GeneralError,
            other => {
                eprintln!("Undefined ({:#X}) error", other);
                LmsRetVal::GeneralError
            }
        }
    }
}

impl LmsRetVal {
    /// Convert the return value into a standard Result type
    pub fn into_result(self) -> Result<(), Error> {
        match self {
            LmsRetVal::Success => Ok(()),
            lms_rv_error => Err(Error::Lms(lms_rv_error)),
        }
    }
}
