//! Temporary read-only hardware probe for validating 0xFA response shape.

use minikeyboard::device::discovery::discover;
use minikeyboard::device::transport::RealHidTransport;
use minikeyboard::domain::config::ProfileIdentity;
use minikeyboard::protocol::{identify, read_config};
use sha2::Digest;

fn main() {
    let api = hidapi::HidApi::new().expect("hidapi");
    let device = discover(&api)
        .expect("discover")
        .into_iter()
        .find(|d| d.vid == 0x1189 && d.pid == 0x8842 && d.interface_number == 0)
        .expect("1189:8842 interface 0");
    eprintln!("opening {}", device.path);
    let hid = api
        .open_path(std::ffi::CString::new(device.path.as_str()).unwrap().as_c_str())
        .expect("open path");
    let mut transport = RealHidTransport::new(hid);

    let variant = identify(&mut transport).expect("identify");
    let identity = ProfileIdentity {
        vid: device.vid,
        pid: device.pid,
        serial: device.serial,
        key_count: variant.key_count,
        extra_count: variant.extra_count,
        subtype: variant.subtype,
    };
    let config = read_config::read_device_config(&mut transport, &identity).expect("read config");

    println!(
        "IDENTITY {:04x}:{:04x} tuple={:?} layers={} records={:?}",
        identity.vid,
        identity.pid,
        identity.tuple(),
        config.layers.len(),
        config
            .layers
            .iter()
            .map(|layer| layer.positions.len())
            .collect::<Vec<_>>()
    );
    println!("PROVENANCE {:?}", config.provenance);
    println!("LAYER 1 RECORDS");
    for position in &config.layers[0].positions {
        println!(
            "POSITION {:02} action={} delay={:?} raw={}",
            position.logical_index + 1,
            position.action.short_label(),
            position.delay_ms,
            hex::encode(position.raw_record)
        );
    }
    for (layer_index, layer) in config.layers.iter().enumerate() {
        let digest = sha2::Sha256::digest(
            layer
                .positions
                .iter()
                .flat_map(|position| position.raw_record)
                .collect::<Vec<_>>(),
        );
        println!(
            "LAYER {} sha256={} first={} last={}",
            layer_index + 1,
            hex::encode(digest),
            hex::encode(layer.positions.first().expect("first").raw_record),
            hex::encode(layer.positions.last().expect("last").raw_record)
        );
    }
}
