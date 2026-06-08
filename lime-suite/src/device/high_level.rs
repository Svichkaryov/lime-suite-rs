//! High-level control functions
//!
//! The functions in this section provide the ability to easily configure the
//! device for operation. They modify multiple internal device settings.

use std::ffi::{CStr, CString};
use std::ops::Deref;

use lime_suite_sys::bindings::*;

use crate::error::{Error, LmsRetVal, LmsRetValNonStandard};

use super::{Device, invoke_lms};

/// Sample rate
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct SampleRate {
    /// Sampling rate used for data exchange with the host.
    pub host_Hz: f64,
    /// RF sampling rate in Hz.
    pub rf_Hz: f64,
}

/// Range
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

/// Enumeration of LMS7 TEST signal types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestSignal {
    /// Disable test signals. Return to normal operation
    NONE = lms_testsig_t::LMS_TESTSIG_NONE as u32,
    /// Test signal from NCO half scale
    NCODIV8,
    /// Test signal from NCO half scale
    NCODIV4,
    /// Test signal from NCO full scale
    NCODIV8F,
    /// Test signal from NCO full scale
    NCODIV4F,
    /// DC test signal
    DC,
}

impl From<lms_testsig_t> for TestSignal {
    fn from(testsig: lms_testsig_t) -> Self {
        match testsig {
            lms_testsig_t::LMS_TESTSIG_NONE => Self::NONE,
            lms_testsig_t::LMS_TESTSIG_NCODIV8 => Self::NCODIV8,
            lms_testsig_t::LMS_TESTSIG_NCODIV4 => Self::NCODIV4,
            lms_testsig_t::LMS_TESTSIG_NCODIV8F => Self::NCODIV8F,
            lms_testsig_t::LMS_TESTSIG_NCODIV4F => Self::NCODIV4F,
            lms_testsig_t::LMS_TESTSIG_DC => Self::DC,
        }
    }
}

impl From<TestSignal> for lms_testsig_t {
    fn from(testsig: TestSignal) -> Self {
        match testsig {
            TestSignal::NONE => lms_testsig_t::LMS_TESTSIG_NONE,
            TestSignal::NCODIV8 => lms_testsig_t::LMS_TESTSIG_NCODIV8,
            TestSignal::NCODIV4 => lms_testsig_t::LMS_TESTSIG_NCODIV4,
            TestSignal::NCODIV8F => lms_testsig_t::LMS_TESTSIG_NCODIV8F,
            TestSignal::NCODIV4F => lms_testsig_t::LMS_TESTSIG_NCODIV4F,
            TestSignal::DC => lms_testsig_t::LMS_TESTSIG_DC,
        }
    }
}

/// Channel direction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[non_exhaustive]
pub struct ChannelDirection(pub bool);

impl ChannelDirection {
    /// Convenience constants for TX selection
    pub const TX: ChannelDirection = ChannelDirection(true);
    /// Convenience constants for RX selection
    pub const RX: ChannelDirection = ChannelDirection(false);
}

impl From<ChannelDirection> for bool {
    fn from(channel_dir: ChannelDirection) -> Self {
        *channel_dir
    }
}

impl Deref for ChannelDirection {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Enumeration of RF ports.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RfPort {
    inner: usize,
    ch_dir: Option<ChannelDirection>,
}

impl RfPort {
    /// No active path (RX or TX)
    pub const NONE: RfPort = RfPort {
        inner: LMS_PATH_NONE,
        ch_dir: None,
    };
    /// RX LNA_H port
    pub const LNAH: RfPort = RfPort {
        inner: LMS_PATH_LNAH,
        ch_dir: Some(ChannelDirection::RX),
    };
    /// RX LNA_L port
    pub const LNAL: RfPort = RfPort {
        inner: LMS_PATH_LNAL,
        ch_dir: Some(ChannelDirection::RX),
    };
    /// RX LNA_W port
    pub const LNAW: RfPort = RfPort {
        inner: LMS_PATH_LNAW,
        ch_dir: Some(ChannelDirection::RX),
    };
    /// TX port 1
    pub const TX1: RfPort = RfPort {
        inner: LMS_PATH_TX1,
        ch_dir: Some(ChannelDirection::TX),
    };
    /// TX port 2
    pub const TX2: RfPort = RfPort {
        inner: LMS_PATH_TX2,
        ch_dir: Some(ChannelDirection::TX),
    };
    /// Automatically select port (if supported)
    pub const AUTO: RfPort = RfPort {
        inner: LMS_PATH_AUTO,
        ch_dir: None,
    };
}

impl From<RfPort> for usize {
    fn from(rf_ports: RfPort) -> Self {
        *rf_ports
    }
}

impl Deref for RfPort {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::fmt::Display for RfPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RfPort::NONE => write!(f, "NONE"),
            RfPort::LNAH => write!(f, "LNAH"),
            RfPort::LNAL => write!(f, "LNAL"),
            RfPort::LNAW => write!(f, "LNAW"),
            RfPort::TX1 => write!(f, "TX1"),
            RfPort::TX2 => write!(f, "TX2"),
            RfPort::AUTO => write!(f, "AUTO"),
            _ => write!(f, "Unknown rf port"),
        }
    }
}

