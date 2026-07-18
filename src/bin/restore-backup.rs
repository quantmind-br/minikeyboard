//! Emergency one-record restore using an explicit pre-roundtrip backup.

use std::thread;

use minikeyboard::device::discovery::discover;
use minikeyboard::device::transport::{HidTransport, RealHidTransport};
use minikeyboard::profile::json;
use minikeyboard::protocol::frame::{commit_report, position_report, POST_COMMIT_DELAY};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: restore-backup <backup.json> [layer] [position]")?;
    let layer: usize = std::env::args()
        .nth(2)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(1);
    let position: u8 = std::env::args()
        .nth(3)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(24);
    if !(1..=3).contains(&layer) || position == 0 {
        return Err("layer must be 1..=3 and position must be one-based".into());
    }

    let backup = json::import_profile(std::path::Path::new(&path))?;
    let record = backup.layers[layer - 1]
        .positions
        .iter()
        .find(|candidate| candidate.logical_index + 1 == position)
        .ok_or("position absent from backup")?
        .raw_record;

    let api = hidapi::HidApi::new()?;
    let device = discover(&api)?
        .into_iter()
        .find(|candidate| {
            candidate.vid == 0x1189
                && candidate.pid == 0x8842
                && candidate.interface_number == 0
        })
        .ok_or("1189:8842 interface 0 not found")?;
    eprintln!("opening {}", device.path);
    let hid = api.open_path(std::ffi::CString::new(device.path.as_str())?.as_c_str())?;
    let mut transport = RealHidTransport::new(hid);
    let drained = transport.drain_pending()?;
    eprintln!("drained {drained} pending reports");
    transport.write_report(&position_report(&record))?;
    transport.write_report(&commit_report())?;
    thread::sleep(POST_COMMIT_DELAY);
    println!(
        "RESTORED layer={layer} position={position} raw={}",
        hex::encode(record)
    );
    Ok(())
}
