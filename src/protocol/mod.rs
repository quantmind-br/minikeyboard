pub mod codec;
pub mod frame;
pub mod identify;
pub mod read_config;
pub mod write_config;

pub use codec::CodecError;
pub use frame::*;
pub use identify::identify;
pub use read_config::{ReadCapability, ReadConfigCommand, DEFAULT_READ};
pub use write_config::{write_dirty_records, WriteAuthorization, WriteError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("transport: {0}")]
    Transport(#[from] crate::device::transport::TransportError),

    #[error("identify response too short ({0} bytes)")]
    ShortIdentifyResponse(usize),

    #[error("unexpected report id 0x{0:02X}")]
    WrongReportId(u8),

    #[error("read protocol not verified — no golden vectors imported")]
    UnverifiedReadProtocol,

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("readback verification failed")]
    VerificationMismatch,
}

impl ProtocolError {
    pub fn category(&self) -> &'static str {
        match self {
            Self::Transport(e) => e.category(),
            Self::ShortIdentifyResponse(_) => "short_response",
            Self::WrongReportId(_) => "wrong_report_id",
            Self::UnverifiedReadProtocol => "unverified_read",
            Self::InvalidResponse(_) => "invalid_response",
            Self::VerificationMismatch => "verification",
        }
    }
}
