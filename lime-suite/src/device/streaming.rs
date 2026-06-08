//! Sample Streaming functions
//!
//! The functions in this section provides support for sending and receiving
//! IQ data samples.

use std::ops::Deref;

use lime_suite_sys::bindings::*;

use crate::error::{Error, LmsRetVal, LmsRetValNonStandard};

use super::{Device, high_level::ChannelDirection, invoke_lms};

/// Metadata structure used in sample transfers
pub type StreamMeta = lms_stream_meta_t;

// Additional streaming options
//
// These can be combined with Stream::channel to
// enable additional streaming options.

/// Board parameter
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[non_exhaustive]
pub struct StreamChFlags(usize);

impl StreamChFlags {
    /// Attempt to align channel phases in MIMO
    /// mode (supported only for Rx channels)
    pub const ALIGN_CH_PHASE: StreamChFlags =
        StreamChFlags(LMS_ALIGN_CH_PHASE);
}

impl From<StreamChFlags> for usize {
    fn from(stream_ch_flags: StreamChFlags) -> Self {
        *stream_ch_flags
    }
}

impl Deref for StreamChFlags {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Data output format
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataFmt {
    /// 32-bit floating point
    FmtF32 = dataFmt_t::LMS_FMT_F32 as u32,
    /// 16-bit integers
    FmtI16,
    /// 12-bit integers stored in 16-bit variables
    FmtI12,
}

impl From<dataFmt_t> for DataFmt {
    fn from(data_fmt: dataFmt_t) -> Self {
        match data_fmt {
            dataFmt_t::LMS_FMT_F32 => Self::FmtF32,
            dataFmt_t::LMS_FMT_I16 => Self::FmtI16,
            dataFmt_t::LMS_FMT_I12 => Self::FmtI12,
        }
    }
}

impl From<DataFmt> for dataFmt_t {
    fn from(data_fmt: DataFmt) -> Self {
        match data_fmt {
            DataFmt::FmtF32 => dataFmt_t::LMS_FMT_F32,
            DataFmt::FmtI16 => dataFmt_t::LMS_FMT_I16,
            DataFmt::FmtI12 => dataFmt_t::LMS_FMT_I12,
        }
    }
}

/// Data output format
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkFmt {
    /// 12-bit integers stored in 16-bit variables
    /// when dataFmt=LMS_FMT_I12, 16-bit otherwise
    LinkFmtDefault = linkFmt_t::LMS_LINK_FMT_DEFAULT as u32,
    /// 16-bit integers
    LinkFmtI16,
    /// 12-bit integers
    LinkFmtI12,
}

impl From<linkFmt_t> for LinkFmt {
    fn from(data_fmt: linkFmt_t) -> Self {
        match data_fmt {
            linkFmt_t::LMS_LINK_FMT_DEFAULT => Self::LinkFmtDefault,
            linkFmt_t::LMS_LINK_FMT_I16 => Self::LinkFmtI16,
            linkFmt_t::LMS_LINK_FMT_I12 => Self::LinkFmtI12,
        }
    }
}

impl From<LinkFmt> for linkFmt_t {
    fn from(data_fmt: LinkFmt) -> Self {
        match data_fmt {
            LinkFmt::LinkFmtDefault => linkFmt_t::LMS_LINK_FMT_DEFAULT,
            LinkFmt::LinkFmtI16 => linkFmt_t::LMS_LINK_FMT_I16,
            LinkFmt::LinkFmtI12 => linkFmt_t::LMS_LINK_FMT_I12,
        }
    }
}

#[derive(Clone)]
/// Stream configuration structure.
pub struct StreamConfiguration {
    /// Indicates whether stream is TX or RX.
    pub channel_dir: ChannelDirection,
    /// Channel number, starts at 0.
    /// Can be combined with additional flags (StreamChFlags).
    pub channel: u32,
    /// FIFO size (in samples) used by stream.
    pub fifo_size: u32,
    /// Parameter for controlling configuration bias toward low latency or high
    /// data throughput range [0,1.0].
    /// 0 - lowest latency, usually results in lower throughput
    /// 1 - higher throughput, usually results in higher latency
    pub throughput_vs_latency: f32,
    /// Data output format
    pub data_fmt: DataFmt,
    /// Data link format
    pub link_fmt: LinkFmt,
}

#[derive(Clone)]
/// Stream.
pub struct Stream {
    // Cloned device
    device: Device,
    /// Stream handle
    handle: usize,
    /// Stream configuration
    pub configuration: StreamConfiguration,
}

#[doc(hidden)]
impl From<&StreamConfiguration> for lms_stream_t {
    fn from(stream: &StreamConfiguration) -> Self {
        Self {
            handle: 0,
            isTx: *stream.channel_dir,
            channel: stream.channel,
            fifoSize: stream.fifo_size,
            throughputVsLatency: stream.throughput_vs_latency,
            dataFmt: stream.data_fmt.into(),
            linkFmt: stream.link_fmt.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Streaming status structure
pub struct StreamStatus {
    /// Indicates whether the stream is currently active
    pub active: bool,
    /// Number of samples in FIFO buffer
    pub fifo_filled_count: u32,
    /// Size (in samples) of FIFO buffer
    pub fifo_size: u32,
    /// FIFO underrun count since the last call to
    /// [`get_stream_status`](Stream::get_stream_status)
    pub underrun: u32,
    /// FIFO overrun count since the last call to
    /// [`get_stream_status`](Stream::get_stream_status)
    pub overrun: u32,
    /// Number of dropped packets by HW since the last call to
    /// [`get_stream_status`](Stream::get_stream_status)
    pub dropped_packets: u32,
    /// Currently not used
    pub sample_rate: f64,
    /// Data transfer rate (B/s) over the last 1 s per direction per LMS chip.
    pub link_rate: f64,
    /// The most recently received Rx timestamp, or the last timestamp
    /// submitted to Tx.
    pub timestamp: u64,
}

#[doc(hidden)]
impl From<lms_stream_status_t> for StreamStatus {
    fn from(stream_status: lms_stream_status_t) -> Self {
        Self {
            active: stream_status.active,
            fifo_filled_count: stream_status.fifoFilledCount,
            fifo_size: stream_status.fifoSize,
            underrun: stream_status.underrun,
            overrun: stream_status.overrun,
            dropped_packets: stream_status.droppedPackets,
            sample_rate: stream_status.sampleRate,
            link_rate: stream_status.linkRate,
            timestamp: stream_status.timestamp,
        }
    }
}

/// Waveform data format
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformDataFmt {
    /// int16 [-2048, 2047]
    FmtI12 = 0,
    /// int16 [-32768, 32767]
    FmtI16,
    /// float [-1.0, 1.0]
    FmtF32,
}

impl Device {
    /// Create new stream based on parameters passed in `configuration`.
    pub fn create_stream(&self, configuration: StreamConfiguration) -> Stream {
        Stream {
            device: self.clone(),
            handle: 0,
            configuration,
        }
    }
}

impl Stream {
    fn as_raw_stream_t(&self) -> lms_stream_t {
        let mut stream_t = lms_stream_t::from(&self.configuration);
        stream_t.handle = self.handle;
        stream_t
    }

    /// Initialize stream based on configuration.
    pub fn setup(&mut self) -> Result<(), Error> {
        let mut stream_t = self.as_raw_stream_t();

        LmsRetVal::from(invoke_lms!(
            LMS_SetupStream,
            self.device.get_handle(),
            &mut stream_t as *mut lms_stream_t
        ))
        .into_result()?;

        self.handle = stream_t.handle;

        Ok(())
    }

    /// Deallocate memory used by previously initialized `stream`
    /// with [`setup`](crate::device::streaming::Stream::setup).
    pub fn destroy(&self) -> Result<(), Error> {
        let mut stream_t = self.as_raw_stream_t();

        LmsRetVal::from(invoke_lms!(
            LMS_DestroyStream,
            self.device.get_handle(),
            &mut stream_t as *mut lms_stream_t
        ))
        .into_result()
    }

    /// Start initialized stream.
    pub fn start(&self) -> Result<(), Error> {
        let mut stream_t = self.as_raw_stream_t();

        LmsRetVal::from(invoke_lms!(
            LMS_StartStream,
            &mut stream_t as *mut lms_stream_t
        ))
        .into_result()
    }

    /// Stop initialized stream.
    pub fn stop(&self) -> Result<(), Error> {
        let mut stream_t = self.as_raw_stream_t();

        LmsRetVal::from(invoke_lms!(
            LMS_StopStream,
            &mut stream_t as *mut lms_stream_t
        ))
        .into_result()
    }

    /// Read samples from the FIFO of the initialized stream.
    pub fn recv(
        &self,
        sample_count: usize,
        timeout_ms: u32,
    ) -> Result<(Vec<u8>, StreamMeta), Error> {
        let sample_size_byte = 4;

        let mut stream_t = self.as_raw_stream_t();
        let mut samples: Vec<u8> = vec![0; 4096];
        let mut stream_meta_t = lms_stream_meta_t::default();

        let samples_received = LmsRetValNonStandard::from(invoke_lms!(
            LMS_RecvStream,
            &mut stream_t as *mut lms_stream_t,
            samples.as_mut_ptr() as *mut core::ffi::c_void,
            sample_count,
            &mut stream_meta_t as *mut lms_stream_meta_t,
            timeout_ms
        ))
        .into_result()? as usize;

        samples.truncate(samples_received * sample_size_byte);

        Ok((samples, stream_meta_t))
    }

    /// Get stream operation status.
    pub fn get_stream_status(&self) -> Result<StreamStatus, Error> {
        let mut stream_t = self.as_raw_stream_t();
        let mut sream_status_t = lms_stream_status_t::default();

        LmsRetVal::from(invoke_lms!(
            LMS_GetStreamStatus,
            &mut stream_t as *mut lms_stream_t,
            &mut sream_status_t as *mut lms_stream_status_t
        ))
        .into_result()?;

        Ok(StreamStatus::from(sream_status_t))
    }

    /// Write samples to the FIFO of the initialized stream
    /// and return number of samples send on success.
    ///
    /// # Note
    ///
    /// `samples_count` is not equal `samples.len()`
    pub fn send(
        &self,
        samples: &[u8],
        samples_count: usize,
        meta: Option<StreamMeta>,
        timeout_ms: u32,
    ) -> Result<usize, Error> {
        let mut stream_t = self.as_raw_stream_t();

        let meta_raw_ptr = match meta {
            Some(m) => &m as *const lms_stream_meta_t,
            None => std::ptr::null(),
        };

        let ret = LmsRetValNonStandard::from(invoke_lms!(
            LMS_SendStream,
            &mut stream_t as *mut lms_stream_t,
            samples.as_ptr() as *const core::ffi::c_void,
            samples_count,
            meta_raw_ptr,
            timeout_ms
        ))
        .into_result()?;

        Ok(ret as usize)
    }
}

impl Device {
    /// Uploads waveform to on board memory for later use.
    ///
    /// `samples` is multiple channel samples data. `samples_count` in each
    /// channel must be multiple of 4.
    pub fn upload_wfm(
        &self,
        samples: &&[u8],
        samples_count: usize,
        format: WaveformDataFmt,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_UploadWFM,
            self.get_handle(),
            samples.as_ptr() as *const *const core::ffi::c_void,
            samples.len() as u8,
            samples_count,
            format as i32
        ))
        .into_result()
    }

    /// Enables/Disables transmitting of uploaded waveform.
    pub fn enable_tx_wfm(
        &self,
        channel: u32,
        active: bool,
    ) -> Result<(), Error> {
        LmsRetVal::from(invoke_lms!(
            LMS_EnableTxWFM,
            self.get_handle(),
            channel,
            active
        ))
        .into_result()
    }
}
