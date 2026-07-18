//! Authorized dirty-record write with single commit and readback.

use std::collections::BTreeSet;
use std::thread;

use thiserror::Error;

use crate::device::transport::HidTransport;
use crate::domain::action::ActionKind;
use crate::domain::config::{ConfigProvenance, DeviceConfig, SupportLevel};
use crate::protocol::frame::{commit_report, position_report, POST_COMMIT_DELAY};
use crate::protocol::read_config::{read_device_config, ReadCapability};
use crate::protocol::ProtocolError;

#[derive(Debug, Clone)]
pub struct WriteAuthorization {
    pub device_support: SupportLevel,
    pub read_capability: ReadCapability,
    pub provenance: ConfigProvenance,
    pub verified_actions: BTreeSet<ActionKind>,
}

#[derive(Debug, Error)]
pub enum WriteError {
    #[error("write not authorized: {0}")]
    NotAuthorized(String),

    #[error("transport: {0}")]
    Transport(#[from] crate::device::transport::TransportError),

    #[error("protocol: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("commit outcome indeterminate — reconnect and read back")]
    IndeterminateCommit,

    #[error("readback unavailable after commit — Write remains disabled")]
    VerificationUnavailable,

    #[error("no dirty records to write")]
    NothingDirty,
}

impl WriteError {
    pub fn category(&self) -> &'static str {
        match self {
            Self::NotAuthorized(_) => "not_authorized",
            Self::Transport(e) => e.category(),
            Self::Protocol(e) => e.category(),
            Self::IndeterminateCommit => "indeterminate_commit",
            Self::VerificationUnavailable => "verification_unavailable",
            Self::NothingDirty => "nothing_dirty",
        }
    }
}

fn authorize(config: &DeviceConfig, auth: &WriteAuthorization) -> Result<(), WriteError> {
    if auth.device_support != SupportLevel::Validated {
        return Err(WriteError::NotAuthorized(
            "device support is not Validated".into(),
        ));
    }
    if !matches!(auth.read_capability, ReadCapability::Verified { .. }) {
        return Err(WriteError::NotAuthorized(
            "read protocol has not been verified".into(),
        ));
    }
    match &auth.provenance {
        ConfigProvenance::VerifiedDeviceRead { .. } | ConfigProvenance::ImportedProfile => {}
        ConfigProvenance::Mock => {
            return Err(WriteError::NotAuthorized(
                "mock provenance is not writable".into(),
            ));
        }
    }
    match &config.provenance {
        ConfigProvenance::Mock => {
            return Err(WriteError::NotAuthorized(
                "mock configuration is not writable".into(),
            ));
        }
        ConfigProvenance::ImportedProfile | ConfigProvenance::VerifiedDeviceRead { .. } => {}
    }
    for kind in config.dirty_action_kinds() {
        if !auth.verified_actions.contains(&kind) {
            return Err(WriteError::NotAuthorized(format!(
                "action kind {kind:?} encoder not verified"
            )));
        }
    }
    Ok(())
}

/// Sort dirty by layer then logical index, send only those frames, one commit, sleep, readback.
pub fn write_dirty_records(
    transport: &mut dyn HidTransport,
    config: &DeviceConfig,
    authorization: &WriteAuthorization,
) -> Result<DeviceConfig, WriteError> {
    authorize(config, authorization)?;

    let mut dirty: Vec<(usize, u8, [u8; 50])> = Vec::new();
    for (layer_idx, layer) in config.layers.iter().enumerate() {
        for pos in &layer.positions {
            if pos.dirty {
                dirty.push((layer_idx, pos.logical_index, pos.raw_record));
            }
        }
    }
    if dirty.is_empty() {
        return Err(WriteError::NothingDirty);
    }
    dirty.sort_by_key(|(l, i, _)| (*l, *i));

    for (_, _, record) in &dirty {
        transport.write_report(&position_report(record))?;
    }

    match transport.write_report(&commit_report()) {
        Ok(()) => {}
        Err(e) => {
            if e.is_disconnect_or_timeout() {
                return Err(WriteError::IndeterminateCommit);
            }
            return Err(WriteError::Transport(e));
        }
    }

    thread::sleep(POST_COMMIT_DELAY);

    match read_device_config(transport, &config.identity) {
        Ok(mut fresh) => {
            let verified = dirty.iter().all(|(layer_index, logical_index, expected)| {
                fresh
                    .layers
                    .get(*layer_index)
                    .and_then(|layer| {
                        layer
                            .positions
                            .iter()
                            .find(|position| position.logical_index == *logical_index)
                    })
                    .is_some_and(|position| records_match_readback(expected, &position.raw_record))
            });
            if !verified {
                return Err(WriteError::Protocol(ProtocolError::VerificationMismatch));
            }
            fresh.clear_dirty();
            Ok(fresh)
        }
        Err(_) => Err(WriteError::IndeterminateCommit),
    }
}

