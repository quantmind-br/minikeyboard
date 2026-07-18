//! Configuration read (`0xFA`) for the captured 1189:8842 protocol.

use std::collections::BTreeMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::device::transport::HidTransport;
use crate::domain::config::{
    ConfigProvenance, DeviceConfig, LayerConfig, PositionConfig, ProfileIdentity, RECORD_SIZE,
    SCHEMA_VERSION,
};
use crate::protocol::codec;
use crate::protocol::frame::{read_config_report, REPORT_ID};
use crate::protocol::ProtocolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadConfigCommand {
    /// Number of complete read passes. The legacy application uses three layer buffers.
    pub layer_count: u8,
    /// Number of ordinary positions read at the start of every pass.
    pub base_position_count: u8,
    /// Number of three-position extra groups appended after the ordinary positions.
    pub extra_group_count: u8,
}

/// Captured legacy invocation: `read_Hidkey_Data(3, 15, 3)`.
///
/// The wire bytes are `03 FA 0F 03 <pass>`. Decompilation proves the final byte is the
/// one-based pass counter, incremented after every successful 24-response pass. Hardware
/// capture proves the three passes are stable and contain distinct payloads.
pub const DEFAULT_READ: ReadConfigCommand = ReadConfigCommand {
    layer_count: 3,
    base_position_count: 15,
    extra_group_count: 3,
};

const READ_TIMEOUT: Duration = Duration::from_millis(100);
const VALIDATED_VID: u16 = 0x1189;
const VALIDATED_PID: u16 = 0x8842;
const CAPTURE_VECTOR_SHA256: &str =
    "fe58641573bb8c843b8088879453d432a5e8a6e8db5b88563763a6c35a20c31a";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReadCapability {
    #[default]
    Unverified,
    Verified { vector_set_sha256: String },
}

/// Read capability is scoped to the captured 1189:8842 tuple.
pub fn current_read_capability() -> ReadCapability {
    ReadCapability::Verified {
        vector_set_sha256: CAPTURE_VECTOR_SHA256.into(),
    }
}

/// Read all three captured passes from the validated device.
///
/// Each pass yields `base_position_count + 3 * extra_group_count` responses. Captured passes
/// contain distinct payloads and map to the legacy application's three 3000-byte layer buffers.
/// Only the 49 bytes copied by the legacy routine enter each preserved record.
pub fn read_device_config(
    transport: &mut dyn HidTransport,
    identity: &ProfileIdentity,
) -> Result<DeviceConfig, ProtocolError> {
    if identity.vid != VALIDATED_VID || identity.pid != VALIDATED_PID {
        return Err(ProtocolError::UnverifiedReadProtocol);
    }

    transport.drain_pending()?;

    let expected_records = expected_records(identity);
    let mut layers = Vec::with_capacity(DEFAULT_READ.layer_count as usize);
    let mut vector_hasher = Sha256::new();

    for pass in 1..=DEFAULT_READ.layer_count {
        transport.write_report(&read_config_report(
            DEFAULT_READ.base_position_count,
            DEFAULT_READ.extra_group_count,
            pass,
        ))?;

        let mut records = BTreeMap::<u8, [u8; RECORD_SIZE]>::new();
        for _ in 0..expected_records {
            let response = transport.read_report(READ_TIMEOUT)?;
            vector_hasher.update(&response);
            let (position, response_pass, raw) = parse_response(&response, expected_records)?;
            if response_pass != pass {
                return Err(ProtocolError::InvalidResponse(format!(
                    "unexpected pass {response_pass}; expected {pass}"
                )));
            }
            if records.insert(position, raw).is_some() {
                return Err(ProtocolError::InvalidResponse(format!(
                    "duplicate pass {pass} position {position} after {} unique responses; seen={:?}",
                    records.len(),
                    records.keys().copied().collect::<Vec<_>>()
                )));
            }
        }

        let positions = records
            .into_iter()
            .map(|(one_based, raw_record)| {
                let decoded = codec::decode_record(raw_record);
                PositionConfig {
                    logical_index: one_based - 1,
                    action: decoded.action,
                    delay_ms: decoded.delay_ms,
                    raw_record,
                    dirty: false,
                }
            })
            .collect();
        layers.push(LayerConfig { positions });
    }

    let capture_sha256 = hex::encode(vector_hasher.finalize());
    let layers: [LayerConfig; 3] = layers
        .try_into()
        .map_err(|_| ProtocolError::InvalidResponse("expected exactly three read passes".into()))?;

    Ok(DeviceConfig {
        schema_version: SCHEMA_VERSION,
        identity: identity.clone(),
        layers,
        opaque_device_data: Vec::new(),
        provenance: ConfigProvenance::VerifiedDeviceRead { capture_sha256 },
    })
}

fn expected_records(_identity: &ProfileIdentity) -> u8 {
    // Wire response count is defined by the command, not by the identify tuple. Extra controls
    // occupy three records each (press, counter-clockwise, clockwise), so `(15, 3)` yields 24.
    DEFAULT_READ.base_position_count + DEFAULT_READ.extra_group_count * 3
}

