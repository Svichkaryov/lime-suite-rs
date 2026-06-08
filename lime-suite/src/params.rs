//! Params

pub use lime_suite_sys::params::*;

/// Create reference to lms parameter
#[macro_export]
macro_rules! ref_of_param {
    ($param_name:ident) => {
        &*params::$param_name as &params::Parameter
    };
}
