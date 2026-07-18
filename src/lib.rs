//! Mini Keyboard — native Linux programmer for USB mini macro keyboards.
//!
//! Clean-room implementation: protocol modules stay capability-gated until
//! golden capture vectors are imported and verified.

pub mod app;
pub mod device;
pub mod domain;
pub mod error;
pub mod profile;
pub mod protocol;
pub mod ui;

pub use error::{AppError, Result};

/// Application ID used for GSettings, resources, and desktop metadata.
pub const APP_ID: &str = "br.com.quantmind.MiniKeyboard";

/// Package version from Cargo.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
