pub mod bindings;
pub mod diagnostics;
pub mod json;
pub mod migration;

pub use diagnostics::export_diagnostics;
pub use json::{export_profile, import_profile, save_last_read_backup};
pub use migration::migrate_schema;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unsupported schema version {0}")]
    UnsupportedSchema(u32),

    #[error("invalid profile: {0}")]
    Invalid(String),

    #[error("identity mismatch: profile {profile} vs device {device}")]
    IdentityMismatch { profile: String, device: String },

    #[error("path error: {0}")]
    Path(String),
}
