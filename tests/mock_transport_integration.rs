//! Integration tests using MockTransport for identify / write ordering.

use std::collections::BTreeSet;

use minikeyboard::device::mock::{MockStep, MockTransport};
use minikeyboard::device::transport::{HidTransport, TransportError};
use minikeyboard::domain::action::Action;
use minikeyboard::domain::config::{
    ConfigProvenance, DeviceConfig, LayerConfig, PositionConfig, ProfileIdentity, SupportLevel,
};
use minikeyboard::protocol::frame::{commit_report, identify_report, position_report};
use minikeyboard::protocol::identify::identify;
use minikeyboard::protocol::write_config::{write_dirty_records, WriteAuthorization};
use minikeyboard::protocol::{ProtocolError, ReadCapability};

fn sample_dirty_config() -> DeviceConfig {
    let mut pos = PositionConfig::empty(0);
    pos.dirty = true;
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
                positions: vec![pos],
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
            capture_sha256: "test".into(),
        },
    }
}

fn auth_verified() -> WriteAuthorization {
    WriteAuthorization {
        device_support: SupportLevel::Validated,
        read_capability: ReadCapability::Verified {
            vector_set_sha256: "test".into(),
        },
        provenance: ConfigProvenance::VerifiedDeviceRead {
            capture_sha256: "test".into(),
        },
        verified_actions: BTreeSet::from([minikeyboard::domain::action::ActionKind::Empty]),
    }
}

#[test]
fn identify_valid() {
    let mut t = MockTransport::new(vec![
        MockStep::ExpectWrite(identify_report()),
        MockStep::Read(vec![0x03, 0x00, 6, 2, 0]),
    ]);
    let v = identify(&mut t).unwrap();
    assert_eq!((v.key_count, v.extra_count, v.subtype), (6, 2, 0));
}

#[test]
fn identify_timeout() {
    let mut t = MockTransport::new(vec![
        MockStep::ExpectWrite(identify_report()),
        MockStep::FailRead(TransportError::Timeout),
    ]);
    let err = identify(&mut t).unwrap_err();
    assert!(matches!(err, ProtocolError::Transport(TransportError::Timeout)));
}

#[test]
fn identify_disconnect() {
    let mut t = MockTransport::new(vec![
        MockStep::ExpectWrite(identify_report()),
        MockStep::FailRead(TransportError::Disconnected),
    ]);
    assert!(matches!(
        identify(&mut t),
        Err(ProtocolError::Transport(TransportError::Disconnected))
    ));
}

#[test]
fn short_write_on_identify() {
    let mut t = MockTransport::new(vec![MockStep::FailWrite(TransportError::ShortWrite {
        expected: 65,
        got: 10,
    })]);
    assert!(matches!(
        identify(&mut t),
        Err(ProtocolError::Transport(TransportError::ShortWrite { .. }))
    ));
}

#[test]
fn pre_commit_failure_no_commit() {
    let config = sample_dirty_config();
    let auth = auth_verified();
    let mut t = MockTransport::new(vec![MockStep::FailWrite(TransportError::Disconnected)]);
    let _ = write_dirty_records(&mut t, &config, &auth);
    assert!(
        !t.writes().iter().any(|w| w == &commit_report()),
        "commit must not be sent after pre-commit failure"
    );
}

#[test]
fn exactly_one_commit_before_indeterminate_readback() {
    let config = sample_dirty_config();
    let auth = auth_verified();
    let rec = config.layers[0].positions[0].raw_record;
    let mut t = MockTransport::new(vec![
        MockStep::ExpectWrite(position_report(&rec)),
        MockStep::ExpectWrite(commit_report()),
    ]);
    let err = write_dirty_records(&mut t, &config, &auth).unwrap_err();
    assert!(matches!(
        err,
        minikeyboard::protocol::WriteError::IndeterminateCommit
    ));
    let commits = t
        .writes()
        .iter()
        .filter(|w| w[0] == 0x03 && w[1] == 0xFD)
        .count();
    assert_eq!(commits, 1);
}

#[test]
fn post_commit_timeout_is_indeterminate() {
    let config = sample_dirty_config();
    let rec = config.layers[0].positions[0].raw_record;
    let auth = auth_verified();
    let mut t = MockTransport::new(vec![
        MockStep::ExpectWrite(position_report(&rec)),
        MockStep::FailWrite(TransportError::Timeout),
    ]);
    let err = write_dirty_records(&mut t, &config, &auth).unwrap_err();
    assert!(matches!(
        err,
        minikeyboard::protocol::WriteError::IndeterminateCommit
    ));
}

#[test]
fn descriptor_accessible() {
    let mut t = MockTransport::new(vec![]);
    let d = t.descriptor().unwrap();
    assert!(d.windows(3).any(|w| w == [0x06, 0x00, 0xFF]));
    assert!(d.windows(2).any(|w| w == [0x85, 0x03]));
}
