//! Background launcher: watches the Mini Keyboard's evdev nodes and runs the
//! command bound to each marker chord (F13–F24 + modifiers) pressed.
//!
//! Bindings live in `bindings.json` (managed by the GUI's "Aplicações e
//! Scripts" editor) and are reloaded automatically when the file changes.
//! Run as a systemd user service: `systemctl --user enable --now
//! minikeyboard-daemon`.

use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use minikeyboard::device::evdev;
use minikeyboard::domain::action::{Modifiers, hid_usage_from_evdev};
use minikeyboard::profile::bindings::{self, AppBinding};

const VID: &str = "1189";
const PID: &str = "8842";

struct Bindings {
    list: Vec<AppBinding>,
    mtime: Option<SystemTime>,
    checked: Instant,
}

impl Bindings {
    fn new() -> Self {
        let mut b = Self {
            list: Vec::new(),
            mtime: None,
            checked: Instant::now(),
        };
        b.reload();
        b
    }

    fn reload(&mut self) {
        self.list = bindings::load_bindings().unwrap_or_default();
        self.mtime = bindings::bindings_path()
            .ok()
            .and_then(|p| std::fs::metadata(p).ok())
            .and_then(|m| m.modified().ok());
    }

    /// Reload when the file's mtime changed; stat at most once per second.
    fn refresh(&mut self) {
        if self.checked.elapsed() < Duration::from_secs(1) {
            return;
        }
        self.checked = Instant::now();
        let mtime = bindings::bindings_path()
            .ok()
            .and_then(|p| std::fs::metadata(p).ok())
            .and_then(|m| m.modified().ok());
        if mtime != self.mtime {
            self.reload();
            eprintln!("bindings reloaded ({} entries)", self.list.len());
        }
    }
}

fn run_command(binding: &AppBinding) {
    // `… &` + wait(): sh backgrounds the job and exits at once, the child is
    // reparented to init — no zombies accumulate in the daemon.
    let spawned = Command::new("sh")
        .arg("-c")
        .arg(format!("{} &", binding.command))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match spawned {
        Ok(mut child) => {
            let _ = child.wait();
            eprintln!("launched: {} ({})", binding.name, binding.command);
        }
        Err(e) => eprintln!("failed to launch {}: {e}", binding.name),
    }
}

/// Read the device until it disappears (unplug) — then return to re-enumerate.
fn watch(files: &mut [File], bindings: &mut Bindings) {
    let mut held = Modifiers::empty();
    let mut buf = [0u8; evdev::EVENT_SIZE * 32];
    loop {
        let mut got_data = false;
        for f in files.iter_mut() {
            loop {
                match f.read(&mut buf) {
                    Ok(n) if n >= evdev::EVENT_SIZE => {
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
                                && bindings::is_marker(usage)
                            {
                                bindings.refresh();
                                if let Some(b) = bindings::find(&bindings.list, held, usage)
                                {
                                    run_command(b);
                                }
                            }
                        }
                    }
                    Ok(_) => break,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(_) => return, // ENODEV: device unplugged
                }
            }
        }
        if !got_data {
            bindings.refresh();
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn main() {
    let mut bindings = Bindings::new();
    eprintln!(
        "minikeyboard-daemon: {} binding(s) loaded",
        bindings.list.len()
    );
    loop {
        let nodes = evdev::keyboard_nodes(Some((VID, PID)));
        let mut files: Vec<File> = nodes
            .iter()
            .filter_map(|p| evdev::open_nonblocking(p).ok())
            .collect();
        if files.is_empty() {
            std::thread::sleep(Duration::from_secs(3));
            continue;
        }
        eprintln!("watching {}", nodes.join(", "));
        watch(&mut files, &mut bindings);
        eprintln!("device lost; re-scanning");
        std::thread::sleep(Duration::from_secs(1));
    }
}
