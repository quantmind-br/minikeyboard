//! Temporary raw response probe for diagnosing 0xFA ordering.

use std::time::Duration;

use minikeyboard::device::discovery::discover;
use minikeyboard::device::transport::{HidTransport, RealHidTransport};
use minikeyboard::protocol::frame::read_config_report;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = hidapi::HidApi::new()?;
    let device = discover(&api)?
        .into_iter()
        .find(|candidate| candidate.vid == 0x1189 && candidate.pid == 0x8842 && candidate.interface_number == 0)
        .ok_or("1189:8842 interface 0 not found")?;
    let hid = api.open_path(std::ffi::CString::new(device.path.as_str())?.as_c_str())?;
    let mut transport = RealHidTransport::new(hid);
    println!("drained={}", transport.drain_pending()?);
    transport.write_report(&read_config_report(15, 3, 1))?;
    for sequence in 1..=32 {
        match transport.read_report(Duration::from_millis(100)) {
            Ok(response) => println!(
                "seq={sequence:02} header={} payload={}",
                hex::encode(response.get(..4).unwrap_or(&response)),
                hex::encode(response.get(..16).unwrap_or(&response))
            ),
            Err(error) => {
                println!("seq={sequence:02} error={error}");
                break;
            }
        }
    }
    Ok(())
}
