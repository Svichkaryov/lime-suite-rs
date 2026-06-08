#![allow(non_camel_case_types, non_snake_case)]
// Documenting in this crate don't follow [RFC 1574]
//
// [RFC 1574]: https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md
#![allow(clippy::four_forward_slashes)]

use super::params::*;


macro_rules! lime_suite_extern {
    ($decl:item) => {
        #[link(name = "LimeSuite")]
        unsafe extern "C" {
            $decl
        }
    }
}

/// C size_t type
pub type c_size_t = usize;

/// Floating point data type
pub type float_type = core::ffi::c_double;

/// Convenience constant for good return code
pub const LMS_SUCCESS: core::ffi::c_int = 0;


//// Initialization/deinitialization
////
//// The functions in this section provide the ability to query available devices,
//// initialize them, and deinitialize them.


/// LMS Device handle
pub type lms_device_t = core::ffi::c_void;

/// Convenience type for fixed length LMS Device information string
pub type lms_info_str_t = [core::ffi::c_char; 256];

lime_suite_extern! {
    /// Obtain a list of LMS devices attached to the system.
    ///
    /// # Parameters
    ///
    /// * `dev_list`: List of available devices.
    ///
    /// # Return
    ///
    /// Number of devices in the list on success, (-1) on failure.
    pub fn LMS_GetDeviceList(
        dev_list: *mut lms_info_str_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Opens device specified by the provided ::lms_info_str_t string
    /// This function should be used to open a device based upon the results of
    /// LMS_GetDeviceList().
    ///
    /// # Precondition
    ///
    /// Device should be initialized to NULL
    ///
    /// # Parameters
    ///
    /// * [out] `device`: Updated with device handle on success.
    /// * [in]  `info`:   Device information string. If NULL, the first
    ///                   available device will be opened.
    /// * [in]  `args`:   additional arguments. Can be NULL.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Open(
        device: *mut *mut lms_device_t,
        info: *const lms_info_str_t,
        args: *mut core::ffi::c_void
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Close device.
    ///
    /// # Postcondition
    ///
    /// Device is deallocated and may no longer be used.
    ///
    /// # Parameters
    ///
    /// * [in] `device`: Device handle previously obtained by LMS_Open().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Close(
        device: *mut lms_device_t
    ) -> core::ffi::c_int;
}


//// High-level control functions
////
//// The functions in this section provide the ability to easily configure the
//// device for operation. They modify multiple internal device settings.


/// Convenience constants for TX selection
pub const LMS_CH_TX: bool = true;
/// Convenience constants for RX selection
pub const LMS_CH_RX: bool = false;

/// Convenience type for fixed length name string
pub type lms_name_t = [core::ffi::c_char; 16];

/// Structure used to represent the allowed value range of various parameters
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lms_range_t {
    /// Minimum allowed value
    pub min: float_type,
    /// Maximum allowed value
    pub max: float_type,
    /// Minimum value step
    pub step: float_type,
}

/// Enumeration of LMS7 TEST signal types
#[repr(C)]
pub enum lms_testsig_t {
    /// Disable test signals. Return to normal operation
    LMS_TESTSIG_NONE = 0,
    /// Test signal from NCO half scale
    LMS_TESTSIG_NCODIV8,
    /// Test signal from NCO half scale
    LMS_TESTSIG_NCODIV4,
    /// Test signal from NCO full scale
    LMS_TESTSIG_NCODIV8F,
    /// Test signal from NCO full scale
    LMS_TESTSIG_NCODIV4F,
    /// DC test signal
    LMS_TESTSIG_DC,
}

