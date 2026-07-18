//! One-shot real-hardware validation: backup, reversible write/readback, restore/readback.

use std::thread;
use std::fs;
use std::path::PathBuf;

use minikeyboard::device::discovery::discover;
use minikeyboard::device::transport::RealHidTransport;
use minikeyboard::device::transport::HidTransport;
use minikeyboard::domain::action::Action;
use minikeyboard::domain::config::{ProfileIdentity, SupportLevel};
use minikeyboard::profile::json;
use minikeyboard::protocol::codec::verified_action_kinds;
use minikeyboard::protocol::frame::{commit_report, position_report, POST_COMMIT_DELAY};
use minikeyboard::protocol::identify;
use minikeyboard::protocol::read_config::{self, current_read_capability};
use minikeyboard::protocol::write_config::{self, WriteAuthorization};
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let variant = identify(&mut transport)?;
    let identity = ProfileIdentity {
        vid: device.vid,
        pid: device.pid,
        serial: device.serial,
        key_count: variant.key_count,
        extra_count: variant.extra_count,
        subtype: variant.subtype,
    };
    let original = read_config::read_device_config(&mut transport, &identity)?;
    let original_digest = config_digest(&original);

    let backup_path = timestamped_backup_path(&original)?;
    json::export_profile(&backup_path, &original, true)?;
    let backup = json::import_profile(&backup_path)?;
    if !same_records(&original, &backup) {
        return Err("backup readback differs from device snapshot".into());
    }
    println!("BACKUP {} sha256={original_digest}", backup_path.display());

    let target_layer = 0usize;
    let target_index = 12u8;
    let original_position = original.layers[target_layer]
        .positions
        .iter()
        .find(|position| position.logical_index == target_index)
        .ok_or("target position not present")?
        .clone();
    let probe_action = choose_probe_action(&original_position.action);

    let authorization = WriteAuthorization {
        device_support: SupportLevel::Validated,
        read_capability: current_read_capability(),
        provenance: original.provenance.clone(),
        verified_actions: verified_action_kinds(),
    };

    let mut probe = original.clone();
    probe
        .layers[target_layer]
        .positions
        .iter_mut()
        .find(|position| position.logical_index == target_index)
        .expect("target position")
        .apply_action(probe_action.clone(), original_position.delay_ms, &verified_action_kinds())?;
    println!(
        "PROBE layer=1 position={} original={} temporary={}", target_index + 1,
        original_position.action.short_label(),
        probe_action.short_label()
    );

    let probed = match write_config::write_dirty_records(&mut transport, &probe, &authorization) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("PROBE_WRITE failed: {error}; restoring original record directly");
            restore_record_direct(&mut transport, original_position.raw_record)?;
            let restored = read_config::read_device_config(&mut transport, &original.identity)?;
            if !same_records(&original, &restored) {
                return Err(format!(
                    "direct emergency restore mismatch; backup={}",
                    backup_path.display()
                )
                .into());
            }
            return Err(error.into());
        }
    };
    let position = probed.layers[target_layer]
        .positions
        .iter()
        .find(|position| position.logical_index == target_index)
        .ok_or("target position absent after probe")?;
    if position.action != probe_action {
        restore_record_direct(&mut transport, original_position.raw_record)?;
        return Err(format!(
            "temporary action semantic mismatch: expected {}, got {}",
            probe_action.short_label(),
            position.action.short_label()
        )
        .into());
    }
    println!("PROBE_READBACK ok raw={}", hex::encode(position.raw_record));

    let mut restore_config = probed;
    let current = restore_config.layers[target_layer]
        .positions
        .iter_mut()
        .find(|position| position.logical_index == target_index)
        .ok_or("target position absent before restore")?;
    current.raw_record = original_position.raw_record;
    current.action = original_position.action.clone();
    current.delay_ms = original_position.delay_ms;
    current.dirty = true;

    let restored = write_config::write_dirty_records(&mut transport, &restore_config, &authorization)?;
    let restored_digest = config_digest(&restored);
    if !same_records(&original, &restored) {
        return Err(format!(
            "restore mismatch: original sha256={original_digest}, restored sha256={restored_digest}; backup={}",
            backup_path.display()
        )
        .into());
    }
    println!("RESTORE_READBACK ok sha256={restored_digest}");
    println!("ROUNDTRIP_OK backup={}", backup_path.display());
    Ok(())
}

fn choose_probe_action(original: &Action) -> Action {
    let candidates = [
        Action::Keyboard {
            modifiers: minikeyboard::domain::action::Modifiers::empty(),
            usage: 0x04,
        },
        Action::Keyboard {
            modifiers: minikeyboard::domain::action::Modifiers::empty(),
            usage: 0x05,
        },
    ];
    candidates
        .into_iter()
        .find(|candidate| candidate != original)
        .expect("distinct probe action")
}

fn restore_record_direct(
    transport: &mut RealHidTransport,
    record: [u8; 50],
) -> Result<(), Box<dyn std::error::Error>> {
    transport.drain_pending()?;
    transport.write_report(&position_report(&record))?;
    transport.write_report(&commit_report())?;
    thread::sleep(POST_COMMIT_DELAY);
    Ok(())
}

fn timestamped_backup_path(
    config: &minikeyboard::domain::config::DeviceConfig,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dirs = directories::ProjectDirs::from("br", "Quantmind", "MiniKeyboard")
        .ok_or("could not resolve state directory")?;
    let state = dirs.state_dir().ok_or("state_dir unavailable")?;
    let dir = state.join("backups").join(config.identity.backup_key());
    fs::create_dir_all(&dir)?;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    Ok(dir.join(format!("pre-roundtrip-{timestamp}.json")))
}

fn same_records(
    left: &minikeyboard::domain::config::DeviceConfig,
    right: &minikeyboard::domain::config::DeviceConfig,
) -> bool {
    left.identity == right.identity
        && left.layers.iter().zip(&right.layers).all(|(left_layer, right_layer)| {
            left_layer.positions.len() == right_layer.positions.len()
                && left_layer.positions.iter().zip(&right_layer.positions).all(
                    |(left_position, right_position)| {
                        left_position.logical_index == right_position.logical_index
                            && left_position.raw_record == right_position.raw_record
                    },
                )
        })
}

fn config_digest(config: &minikeyboard::domain::config::DeviceConfig) -> String {
    let mut hasher = Sha256::new();
    for layer in &config.layers {
        for position in &layer.positions {
            hasher.update(position.raw_record);
        }
    }
    hex::encode(hasher.finalize())
}

