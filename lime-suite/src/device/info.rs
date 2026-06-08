//! Version and update functions
//!
//! The functions in this section provides ability to check device version
//! and perform updates

use std::ffi::{CStr, CString};
use std::ops::Deref;

use lime_suite_sys::bindings::*;

use crate::error::{Error, LmsRetVal, LmsRetValNonStandard};

use super::{Device, invoke_lms};

/// Callback from programming processes.
pub type ProgCallback = lms_prog_callback_t;

/// Device information structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DevInfo {
    /// The display name of the device
    pub device_name: String,
    /// The display name of the expansion card
    pub expension_name: String,
    /// The firmware version as a string
    pub firmware_version: String,
    /// The hardware version as a string
    pub hardware_version: String,
    /// The protocol version as a string
    pub protocol_version: String,
    /// A unique board serial number
    pub board_serial_number: u64,
    /// Gateware version as a string
    pub gateware_version: String,
    /// Which board should use this gateware
    pub gateware_target_board: String,
}

/// Convert slice to string
pub(crate) fn string_from_slice(slice: &[core::ffi::c_char]) -> String {
    unsafe {
        CStr::from_ptr(slice.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}

impl From<lms_dev_info_t> for DevInfo {
    fn from(dev_info: lms_dev_info_t) -> Self {
        Self {
            device_name: string_from_slice(&dev_info.deviceName),
            expension_name: string_from_slice(&dev_info.expensionName),
            firmware_version: string_from_slice(&dev_info.firmwareVersion),
            hardware_version: string_from_slice(&dev_info.hardwareVersion),
            protocol_version: string_from_slice(&dev_info.protocolVersion),
            board_serial_number: dev_info.boardSerialNumber,
            gateware_version: string_from_slice(&dev_info.gatewareVersion),
            gateware_target_board: string_from_slice(
                &dev_info.gatewareTargetBoard,
            ),
        }
    }
}

/// Message logging level
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[non_exhaustive]
pub struct LoggingLevel(i32);

impl LoggingLevel {
    /// A critical error. The application might not be able to continue
    /// running successfully.
    pub const LOG_CRITICAL: LoggingLevel = LoggingLevel(LMS_LOG_CRITICAL);
    /// An error message. An operation did not complete successfully.
    pub const LOG_ERROR: LoggingLevel = LoggingLevel(LMS_LOG_ERROR);
    /// A warning message. An operation completed with an unexpected result.
    pub const LOG_WARNING: LoggingLevel = LoggingLevel(LMS_LOG_WARNING);
    /// An informational message, usually denoting the successful
    /// completion of an operation.
    pub const LOG_INFO: LoggingLevel = LoggingLevel(LMS_LOG_INFO);
    /// A debugging message.
    pub const LOG_DEBUG: LoggingLevel = LoggingLevel(LMS_LOG_DEBUG);
}

impl From<LoggingLevel> for i32 {
    fn from(board_param: LoggingLevel) -> Self {
        *board_param
    }
}

impl Deref for LoggingLevel {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Callback function for redirecting API messages.
pub type LogHandler = LMS_LogHandler;

#[derive(Debug, Clone)]
pub enum ProgramMode {
    AutoUpdate,
    Fx3Flash,
    Fx3RAM,
    Fx3Reset,
    FpgaFlash,
    FpgaRAM,
    FpgaReset,
    McuRAM,
    McuEEPROM,
    McuReset,
    /// Other modes
    Custom(String),
}

impl From<String> for ProgramMode {
    fn from(mode: String) -> Self {
        match &mode as &str {
            "Automatic" => Self::AutoUpdate,
            "FX3 FLASH" => Self::Fx3Flash,
            "FX3 RAM" => Self::Fx3RAM,
            "FX3 Reset" => Self::Fx3Reset,
            "FPGA FLASH" => Self::FpgaFlash,
            "FPGA RAM" => Self::FpgaRAM,
            "FPGA Reset" => Self::FpgaReset,
            "LMSMCU SRAM" => Self::McuRAM,
            "LMSMCU EEPROM" => Self::McuEEPROM,
            "LMSMCU Reset" => Self::McuReset,
            _ => Self::Custom(mode),
        }
    }
}

impl From<ProgramMode> for String {
    fn from(mode: ProgramMode) -> String {
        match mode {
            ProgramMode::AutoUpdate => "Automatic".to_owned(),
            ProgramMode::Fx3Flash => "FX3 FLASH".to_owned(),
            ProgramMode::Fx3RAM => "FX3 RAM".to_owned(),
            ProgramMode::Fx3Reset => "FX3 Reset".to_owned(),
            ProgramMode::FpgaFlash => "FPGA FLASH".to_owned(),
            ProgramMode::FpgaRAM => "FPGA RAM".to_owned(),
            ProgramMode::FpgaReset => "FPGA Reset".to_owned(),
            ProgramMode::McuRAM => "LMSMCU SRAM".to_owned(),
            ProgramMode::McuEEPROM => "LMSMCU EEPROM".to_owned(),
            ProgramMode::McuReset => "LMSMCU Reset".to_owned(),
            ProgramMode::Custom(mode) => mode.clone(),
        }
    }
}

impl Device {
    /// Obtain the list of supported programming modes.
    pub fn get_program_modes(&self) -> Result<Vec<ProgramMode>, Error> {
        let num_modes = LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetProgramModes,
            self.get_handle(),
            std::ptr::null_mut() as *mut lms_name_t
        ))
        .into_result()? as usize;

        let mut list: Vec<lms_name_t> =
            vec![[0; std::mem::size_of::<lms_name_t>()]; num_modes];

        LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetProgramModes,
            self.get_handle(),
            list.as_mut_ptr() as *mut lms_name_t
        ))
        .into_result()?;

        let mut program_modes_list: Vec<ProgramMode> = Vec::new();
        for mode in list.iter() {
            unsafe {
                program_modes_list.push(
                    CStr::from_ptr(mode.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .into(),
                );
            }
        }

        Ok(program_modes_list)
    }

    /// Write binary firmware/bitsteam image to specified device
    /// component using the programming `mode` obtained in
    /// [`get_program_modes`](crate::device::Device::get_program_modes).
    pub fn program(
        &self,
        data: &[u8],
        mode: ProgramMode,
        callback: ProgCallback,
    ) -> Result<(), Error> {
        let mode = CString::new(String::from(mode))
            .map_err(|_| Error::InvalidValue)?;

        let mode_cstr = mode.as_c_str();
        if mode_cstr.to_bytes().len() > std::mem::size_of::<lms_name_t>() - 1 {
            return Err(Error::InvalidValue);
        }

        LmsRetVal::from(invoke_lms!(
            LMS_Program,
            self.get_handle(),
            data.as_ptr() as *const core::ffi::c_char,
            data.len(),
            mode_cstr.as_ptr() as *const lms_name_t,
            callback
        ))
        .into_result()
    }

    /// Obtain device serial number and version information.
    pub fn get_device_info(&self) -> Result<Option<DevInfo>, Error> {
        let dev_info_t = invoke_lms!(LMS_GetDeviceInfo, self.get_handle());

        let dev_info_t_ref = unsafe { dev_info_t.as_ref() };

        Ok(dev_info_t_ref.map(|v| DevInfo::from(*v)))
    }

    /// Register a new system log handler for processing API messages.
    /// Should be called to replace the default stdio handler.
    pub fn register_log_handler(&self, handler: LogHandler) {
        invoke_lms!(LMS_RegisterLogHandler, handler);
    }
}

/// Obtain API library version.
pub fn get_library_version() -> Result<String, Error> {
    let version_ptr = invoke_lms!(LMS_GetLibraryVersion,);
    let version = unsafe {
        CStr::from_ptr(version_ptr)
            .to_owned()
            .into_string()
            .map_err(|_| Error::InvalidValue)?
    };

    Ok(version)
}
