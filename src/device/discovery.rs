//! Safe device discovery with compiled allowlist.

use serde::{Deserialize, Serialize};

use super::DeviceError;
use crate::domain::config::SupportLevel;

pub const VALIDATED_VID: u16 = 0x1189;
pub const VALIDATED_PID: u16 = 0x8842;

/// Vendor-defined usage page for the configuration interface.
pub const CONFIG_USAGE_PAGE: u16 = 0xFF00;

/// Required interface number for configuration.
pub const CONFIG_INTERFACE: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub vid: u16,
    pub pid: u16,
    pub serial: Option<String>,
    pub path: String,
    pub interface_number: i32,
    pub usage_page: u16,
    pub usage: u16,
    pub support: SupportLevel,
    pub product: Option<String>,
}

impl DeviceIdentity {
    pub fn label(&self) -> String {
        let base = format!("{:04x}:{:04x}", self.vid, self.pid);
        match &self.serial {
            Some(s) if !s.is_empty() => format!("{base} ({s})"),
            _ => base,
        }
    }

    pub fn is_validated(&self) -> bool {
        self.support == SupportLevel::Validated
    }
}

#[derive(Debug, Clone, Copy)]
struct AllowEntry {
    vid: u16,
    pid: u16,
    support: SupportLevel,
}

const ALLOWLIST: &[AllowEntry] = &[
    AllowEntry {
        vid: 0x1189,
        pid: 0x8842,
        support: SupportLevel::Validated,
    },
    // SPEC §3.3 alternatives — experimental / read-only.
    AllowEntry {
        vid: 0x1189,
        pid: 0x8840,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8830,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8831,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8832,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8833,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8850,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x1189,
        pid: 0x8851,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8842,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8840,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8830,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8831,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8832,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8833,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8850,
        support: SupportLevel::Experimental,
    },
    AllowEntry {
        vid: 0x514C,
        pid: 0x8851,
        support: SupportLevel::Experimental,
    },
];

fn allow_entry(vid: u16, pid: u16) -> Option<&'static AllowEntry> {
    ALLOWLIST.iter().find(|e| e.vid == vid && e.pid == pid)
}

pub fn experimental_enabled() -> bool {
    matches!(
        std::env::var("MINIKEYBOARD_EXPERIMENTAL_DEVICES").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes")
    )
}

/// Discover allowed configuration interfaces (interface 0, usage page 0xFF00).
pub fn discover(api: &hidapi::HidApi) -> Result<Vec<DeviceIdentity>, DeviceError> {
    let experimental = experimental_enabled();
    let mut out = Vec::new();

    for info in api.device_list() {
        let vid = info.vendor_id();
        let pid = info.product_id();
        let Some(entry) = allow_entry(vid, pid) else {
            continue;
        };
        if entry.support == SupportLevel::Experimental && !experimental {
            continue;
        }

        let interface_number = info.interface_number();
        let usage_page = info.usage_page();
        let usage = info.usage();

        // Retain only configuration interface.
        if interface_number != CONFIG_INTERFACE {
            continue;
        }
        // linux-native should report the usage page; if 0, defer to the
        // descriptor check on open.
        if usage_page != 0 && usage_page != CONFIG_USAGE_PAGE {
            continue;
        }

        let path = info.path().to_string_lossy().into_owned();
        let serial = info.serial_number().map(str::to_owned);
        let product = info.product_string().map(str::to_owned);

        out.push(DeviceIdentity {
            vid,
            pid,
            serial,
            path,
            interface_number,
            usage_page: if usage_page == 0 {
                CONFIG_USAGE_PAGE
            } else {
                usage_page
            },
            usage,
            support: entry.support,
            product,
        });
    }

    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

/// Validate opened device report descriptor.
/// Must contain vendor Usage Page `06 00 FF`, Report ID `85 03`, and 64-byte counts.
pub fn validate_descriptor(desc: &[u8]) -> Result<(), DeviceError> {
    let has_usage_page = find_bytes(desc, &[0x06, 0x00, 0xFF]);
    let has_report_id = find_bytes(desc, &[0x85, 0x03]);
    // Report Count 64 = 0x95 0x40
    let has_count_64 = find_bytes(desc, &[0x95, 0x40]);

    if has_usage_page && has_report_id && has_count_64 {
        Ok(())
    } else {
        Err(DeviceError::DescriptorMismatch)
    }
}

fn find_bytes(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mock::VALID_DESCRIPTOR;

    #[test]
    fn valid_descriptor_passes() {
        validate_descriptor(VALID_DESCRIPTOR).unwrap();
    }

    #[test]
    fn missing_report_id_fails() {
        let desc = [0x06, 0x00, 0xFF, 0x95, 0x40];
        assert!(matches!(
            validate_descriptor(&desc),
            Err(DeviceError::DescriptorMismatch)
        ));
    }

    #[test]
    fn allowlist_contains_validated() {
        let e = allow_entry(VALIDATED_VID, VALIDATED_PID).unwrap();
        assert_eq!(e.support, SupportLevel::Validated);
    }
}
