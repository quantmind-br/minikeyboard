pub mod discovery;
pub mod evdev;
pub mod mock;
pub mod session;
pub mod transport;
pub mod worker;

pub use discovery::{discover, DeviceIdentity, VALIDATED_VID, VALIDATED_PID};
pub use session::{InvalidTransition, SessionEvent, SessionMachine, SessionState};
pub use transport::{HidTransport, RealHidTransport, TransportError};
pub use worker::{WorkerCommand, WorkerEvent, WorkerHandle};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("hidapi: {0}")]
    HidApi(String),

    #[error("device not found")]
    NotFound,

    #[error("descriptor mismatch for selected interface")]
    DescriptorMismatch,

    #[error("permission denied opening hidraw — install data/udev/70-minikeyboard.rules via linux/setup-hid-permissions.sh")]
    PermissionDenied,

    #[error("device busy or unavailable")]
    Busy,

    #[error("{0}")]
    Message(String),
}

impl DeviceError {
    pub fn category(&self) -> &'static str {
        match self {
            Self::HidApi(_) => "hidapi",
            Self::NotFound => "not_found",
            Self::DescriptorMismatch => "descriptor_mismatch",
            Self::PermissionDenied => "permission_denied",
            Self::Busy => "busy",
            Self::Message(_) => "device",
        }
    }
}
