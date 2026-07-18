//! Dedicated HID worker. No GTK types cross this boundary.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use tracing::{debug, info, warn};

use super::discovery::{self, DeviceIdentity};
use super::session::{SessionEvent, SessionMachine, SessionState};
use super::transport::{HidTransport, RealHidTransport, TransportError};
use super::DeviceError;
use crate::domain::config::{DeviceConfig, ProfileIdentity, SupportLevel};
use crate::domain::geometry::DeviceVariant;
use crate::error::AppError;
use crate::protocol::codec::verified_action_kinds;
use crate::protocol::identify;
use crate::protocol::read_config;
use crate::protocol::write_config::{self, WriteAuthorization};

#[derive(Debug)]
pub enum WorkerCommand {
    Refresh,
    Connect(DeviceIdentity),
    Identify,
    ReadConfig,
    WriteConfig(DeviceConfig),
    Disconnect,
    Shutdown,
}

#[derive(Debug)]
pub enum WorkerEvent {
    Devices(Vec<DeviceIdentity>),
    Connected(DeviceIdentity),
    Identified(DeviceVariant),
    ConfigRead(DeviceConfig),
    WriteVerified(DeviceConfig),
    Session(SessionState),
    Failed(AppError),
    Disconnected,
}

pub struct WorkerHandle {
    cmd_tx: mpsc::Sender<WorkerCommand>,
    pub events: async_channel::Receiver<WorkerEvent>,
    join: Option<thread::JoinHandle<()>>,
}

impl WorkerHandle {
    pub fn spawn() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerCommand>();
        let (evt_tx, evt_rx) = async_channel::unbounded::<WorkerEvent>();

        let join = thread::Builder::new()
            .name("minikeyboard-hid".into())
            .spawn(move || worker_loop(cmd_rx, evt_tx))
            .expect("spawn HID worker");

