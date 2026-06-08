//! Low-Level control functions
//!
//! The functions in this section provide a low access to device such as
//! modifying device internal register or clock frequency. Low-Level functions
//! can be used to configure device entirely, however a more practical use
//! is to fine-tune device settings after configuring it with high-level
//! control functions.

use std::ffi::{CStr, CString};
use std::ops::Deref;

use lime_suite_sys::bindings::*;
use lime_suite_sys::params::*;

use crate::error::{Error, LmsRetVal};

use super::{invoke_lms, Device};

impl Device {
    /// Send Reset signal to LMS chip. This initializes LMS chip with default
    /// configuration as described in LMS chip datasheet.
    pub fn reset(&self) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(LMS_Reset, self.get_handle()))
            .into_result()
    }

    /// Read device LMS chip register.
    pub fn read_lms_reg(&self, address: u32) -> Result<u16, Error> {
        let mut val = u16::default();

        LmsRetVal::from(invoke_lms!(
            LMS_ReadLMSReg,
            self.get_handle(),
            address,
            &mut val as *mut u16
        ))
        .into_result()?;

        Ok(val)
    }

    /// Write device LMS chip register.
    pub fn write_lms_reg(
        &self,
        address: u32,
        value: u16,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_WriteLMSReg,
            self.get_handle(),
            address,
            value
        ))
        .into_result()
    }

    /// Read device parameter. Parameter defines specific bits in device
    /// register.
    pub fn read_param(&self, param: &Parameter) -> Result<u16, Error> {
        let mut val = u16::default();
        let lms_param: LMS7Parameter = LMS7Parameter::from(param);

        LmsRetVal::from(invoke_lms!(
            LMS_ReadParam,
            self.get_handle(),
            lms_param,
            &mut val as *mut u16
        ))
        .into_result()?;

        Ok(val)
    }

    /// Write device parameter. Parameter defines specific bits in device
    /// register.
    pub fn write_param(
        &self,
        param: &Parameter,
        val: u16,
    ) -> Result<(), Error> {
        let lms_param: LMS7Parameter = LMS7Parameter::from(param);

        LmsRetVal::from(invoke_lms!(
            LMS_WriteParam,
            self.get_handle(),
            lms_param,
            val
        ))
        .into_result()
    }

    /// Read device FPGA register.
    pub fn read_fpga_reg(&self, address: u32) -> Result<u16, Error> {
        let mut val = u16::default();

        LmsRetVal::from(invoke_lms!(
            LMS_ReadFPGAReg,
            self.get_handle(),
            address,
            &mut val as *mut u16
        ))
        .into_result()?;

        Ok(val)
    }

    /// Write device FPGA register.
    pub fn write_fpga_reg(&self, address: u32, val: u16) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_WriteFPGAReg,
            self.get_handle(),
            address,
            val
        ))
        .into_result()
    }
}

/// Board parameter
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[non_exhaustive]
pub struct BoardParameter(u8);

impl BoardParameter {
    /// Runtime VCTCXO DAC trim value. Does not persist over power-cycle.
    pub const PARAM_DAC: BoardParameter = BoardParameter(BOARD_PARAM_DAC);
    /// The value of board temperature sensor (if present), read-only.
    pub const PARAM_TEMP: BoardParameter = BoardParameter(BOARD_PARAM_TEMP);
}

impl From<BoardParameter> for u8 {
    fn from(board_param: BoardParameter) -> Self {
        *board_param
    }
}

impl Deref for BoardParameter {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Device {
    /// Read custom parameter from board.
    ///
    /// Return current register value and measurement units of parameter
    /// if available.
    pub fn read_custom_board_param(
        &self,
        id: BoardParameter,
    ) -> Result<(f64, Option<String>), Error> {
        let mut val = f64::default();
        let mut units: lms_name_t = lms_name_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_ReadCustomBoardParam,
            self.get_handle(),
            *id,
            &mut val as *mut float_type,
            units.as_mut_ptr() as *mut lms_name_t
        ))
        .into_result()?;

        unsafe {
            let cstr = CStr::from_ptr(units.as_ptr());
            if cstr.is_empty() {
                return Ok((val, None));
            }
            Ok((val, Some(cstr.to_str().unwrap().to_owned())))
        }
    }

    /// Write custom parameter from board.
    pub fn write_custom_board_param(
        &self,
        id: BoardParameter,
        value: f64,
        units: Option<String>,
    ) -> Result<(), Error> {
        match units {
            Some(u) => {
                let units =
                    CString::new(u).map_err(|_| Error::InvalidValue)?;

                let units_cstr = units.as_c_str();
                if units_cstr.to_bytes().len()
                    > std::mem::size_of::<lms_name_t>() - 1
                {
                    return Err(Error::InvalidValue);
                }

                LmsRetVal::from(invoke_lms!(
                    LMS_WriteCustomBoardParam,
                    self.get_handle(),
                    *id,
                    value,
                    units_cstr.as_ptr() as *const lms_name_t
                ))
                .into_result()
            }
            None => LmsRetVal::from(invoke_lms!(
                LMS_WriteCustomBoardParam,
                self.get_handle(),
                *id,
                value,
                std::ptr::null() as *const lms_name_t
            ))
            .into_result(),
        }
    }
}

/// Clock definitions for accessing specific internal clocks
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[non_exhaustive]
pub struct ClockIdentifier(usize);

