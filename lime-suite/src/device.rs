use std::sync::Arc;

use lime_suite_sys::bindings::*;

pub mod context;
pub mod high_level;
pub mod info;
pub mod low_level;
pub mod streaming;

macro_rules! invoke_lms {
    ($func_name:ident, $($params:expr),*) => {
        unsafe {
            $func_name($($params),*)
        }
    };
}

pub(super) use invoke_lms;

struct ContextInner {
    device_handle: *mut lms_device_t,
}

/// Library context to a lime device.
pub struct Context {
    inner: Arc<ContextInner>,
}

unsafe impl Send for ContextInner {}
unsafe impl Sync for ContextInner {}

/// A connection to a lime device.
#[derive(Clone)]
pub struct Device {
    context: Context,
}
