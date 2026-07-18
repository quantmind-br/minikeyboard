//! Device identify: send `03 FB FB FB`, parse variant tuple.

use crate::device::transport::HidTransport;
use crate::domain::geometry::{resolve_variant, DeviceVariant};
use crate::protocol::frame::{identify_report, IDENTIFY_TIMEOUT, REPORT_ID};
use crate::protocol::ProtocolError;

/// Send one identify report, read once at 10 ms, map indices 2/3/4 to variant.
pub fn identify(transport: &mut dyn HidTransport) -> Result<DeviceVariant, ProtocolError> {
    transport.write_report(&identify_report())?;
    let response = transport.read_report(IDENTIFY_TIMEOUT)?;
    if response.len() < 5 {
        return Err(ProtocolError::ShortIdentifyResponse(response.len()));
    }
    if response[0] != REPORT_ID {
        return Err(ProtocolError::WrongReportId(response[0]));
    }
    let key_count = response[2];
    let extra_count = response[3];
    let subtype = response[4];
    Ok(resolve_variant(key_count, extra_count, subtype))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::mock::{MockStep, MockTransport};
    use crate::protocol::frame::identify_report;

    #[test]
    fn valid_identify() {
        let mut t = MockTransport::new(vec![
            MockStep::ExpectWrite(identify_report()),
            MockStep::Read(vec![0x03, 0x00, 6, 2, 0]),
        ]);
        let v = identify(&mut t).unwrap();
        assert_eq!(v.key_count, 6);
        assert_eq!(v.extra_count, 2);
        assert_eq!(v.subtype, 0);
        assert_eq!(v.geometry_id, "k6-e2-s0");
    }

    #[test]
    fn short_response() {
        let mut t = MockTransport::new(vec![
            MockStep::ExpectWrite(identify_report()),
            MockStep::Read(vec![0x03, 0x00]),
        ]);
        assert!(matches!(
            identify(&mut t),
            Err(ProtocolError::ShortIdentifyResponse(2))
        ));
    }

    #[test]
    fn wrong_report_id() {
        let mut t = MockTransport::new(vec![
            MockStep::ExpectWrite(identify_report()),
            MockStep::Read(vec![0x01, 0x00, 6, 2, 0]),
        ]);
        assert!(matches!(
            identify(&mut t),
            Err(ProtocolError::WrongReportId(0x01))
        ));
    }
}
