//! Exact 65-byte HID report constructors.

use std::time::Duration;

pub const REPORT_ID: u8 = 0x03;
pub const REPORT_LEN: usize = 65;
pub const INPUT_LEN: usize = 64;
pub const RECORD_PAYLOAD: usize = 50;
pub const IDENTIFY_TIMEOUT: Duration = Duration::from_millis(10);
pub const POST_COMMIT_DELAY: Duration = Duration::from_millis(200);

fn blank() -> [u8; REPORT_LEN] {
    [0u8; REPORT_LEN]
}

/// Identify: `03 FB FB FB` + zeroes.
pub fn identify_report() -> [u8; REPORT_LEN] {
    let mut r = blank();
    r[0] = REPORT_ID;
    r[1] = 0xFB;
    r[2] = 0xFB;
    r[3] = 0xFB;
    r
}

/// Read config header: `03 FA <layer_or_count> <range> <block_index>` + zeroes.
pub fn read_config_report(layer_or_count: u8, range: u8, block_index: u8) -> [u8; REPORT_LEN] {
    let mut r = blank();
    r[0] = REPORT_ID;
    r[1] = 0xFA;
    r[2] = layer_or_count;
    r[3] = range;
    r[4] = block_index;
    r
}

/// Position write: Report ID + 50-byte record + zeroes.
pub fn position_report(record: &[u8; RECORD_PAYLOAD]) -> [u8; REPORT_LEN] {
    let mut r = blank();
    r[0] = REPORT_ID;
    r[1..1 + RECORD_PAYLOAD].copy_from_slice(record);
    r
}

/// Commit: `03 FD FE FF` + zeroes.
pub fn commit_report() -> [u8; REPORT_LEN] {
    let mut r = blank();
    r[0] = REPORT_ID;
    r[1] = 0xFD;
    r[2] = 0xFE;
    r[3] = 0xFF;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_bytes() {
        let r = identify_report();
        assert_eq!(r.len(), 65);
        assert_eq!(&r[..4], &[0x03, 0xFB, 0xFB, 0xFB]);
        assert!(r[4..].iter().all(|&b| b == 0));
    }

    #[test]
    fn read_config_bytes() {
        let r = read_config_report(3, 15, 3);
        assert_eq!(&r[..5], &[0x03, 0xFA, 3, 15, 3]);
        assert!(r[5..].iter().all(|&b| b == 0));
    }

    #[test]
    fn commit_bytes() {
        let r = commit_report();
        assert_eq!(&r[..4], &[0x03, 0xFD, 0xFE, 0xFF]);
    }

    #[test]
    fn position_embeds_record() {
        let mut rec = [0u8; 50];
        rec[0] = 0xAA;
        rec[49] = 0xBB;
        let r = position_report(&rec);
        assert_eq!(r[0], 0x03);
        assert_eq!(r[1], 0xAA);
        assert_eq!(r[50], 0xBB);
        assert!(r[51..].iter().all(|&b| b == 0));
    }
}