fn parse_response(
    response: &[u8],
    expected_records: u8,
) -> Result<(u8, u8, [u8; RECORD_SIZE]), ProtocolError> {
    if response.len() < 51 {
        return Err(ProtocolError::InvalidResponse(format!(
            "short read response: expected at least 51 bytes, got {}",
            response.len()
        )));
    }
    if response[0] != REPORT_ID || response[1] != 0xFA {
        return Err(ProtocolError::InvalidResponse(format!(
            "unexpected read header {:02X} {:02X}",
            response[0], response[1]
        )));
    }

    let position = response[2];
    let pass = response[3];
    if position == 0 || position > expected_records {
        return Err(ProtocolError::InvalidResponse(format!(
            "position {position} outside 1..={expected_records}"
        )));
    }
    if pass == 0 || pass > DEFAULT_READ.layer_count {
        return Err(ProtocolError::InvalidResponse(format!(
            "pass {pass} outside 1..={}",
            DEFAULT_READ.layer_count
        )));
    }

    let mut raw = [0u8; RECORD_SIZE];
    // Legacy read_Hidkey_Data copies auStack_5b[0..49] into record offsets 1..49. Because
    // auStack_5b starts two bytes after local_5d, these are response bytes 2..50: position,
    // pass, and 47 payload bytes. Record byte 0 is not returned by 0xFA. Ordinary position
    // records require the observed 0xFD write marker; RGB records expose their own 0xFE marker.
    raw[1..].copy_from_slice(&response[2..51]);
    raw[0] = if raw[3] == 5 { 0xFE } else { 0xFD };
    Ok((position, pass, raw))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mock::{MockStep, MockTransport};
    use crate::protocol::frame::read_config_report;

    fn identity() -> ProfileIdentity {
        ProfileIdentity {
            vid: VALIDATED_VID,
            pid: VALIDATED_PID,
            serial: None,
            key_count: 15,
            extra_count: 3,
            subtype: 0,
        }
    }

    fn response(position: u8, pass: u8, usage: u8) -> Vec<u8> {
        let mut response = vec![0u8; 64];
        response[..4].copy_from_slice(&[REPORT_ID, 0xFA, position, pass]);
        response[4] = 1;
        response[10] = 1;
        response[11] = 0x08;
        response[12] = usage;
        response
    }

    fn scripted_read() -> MockTransport {
        let mut steps = Vec::new();
        for pass in 1..=3 {
            steps.push(MockStep::ExpectWrite(read_config_report(15, 3, pass)));
            for position in 1..=24 {
                steps.push(MockStep::Read(response(position, pass, 0x04 + position)));
            }
        }
        MockTransport::new(steps)
    }

    #[test]
    fn parses_three_passes_and_preserves_legacy_records() {
        let mut transport = scripted_read();
        let config = read_device_config(&mut transport, &identity()).unwrap();
        assert_eq!(
            config
                .layers
                .iter()
                .map(|layer| layer.positions.len())
                .collect::<Vec<_>>(),
            [24, 24, 24]
        );
        assert_eq!(config.layers[0].positions[0].logical_index, 0);
        assert_eq!(&config.layers[0].positions[0].raw_record[..4], &[0xFD, 1, 1, 1]);
        assert_eq!(transport.remaining(), 0);
        assert!(matches!(
            config.provenance,
            ConfigProvenance::VerifiedDeviceRead { .. }
        ));
    }

    #[test]
    fn reconstructs_rgb_write_marker() {
        let mut raw_response = response(1, 1, 0x04);
        raw_response[4] = 5;
        raw_response[11] = 1;
        raw_response[13] = 0x23;
        let (_, _, raw) = parse_response(&raw_response, 24).unwrap();
        assert_eq!(raw[0], 0xFE);
        assert_eq!(raw[3], 5);
    }

    #[test]
    fn accepts_out_of_order_responses_but_sorts_positions() {
        let mut steps = vec![MockStep::ExpectWrite(read_config_report(15, 3, 1))];
        for position in (1..=24).rev() {
            steps.push(MockStep::Read(response(position, 1, position)));
        }
        for pass in 2..=3 {
            steps.push(MockStep::ExpectWrite(read_config_report(15, 3, pass)));
            for position in 1..=24 {
                steps.push(MockStep::Read(response(position, pass, position)));
            }
        }
        let mut transport = MockTransport::new(steps);
        let config = read_device_config(&mut transport, &identity()).unwrap();
        assert_eq!(config.layers[0].positions[0].logical_index, 0);
        assert_eq!(config.layers[0].positions[23].logical_index, 23);
    }

    #[test]
    fn rejects_duplicate_position() {
        let mut steps = vec![MockStep::ExpectWrite(read_config_report(15, 3, 1))];
        for _ in 0..24 {
            steps.push(MockStep::Read(response(1, 1, 0x04)));
        }
        let mut transport = MockTransport::new(steps);
        let error = read_device_config(&mut transport, &identity()).unwrap_err();
        assert!(
            matches!(error, ProtocolError::InvalidResponse(message) if message.contains("duplicate"))
        );
    }

    #[test]
    fn rejects_other_vid_pid_before_io() {
        let mut transport = MockTransport::new(vec![]);
        let mut id = identity();
        id.pid = 0x8840;
        let error = read_device_config(&mut transport, &id).unwrap_err();
        assert!(matches!(error, ProtocolError::UnverifiedReadProtocol));
        assert!(transport.writes().is_empty());
    }
}
