//! Local application/script bindings triggered by marker keys.
//!
//! A binding pairs a marker chord — HID usage F13–F24 (`0x68..=0x73`) plus
//! optional modifiers, keys that do not exist on ordinary keyboards — with a
//! shell command. The marker is written to the device as a plain Keyboard
//! action; `minikeyboard-daemon` watches the device's evdev nodes and runs
//! the command when the marker fires.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::ProfileError;
use crate::domain::action::Modifiers;

pub const MARKER_MIN: u8 = 0x68; // F13
pub const MARKER_MAX: u8 = 0x73; // F24

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppBinding {
    pub usage: u8,
    pub modifiers: Modifiers,
    pub name: String,
    pub command: String,
}

pub fn is_marker(usage: u8) -> bool {
    (MARKER_MIN..=MARKER_MAX).contains(&usage)
}

pub fn find<'a>(
    bindings: &'a [AppBinding],
    modifiers: Modifiers,
    usage: u8,
) -> Option<&'a AppBinding> {
    bindings
        .iter()
        .find(|b| b.usage == usage && b.modifiers == modifiers)
}

/// First free (usage, modifiers) pair: bare F13–F24 first, then each
/// modifier combination. 12 × 16 = 192 slots.
pub fn alloc_marker(existing: &[AppBinding]) -> Option<(u8, Modifiers)> {
    for bits in 0u8..16 {
        let mods = Modifiers::from_bits_truncate(bits);
        for usage in MARKER_MIN..=MARKER_MAX {
            if find(existing, mods, usage).is_none() {
                return Some((usage, mods));
            }
        }
    }
    None
}

pub fn bindings_path() -> Result<PathBuf, ProfileError> {
    let dirs = directories::ProjectDirs::from("br", "Quantmind", "MiniKeyboard")
        .ok_or_else(|| ProfileError::Path("cannot determine config directory".into()))?;
    let dir = dirs.config_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("bindings.json"))
}

pub fn load_bindings() -> Result<Vec<AppBinding>, ProfileError> {
    load_from(&bindings_path()?)
}

pub fn save_bindings(bindings: &[AppBinding]) -> Result<(), ProfileError> {
    save_to(&bindings_path()?, bindings)
}

fn load_from(path: &std::path::Path) -> Result<Vec<AppBinding>, ProfileError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

fn save_to(path: &std::path::Path, bindings: &[AppBinding]) -> Result<(), ProfileError> {
    std::fs::write(path, serde_json::to_string_pretty(bindings)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bindings.json");
        assert!(load_from(&path).unwrap().is_empty());
        let list = vec![AppBinding {
            usage: MARKER_MIN,
            modifiers: Modifiers::empty(),
            name: "Term".into(),
            command: "kitty".into(),
        }];
        save_to(&path, &list).unwrap();
        assert_eq!(load_from(&path).unwrap(), list);
    }

    #[test]
    fn marker_allocation() {
        let mut list = Vec::new();
        // Bare usages go first.
        let (u, m) = alloc_marker(&list).unwrap();
        assert_eq!((u, m), (MARKER_MIN, Modifiers::empty()));
        for usage in MARKER_MIN..=MARKER_MAX {
            list.push(AppBinding {
                usage,
                modifiers: Modifiers::empty(),
                name: String::new(),
                command: String::new(),
            });
        }
        let (u, m) = alloc_marker(&list).unwrap();
        assert_eq!(u, MARKER_MIN);
        assert_ne!(m, Modifiers::empty());
    }

    #[test]
    fn marker_range() {
        assert!(is_marker(0x68));
        assert!(is_marker(0x73));
        assert!(!is_marker(0x45)); // F12
    }
}