impl ClockIdentifier {
    /// Chip reference clock
    pub const CLOCK_REF: ClockIdentifier = ClockIdentifier(LMS_CLOCK_REF);
    /// RX LO clock
    pub const CLOCK_SXR: ClockIdentifier = ClockIdentifier(LMS_CLOCK_SXR);
    /// RX LO clock
    pub const LMS_CLOCK_SXT: ClockIdentifier = ClockIdentifier(LMS_CLOCK_SXT);
    /// CGEN clock
    pub const CLOCK_CGEN: ClockIdentifier = ClockIdentifier(LMS_CLOCK_CGEN);
    /// RXTSP reference clock (read-only)
    pub const CLOCK_RXTSP: ClockIdentifier = ClockIdentifier(LMS_CLOCK_RXTSP);
    /// TXTSP reference clock (read-only)
    pub const CLOCK_TXTSP: ClockIdentifier = ClockIdentifier(LMS_CLOCK_TXTSP);
    /// External reference clock (write-only)
    ///
    /// Set to positive value to enable usage of external reference
    /// clock of the specified frequency. Set to 0 or negative
    /// value to disable usage of external reference clock (if switching
    /// reference clock source is supported by HW)
    pub const CLOCK_EXTREF: ClockIdentifier =
        ClockIdentifier(LMS_CLOCK_EXTREF);
}

impl From<ClockIdentifier> for usize {
    fn from(clk_id: ClockIdentifier) -> Self {
        *clk_id
    }
}

impl Deref for ClockIdentifier {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Device {
    /// Obtain the frequency of the specified clock in Hz.
    pub fn get_clock_freq(
        &self,
        clk_id: ClockIdentifier,
    ) -> Result<f64, Error> {
        let mut freq = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetClockFreq,
            self.get_handle(),
            *clk_id,
            &mut freq as *mut float_type
        ))
        .into_result()?;

        Ok(freq)
    }

    /// Set frequency of the specified clock in Hz.
    ///
    /// Pass zero or negative `freq` value to only perform tune (if supported)
    /// without recalculating values.
    pub fn set_clock_freq(
        &self,
        clk_id: ClockIdentifier,
        freq: f64,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetClockFreq,
            self.get_handle(),
            *clk_id,
            freq
        ))
        .into_result()
    }

    /// Write value to VCTCXO trim DAC. Used to adjust/calibrate reference
    /// clock generated by voltage controlled oscillator. Value is written to
    /// non-volatile storage.
    ///
    /// # Note
    ///
    /// Calling this functions switches clock source to VCTCXO.
    pub fn vctcxo_write(&self, value: u16) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(LMS_VCTCXOWrite, self.get_handle(), value))
            .into_result()
    }

    /// Read VCTCXO trim DAC value from non-volatile storage.
    /// Returned value is value that is loaded on power-on and may different
    /// from current runtime value.
    pub fn vctcxo_read(&self) -> Result<u16, Error> {
        let mut val = u16::default();

        LmsRetVal::from(invoke_lms!(
            LMS_VCTCXORead,
            self.get_handle(),
            &mut val as *mut u16
        ))
        .into_result()?;

        Ok(val)
    }

    /// Synchronizes register values between API cache and chip.
    pub fn synchronize(&self, to_chip: bool) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_Synchronize,
            self.get_handle(),
            to_chip
        ))
        .into_result()
    }

    /// Obtain vector of the specified length with read values.
    /// 8 GPIO values per byte, LSB first.
    pub fn gpio_read(&self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![0; len];

        LmsRetVal::from(invoke_lms!(
            LMS_GPIORead,
            self.get_handle(),
            buffer.as_mut_ptr(),
            len
        ))
        .into_result()?;

        Ok(buffer)
    }

    /// Change GPIO pins using byte slice.
    /// 8 GPIO values per byte, LSB firs.
    ///
    /// # Examples
    ///
    /// ```
    /// use lime_suite::device::Context;
    ///
    /// let dev = Context::open(None).expect("failed to open device");
    ///
    /// // Set GPIO pins direction
    /// let gpio_dir: [u8; 1] = [0x8f];
    /// dev.gpio_dir_write(&gpio_dir)
    ///     .expect("failed to change gpio direction");
    /// println!("Set GPIO0, GPIO1, GPIO2, GPIO3 and GPIO7 to output");
    ///
    /// // Set GPIO pins output level. Only affect GPIO that
    /// // configured as output.
    /// let gpio_dir: [u8; 1] = [0x81];
    /// dev.gpio_write(&gpio_dir).expect("failed to change gpio values");
    /// println!("Set GPIO0, GPIO7 output to High level");
    /// ```
    pub fn gpio_write(&self, buffer: &[u8]) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_GPIOWrite,
            self.get_handle(),
            buffer.as_ptr(),
            buffer.len()
        ))
        .into_result()
    }

    /// Obtain vector of the specified length with GPIO direction
    /// configuration. 8 GPIO per byte, LSB first; 0 input, 1 output.
    pub fn gpio_dir_read(&self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![0; len];

        LmsRetVal::from(invoke_lms!(
            LMS_GPIODirRead,
            self.get_handle(),
            buffer.as_mut_ptr(),
            len
        ))
        .into_result()?;

        Ok(buffer)
    }

    /// Change GPIO pins direction using byte slice.
    /// 8 GPIO per byte, LSB first; 0 input, 1 output.
    ///
    /// # Examples
    ///
    /// ```
    /// use lime_suite::device::Context;
    ///
    /// let dev = Context::open(None).expect("failed to open device");
    ///
    /// let gpio_dir: [u8; 1] = [0x8f];
    /// dev.gpio_dir_write(&gpio_dir)
    ///     .expect("failed to change gpio direction");
    /// println!("Set GPIO0, GPIO1, GPIO2, GPIO3 and GPIO7 to output");
    /// ```
    pub fn gpio_dir_write(&self, buffer: &[u8]) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_GPIODirWrite,
            self.get_handle(),
            buffer.as_ptr(),
            buffer.len()
        ))
        .into_result()
    }
}