        Self {
            cmd_tx,
            events: evt_rx,
            join: Some(join),
        }
    }

    pub fn send(&self, cmd: WorkerCommand) {
        if let Err(e) = self.cmd_tx.send(cmd) {
            warn!(error = %e, "worker command channel closed");
        }
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        let _ = self.cmd_tx.send(WorkerCommand::Shutdown);
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

struct WorkerState {
    api: Option<hidapi::HidApi>,
    transport: Option<RealHidTransport>,
    identity: Option<DeviceIdentity>,
    variant: Option<DeviceVariant>,
    session: SessionMachine,
    last_refresh: Instant,
    connected: bool,
}

impl WorkerState {
    fn new() -> Self {
        Self {
            api: hidapi::HidApi::new().ok(),
            transport: None,
            identity: None,
            variant: None,
            session: SessionMachine::new(),
            last_refresh: Instant::now() - Duration::from_secs(10),
            connected: false,
        }
    }

    fn emit(tx: &async_channel::Sender<WorkerEvent>, ev: WorkerEvent) {
        if let Err(e) = tx.send_blocking(ev) {
            warn!(error = %e, "worker event channel closed");
        }
    }

    fn set_session(
        &mut self,
        tx: &async_channel::Sender<WorkerEvent>,
        ev: SessionEvent,
    ) -> Result<(), ()> {
        match self.session.transition(ev) {
            Ok(s) => {
                Self::emit(tx, WorkerEvent::Session(s));
                Ok(())
            }
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Session(e)));
                Err(())
            }
        }
    }

    fn refresh(&mut self, tx: &async_channel::Sender<WorkerEvent>) {
        let Some(api) = self.api.as_mut() else {
            Self::emit(
                tx,
                WorkerEvent::Failed(AppError::Device(DeviceError::HidApi(
                    "hidapi init failed".into(),
                ))),
            );
            return;
        };
        if let Err(e) = api.refresh_devices() {
            debug!(error = %e, "refresh_devices");
        }
        match discovery::discover(api) {
            Ok(list) => {
                if self.connected
                    && let Some(id) = &self.identity
                    && !list.iter().any(|d| d.path == id.path)
                {
                    info!("device path disappeared");
                    self.drop_connection();
                    let _ = self.set_session(tx, SessionEvent::Disconnect);
                    Self::emit(tx, WorkerEvent::Disconnected);
                }
                Self::emit(tx, WorkerEvent::Devices(list));
            }
            Err(e) => Self::emit(tx, WorkerEvent::Failed(AppError::Device(e))),
        }
        self.last_refresh = Instant::now();
    }

    fn drop_connection(&mut self) {
        self.transport = None;
        self.identity = None;
        self.variant = None;
        self.connected = false;
    }

    fn connect(&mut self, tx: &async_channel::Sender<WorkerEvent>, id: DeviceIdentity) {
        if self.connected {
            if self.identity.as_ref().is_some_and(|cur| cur.path == id.path) {
                debug!(path = %id.path, "already connected; ignoring Connect");
                return;
            }
            // Switching devices: close the current session first so the
            // BeginOpen transition below is valid.
            self.drop_connection();
            if self.set_session(tx, SessionEvent::Disconnect).is_err() {
                return;
            }
            Self::emit(tx, WorkerEvent::Disconnected);
        }
        if self.set_session(tx, SessionEvent::BeginOpen).is_err() {
            return;
        }
        let Some(api) = self.api.as_ref() else {
            Self::emit(
                tx,
                WorkerEvent::Failed(AppError::Device(DeviceError::HidApi(
                    "hidapi unavailable".into(),
                ))),
            );
            let _ = self.set_session(tx, SessionEvent::Fail);
            return;
        };

        let c_path = match std::ffi::CString::new(id.path.as_str()) {
            Ok(p) => p,
            Err(_) => {
                Self::emit(
                    tx,
                    WorkerEvent::Failed(AppError::Device(DeviceError::Message(
                        "invalid device path".into(),
                    ))),
                );
                let _ = self.set_session(tx, SessionEvent::Fail);
                return;
            }
        };

        let device = match api.open_path(&c_path) {
            Ok(d) => d,
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Device(map_open_error(e))));
                let _ = self.set_session(tx, SessionEvent::Fail);
                return;
            }
        };

        let mut transport = RealHidTransport::new(device);
        match transport.descriptor() {
            Ok(desc) => {
                if let Err(e) = discovery::validate_descriptor(&desc) {
                    Self::emit(tx, WorkerEvent::Failed(AppError::Device(e)));
                    let _ = self.set_session(tx, SessionEvent::Fail);
                    return;
                }
            }
            Err(TransportError::PermissionDenied) => {
                Self::emit(
                    tx,
                    WorkerEvent::Failed(AppError::Device(DeviceError::PermissionDenied)),
                );
                let _ = self.set_session(tx, SessionEvent::Fail);
                return;
            }
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Transport(e)));
                let _ = self.set_session(tx, SessionEvent::Fail);
                return;
            }
        }

        info!(
            vid = format_args!("{:04x}", id.vid),
            pid = format_args!("{:04x}", id.pid),
            interface = id.interface_number,
            "device opened"
        );

        self.transport = Some(transport);
        self.identity = Some(id.clone());
        self.connected = true;
        if self.set_session(tx, SessionEvent::Opened).is_err() {
            self.drop_connection();
            return;
        }
        Self::emit(tx, WorkerEvent::Connected(id));
        self.identify(tx);
    }

    fn identify(&mut self, tx: &async_channel::Sender<WorkerEvent>) {
        let Some(transport) = self.transport.as_mut() else {
            Self::emit(
                tx,
                WorkerEvent::Failed(AppError::Message("not connected".into())),
            );
            return;
        };
        match identify::identify(transport) {
            Ok(variant) => {
                info!(geometry = %variant.geometry_id, support = ?variant.support, "identified");
                let unknown = variant.support == SupportLevel::Unknown;
                self.variant = Some(variant.clone());
                let transition = if unknown {
                    self.set_session(tx, SessionEvent::IdentifiedUnknown)
                } else {
                    self.set_session(tx, SessionEvent::IdentifiedKnown)
                };
                if transition.is_err() {
                    return;
                }
                Self::emit(tx, WorkerEvent::Identified(variant));
                // Populate the editor immediately after a known identify.
                if !unknown {
                    self.read_config(tx);
                }
            }
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Protocol(e)));
                let _ = self.set_session(tx, SessionEvent::Fail);
            }
        }
    }

    fn read_config(&mut self, tx: &async_channel::Sender<WorkerEvent>) {
        let identity = match (&self.identity, &self.variant) {
            (Some(id), Some(v)) => ProfileIdentity {
                vid: id.vid,
                pid: id.pid,
                serial: id.serial.clone(),
                key_count: v.key_count,
                extra_count: v.extra_count,
                subtype: v.subtype,
            },
            _ => {
                Self::emit(
                    tx,
                    WorkerEvent::Failed(AppError::Message("not identified".into())),
                );
                return;
            }
        };
        if self.transport.is_none() {
            Self::emit(
                tx,
                WorkerEvent::Failed(AppError::Message("not connected".into())),
            );
            return;
        }
        if self.set_session(tx, SessionEvent::BeginRead).is_err() {
            return;
        }
        let transport = self.transport.as_mut().expect("transport checked above");
        match read_config::read_device_config(transport, &identity) {
            Ok(cfg) => {
                if self
                    .set_session(tx, SessionEvent::ReadComplete { dirty: false })
                    .is_ok()
                {
                    Self::emit(tx, WorkerEvent::ConfigRead(cfg));
                }
            }
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Protocol(e)));
                if self.session.state() == SessionState::Reading {
                    let _ = self.set_session(tx, SessionEvent::Fail);
                }
            }
        }
    }

    fn write_config(&mut self, tx: &async_channel::Sender<WorkerEvent>, config: DeviceConfig) {
        if self.transport.is_none() {
            Self::emit(
                tx,
                WorkerEvent::Failed(AppError::Message("not connected".into())),
            );
            return;
        }
        // Edits happen in the UI model; mirror them here so BeginWrite is a
        // valid transition (ReadyClean -> ReadyDirty -> Writing).
        if self.session.state() == SessionState::ReadyClean
            && config.dirty_count() > 0
            && self
                .set_session(tx, SessionEvent::EditApplied { has_dirty: true })
                .is_err()
        {
            return;
        }
        if self.set_session(tx, SessionEvent::BeginWrite).is_err() {
            return;
        }
        let support = self
            .identity
            .as_ref()
            .map(|i| i.support)
            .unwrap_or(SupportLevel::Unknown);
        let auth = WriteAuthorization {
            device_support: support,
            read_capability: read_config::current_read_capability(),
            provenance: config.provenance.clone(),
            verified_actions: verified_action_kinds(),
        };
        let transport = self.transport.as_mut().expect("transport checked above");
        match write_config::write_dirty_records(transport, &config, &auth) {
            Ok(cfg) => {
                if self.set_session(tx, SessionEvent::WriteVerified).is_ok() {
                    Self::emit(tx, WorkerEvent::WriteVerified(cfg));
                }
            }
            Err(e) => {
                Self::emit(tx, WorkerEvent::Failed(AppError::Write(e)));
                let _ = self.set_session(tx, SessionEvent::Fail);
            }
        }
    }
}

