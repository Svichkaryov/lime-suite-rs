//! Initialization/deinitialization
//!
//! The functions in this section provide the ability to query available
//! devices, initialize them, and deinitialize them.

use std::ffi::{CStr, CString};
use std::mem::forget;
use std::sync::Arc;

use lime_suite_sys::bindings::*;

use crate::error::{Error, LmsRetVal, LmsRetValNonStandard};

use super::{invoke_lms, Context, ContextInner, Device};

impl Context {
    /// Obtain a list of LMS devices attached to the system.
    /// You can `validate` the list to retrieve only those devices
    /// that have had no problems getting the usb descriptor and
    /// serial number.
    pub fn get_device_list(validate: bool) -> Result<Vec<String>, Error> {
        let num_devices = LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetDeviceList,
            std::ptr::null_mut() as *mut lms_info_str_t
        ))
        .into_result()? as usize;

        let mut list: Vec<lms_info_str_t> =
            vec![[0; std::mem::size_of::<lms_info_str_t>()]; num_devices];

        LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetDeviceList,
            list.as_mut_ptr() as *mut lms_info_str_t
        ))
        .into_result()?;

        let mut dev_list: Vec<String> = Vec::new();
        for device in list.iter() {
            unsafe {
                dev_list.push(
                    CStr::from_ptr(device.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            }
        }

        if validate {
            dev_list.retain(|s| s.contains("serial="));
        }

        Ok(dev_list)
    }

    /// Opens device specified by the provided device information string.
    /// If None, the first available device will be opened.
    /// This function should be used to open a device based upon the results of
    /// [`get_device_list`](crate::device::context::Context::get_device_list).
    pub fn open(device: Option<String>) -> Result<Device, Error> {
        let mut dev: *mut lms_device_t = std::ptr::null_mut();

        match device {
            Some(v) => {
                let info_cstr = CString::new(v).unwrap();

                LmsRetVal::from(invoke_lms!(
                    LMS_Open,
                    std::ptr::addr_of_mut!(dev),
                    info_cstr.as_ptr() as *const lms_info_str_t,
                    std::ptr::null_mut()
                ))
                .into_result()?;
            }
            None => {
                LmsRetVal::from(invoke_lms!(
                    LMS_Open,
                    std::ptr::addr_of_mut!(dev),
                    std::ptr::null_mut() as *const lms_info_str_t,
                    std::ptr::null_mut()
                ))
                .into_result()?;
            }
        };

        Ok(Device {
            context: Context {
                inner: Arc::new(ContextInner { device_handle: dev }),
            },
        })
    }
}

impl Device {
    pub(crate) fn get_handle(&self) -> *mut lms_device_t {
        self.context.inner.device_handle
    }

    /// Close device.
    ///
    /// Device is deallocated and may no longer be used.
    pub fn close(self) -> Result<(), (Device, Error)> {
        match Arc::try_unwrap(self.context.inner) {
            Ok(inner) => {
                if let Err(e) = LmsRetVal::from(invoke_lms!(
                    LMS_Close,
                    inner.device_handle
                ))
                .into_result()
                {
                    let device = Device {
                        context: Context {
                            inner: Arc::new(inner),
                        },
                    };
                    return Err((device, e));
                }

                // Skip the drop, we did it manually.
                forget(inner);

                Ok(())
            }
            Err(arc_inner) => {
                let device = Device {
                    context: Context { inner: arc_inner },
                };
                Err((device, Error::CantDispose))
            }
        }
    }
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        if let Err(e) =
            LmsRetVal::from(invoke_lms!(LMS_Close, self.device_handle))
                .into_result()
        {
            println!("Failed to close device: {}", e);
        }
    }
}

impl Clone for Context {
    /// Clone the `Context`.
    fn clone(&self) -> Self {
        Context {
            inner: Arc::clone(&self.inner),
        }
    }
}