lime_suite_extern! {
    /// Configure LMS chip with settings that make it ready for operation.
    ///
    /// # Notes
    ///
    /// This configuration differs from default LMS chip configuration which is
    /// described in chip datasheet. In order to load default chip configuration use
    /// LMS_Reset().
    ///
    /// # Parameters
    ///
    /// * [in] `device`: Device handle previously obtained by LMS_Open().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Init(
        device: *mut lms_device_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain number of RX or TX channels. Use this to determine the maximum
    /// channel index (specifying channel index is required by most API functions).
    /// The maximum channel index is N-1, where N is number returned by this function.
    ///
    /// # Parameters
    ///
    /// * [in] `device`: Device handle previously obtained by LMS_Open().
    /// * [in] `dir_tx`: Select RX or TX.
    ///
    /// # Return
    ///
    /// Number of channels on success, (-1) on failure.
    pub fn LMS_GetNumChannels(
        device: *mut lms_device_t,
        dir_tx: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Enable or disable specified RX or TX channel. Some API functions will fail
    /// when performed on disabled channel.
    ///
    /// # Parameters
    ///
    /// * [in] `device`:  Device handle previously obtained by LMS_Open().
    /// *      `dir_tx`:  Select RX or TX.
    /// *      `chan`:    Channel index.
    /// *      `enabled`: true(1) to enable, false(0) to disable.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_EnableChannel(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        enabled: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set sampling rate for all RX/TX channels. Sample rate is in complex samples
    /// (1 sample = I + Q). The function sets sampling rate that is used for data
    /// exchange with the host. It also allows to specify higher sampling rate to be
    /// used in RF by setting oversampling ratio. Valid oversampling values are 1, 2,
    /// 4, 8, 16, 32 or 0 (use device default oversampling value).
    ///
    /// # Parameters
    ///
    /// * [in] `device`:     Device handle previously obtained by LMS_Open().
    /// *      `rate`:       sampling rate in Hz to set.
    /// *      `oversample`: RF oversampling ratio.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetSampleRate(
        device: *mut lms_device_t,
        rate: float_type,
        oversample: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the sampling rate of the specified RX or TX channel.
    /// The function obtains the sample rate used in data interface with the host and
    /// the RF sample rate used by DAC/ADC.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:  Select RX or TX.
    /// *       `chan`:    Channel index.
    /// * [out] `host_Hz`: sampling rate used for data exchange with the host.
    /// * [out] `rf_Hz`:   RF sampling rate in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetSampleRate(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        host_Hz: *mut float_type,
        rf_Hz: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the range of supported sampling rates.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// * [in]  `range`:  Allowed sample rate range in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetSampleRateRange(
        device: *mut lms_device_t,
        dir_tx: bool,
        range: *mut lms_range_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set RF center frequency in Hz.
    ///
    /// # Note
    ///
    /// Channels A and B in LMS7 chip share the same clock so ability to set
    /// different frequencies for channels A and B is very limited. This function
    /// will attempt to achieve different requested frequencies using NCO when
    /// possible, however often changing frequency for one (A or B) channel will
    /// result in frequency being changed for both (A and B) channels.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:    Select RX or TX.
    /// *       `chan`:      Channel index.
    /// *       `frequency`: Desired RF center frequency in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetLOFrequency(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        frequency: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain the current RF center frequency in Hz.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:    Select RX or TX.
    /// *       `chan`:      Channel index.
    /// * [out] `frequency`: Current RF center frequency in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetLOFrequency(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        frequency: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain the supported RF center frequency range in Hz.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// * [out] `range`:  Supported RF center frequency in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetLOFrequencyRange(
        device: *mut lms_device_t,
        dir_tx: bool,
        range: *mut lms_range_t
    ) -> core::ffi::c_int;
}


// Enumeration of RF ports
/// No active path (RX or TX)
pub const LMS_PATH_NONE: c_size_t = 0;
/// RX LNA_H port
pub const LMS_PATH_LNAH: c_size_t = 1;
/// RX LNA_L port
pub const LMS_PATH_LNAL: c_size_t = 2;
/// RX LNA_W port
pub const LMS_PATH_LNAW: c_size_t = 3;
/// TX port 1
pub const LMS_PATH_TX1: c_size_t = 1;
/// TX port 2
pub const LMS_PATH_TX2: c_size_t = 2;
/// Automatically select port (if supported)
pub const LMS_PATH_AUTO: c_size_t = 255;


