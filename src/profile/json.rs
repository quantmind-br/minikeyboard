//! Atomic, versioned profile import/export.

use std::fs;
use std::io::Write;
use std::path::Path;

use tempfile::NamedTempFile;

use super::migration;
use super::ProfileError;
use crate::domain::action::Action;
use crate::domain::config::{
    ConfigProvenance, DeviceConfig, LAYER_COUNT, MAX_POSITIONS, RECORD_SIZE, SCHEMA_VERSION,
};

pub fn export_profile(
    path: &Path,
    config: &DeviceConfig,
    include_serial: bool,
) -> Result<(), ProfileError> {
    let mut out = config.clone();
    if !include_serial {
        out.identity.serial = None;
    }
    out.clear_dirty();
    write_json_atomic(path, &out)
}

pub fn import_profile(path: &Path) -> Result<DeviceConfig, ProfileError> {
    let text = fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    let mut config: DeviceConfig = migration::migrate_schema(value)?;
    validate_config(&config)?;
    config.provenance = ConfigProvenance::ImportedProfile;
    config.clear_dirty();
    for layer in &mut config.layers {
        for pos in &mut layer.positions {
            let decoded = crate::protocol::codec::decode_record(pos.raw_record);
            pos.action = decoded.action;
            pos.delay_ms = decoded.delay_ms;
            pos.dirty = false;
        }
    }
    Ok(config)
}

pub fn save_last_read_backup(config: &DeviceConfig) -> Result<std::path::PathBuf, ProfileError> {
    let dirs = directories::ProjectDirs::from("br", "Quantmind", "MiniKeyboard")
        .ok_or_else(|| ProfileError::Path("could not resolve state directory".into()))?;
    let state = dirs
        .state_dir()
        .ok_or_else(|| ProfileError::Path("state_dir unavailable".into()))?;
    let dir = state.join("backups").join(config.identity.backup_key());
    fs::create_dir_all(&dir)?;
    let path = dir.join("last-read.json");
    export_profile(&path, config, true)?;
    Ok(path)
}

fn write_json_atomic(path: &Path, value: &impl serde::Serialize) -> Result<(), ProfileError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let mut tmp = NamedTempFile::new_in(parent)?;
    let text = serde_json::to_string_pretty(value)?;
    tmp.write_all(text.as_bytes())?;
    tmp.write_all(b"\n")?;
    tmp.flush()?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| ProfileError::Io(e.error))?;
    Ok(())
}

fn validate_config(config: &DeviceConfig) -> Result<(), ProfileError> {
    if config.schema_version != SCHEMA_VERSION {
        return Err(ProfileError::UnsupportedSchema(config.schema_version));
    }
    if config.layers.len() != LAYER_COUNT {
        return Err(ProfileError::Invalid(format!(
            "expected {LAYER_COUNT} layers"
        )));
    }
    for (i, layer) in config.layers.iter().enumerate() {
        if layer.positions.len() > MAX_POSITIONS {
            return Err(ProfileError::Invalid(format!(
                "layer {i} has more than {MAX_POSITIONS} positions"
            )));
        }
        let mut seen = std::collections::BTreeSet::new();
        for pos in &layer.positions {
            if !seen.insert(pos.logical_index) {
                return Err(ProfileError::Invalid(format!(
                    "layer {i} duplicate logical_index {}",
                    pos.logical_index
                )));
            }
            if pos.raw_record.len() != RECORD_SIZE {
                return Err(ProfileError::Invalid("raw_record size".into()));
            }
            if let Action::Lighting { mode, color } = pos.action {
                if mode > 5 {
                    return Err(ProfileError::Invalid(format!("lighting mode {mode}")));
                }
                if color > 7 {
                    return Err(ProfileError::Invalid(format!("lighting color {color}")));
                }
            }
        }
    }
    Ok(())
}

pub fn check_identity_compatible(
    profile: &DeviceConfig,
    device: &crate::domain::config::ProfileIdentity,
) -> Result<IdentityImportNote, ProfileError> {
    let p = &profile.identity;
    if p.vid != device.vid
        || p.pid != device.pid
        || p.key_count != device.key_count
        || p.extra_count != device.extra_count
        || p.subtype != device.subtype
    {
        return Err(ProfileError::IdentityMismatch {
            profile: format!(
                "{:04x}:{:04x} {}+{} s{}",
                p.vid, p.pid, p.key_count, p.extra_count, p.subtype
            ),
            device: format!(
                "{:04x}:{:04x} {}+{} s{}",
                device.vid, device.pid, device.key_count, device.extra_count, device.subtype
            ),
        });
    }
    if p.serial != device.serial {
        Ok(IdentityImportNote::SerialDiffers)
    } else {
        Ok(IdentityImportNote::Exact)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityImportNote {
    Exact,
    SerialDiffers,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::{LayerConfig, PositionConfig, ProfileIdentity};
    use tempfile::tempdir;

    fn sample() -> DeviceConfig {
        DeviceConfig {
            schema_version: SCHEMA_VERSION,
            identity: ProfileIdentity {
                vid: 0x1189,
                pid: 0x8842,
                serial: Some("TEST".into()),
                key_count: 6,
                extra_count: 2,
                subtype: 0,
            },
            layers: [
                LayerConfig {
                    positions: vec![PositionConfig::empty(0)],
                },
                LayerConfig {
                    positions: vec![PositionConfig::empty(0)],
                },
                LayerConfig {
                    positions: vec![PositionConfig::empty(0)],
                },
            ],
            opaque_device_data: vec![],
            provenance: ConfigProvenance::Mock,
        }
    }

    #[test]
    fn atomic_roundtrip_strips_serial_when_requested() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("p.json");
        let cfg = sample();
        export_profile(&path, &cfg, false).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("TEST"));
        let back = import_profile(&path).unwrap();
        assert_eq!(back.identity.key_count, 6);
        assert!(matches!(back.provenance, ConfigProvenance::ImportedProfile));
    }

    #[test]
    fn rejects_bad_schema() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.json");
        fs::write(&path, r#"{"schema_version":99,"identity":{},"layers":[]}"#).unwrap();
        let err = import_profile(&path).unwrap_err();
        assert!(matches!(err, ProfileError::UnsupportedSchema(99)));
    }
}
