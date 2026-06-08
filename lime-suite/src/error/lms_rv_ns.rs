use super::{Error, LmsRetVal};
use crate::error::LMS_GENERAL_ERROR;

#[allow(non_camel_case_types)]
pub type LMS_RV_NS = core::ffi::c_int;

/// LMS function return values.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LmsRetValNonStandard {
    Value(LMS_RV_NS),
    GeneralError,
}

impl std::fmt::Display for LmsRetValNonStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LmsRetValNonStandard::Value(v) => {
                write!(f, "Identifier: Value. Description: The function executed successfully with value({v}).")
            }
            LmsRetValNonStandard::GeneralError => {
                write!(f, "Identifier: GeneralError. Description: Some horrible error has occurred.")
            }
        }
    }
}

impl From<LMS_RV_NS> for LmsRetValNonStandard {
    fn from(lms_rv_ns: LMS_RV_NS) -> Self {
        match lms_rv_ns {
            LMS_GENERAL_ERROR => LmsRetValNonStandard::GeneralError,
            value => LmsRetValNonStandard::Value(value),
        }
    }
}

impl LmsRetValNonStandard {
    /// Convert the return value into a standard Result type
    pub fn into_result(self) -> Result<i32, Error> {
        match self {
            LmsRetValNonStandard::GeneralError => {
                Err(Error::Lms(LmsRetVal::GeneralError))
            }
            LmsRetValNonStandard::Value(value) => Ok(value),
        }
    }
}
