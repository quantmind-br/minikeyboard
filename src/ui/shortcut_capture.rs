//! Background evdev capture for the shortcut recorder.
//!
//! Reads key events straight from `/dev/input/event*` so combos the
//! compositor would swallow (Super+…, media keys) are still captured, and
//! grabs the nodes exclusively so the recorded combo does not trigger
//! anything else. Requires read access to the event nodes (`input` group or
//! uaccess).

use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::device::evdev;
use crate::domain::action::{Modifiers, hid_usage_from_evdev};

#[derive(Debug, Clone, Copy)]
pub struct Captured {
    pub modifiers: Modifiers,
    pub usage: u8,
}

/// Spawn the capture thread. The returned channel yields exactly one
/// message: `Some(combo)` on success, `None` on timeout, cancellation, or
/// missing /dev/input access. Set `cancel` to stop early.
pub fn spawn(cancel: Arc<AtomicBool>) -> async_channel::Receiver<Option<Captured>> {
    let (tx, rx) = async_channel::bounded(1);
    std::thread::spawn(move || {
        let mut first = None;
        capture_loop(&cancel, 15, |c| {
            first = Some(c);
            false
        });
        let _ = tx.send_blocking(first);
    });
    rx
}

/// Spawn a multi-stroke capture: every combo pressed is streamed through the
/// channel until cancellation or 60 s. The channel closes when the recording
/// ends. Callers decide what ends the sequence (e.g. bare Esc).
pub fn spawn_sequence(cancel: Arc<AtomicBool>) -> async_channel::Receiver<Captured> {
    let (tx, rx) = async_channel::unbounded();
    std::thread::spawn(move || {
        capture_loop(&cancel, 60, |c| tx.send_blocking(c).is_ok());
    });
    rx
}

/// Read grabbed keyboards, invoking `on_combo` for each non-modifier press
/// (with the modifiers held at that moment). Stops when the callback returns
/// false, on cancellation, or timeout.
fn capture_loop(
    cancel: &AtomicBool,
    timeout_secs: u64,
    mut on_combo: impl FnMut(Captured) -> bool,
) {
    let mut files: Vec<File> = evdev::keyboard_nodes(None)
        .iter()
        .filter_map(|p| evdev::open_nonblocking(p).ok())
        .collect();
    if files.is_empty() {
        return;
    }

    // Exclusive grab: the combo being recorded must not trigger compositor
    // or application shortcuts. The kernel releases the grab automatically
    // when the fd is closed (thread exit), so no explicit ungrab is needed.
    for f in &files {
        evdev::grab(f);
    }

    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let mut held = Modifiers::empty();
    let mut buf = [0u8; evdev::EVENT_SIZE * 32];
    while Instant::now() < deadline && !cancel.load(Ordering::Relaxed) {
        let mut got_data = false;
        for f in &mut files {
            while let Ok(n) = f.read(&mut buf) {
                if n < evdev::EVENT_SIZE {
                    break;
                }
                got_data = true;
                for (etype, code, value) in evdev::parse_events(&buf[..n]) {
                    if etype != evdev::EV_KEY {
                        continue;
                    }
                    if let Some(m) = evdev::modifier_bit(code) {
                        if value == 1 {
                            held |= m;
                        } else if value == 0 {
                            held.remove(m);
                        }
                    } else if value == 1
                        && let Some(usage) = hid_usage_from_evdev(code)
                        && !on_combo(Captured {
                            modifiers: held,
                            usage,
                        })
                    {
                        return;
                    }
                }
            }
        }
        if !got_data {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
