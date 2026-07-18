//! Minimal evdev helpers shared by the shortcut recorder and the
//! application-launcher daemon: node discovery via `/proc/bus/input/devices`,
//! non-blocking reads, raw `input_event` parsing, and modifier mapping.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;

use crate::domain::action::Modifiers;

pub const EVENT_SIZE: usize = 24; // struct input_event on 64-bit
pub const EV_KEY: u16 = 0x01;
const O_NONBLOCK: i32 = 0o4000;
const EVIOCGRAB: libc::c_ulong = 0x4004_4590;

pub fn modifier_bit(code: u16) -> Option<Modifiers> {
    match code {
        29 | 97 => Some(Modifiers::CTRL),   // KEY_LEFTCTRL / KEY_RIGHTCTRL
        42 | 54 => Some(Modifiers::SHIFT),  // KEY_LEFTSHIFT / KEY_RIGHTSHIFT
        56 | 100 => Some(Modifiers::ALT),   // KEY_LEFTALT / KEY_RIGHTALT
        125 | 126 => Some(Modifiers::GUI),  // KEY_LEFTMETA / KEY_RIGHTMETA
        _ => None,
    }
}

/// Event nodes advertising a `kbd` handler. With `vendor_product`
/// (lower-case hex, e.g. `("1189", "8842")`) only nodes of that USB device
/// are returned; with `None`, every keyboard on the system.
pub fn keyboard_nodes(vendor_product: Option<(&str, &str)>) -> Vec<String> {
    let Ok(devices) = std::fs::read_to_string("/proc/bus/input/devices") else {
        return Vec::new();
    };
    let mut nodes = Vec::new();
    let mut ident = "";
    for line in devices.lines() {
        if line.starts_with("I:") {
            ident = line;
        } else if let Some(handlers) = line.strip_prefix("H: Handlers=")
            && handlers.split_whitespace().any(|h| h == "kbd")
            && vendor_product.is_none_or(|(v, p)| {
                ident.contains(&format!("Vendor={v} Product={p}"))
            })
        {
            nodes.extend(
                handlers
                    .split_whitespace()
                    .filter(|h| h.starts_with("event"))
                    .map(|h| format!("/dev/input/{h}")),
            );
        }
    }
    nodes
}

pub fn open_nonblocking(path: &str) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(O_NONBLOCK)
        .open(path)
}

/// Exclusive grab; the kernel releases it automatically when the fd closes.
pub fn grab(file: &File) {
    // SAFETY: EVIOCGRAB on a valid evdev fd; failure only leaves the node
    // ungrabbed, which is tolerable.
    unsafe {
        libc::ioctl(file.as_raw_fd(), EVIOCGRAB, 1_i32);
    }
}

/// Iterate `(type, code, value)` triples over a raw evdev read buffer.
pub fn parse_events(buf: &[u8]) -> impl Iterator<Item = (u16, u16, i32)> + '_ {
    buf.chunks_exact(EVENT_SIZE).map(|ev| {
        (
            u16::from_ne_bytes([ev[16], ev[17]]),
            u16::from_ne_bytes([ev[18], ev[19]]),
            i32::from_ne_bytes([ev[20], ev[21], ev[22], ev[23]]),
        )
    })
}