fn map_open_error(e: hidapi::HidError) -> DeviceError {
    let msg = e.to_string();
    let lower = msg.to_lowercase();
    if lower.contains("permission") || lower.contains("access") || lower.contains("eacces") {
        DeviceError::PermissionDenied
    } else if lower.contains("busy") {
        DeviceError::Busy
    } else {
        DeviceError::HidApi(msg)
    }
}

fn worker_loop(cmd_rx: mpsc::Receiver<WorkerCommand>, tx: async_channel::Sender<WorkerEvent>) {
    let mut state = WorkerState::new();
    state.refresh(&tx);

    loop {
        let timeout = if state.connected {
            Duration::from_secs(1)
        } else {
            Duration::from_millis(500)
        };

        match cmd_rx.recv_timeout(timeout) {
            Ok(WorkerCommand::Shutdown) => break,
            Ok(WorkerCommand::Refresh) => state.refresh(&tx),
            Ok(WorkerCommand::Connect(id)) => state.connect(&tx, id),
            Ok(WorkerCommand::Identify) => state.identify(&tx),
            Ok(WorkerCommand::ReadConfig) => state.read_config(&tx),
            Ok(WorkerCommand::WriteConfig(cfg)) => state.write_config(&tx, cfg),
            Ok(WorkerCommand::Disconnect) => {
                state.drop_connection();
                let _ = state.set_session(&tx, SessionEvent::Disconnect);
                WorkerState::emit(&tx, WorkerEvent::Disconnected);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                state.refresh(&tx);
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    debug!("HID worker stopped");
}
