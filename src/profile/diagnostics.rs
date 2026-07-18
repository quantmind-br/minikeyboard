//! Redacted diagnostic export — never includes serial (default), macros, or raw records.

use std::io::Write;
use std::path::Path;

use serde::Serialize;
use tempfile::NamedTempFile;

use super::ProfileError;
use crate::device::discovery::{DeviceIdentity, VALIDATED_PID, VALIDATED_VID};
use crate::device::session::SessionState;
use crate::domain::geometry::DeviceVariant;
use crate::{APP_ID, VERSION};

#[derive(Debug, Serialize)]
pub struct DiagnosticReport {
    pub app_version: String,
    pub app_id: String,
    pub os: String,
    pub allowed_vid_pid: Vec<String>,
    pub interface: Option<i32>,
    pub usage_page: Option<String>,
    pub descriptor_hex: Option<String>,
    pub variant: Option<VariantDiag>,
    pub session_state: String,
    pub last_error_category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VariantDiag {
    pub key_count: u8,
    pub extra_count: u8,
    pub subtype: u8,
    pub geometry_id: String,
    pub support: String,
}

pub struct DiagnosticInput<'a> {
    pub device: Option<&'a DeviceIdentity>,
    pub variant: Option<&'a DeviceVariant>,
    pub session: SessionState,
    pub descriptor: Option<&'a [u8]>,
    pub last_error_category: Option<&'a str>,
}

pub fn build_diagnostics(input: DiagnosticInput<'_>) -> DiagnosticReport {
    DiagnosticReport {
        app_version: VERSION.into(),
        app_id: APP_ID.into(),
        os: std::env::consts::OS.into(),
        allowed_vid_pid: vec![format!("{VALIDATED_VID:04x}:{VALIDATED_PID:04x}")],
        interface: input.device.map(|d| d.interface_number),
        usage_page: input.device.map(|d| format!("0x{:04X}", d.usage_page)),
        descriptor_hex: input.descriptor.map(hex::encode),
        variant: input.variant.map(|v| VariantDiag {
            key_count: v.key_count,
            extra_count: v.extra_count,
            subtype: v.subtype,
            geometry_id: v.geometry_id.clone(),
            support: format!("{:?}", v.support),
        }),
        session_state: input.session.rail_label().into(),
        last_error_category: input.last_error_category.map(str::to_owned),
    }
}

pub fn export_diagnostics(path: &Path, input: DiagnosticInput<'_>) -> Result<(), ProfileError> {
    let report = build_diagnostics(input);
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let mut tmp = NamedTempFile::new_in(parent)?;
    let text = serde_json::to_string_pretty(&report)?;
    tmp.write_all(text.as_bytes())?;
    tmp.write_all(b"\n")?;
    tmp.flush()?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| ProfileError::Io(e.error))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::session::SessionState;
    use tempfile::tempdir;

    #[test]
    fn redacted_export_omits_serial_and_raw() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("diag.json");
        export_diagnostics(
            &path,
            DiagnosticInput {
                device: None,
                variant: None,
                session: SessionState::Disconnected,
                descriptor: Some(&[0x06, 0x00, 0xFF]),
                last_error_category: Some("permission_denied"),
            },
        )
        .unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("serial"));
        assert!(!text.contains("raw_record"));
        assert!(text.contains("permission_denied"));
        assert!(text.contains(VERSION));
    }
}