impl Device {
    /// Configure LMS chip with settings that make it ready for operation.
    ///
    /// This configuration differs from default LMS chip configuration which is
    /// described in chip datasheet. In order to load default chip
    /// configuration use [`reset`](crate::device::Device::reset).
    pub fn init(&self) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(LMS_Init, self.get_handle())).into_result()
    }

    /// Obtain number of RX or TX channels. Use this to determine the maximum
    /// channel index (specifying channel index is required by most functions).
    /// The maximum channel index is N-1, where N is number returned by this
    /// function.
    pub fn get_num_channels(
        &self,
        channel_dir: ChannelDirection,
    ) -> Result<i32, Error> {
        LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetNumChannels,
            self.get_handle(),
            *channel_dir
        ))
        .into_result()
    }

    /// Enable or disable specified RX or TX channel.
    ///
    /// Some API functions will fail when performed on disabled channel.
    pub fn enable_channel(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        enabled: bool,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_EnableChannel,
            self.get_handle(),
            *channel_dir,
            channel,
            enabled
        ))
        .into_result()
    }

    /// Set sampling `rate` in Hz for all RX/TX channels.
    /// Sample rate is in complex samples (1 sample = I + Q). The function
    /// sets sampling rate that is used for data exchange with the host.
    /// It also allows to specify higher sampling rate to be used in RF
    /// by setting oversampling ratio. Valid oversampling values
    /// are 1, 2, 4, 8, 16, 32 or 0 (use device default oversampling value).
    pub fn set_sample_rate(
        &self,
        rate: f64,
        oversample: usize,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetSampleRate,
            self.get_handle(),
            rate,
            oversample
        ))
        .into_result()
    }

    /// Obtain the sampling rate of the specified RX or TX channel.
    ///
    /// The function obtains the sample rate used in data interface
    /// with the host and the RF sample rate used by DAC/ADC.
    #[allow(non_snake_case)]
    pub fn get_sample_rate(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<SampleRate, Error> {
        let mut host_Hz = f64::default();
        let mut rf_Hz = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetSampleRate,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut host_Hz as *mut float_type,
            &mut rf_Hz as *mut float_type
        ))
        .into_result()?;

        Ok(SampleRate { host_Hz, rf_Hz })
    }

    /// Obtain the range of supported sampling rates.
    pub fn get_sample_rate_range(
        &self,
        channel_dir: ChannelDirection,
    ) -> Result<Range, Error> {
        let mut crange = lms_range_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetSampleRateRange,
            self.get_handle(),
            *channel_dir,
            &mut crange as *mut lms_range_t
        ))
        .into_result()?;

        Ok(Range {
            min: crange.min,
            max: crange.max,
            step: crange.step,
        })
    }

    /// Set RF center frequency in Hz.
    ///
    /// # Note
    ///
    /// Channels A and B in LMS7 chip share the same clock so ability to set
    /// different frequencies for channels A and B is very limited. This
    /// function will attempt to achieve different requested frequencies
    /// using NCO when possible, however often changing frequency for
    /// one (A or B) channel will result in frequency being changed for
    /// both (A and B) channels.
    pub fn set_lo_frequency(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        frequency: f64,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetLOFrequency,
            self.get_handle(),
            *channel_dir,
            channel,
            frequency
        ))
        .into_result()
    }

    /// Obtain the current RF center frequency in Hz.
    pub fn get_lo_frequency(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<f64, Error> {
        let mut frequency = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetLOFrequency,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut frequency as *mut float_type
        ))
        .into_result()?;

        Ok(frequency)
    }

    /// Obtain the supported RF center frequency range in Hz.
    pub fn get_lo_frequency_range(
        &self,
        channel_dir: ChannelDirection,
    ) -> Result<Range, Error> {
        let mut crange = lms_range_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetLOFrequencyRange,
            self.get_handle(),
            *channel_dir,
            &mut crange as *mut lms_range_t
        ))
        .into_result()?;

        Ok(Range {
            min: crange.min,
            max: crange.max,
            step: crange.step,
        })
    }

    /// Obtain antenna list with names.
    ///
    /// First item in the list is the name of antenna index 0.
    pub fn get_antenna_list(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<Vec<String>, Error> {
        let num_antenna = LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetAntennaList,
            self.get_handle(),
            *channel_dir,
            channel,
            std::ptr::null_mut() as *mut lms_name_t
        ))
        .into_result()? as usize;

        let mut list: Vec<lms_name_t> =
            vec![[0; std::mem::size_of::<lms_name_t>()]; num_antenna];

        LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetAntennaList,
            self.get_handle(),
            *channel_dir,
            channel,
            list.as_mut_ptr() as *mut lms_name_t
        ))
        .into_result()?;

        let mut antenna_list: Vec<String> = Vec::new();
        for antenna in list.iter() {
            unsafe {
                antenna_list.push(
                    CStr::from_ptr(antenna.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            }
        }

        Ok(antenna_list)
    }

    /// Select the antenna for the specified RX or TX channel.
    pub fn set_antenna(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        port: RfPort,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetAntenna,
            self.get_handle(),
            *channel_dir,
            channel,
            *port
        ))
        .into_result()
    }

    /// Obtain currently selected antenna of the specified RX or TX channel.
    pub fn get_antenna(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<RfPort, Error> {
        let ret = LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetAntenna,
            self.get_handle(),
            *channel_dir,
            channel
        ))
        .into_result()? as usize;

        let ch_dir = match ret {
            LMS_PATH_NONE | LMS_PATH_AUTO => None,
            _ => Some(channel_dir),
        };

        Ok(RfPort { inner: ret, ch_dir })
    }

    /// Obtains recommended bandwidth (lower and upper frequency) for the
    /// specified antenna `port`. The ports can be used outside this range.
    pub fn get_antenna_bw(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        port: RfPort,
    ) -> Result<Range, Error> {
        let mut crange = lms_range_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetAntennaBW,
            self.get_handle(),
            *channel_dir,
            channel,
            *port,
            &mut crange as *mut lms_range_t
        ))
        .into_result()?;

        Ok(Range {
            min: crange.min,
            max: crange.max,
            step: crange.step,
        })
    }

    /// Set the combined gain value.
    ///
    /// This function computes and sets the optimal gain values of various
    /// amplifiers that are present in the device based on desired
    /// normalized `gain` from the range \[0, 1.0\], where 1.0 represents the
    /// maximum gain.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values are
    /// changed.
    ///
    /// # Attention
    ///
    /// Gain functionality will be changed in the future. IAMP
    /// and TIA gain elements won't configured via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    pub fn set_normalized_gain(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        gain: f64,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetNormalizedGain,
            self.get_handle(),
            *channel_dir,
            channel,
            gain
        ))
        .into_result()
    }

    /// Set the combined gain value in dB.
    ///
    /// This function computes and sets the optimal gain values of various
    /// amplifiers that are present in the device based on desired `gain`
    /// value in dB from range \[0, 73\].
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values
    /// are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality and range will be changed in the future. IAMP
    /// and TIA gain elements won't configured via this function. To enable
    /// new behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    pub fn set_gain_db(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        gain: u32,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetGaindB,
            self.get_handle(),
            *channel_dir,
            channel,
            gain
        ))
        .into_result()
    }

    /// Obtain the current combined gain value from range \[0, 1.0\], where
    /// 1.0 represents the maximum gain..
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values
    /// are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality will be changed in the future. IAMP and TIA gain
    /// element values won't be obtained via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    pub fn get_normalized_gain(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<f64, Error> {
        let mut gain = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetNormalizedGain,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut gain as *mut float_type
        ))
        .into_result()?;

        Ok(gain)
    }

    /// Obtain the current combined gain value in dB.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values
    /// are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality and range will be changed in the future. IAMP and
    /// TIA gain element values won't be obtained via this function. To enable
    /// new behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    pub fn get_gain_db(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<u32, Error> {
        let mut gain = u32::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetGaindB,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut gain as *mut core::ffi::c_uint
        ))
        .into_result()?;

        Ok(gain)
    }

    /// Configure analog LPF of the LMS chip for the desired RF
    /// `bandwidth` in Hz. This function automatically enables LPF.
    pub fn set_lpf_bw(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        bandwidth: f64,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetLPFBW,
            self.get_handle(),
            *channel_dir,
            channel,
            bandwidth
        ))
        .into_result()
    }

    /// Get the currently configured analog LPF RF bandwidth in HZ.
    ///
    /// # Note
    ///
    /// Readback from board is currently not supported, only returns last set
    /// value cached by software.
    pub fn get_lpf_bw(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<f64, Error> {
        let mut bandwidth = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetLPFBW,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut bandwidth as *mut float_type
        ))
        .into_result()?;

        Ok(bandwidth)
    }

    /// Obtain the RF bandwidth setting range in Hz supported by the
    /// analog LPF of LMS chip.
    pub fn get_lpf_bw_range(
        &self,
        channel_dir: ChannelDirection,
    ) -> Result<Range, Error> {
        let mut crange = lms_range_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetLPFBWRange,
            self.get_handle(),
            *channel_dir,
            &mut crange as *mut lms_range_t
        ))
        .into_result()?;

        Ok(Range {
            min: crange.min,
            max: crange.max,
            step: crange.step,
        })
    }

    /// Disables or enables the analog LPF of LMS chip
    /// without reconfiguring it.
    pub fn set_lpf(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        enable: bool,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetLPF,
            self.get_handle(),
            *channel_dir,
            channel,
            enable
        ))
        .into_result()
    }

    /// Set up digital LPF using LMS chip GFIRS.
    ///
    /// This is a convenience function to quickly configure GFIRS as LPF with
    /// desired RF `bandwidth` in Hz. Has no effect if enabled is false.
    ///
    /// # Pre
    ///
    /// Sampling rate must be set
    pub fn set_gfir_lpf(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        enable: bool,
        bandwidth: f64,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetGFIRLPF,
            self.get_handle(),
            *channel_dir,
            channel,
            enable,
            bandwidth
        ))
        .into_result()
    }

    /// Perform the automatic calibration of specified RX/TX channel
    /// with `bandwidth` and additional calibration `flags`
    /// (normally should be 0).
    ///
    /// The automatic calibration must be run after device configuration
    /// is finished because calibration values are dependant on various
    /// configuration settings.
    pub fn calibrate(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        bandwidth: f64,
        flags: u32,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_Calibrate,
            self.get_handle(),
            *channel_dir,
            channel,
            bandwidth,
            flags
        ))
        .into_result()
    }

    /// Load LMS chip configuration from a file.
    ///
    /// This only loads LMS chip configuration, in oder for streaming to work
    /// properly FPGA has also to be configured.
    /// Use [`set_sample_rate`](crate::device::Device::set_sample_rate) to
    /// configure LMS and FPGA for streaming.
    pub fn load_config(&self, filename: &str) -> Result<(), Error> {
        let filename =
            CString::new(filename).map_err(|_| Error::InvalidValue)?;

        LmsRetVal::from(invoke_lms!(
            LMS_LoadConfig,
            self.get_handle(),
            filename.as_c_str().as_ptr() as *const core::ffi::c_char
        ))
        .into_result()
    }

    /// Save LMS chip configuration to a file.
    pub fn save_config(&self, filename: &str) -> Result<(), Error> {
        let filename =
            CString::new(filename).map_err(|_| Error::InvalidValue)?;

        LmsRetVal::from(invoke_lms!(
            LMS_SaveConfig,
            self.get_handle(),
            filename.as_c_str().as_ptr() as *const core::ffi::c_char
        ))
        .into_result()
    }

    /// Apply the specified test signal.
    pub fn set_test_signal(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        signal: TestSignal,
        dc_i: Option<i16>,
        dc_q: Option<i16>,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetTestSignal,
            self.get_handle(),
            *channel_dir,
            channel,
            signal.into(),
            dc_i.unwrap_or_default(),
            dc_q.unwrap_or_default()
        ))
        .into_result()
    }

    /// Obtain the currently active test signal.
    pub fn get_test_signal(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<TestSignal, Error> {
        let mut signal = lms_testsig_t::LMS_TESTSIG_NONE;

        LmsRetVal::from(invoke_lms!(
            LMS_GetTestSignal,
            self.get_handle(),
            *channel_dir,
            channel,
            &mut signal as *mut lms_testsig_t
        ))
        .into_result()?;

        Ok(TestSignal::from(signal))
    }

    /// Read internal temperature sensor of a LMS7 chip
    /// selected by the specified `index`.
    pub fn get_chip_temperature(&self, index: u64) -> Result<f64, Error> {
        let mut temp = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetChipTemperature,
            self.get_handle(),
            index,
            &mut temp as *mut float_type
        ))
        .into_result()?;

        Ok(temp)
    }
}