lime_suite_extern! {
    /// Obtain antenna list with names. First item in the list is the name of antenna
    /// index 0.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// * [out] `list`:   List of antenna names (can be NULL).
    ///
    /// # Return
    ///
    /// Number of available antennae, (-1) on failure.
    pub fn LMS_GetAntennaList(
        dev: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        list: *mut lms_name_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Select the antenna for the specified RX or TX channel.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// *       `index`:  Index of antenna to select.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetAntenna(
        dev: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        index: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain currently selected antenna of the the specified RX or TX channel.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    ///
    /// # Return
    ///
    /// Index of selected antenna on success, (-1) on failure.
    pub fn LMS_GetAntenna(
        dev: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtains recommended bandwidth (lower and upper frequency) for the specified
    /// antenna port. The ports can be used outside this range.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// *       `index`:  Antenna index.
    /// * [out] `range`:  Antenna bandwidth.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetAntennaBW(
        dev: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        index: c_size_t,
        range: *mut lms_range_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set the combined gain value.
    /// This function computes and sets the optimal gain values of various amplifiers
    /// that are present in the device based on desired normalized gain value.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality will be changed in the future. IAMP
    /// and TIA gain elements won't configured via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// *       `gain`:   Desired gain, range [0, 1.0], where 1.0 represents the
    ///                   maximum gain.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetNormalizedGain(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        gain: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set the combined gain value in dB.
    /// This function computes and sets the optimal gain values of various amplifiers
    /// that are present in the device based on desired  gain value in dB.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality and range will be changed in the future. IAMP
    /// and TIA gain elements won't configured via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// *       `gain`:   Desired gain, range [0, 73].
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetGaindB(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        gain: core::ffi::c_uint
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain the current combined gain value.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality will be changed in the future. IAMP and TIA gain
    /// element values won't be obtained via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// * [out] `gain`:   Current gain, range [0, 1.0], where 1.0 represents
    ///                   the maximum gain.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetNormalizedGain(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        gain: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Obtain the current combined gain value in dB.
    ///
    /// # Note
    ///
    /// Actual gain depends on LO frequency and analog LPF configuration and
    /// resulting output signal level may be different when those values are changed.
    ///
    /// # Attention
    ///
    /// Gain functionality and range will be changed in the future. IAMP
    /// and TIA gain element values won't be obtained via this function. To enable new
    /// behaviour, turn on ENABLE_NEW_GAIN_BEHAVIOUR CMake option.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   channel index.
    /// * [out] `gain`:   Current gain.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetGaindB(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        gain: *mut core::ffi::c_uint
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Configure analog LPF of the LMS chip for the desired RF bandwidth.
    /// This function automatically enables LPF.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:    Select RX or TX.
    /// *       `chan`:      channel index.
    /// *       `bandwidth`: LPF bandwidth in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetLPFBW(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        bandwidth: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the currently configured analog LPF RF bandwidth.
    ///
    /// # Note
    ///
    /// Readback from board is currently not supported, only returns last set
    /// value cached by software.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:    Select RX or TX.
    /// *       `chan`:      channel index.
    /// * [out] `bandwidth`: Current LPF bandwidth in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetLPFBW(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        bandwidth: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the RF bandwidth setting range supported by the analog LPF of LMS chip.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// * [out] `range`:  Supported RF bandwidth range in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetLPFBWRange(
        device: *mut lms_device_t,
        dir_tx: bool,
        range: *mut lms_range_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Disables or enables the analog LPF of LMS chip without reconfiguring it.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// *       `enable`: true(1) to enable, false(0) to disable.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetLPF(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        enable: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set up digital LPF using LMS chip GFIRS. This is a convenience function to
    /// quickly configure GFIRS as LPF with desired RF bandwidth.
    ///
    /// # Pre
    ///
    /// Sampling rate must be set
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:    Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:    Select RX or TX.
    /// *       `chan`:      Channel index.
    /// *       `enable`:    Disable (false) or enable (true) GFIRS.
    /// *       `bandwidth`: LPF bandwidth in Hz. Has no effect if enabled is false.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetGFIRLPF(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        enable: bool,
        bandwidth: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Perform the automatic calibration of specified RX/TX channel. The automatic
    /// calibration must be run after device configuration is finished because
    /// calibration values are dependant on various configuration settings.
    ///
    /// # Pre
    ///
    /// Device should be configured
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// *       `bw`:     bandwidth.
    /// *       `flags`:  additional calibration flags (normally should be 0).
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Calibrate(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        bw: core::ffi::c_double,
        flags: core::ffi::c_uint
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Load LMS chip configuration from a file.
    ///
    /// # Note
    ///
    /// This only loads LMS chip configuration, in oder for streaming to work
    /// properly FPGA has also to be configured. Use LMS_SetSampleRate() to configure
    /// LMS and FPGA for streaming.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:   Device handle.
    /// *       `filename`: path to file.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_LoadConfig(
        device: *mut lms_device_t,
        filename: *const core::ffi::c_char
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Save LMS chip configuration to a file.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:   Device handle.
    /// *       `filename`: path to file with LMS chip configuration.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SaveConfig(
        device: *mut lms_device_t,
        filename: *const core::ffi::c_char
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Apply the specified test signal.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// *       `sig`:    Test signal. LMS_TESTSIG_NONE disables test signal.
    /// *       `dc_i`:   DC I value for LMS_TESTSIG_DC mode. Ignored in other modes.
    /// *       `dc_q`:   DC Q value for LMS_TESTSIG_DC mode. Ignored in other modes.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetTestSignal(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        sig: lms_testsig_t,
        dc_i: i16,
        dc_q: i16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the currently active test signal.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// * [out] `sig`:    Currently active test signal.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetTestSignal(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        sig: *mut lms_testsig_t
    ) -> core::ffi::c_int;
}


lime_suite_extern! {
    /// Read LMS7 chip internal temperature sensor.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:  Device handle previously obtained by LMS_Open().
    /// *       `ind`:  chip index.
    /// * [out] `temp`: temperature.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetChipTemperature(
        dev: *mut lms_device_t,
        ind: core::ffi::c_ulonglong,
        temp: *mut float_type
    ) -> core::ffi::c_int;
}


// Advanced control functions.
//
// The functions in this section provides some additional control compared to
// High-Level functions. They are labeled advanced because they require better
// understanding of hardware and provide functionality that may conflict with
// other High-Level functions.


/// Enumeration of LMS7 GFIRS
#[repr(C)]
pub enum lms_gfir_t {
    LMS_GFIR1 = 0,
    LMS_GFIR2,
    LMS_GFIR3,
}

/// Number of NCO frequency/phase offset values
pub const LMS_NCO_VAL_COUNT: core::ffi::c_int = 16;

lime_suite_extern! {
    /// Set sampling rate for all RX or TX channels. Sample rate is in complex
    /// samples (1 sample = I + Q). The function sets sampling rate that is used for
    /// data exchange with the host. It also allows to specify higher sampling rate
    /// to be used in RF by setting oversampling ratio. Valid oversampling values are
    /// 1, 2, 4, 8, 16, 32 or 0 (use device default oversampling value).
    ///
    /// # Note
    ///
    /// RX and TX rates sampling are closely tied in LMS7 chip. Changing RX or
    /// TX will often result in change of both (RX and TX). RX/TX ratio can only be
    /// power of 2 and is also limited by other factors. Use LMS_GetSampleRate() to
    /// obtain actual sample rate values. The function returns success if it is able
    /// to achieve  desired sample rate and oversampling for the specified direction
    /// (RX or TX) ignoring possible value changes in other direction channels.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:     Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:     Select RX or TX.
    /// *       `rate`:       Sampling rate in Hz to set.
    /// *       `oversample`: RF oversampling ratio.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetSampleRateDir(
        device: *mut lms_device_t,
        dir_tx: bool,
        rate: float_type,
        oversample: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Configure NCO to operate in FCW mode. Configures NCO with up to 16
    /// frequencies that can be quickly switched between.
    /// Automatically starts NCO with frequency at index 0
    /// Use LMS_SetNCOindex() to switch between NCO frequencies.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// * [in]  `freq`:   List of NCO frequencies. Values cannot be negative.
    ///                   Must be at least ::LMS_NCO_VAL_COUNT length;
    /// *       `pho`:    NCO phase offset in deg.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetNCOFrequency(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        freq: *const float_type,
        pho: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the current NCO FCW mode configuration.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// * [out] `freq`:   List of NCO frequencies. Must be at least
    ///                   ::LMS_NCO_VAL_COUNT length;
    /// * [out] `pho`:    Phase offset in deg.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetNCOFrequency(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        freq: *mut float_type,
        pho: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Configure NCO to operate in PHO mode. Configures NCO with up to 16
    /// phase offsets that can be quickly switched between.
    /// Automatically starts NCO with phase at index 0
    /// Use LMS_SetNCOindex() to switch between NCO phases.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// * [in]  `phases`: List of NCO phases. Values cannot be negative.
    ///                   Must be at least ::LMS_NCO_VAL_COUNT length;
    /// *       `fcw`:    NCO frequency in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetNCOPhase(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        phases: *const float_type,
        fcw: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the current NCO PHO mode configuration.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// * [out] `phases`: List of configured NCO phases.
    ///                   Must be at least ::LMS_NCO_VAL_COUNT length;
    /// * [out] `fcw`:    Current NCO frequency.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetNCOPhase(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        phases: *mut float_type,
        fcw: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Switches between configured list of NCO frequencies/phase offsets. Also
    /// Allows to switch CMIX mode to either downconvert or upconvert.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:   Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:   Select RX or TX.
    /// *       `chan`:     Channel index.
    /// *       `index`:    NCO frequency/phase index to activate or (-1) to disable NCO.
    /// *       `downconv`: true(1) CMIX downconvert, false(0) CMIX upconvert.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetNCOIndex(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        index: core::ffi::c_int,
        downconv: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get the currently active NCO frequency/phase offset index.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    ///
    /// # Return
    ///
    /// Current NCO frequency/phase index on success, (-1) on failure.
    pub fn LMS_GetNCOIndex(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Configure LMS GFIR using specified filter coefficients. Maximum number of
    /// coefficients is 40 for GFIR1 and GFIR2, and 120 for GFIR3.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// *       `filt`:   GFIR to configure.
    /// * [in]  `coef`:   Array of filter coefficients. Coeff range [-1.0, 1.0].
    /// *       `count`:  number of filter coefficients.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetGFIRCoeff(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        filt: lms_gfir_t,
        coef: *const float_type,
        count: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get currently set GFIR coefficients.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`: Select RX or TX.
    /// *       `chan`:   Channel index.
    /// *       `filt`:   GFIR to configure.
    /// * [out] `coef`:   Current GFIR coefficients. Array must be big enough to
    ///                   hold 40 (GFIR1, GFIR2) or 120 (GFIR3) values.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetGFIRCoeff(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        filt: lms_gfir_t,
        coef: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Enables or disables specified GFIR.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `dir_tx`:  Select RX or TX.
    /// *       `chan`:    Channel index.
    /// *       `filt`:    GFIR to configure.
    /// *       `enabled`: true(1) enable, false(0) disable.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetGFIR(
        device: *mut lms_device_t,
        dir_tx: bool,
        chan: c_size_t,
        filt: lms_gfir_t,
        enabled: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Enables or disable caching of LMS7 and FPGA register values.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:     Device handle previously obtained by LMS_Open().
    /// *       `enabled`: true to enable cache.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_EnableCache(
        dev: *mut lms_device_t,
        enabled: bool
    ) -> core::ffi::c_int;
}


//// Low-Level control functions
////
//// The functions in this section provide a low access to device such as modifying
//// device internal register or clock frequency. Low-Level functions can be used
//// to configure device entirely, however a more practical use is to fine-tune
//// device settings after configuring it with /ref FN_HIGH_LVL.

lime_suite_extern! {
    /// Send Reset signal to LMS chip. This initializes LMS chip with default
    /// configuration as described in LMS chip datasheet.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Reset(
        device: *mut lms_device_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Read device LMS chip register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `address`: Register address.
    /// * [out] `val`:     Current register value.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_ReadLMSReg(
        device: *mut lms_device_t,
        address: u32,
        val: *mut u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write device LMS chip register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `address`: Register address.
    /// *       `val`:     Value to write.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_WriteLMSReg(
        device: *mut lms_device_t,
        address: u32,
        val: u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Read device parameter. Parameter defines specific bits in device register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `param`:  Parameter.
    /// * [out] `val`:    Current parameter value.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_ReadParam(
        device: *mut lms_device_t,
        param: LMS7Parameter,
        val: *mut u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write device parameter. Parameter defines specific bits in device register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `param`:  Parameter.
    /// *       `val`:    Parameter value to write.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_WriteParam(
        device: *mut lms_device_t,
        param: LMS7Parameter,
        val: u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Read device FPGA register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `address`: Register address.
    /// * [out] `val`:     Current register value.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_ReadFPGAReg(
        device: *mut lms_device_t,
        address: u32,
        val: *mut u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write device FPGA register.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:  Device handle previously obtained by LMS_Open().
    /// *       `address`: Register address.
    /// *       `val`:     Value to write.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_WriteFPGAReg(
        device: *mut lms_device_t,
        address: u32,
        val: u16
    ) -> core::ffi::c_int;
}


// Board parameter


/// Runtime VCTCXO DAC trim value. Does not persist over power-cycle
pub const BOARD_PARAM_DAC: u8 = 0;
/// The value of board temperature sensor (if present), read-only.
pub const BOARD_PARAM_TEMP: u8 = 1;

lime_suite_extern! {
    /// Read custom parameter from board.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `id`:     Parameter identifier (BOARD_PARAM_*).
    /// * [out] `val`:    Current register value.
    /// * [out] `units`:  [optional] measurement units of parameter if available.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_ReadCustomBoardParam(
        device: *mut lms_device_t,
        id: u8,
        val: *mut float_type,
        units: *mut lms_name_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write custom parameter from board.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `id`:     Parameter identifier (BOARD_PARAM_*).
    /// *       `val`:    Current register value.
    /// * [in]  `units`:  [optional] measurement units of parameter if available.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_WriteCustomBoardParam(
        device: *mut lms_device_t,
        id: u8,
        val: float_type,
        units: *const lms_name_t
    ) -> core::ffi::c_int;
}


// Clock definitions
//
// Clock definitions for accessing specific internal clocks


/// Chip reference clock
pub const LMS_CLOCK_REF: c_size_t = 0x0000;
/// RX LO clock
pub const LMS_CLOCK_SXR: c_size_t = 0x0001;
/// TX LO clock
pub const LMS_CLOCK_SXT: c_size_t = 0x0002;
/// CGEN clock
pub const LMS_CLOCK_CGEN: c_size_t = 0x0003;
/// RXTSP reference clock (read-only)
pub const LMS_CLOCK_RXTSP: c_size_t = 0x0004;
/// TXTSP reference clock (read-only)
pub const LMS_CLOCK_TXTSP: c_size_t = 0x0005;

/// External reference clock (write-only)
///
/// Set to positive value to enable usage of external reference clock of the
/// specified frequency. Set to 0 or negative value to disable usage of external
/// reference clock (if switching reference clock source is supported by HW)
pub const LMS_CLOCK_EXTREF: c_size_t = 0x0006;

lime_suite_extern! {
    /// Get frequency of the specified clock.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `clk_id`: Clock identifier (LMS_CLOCK_*).
    /// * [out] `freq`:   Clock frequency in Hz.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetClockFreq(
        dev: *mut lms_device_t,
        clk_id: c_size_t,
        freq: *mut float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Set frequency of the specified clock.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `clk_id`: Clock identifier (LMS_CLOCK_*).
    /// *       `freq`:   Clock frequency in Hz. Pass zero or negative value to only
    ///                   perform tune (if supported) without recalculating values.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetClockFreq(
        dev: *mut lms_device_t,
        clk_id: c_size_t,
        freq: float_type
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write value to VCTCXO trim DAC. Used to adjust/calibrate reference clock
    /// generated by voltage controlled oscillator. Value is written to non-volatile
    /// storage.
    ///
    /// # Note
    ///
    /// Calling this functions switches clock source to VCTCXO.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`: Device handle previously obtained by LMS_Open().
    /// *       `val`: Value to write to VCTCXO trim DAC.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_VCTCXOWrite(
        dev: *mut lms_device_t,
        val: u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Read VCTCXO trim DAC value from non-volatile storage. Returned value is value
    /// that is loaded on power-on and may different from current runtime value.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`: Device handle previously obtained by LMS_Open().
    /// * [out] `val`: VCTCXO trim DAC value.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_VCTCXORead(
        dev: *mut lms_device_t,
        val: *mut u16
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Synchronizes register values between API cache and chip.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// *       `toChip`: if true copies values from API cache to chip.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Synchronize(
        dev: *mut lms_device_t,
        toChip: bool
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// GPIO read.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// * [out] `buffer`: read values (8 GPIO values per byte, LSB first).
    /// *       `len`:    number of bytes to read.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GPIORead(
        dev: *mut lms_device_t,
        buffer: *mut u8,
        len: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// GPIO write.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// * [in]  `buffer`: values to write (8 GPIO values per byte, LSB first).
    /// *       `len`:    number of bytes to write.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GPIOWrite(
        dev: *mut lms_device_t,
        buffer: *const u8,
        len: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// GPIO dir read.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// * [out] `buffer`: GPIO direction configuration(8 GPIO per byte, LSB first; 0 input, 1 output).
    /// *       `len`:    number of bytes to read.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GPIODirRead(
        dev: *mut lms_device_t,
        buffer: *mut u8,
        len: c_size_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// GPIO dir write.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// * [in]  `buffer`: GPIO direction configuration(8 GPIO per byte, LSB first; 0 input, 1 output)
    /// *       `len`:    number of bytes to write.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GPIODirWrite(
        dev: *mut lms_device_t,
        buffer: *const u8,
        len: c_size_t
    ) -> core::ffi::c_int;
}


//// Sample Streaming functions
////
//// The functions in this section provides support for sending and receiving
//// IQ data samples.


/// Metadata structure used in sample transfers
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lms_stream_meta_t {
    /// Timestamp is a value of HW counter with a tick based on sample rate.
    /// In RX: time when the first sample in the returned buffer was received.
    /// In TX: time when the first sample in the submitted buffer should be send.
    pub timestamp: u64,
    /// In TX: wait for the specified HW timestamp before broadcasting data over
    /// the air.
    /// In RX: not used/ignored.
    pub waitForTimestamp: bool,
    /// In TX: send samples to HW even if packet is not completely filled (end TX burst).
    /// In RX: not used/ignored.
    pub flushPartialPacket: bool,
}


// Additional streaming options
//
// These can be combined with lms_stream_t::channel to
// enable additional streaming options.


/// Attempt to align channel phases in MIMO mode (supported only for Rx channels)
pub const LMS_ALIGN_CH_PHASE: c_size_t = 1 << 16;

#[repr(C)]
pub enum dataFmt_t {
    /// 32-bit floating point
    LMS_FMT_F32 = 0,
    /// 16-bit integers
    LMS_FMT_I16,
    /// 12-bit integers stored in 16-bit variables
    LMS_FMT_I12,
}

#[repr(C)]
pub enum linkFmt_t {
    /// 12-bit integers stored in 16-bit variables
    /// when dataFmt=LMS_FMT_I12, 16-bit otherwise
    LMS_LINK_FMT_DEFAULT = 0,
    /// 16-bit integers
    LMS_LINK_FMT_I16,
    /// 12-bit integers
    LMS_LINK_FMT_I12,
}

/// Stream structure
#[repr(C)]
pub struct lms_stream_t {
    /// Stream handle. Should not be modified manually.
    /// Assigned by LMS_SetupStream().
    pub handle: c_size_t,
    /// Indicates whether stream is TX (true) or RX (false)
    pub isTx: bool,
    // Channel number, starts at 0.
    // Can be combined with additional flags  (STREAM_CH_FLAGS).
    pub channel: u32,
    /// FIFO size (in samples) used by stream.
    pub fifoSize: u32,
    /// Parameter for controlling configuration bias toward low latency or high
    /// data throughput range [0,1.0].
    /// 0 - lowest latency, usually results in lower throughput
    /// 1 - higher throughput, usually results in higher latency
    pub throughputVsLatency: core::ffi::c_float,
    /// Data output format
    pub dataFmt: dataFmt_t,
    /// Data link format
    pub linkFmt: linkFmt_t,
}


/// Streaming status structure
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lms_stream_status_t {
    /// Indicates whether the stream is currently active
    pub active: bool,
    /// Number of samples in FIFO buffer
    pub fifoFilledCount: u32,
    /// Size (in samples) of FIFO buffer
    pub fifoSize: u32,
    /// FIFO underrun count since the last call to LMS_GetStreamStatus()
    pub underrun: u32,
    /// FIFO overrun count since the last call to LMS_GetStreamStatus()
    pub overrun: u32,
    /// Number of dropped packets by HW since the last call to LMS_GetStreamStatus()
    pub droppedPackets: u32,
    /// Currently not used
    pub sampleRate: float_type,
    /// Data transfer rate (B/s) over the last 1 s per direction per LMS chip.
    pub linkRate: float_type,
    /// The most recently received Rx timestamp, or the last timestamp submitted to Tx.
    pub timestamp: u64,
}

lime_suite_extern! {
    /// Create new stream based on parameters passed in configuration structure.
    /// The structure is initialized with stream handle.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// * [in]  `stream`: Stream configuration. See the ::lms_stream_t description.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_SetupStream(
        device: *mut lms_device_t,
        stream: *mut lms_stream_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Deallocate memory used by stream.
    ///
    /// # Parameters
    ///
    /// * [in]  `dev`:    Device handle previously obtained by LMS_Open().
    /// * [in]  `stream`: Stream structure previously initialized with LMS_SetupStream().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_DestroyStream(
        device: *mut lms_device_t,
        stream: *mut lms_stream_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Start stream.
    ///
    /// # Parameters
    ///
    /// * [in]  `stream`: Stream structure previously initialized with LMS_SetupStream().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_StartStream(
        stream: *mut lms_stream_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Stop stream.
    ///
    /// # Parameters
    ///
    /// * [in]  `stream`: Stream structure previously initialized with LMS_SetupStream().
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_StopStream(
        stream: *mut lms_stream_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Read samples from the FIFO of the specified stream.
    /// Sample buffer must be big enough to hold requested number of samples.
    ///
    /// # Parameters
    ///
    /// * [in]  `stream`:       Stream structure previously initialized with LMS_SetupStream().
    /// * [out] `samples`:      sample buffer.
    /// *       `sample_count`: Number of samples to read.
    /// * [out] `meta`:         Metadata. See the ::lms_stream_meta_t description.
    /// *       `timeout_ms`:   how long to wait for data before timing out.
    ///
    /// # Return
    ///
    /// number of samples received on success, (-1) on failure.
    pub fn LMS_RecvStream(
        stream: *mut lms_stream_t,
        samples: *mut core::ffi::c_void,
        sample_count: c_size_t,
        meta: *mut lms_stream_meta_t,
        timeout_ms: core::ffi::c_uint
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Get stream operation status.
    ///
    /// # Parameters
    ///
    /// * [in]  `stream`:       Stream structure previously initialized with LMS_SetupStream().
    /// * [out] `status`:       Stream status. See the ::lms_stream_status_t for description.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_GetStreamStatus(
        stream: *mut lms_stream_t,
        status: *mut lms_stream_status_t
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Write samples to the FIFO of the specified stream.
    ///
    /// # Parameters
    ///
    /// * [in]  `stream`:       Stream structure previously initialized with LMS_SetupStream().
    /// * [in]  `samples`:      sample buffer.
    /// *       `sample_count`: Number of samples to write.
    /// * [in]  `meta`:         Metadata. See the ::lms_stream_meta_t description.
    /// *       `timeout_ms`:   how long to wait for data before timing out.
    ///
    /// # Return
    ///
    /// Number of samples send on success, (-1) on failure.
    pub fn LMS_SendStream(
        stream: *mut lms_stream_t,
        samples: *const core::ffi::c_void,
        sample_count: c_size_t,
        meta: *const lms_stream_meta_t,
        timeout_ms: core::ffi::c_uint
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Uploads waveform to on board memory for later use.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:       Device handle previously obtained by LMS_Open().
    /// * [in]  `samples`:      multiple channel samples data.
    /// *       `chCount`:      number of waveform channels (1 or 2).
    /// *       `sample_count`: number of samples in each channel. Must be multiple of 4.
    /// *       `format`:       waveform data format: 0 - int16 [-2048, 2047],
    ///                                               1 - int16 [-32768, 32767]
    ///                                               2 - float [-1.0, 1.0]
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_UploadWFM(
        device: *mut lms_device_t,
        samples: *const *const core::ffi::c_void,
        chCount: u8,
        sample_count: c_size_t,
        format: core::ffi::c_int
    ) -> core::ffi::c_int;
}

lime_suite_extern! {
    /// Enables/Disables transmitting of uploaded waveform.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// *       `chan`:   Channel index.
    /// *       `active`: Enable/Disable waveform playback.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_EnableTxWFM(
        device: *mut lms_device_t,
        chan: core::ffi::c_uint,
        active: bool
    ) -> core::ffi::c_int;
}


//// Version and update functions
////
//// The functions in this section provides ability to check device version
//// and perform updates


lime_suite_extern! {
    /// Get the list of supported programming modes.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    /// * [out] `list`:   list of programming modes (can be NULL).
    ///
    /// # Return
    ///
    /// Number of modes in the list, (-1) on failure.
    pub fn LMS_GetProgramModes(
        device: *mut lms_device_t,
        list: *mut lms_name_t
    ) -> core::ffi::c_int;
}

/// Callback from programming processes.
///
/// # Parameters
///
/// * `bsent`:       number of bytes transferred.
/// * `btotal`:      total number of bytes to send.
/// * `progressMsg`: string describing current progress state.
///
/// # Return
///
/// 0-continue programming, 1-abort operation.
pub type lms_prog_callback_t = Option<
    extern "C" fn(
        bsent: core::ffi::c_int,
        btotal: core::ffi::c_int,
        progressMsg: *const core::ffi::c_char,
    ) -> bool,
>;

lime_suite_extern! {
    /// Write binary firmware/bitsteam image to specified device component.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`:   Device handle previously obtained by LMS_Open().
    /// * [in]  `data`:     Pointer to memory containing firmware/bitsteam image.
    /// *       `size`:     Size of firmware/bitsteam image in bytes.
    /// * [in]  `mode`:     programming mode, use LMS_GetProgramModes to get list of modes.
    /// * [in]  `callback`: callback function for monitoring progress.
    ///
    /// # Return
    ///
    /// 0 on success, (-1) on failure.
    pub fn LMS_Program(
        device: *mut lms_device_t,
        data: *const core::ffi::c_char,
        size: c_size_t,
        mode: *const lms_name_t,
        callback: lms_prog_callback_t
    ) -> core::ffi::c_int;
}

/// Device information structure
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lms_dev_info_t {
    /// The display name of the device
    pub deviceName: [core::ffi::c_char; 32],
    /// The display name of the expansion card
    pub expensionName: [core::ffi::c_char; 32],
    /// The firmware version as a string
    pub firmwareVersion: [core::ffi::c_char; 16],
    /// The hardware version as a string
    pub hardwareVersion: [core::ffi::c_char; 16],
    /// The protocol version as a string
    pub protocolVersion: [core::ffi::c_char; 16],
    /// A unique board serial number
    pub boardSerialNumber: u64,
    /// Gateware version as a string
    pub gatewareVersion: [core::ffi::c_char; 16],
    /// Which board should use this gateware
    pub gatewareTargetBoard: [core::ffi::c_char; 32],
}

lime_suite_extern! {
    /// Get device serial number and version information.
    ///
    /// # Note
    ///
    /// This function returns pointer to internal data structure that gets
    /// deallocated when device is closed. Do not attempt to read from it after
    /// closing the device. If you need to keep using device info returned by this
    /// function after closing the device, make a copy before closing the device.
    ///
    /// # Parameters
    ///
    /// * [in]  `device`: Device handle previously obtained by LMS_Open().
    ///
    /// # Return
    ///
    /// pointer to device info structure ::lms_dev_info_t.
    pub fn LMS_GetDeviceInfo(
        device: *mut lms_device_t
    ) -> *const lms_dev_info_t;
}

lime_suite_extern! {
    /// Returns API library version.
    pub fn LMS_GetLibraryVersion() -> *const core::ffi::c_char;
}

lime_suite_extern! {
    /// Get the error message detailing why the last error occurred.
    ///
    /// # Deprecated
    ///
    /// Use LMS_RegisterLogHandler() to obtain error messages.
    ///
    /// # Returns
    ///
    /// Last error message.
    pub fn LMS_GetLastErrorMessage() -> *const core::ffi::c_char;
}


// Message logging level

/// A critical error. The application might not be able to continue running successfully.
pub const LMS_LOG_CRITICAL: core::ffi::c_int = 0;
/// An error message. An operation did not complete successfully.
pub const LMS_LOG_ERROR: core::ffi::c_int = 1;
/// A warning message. An operation completed with an unexpected result.
pub const LMS_LOG_WARNING: core::ffi::c_int = 2;
/// An informational message, usually denoting the successful completion of an operation.
pub const LMS_LOG_INFO: core::ffi::c_int = 3;
/// A debugging message.
pub const LMS_LOG_DEBUG: core::ffi::c_int = 4;

/// Callback function for redirecting API messages.
///
/// # Parameters
///
/// * `lvl`: LMS_LOG_* value.
/// * `msg`: string containing log message text.
pub type LMS_LogHandler = Option<
    extern "C" fn(lvl: core::ffi::c_int, msg: *const core::ffi::c_char),
>;

lime_suite_extern! {
    /// Register a new system log handler. Should be called to replace the default
    /// stdio handler.
    ///
    /// # Parameters
    ///
    /// * [in]  `handler`: function for handling API messages.
    pub fn LMS_RegisterLogHandler(
        handler: LMS_LogHandler
    );
}
