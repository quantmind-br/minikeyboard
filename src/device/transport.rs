//! Sole HID I/O boundary. Real + mock implement this trait.

use std::time::Duration;

use thiserror::Error;

use crate::protocol::frame::{INPUT_LEN, REPORT_LEN};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransportError {
    #[error("HID write timed out")]
    Timeout,

    #[error("permission denied on hidraw device")]
    PermissionDenied,

    #[error("device disconnected")]
    Disconnected,

    #[error("short write: expected {expected} bytes, got {got}")]
    ShortWrite { expected: usize, got: usize },

    #[error("HID I/O: {0}")]
    Io(String),
}

impl TransportError {
    pub fn category(&self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::PermissionDenied => "permission_denied",
            Self::Disconnected => "disconnected",
            Self::ShortWrite { .. } => "short_write",
            Self::Io(_) => "io",
        }
    }

    pub fn is_disconnect_or_timeout(&self) -> bool {
        matches!(self, Self::Timeout | Self::Disconnected)
    }
}

pub trait HidTransport: Send {
    fn write_report(&mut self, report: &[u8; REPORT_LEN]) -> Result<(), TransportError>;
    fn read_report(&mut self, timeout: Duration) -> Result<Vec<u8>, TransportError>;
    fn descriptor(&mut self) -> Result<Vec<u8>, TransportError>;

    /// Drain queued input reports without waiting. Used before request/response commands so a
    /// previous commit cannot contaminate the next read transaction.
    fn drain_pending(&mut self) -> Result<usize, TransportError> {
        Ok(0)
    }
}

/// Real hidapi transport. `write()` must return exactly 65; `read_timeout` into 64-byte buffer.
pub struct RealHidTransport {
    device: hidapi::HidDevice,
}

impl RealHidTransport {
    pub fn new(device: hidapi::HidDevice) -> Self {
        Self { device }
    }

    pub fn into_inner(self) -> hidapi::HidDevice {
        self.device
    }
}

impl HidTransport for RealHidTransport {
    fn write_report(&mut self, report: &[u8; REPORT_LEN]) -> Result<(), TransportError> {
        match self.device.write(report) {
            Ok(n) if n == REPORT_LEN => Ok(()),
            Ok(n) => Err(TransportError::ShortWrite {
                expected: REPORT_LEN,
                got: n,
            }),
            Err(e) => Err(map_hid_error(e)),
        }
    }

    fn read_report(&mut self, timeout: Duration) -> Result<Vec<u8>, TransportError> {
        let mut buf = [0u8; INPUT_LEN];
        let ms = timeout.as_millis().min(i32::MAX as u128) as i32;
        match self.device.read_timeout(&mut buf, ms) {
            Ok(0) => Err(TransportError::Timeout),
            Ok(n) => Ok(buf[..n].to_vec()),
            Err(e) => Err(map_hid_error(e)),
        }
    }

    fn drain_pending(&mut self) -> Result<usize, TransportError> {
        let mut drained = 0;
        while self.read_with_timeout(0)?.is_some() {
            drained += 1;
            if drained == 256 {
                return Err(TransportError::Io(
                    "more than 256 pending HID reports; device did not quiesce".into(),
                ));
            }
        }
        Ok(drained)
    }

    fn descriptor(&mut self) -> Result<Vec<u8>, TransportError> {
        let mut buf = [0u8; 4096];
        match self.device.get_report_descriptor(&mut buf) {
            Ok(n) => Ok(buf[..n].to_vec()),
            Err(e) => Err(map_hid_error(e)),
        }
    }
}

impl RealHidTransport {
    fn read_with_timeout(&self, timeout_ms: i32) -> Result<Option<Vec<u8>>, TransportError> {
        let mut buf = [0u8; INPUT_LEN];
        match self.device.read_timeout(&mut buf, timeout_ms) {
            Ok(0) => Ok(None),
            Ok(n) => Ok(Some(buf[..n].to_vec())),
            Err(e) => Err(map_hid_error(e)),
        }
    }
}

fn map_hid_error(e: hidapi::HidError) -> TransportError {
    let msg = e.to_string();
    let lower = msg.to_lowercase();
    if lower.contains("permission") || lower.contains("access") || lower.contains("eacces") {
        TransportError::PermissionDenied
    } else if lower.contains("disconnect")
        || lower.contains("no such device")
        || lower.contains("nodev")
        || lower.contains("not connected")
    {
        TransportError::Disconnected
    } else if lower.contains("timeout") || lower.contains("timed out") {
        TransportError::Timeout
    } else {
        TransportError::Io(msg)
    }
}
