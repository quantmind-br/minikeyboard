//! Top-level application errors. Protocol/device details stay in their modules.

use thiserror::Error;

use crate::device::session::InvalidTransition;
use crate::device::transport::TransportError;
use crate::profile::ProfileError;
use crate::protocol::{CodecError, ProtocolError, WriteError};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("device error: {0}")]
    Device(#[from] crate::device::DeviceError),

    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("codec error: {0}")]
    Codec(#[from] CodecError),

    #[error("write error: {0}")]
    Write(#[from] WriteError),

    #[error("profile error: {0}")]
    Profile(#[from] ProfileError),

    #[error("invalid session transition: {0}")]
    Session(#[from] InvalidTransition),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Message(String),
}

impl AppError {
    /// Coarse category for status UI and redacted diagnostics (never raw bytes).
    pub fn category(&self) -> &'static str {
        match self {
            Self::Device(_) => "device",
            Self::Transport(e) => e.category(),
            Self::Protocol(e) => e.category(),
            Self::Codec(_) => "codec",
            Self::Write(e) => e.category(),
            Self::Profile(_) => "profile",
            Self::Session(_) => "session",
            Self::Io(_) => "io",
            Self::Message(_) => "app",
        }
    }
}
