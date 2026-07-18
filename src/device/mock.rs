//! Scripted HID transport for deterministic tests and UI mock scenarios.

use std::collections::VecDeque;
use std::time::Duration;

use super::transport::{HidTransport, TransportError};
use crate::protocol::frame::REPORT_LEN;

#[derive(Debug, Clone)]
pub enum MockStep {
    ExpectWrite([u8; REPORT_LEN]),
    /// Accept any write and record it.
    AcceptWrite,
    Read(Vec<u8>),
    FailWrite(TransportError),
    FailRead(TransportError),
    Descriptor(Vec<u8>),
    FailDescriptor(TransportError),
}


pub struct MockTransport {
    steps: VecDeque<MockStep>,
    writes: Vec<[u8; REPORT_LEN]>,
    default_descriptor: Vec<u8>,
}

impl MockTransport {
    pub fn new(steps: Vec<MockStep>) -> Self {
        Self {
            steps: steps.into(),
            writes: Vec::new(),
            default_descriptor: VALID_DESCRIPTOR.to_vec(),
        }
    }

    pub fn writes(&self) -> &[[u8; REPORT_LEN]] {
        &self.writes
    }

    pub fn remaining(&self) -> usize {
        self.steps.len()
    }
}

/// Minimal vendor-defined descriptor fragment: Usage Page FF00, Report ID 03, 64-byte counts.
pub const VALID_DESCRIPTOR: &[u8] = &[
    0x06, 0x00, 0xFF, // Usage Page (Vendor 0xFF00)
    0x09, 0x01, // Usage
    0xA1, 0x01, // Collection
    0x85, 0x03, // Report ID 0x03
    0x09, 0x02, //
    0x15, 0x00, //
    0x26, 0x00, 0xFF, //
    0x75, 0x08, // Report Size 8
    0x95, 0x40, // Report Count 64
    0x81, 0x06, // Input
    0x09, 0x02, //
    0x15, 0x00, //
    0x26, 0x00, 0xFF, //
    0x75, 0x08, //
    0x95, 0x40, // Report Count 64
    0x91, 0x06, // Output
    0xC0, // End Collection
];

impl HidTransport for MockTransport {
    fn write_report(&mut self, report: &[u8; REPORT_LEN]) -> Result<(), TransportError> {
        let step = self
            .steps
            .pop_front()
            .ok_or_else(|| TransportError::Io("unexpected write: script exhausted".into()))?;
        match step {
            MockStep::ExpectWrite(expected) => {
                if &expected != report {
                    let diff = first_diff(&expected, report);
                    return Err(TransportError::Io(format!(
                        "write mismatch at index {diff}: expected {:02X?}… got {:02X?}…",
                        &expected[..8.min(expected.len())],
                        &report[..8.min(report.len())]
                    )));
                }
                self.writes.push(*report);
                Ok(())
            }
            MockStep::AcceptWrite => {
                self.writes.push(*report);
                Ok(())
            }
            MockStep::FailWrite(e) => Err(e),
            other => {
                self.steps.push_front(other);
                Err(TransportError::Io(
                    "script expected non-write step".into(),
                ))
            }
        }
    }

    fn read_report(&mut self, _timeout: Duration) -> Result<Vec<u8>, TransportError> {
        let step = self
            .steps
            .pop_front()
            .ok_or_else(|| TransportError::Io("unexpected read: script exhausted".into()))?;
        match step {
            MockStep::Read(data) => Ok(data),
            MockStep::FailRead(e) => Err(e),
            other => {
                self.steps.push_front(other);
                Err(TransportError::Io("script expected non-read step".into()))
            }
        }
    }

    fn descriptor(&mut self) -> Result<Vec<u8>, TransportError> {
        if let Some(front) = self.steps.front() {
            match front {
                MockStep::Descriptor(_) | MockStep::FailDescriptor(_) => {
                    let step = self.steps.pop_front().unwrap();
                    match step {
                        MockStep::Descriptor(d) => Ok(d),
                        MockStep::FailDescriptor(e) => Err(e),
                        _ => unreachable!(),
                    }
                }
                _ => Ok(self.default_descriptor.clone()),
            }
        } else {
            Ok(self.default_descriptor.clone())
        }
    }
}

fn first_diff(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .position(|(x, y)| x != y)
        .unwrap_or(a.len().min(b.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::frame::identify_report;

    #[test]
    fn unexpected_bytes_fail_with_index() {
        let mut t = MockTransport::new(vec![MockStep::ExpectWrite(identify_report())]);
        let mut bad = identify_report();
        bad[1] = 0x00;
        let err = t.write_report(&bad).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("index"), "{msg}");
    }
}