// Advanced control functions.
//
// The functions in this section provides some additional control compared to
// High-Level functions. They are labeled advanced because they require better
// understanding of hardware and provide functionality that may conflict with
// other High-Level functions.

/// Enumeration of LMS7 GFIRS.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gfir {
    GFIR1 = lms_gfir_t::LMS_GFIR1 as u32,
    GFIR2,
    GFIR3,
}

impl From<lms_gfir_t> for Gfir {
    fn from(gfir: lms_gfir_t) -> Self {
        match gfir {
            lms_gfir_t::LMS_GFIR1 => Self::GFIR1,
            lms_gfir_t::LMS_GFIR2 => Self::GFIR2,
            lms_gfir_t::LMS_GFIR3 => Self::GFIR3,
        }
    }
}

impl From<Gfir> for lms_gfir_t {
    fn from(gfir: Gfir) -> Self {
        match gfir {
            Gfir::GFIR1 => lms_gfir_t::LMS_GFIR1,
            Gfir::GFIR2 => lms_gfir_t::LMS_GFIR2,
            Gfir::GFIR3 => lms_gfir_t::LMS_GFIR3,
        }
    }
}

impl Device {
    /// Number of NCO frequency/phase offset values.
    pub const NCO_VAL_COUNT: usize = LMS_NCO_VAL_COUNT as usize;