/// `0xFA` omits record byte 0, but the validated protocol reconstructs its deterministic
/// write marker (`0xFD`, or `0xFE` for RGB), enabling complete record verification.
fn records_match_readback(expected: &[u8; 50], actual: &[u8; 50]) -> bool {
    expected == actual
}
#[cfg(test)]
mod readback_tests {
    use super::records_match_readback;

    #[test]
    fn compares_complete_reconstructed_record() {
        let expected = [0xFD; 50];
        let mut actual = expected;
        assert!(records_match_readback(&expected, &actual));
        actual[0] = 0;
        assert!(!records_match_readback(&expected, &actual));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mock::{MockStep, MockTransport};
    use crate::domain::action::Action;
    use crate::domain::config::{LayerConfig, PositionConfig, ProfileIdentity};
    use crate::protocol::frame::{commit_report, position_report};

    fn sample_config(dirty: bool) -> DeviceConfig {
        let mut pos = PositionConfig::empty(0);
        pos.dirty = dirty;
        pos.action = Action::Empty;
        DeviceConfig {
            schema_version: 1,
            identity: ProfileIdentity {
                vid: 0x1189,
                pid: 0x8842,
                serial: None,
                key_count: 1,
                extra_count: 0,
                subtype: 0,
            },
            layers: [
                LayerConfig {
                    positions: vec![pos.clone()],
                },
                LayerConfig {
                    positions: vec![PositionConfig::empty(0)],
                },
                LayerConfig {
                    positions: vec![PositionConfig::empty(0)],
                },
            ],
            opaque_device_data: vec![],
            provenance: ConfigProvenance::VerifiedDeviceRead {
                capture_sha256: "abc".into(),
            },
        }
    }

    fn full_auth() -> WriteAuthorization {
        WriteAuthorization {
            device_support: SupportLevel::Validated,
            read_capability: ReadCapability::Verified {
                vector_set_sha256: "vec".into(),
            },
            provenance: ConfigProvenance::VerifiedDeviceRead {
                capture_sha256: "abc".into(),
            },
            verified_actions: BTreeSet::from([ActionKind::Empty]),
        }
    }

    #[test]
    fn rejects_unverified_read() {
        let config = sample_config(true);
        let mut auth = full_auth();
        auth.read_capability = ReadCapability::Unverified;
        let mut t = MockTransport::new(vec![]);
        let err = write_dirty_records(&mut t, &config, &auth).unwrap_err();
        assert!(matches!(err, WriteError::NotAuthorized(_)));
        assert_eq!(t.writes().len(), 0);
    }

    #[test]
    fn pre_commit_failure_skips_commit() {
        let config = sample_config(true);
        let auth = full_auth();
        let mut t = MockTransport::new(vec![MockStep::FailWrite(
            crate::device::transport::TransportError::ShortWrite {
                expected: 65,
                got: 10,
            },
        )]);
        let err = write_dirty_records(&mut t, &config, &auth).unwrap_err();
        assert!(matches!(err, WriteError::Transport(_)));
        assert!(!t.writes().iter().any(|w| w == &commit_report()));
    }

    #[test]
    fn one_commit_when_readback_fails_is_indeterminate() {
        let config = sample_config(true);
        let auth = full_auth();
        let rec = config.layers[0].positions[0].raw_record;
        let mut t = MockTransport::new(vec![
            MockStep::ExpectWrite(position_report(&rec)),
            MockStep::ExpectWrite(commit_report()),
        ]);
        let err = write_dirty_records(&mut t, &config, &auth).unwrap_err();
        assert!(matches!(err, WriteError::IndeterminateCommit));
        let commits = t
            .writes()
            .iter()
            .filter(|w| w[1] == 0xFD && w[2] == 0xFE && w[3] == 0xFF)
            .count();
        assert_eq!(commits, 1);
    }

    fn read_response(position: u8, pass: u8, usage: u8) -> Vec<u8> {
        let mut response = vec![0u8; 64];
        response[..4].copy_from_slice(&[crate::protocol::frame::REPORT_ID, 0xFA, position, pass]);
        response[4] = 1;
        response[10] = 1;
        response[11] = 0x08;
        response[12] = usage;
        response
    }

    #[test]
    fn successful_readback_with_different_record_is_verification_mismatch() {
        let config = sample_config(true);
        let auth = full_auth();
        let rec = config.layers[0].positions[0].raw_record;
        let mut steps = vec![
            MockStep::ExpectWrite(position_report(&rec)),
            MockStep::ExpectWrite(commit_report()),
        ];
        for pass in 1..=3 {
            steps.push(MockStep::ExpectWrite(crate::protocol::frame::read_config_report(
                15, 3, pass,
            )));
            for position in 1..=24 {
                steps.push(MockStep::Read(read_response(position, pass, 0x04 + position)));
            }
        }
        let mut transport = MockTransport::new(steps);
        let error = write_dirty_records(&mut transport, &config, &auth).unwrap_err();
        assert!(matches!(
            error,
            WriteError::Protocol(ProtocolError::VerificationMismatch)
        ));
    }
}
