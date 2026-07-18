//! Read-only dump of the connected device's configuration (diagnostic).

use minikeyboard::device::discovery::discover;
use minikeyboard::device::transport::RealHidTransport;
use minikeyboard::domain::config::ProfileIdentity;
use minikeyboard::protocol::{identify, read_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = hidapi::HidApi::new()?;
    let device = discover(&api)?
        .into_iter()
        .find(|c| c.vid == 0x1189 && c.pid == 0x8842 && c.interface_number == 0)
        .ok_or("1189:8842 interface 0 not found")?;
    let hid = api.open_path(std::ffi::CString::new(device.path.as_str())?.as_c_str())?;
    let mut transport = RealHidTransport::new(hid);

    let variant = identify::identify(&mut transport)?;
    eprintln!("variant: {} ({} keys + {} extras)", variant.geometry_id, variant.key_count, variant.extra_count);
    let identity = ProfileIdentity {
        vid: device.vid,
        pid: device.pid,
        serial: device.serial,
        key_count: variant.key_count,
        extra_count: variant.extra_count,
        subtype: variant.subtype,
    };
    let config = read_config::read_device_config(&mut transport, &identity)?;
    for (li, layer) in config.layers.iter().enumerate() {
        println!("== layer {}", li + 1);
        for pos in &layer.positions {
            println!(
                "  pos {:2} (wire {:2}): {:<24} raw[0..24]={}",
                pos.logical_index,
                pos.logical_index + 1,
                pos.action.short_label(),
                hex::encode(&pos.raw_record[..24])
            );
        }
    }
    Ok(())
}