    /// Set sampling `rate` in Hz for all RX or TX channels. Sample rate is
    /// in complex samples (1 sample = I + Q). The function sets sampling rate
    /// that is used for data exchange with the host. It also allows to
    /// specify higher sampling rate to be used in RF by setting oversampling
    /// ratio. Valid oversampling values are 1, 2, 4, 8, 16, 32 or 0 (use
    /// device default oversampling value).
    ///
    /// # Note
    ///
    /// RX and TX rates sampling are closely tied in LMS7 chip. Changing RX or
    /// TX will often result in change of both (RX and TX). RX/TX ratio can
    /// only be power of 2 and is also limited by other factors. Use
    /// [`get_sample_rate`](crate::device::Device::get_sample_rate) to
    /// obtain actual sample rate values. The function returns success if
    /// it is able to achieve  desired sample rate and oversampling for
    /// the specified direction (RX or TX) ignoring possible value changes
    /// in other direction channels.
    pub fn set_sample_rate_dir(
        &self,
        channel_dir: ChannelDirection,
        rate: f64,
        oversample: usize,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetSampleRateDir,
            self.get_handle(),
            *channel_dir,
            rate,
            oversample
        ))
        .into_result()
    }

    /// Configure NCO to operate in FCW mode. Configures NCO with up to 16
    /// frequencies that can be quickly switched between.
    /// Automatically starts NCO with frequency at index 0
    /// Use [`set_nco_index`](crate::device::Device::set_nco_index)
    /// to switch between NCO frequencies.
    ///
    /// `freq` values cannot be negative and must be at least
    /// [`NCO_VAL_COUNT`](crate::device::Device::NCO_VAL_COUNT) length.
    /// `pho` is NCO phase offset in deg.
    pub fn set_nco_frequency(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        freq: Vec<f64>,
        pho: f64,
    ) -> Result<(), Error> {
        if freq.len() < Device::NCO_VAL_COUNT {
            return Err(Error::InvalidInput);
        }

        LmsRetVal::from(invoke_lms!(
            LMS_SetNCOFrequency,
            self.get_handle(),
            *channel_dir,
            channel,
            freq.as_ptr() as *const float_type,
            pho
        ))
        .into_result()
    }

    /// Get the current NCO FCW mode configuration.
    ///
    /// Return list of NCO frequencies and phase offset in deg.
    pub fn get_nco_frequency(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<(Vec<f64>, f64), Error> {
        let mut freq: Vec<float_type> = vec![0.0; Device::NCO_VAL_COUNT];
        let mut pho = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetNCOFrequency,
            self.get_handle(),
            *channel_dir,
            channel,
            freq.as_mut_ptr() as *mut float_type,
            &mut pho as *mut float_type
        ))
        .into_result()?;

        Ok((freq, pho))
    }

    /// Configure NCO to operate in PHO mode. Configures NCO with up to 16
    /// phase offsets that can be quickly switched between.
    /// Automatically starts NCO with phase at index 0
    /// Use [`set_nco_index`](crate::device::Device::set_nco_index) to
    /// switch between NCO phases.
    ///
    /// `phases` values cannot be negative. Must be at least
    /// [`NCO_VAL_COUNT`](crate::device::Device::NCO_VAL_COUNT) length.
    /// `fcw` is NCO frequency in Hz.
    pub fn set_nco_phase(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        phases: Vec<f64>,
        fcw: f64,
    ) -> Result<(), Error> {
        if phases.len() < Device::NCO_VAL_COUNT {
            return Err(Error::InvalidInput);
        }

        LmsRetVal::from(invoke_lms!(
            LMS_SetNCOPhase,
            self.get_handle(),
            *channel_dir,
            channel,
            phases.as_ptr() as *const float_type,
            fcw
        ))
        .into_result()
    }

    /// Get the current NCO PHO mode configuration.
    ///
    /// Return list of configured NCO phases and current NCO frequency.
    pub fn get_nco_phase(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<(Vec<f64>, f64), Error> {
        let mut phases: Vec<float_type> = vec![0.0; Device::NCO_VAL_COUNT];
        let mut fcw = f64::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetNCOPhase,
            self.get_handle(),
            *channel_dir,
            channel,
            phases.as_mut_ptr() as *mut float_type,
            &mut fcw as *mut float_type
        ))
        .into_result()?;

        Ok((phases, fcw))
    }

    /// Switches between configured list of NCO frequencies/phase offsets.
    /// Also allows to switch CMIX mode with `downconv` to either downconvert
    /// or upconvert
    ///
    /// `index` is NCO frequency/phase index to activate or (-1) to
    /// disable NCO.
    pub fn set_nco_index(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        index: i32,
        downconv: bool,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetNCOIndex,
            self.get_handle(),
            *channel_dir,
            channel,
            index,
            downconv
        ))
        .into_result()
    }

    /// Get the currently active NCO frequency/phase offset index.
    pub fn get_nco_index(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
    ) -> Result<i32, Error> {
        LmsRetValNonStandard::from(invoke_lms!(
            LMS_GetNCOIndex,
            self.get_handle(),
            *channel_dir,
            channel
        ))
        .into_result()
    }

    /// Configure LMS GFIR using specified filter coefficients. Maximum
    /// number of coefficients is 40 for GFIR1 and GFIR2, and 120 for GFIR3.
    /// Coefficients range is \[-1.0, 1.0\].
    pub fn set_gfir_coeff(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        filt: Gfir,
        coef: Vec<f64>,
        count: usize,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetGFIRCoeff,
            self.get_handle(),
            *channel_dir,
            channel,
            filt.into(),
            coef.as_ptr() as *const float_type,
            count
        ))
        .into_result()
    }

    /// Get currently set GFIR coefficients.
    pub fn get_gfir_coeff(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        filt: Gfir,
    ) -> Result<Vec<f64>, Error> {
        let array_len = match filt {
            Gfir::GFIR1 | Gfir::GFIR2 => 40,
            Gfir::GFIR3 => 120,
        };
        let mut coef: Vec<float_type> = vec![0.0; array_len as usize];

        LmsRetVal::from(invoke_lms!(
            LMS_GetGFIRCoeff,
            self.get_handle(),
            *channel_dir,
            channel,
            filt.into(),
            coef.as_mut_ptr() as *mut float_type
        ))
        .into_result()?;

        Ok(coef)
    }

    /// Enables or disables specified GFIR.
    pub fn set_gfir(
        &self,
        channel_dir: ChannelDirection,
        channel: usize,
        filt: Gfir,
        enabled: bool,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_SetGFIR,
            self.get_handle(),
            *channel_dir,
            channel,
            filt.into(),
            enabled
        ))
        .into_result()
    }

    /// Enables or disable caching of LMS7 and FPGA register values.
    pub fn enable_cache(&self, enabled: bool) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_EnableCache,
            self.get_handle(),
            enabled
        ))
        .into_result()
    }
}
