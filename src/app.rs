//! Application controller: pure model + worker commands. UI renders from model.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::device::discovery::DeviceIdentity;
use crate::device::session::SessionState;
use crate::device::worker::{WorkerCommand, WorkerEvent, WorkerHandle};
use crate::domain::action::{Action, ActionKind};
use crate::domain::config::{ConfigProvenance, DeviceConfig, ProfileIdentity, SupportLevel};
use crate::domain::geometry::DeviceVariant;
use crate::error::AppError;
use crate::profile::diagnostics::{self, DiagnosticInput};
use crate::profile::json::{self, IdentityImportNote};
use crate::protocol::codec::verified_action_kinds;
use crate::protocol::read_config::{current_read_capability, ReadCapability};

#[derive(Debug, Clone)]
pub struct AppModel {
    pub devices: Vec<DeviceIdentity>,
    pub selected_device: Option<DeviceIdentity>,
    pub variant: Option<DeviceVariant>,
    pub config: Option<DeviceConfig>,
    pub baseline: Option<DeviceConfig>,
    pub session_state: SessionState,
    pub selected_layer: usize,
    pub selected_position: Option<u8>,
    pub status: String,
    pub last_error_category: Option<String>,
    pub read_capability: ReadCapability,
    pub verified_actions: std::collections::BTreeSet<ActionKind>,
    pub mock_scenario: Option<String>,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
            selected_device: None,
            variant: None,
            config: None,
            baseline: None,
            session_state: SessionState::Disconnected,
            selected_layer: 0,
            selected_position: None,
            status: "Connect a supported Mini Keyboard to begin.".into(),
            last_error_category: None,
            read_capability: current_read_capability(),
            verified_actions: verified_action_kinds(),
            mock_scenario: std::env::var("MINIKEYBOARD_MOCK_SCENARIO").ok(),
        }
    }
}

impl AppModel {
    pub fn write_enabled(&self) -> bool {
        if self.session_state != SessionState::ReadyDirty {
            return false;
        }
        let Some(dev) = &self.selected_device else {
            return false;
        };
        if dev.support != SupportLevel::Validated {
            return false;
        }
        let Some(cfg) = &self.config else {
            return false;
        };
        cfg.is_writable(dev.support, &self.read_capability, &self.verified_actions)
    }

    pub fn dirty_count(&self) -> usize {
        self.config.as_ref().map(|c| c.dirty_count()).unwrap_or(0)
    }

    pub fn selected_position_config(
        &self,
    ) -> Option<&crate::domain::config::PositionConfig> {
        let cfg = self.config.as_ref()?;
        let idx = self.selected_position?;
        cfg.layers
            .get(self.selected_layer)?
            .positions
            .iter()
            .find(|p| p.logical_index == idx)
    }
}

pub struct AppController {
    pub model: AppModel,
    worker: Option<WorkerHandle>,
}

impl Default for AppController {
    fn default() -> Self {
        Self::new()
    }
}

impl AppController {
    pub fn new() -> Self {
        let mut model = AppModel::default();
        if let Some(scenario) = model.mock_scenario.clone() {
            apply_mock_scenario(&mut model, &scenario);
            return Self {
                model,
                worker: None,
            };
        }
        let worker = WorkerHandle::spawn();
        Self {
            model,
            worker: Some(worker),
        }
    }

    pub fn event_receiver(&self) -> Option<async_channel::Receiver<WorkerEvent>> {
        self.worker.as_ref().map(|w| w.events.clone())
    }

    pub fn send(&self, cmd: WorkerCommand) {
        if let Some(w) = &self.worker {
            w.send(cmd);
        }
    }

    pub fn refresh(&self) {
        self.send(WorkerCommand::Refresh);
    }

    pub fn connect_selected(&mut self) {
        if let Some(dev) = self.model.selected_device.clone() {
            self.send(WorkerCommand::Connect(dev));
        }
    }

    pub fn read_from_device(&mut self) {
        self.send(WorkerCommand::ReadConfig);
    }

    pub fn write_changes(&mut self) {
        if let Some(cfg) = self.model.config.clone() {
            self.send(WorkerCommand::WriteConfig(cfg));
        }
    }

    pub fn disconnect(&mut self) {
        self.send(WorkerCommand::Disconnect);
    }

    pub fn select_device(&mut self, path: &str) {
        self.model.selected_device = self
            .model
            .devices
            .iter()
            .find(|d| d.path == path)
            .cloned();
    }

    pub fn select_layer(&mut self, layer: usize) {
        if layer < 3 {
            self.model.selected_layer = layer;
            self.model.selected_position = None;
        }
    }

    pub fn select_position(&mut self, logical_index: u8) {
        self.model.selected_position = Some(logical_index);
    }

    pub fn revert_changes(&mut self) {
        if let Some(base) = self.model.baseline.clone() {
            self.model.config = Some(base);
            self.model.session_state = SessionState::ReadyClean;
            self.model.status = "Changes reverted.".into();
        }
    }

    pub fn apply_action_to_selected(
        &mut self,
        action: Action,
        delay_ms: Option<u16>,
    ) -> Result<(), AppError> {
        if !self.model.session_state.allows_edit() {
            return Err(AppError::Message(
                "editing is not allowed in the current session state".into(),
            ));
        }
        let layer = self.model.selected_layer;
        let idx = self
            .model
            .selected_position
            .ok_or_else(|| AppError::Message("no position selected".into()))?;
        let verified = self.model.verified_actions.clone();
        let cfg = self
            .model
            .config
            .as_mut()
            .ok_or_else(|| AppError::Message("no configuration loaded".into()))?;
        let pos = cfg
            .layers
            .get_mut(layer)
            .and_then(|l| l.positions.iter_mut().find(|p| p.logical_index == idx))
            .ok_or_else(|| AppError::Message("position not found".into()))?;
        pos.apply_action(action, delay_ms, &verified)?;
        let dirty = cfg.dirty_count() > 0;
        self.model.session_state = if dirty {
            SessionState::ReadyDirty
        } else {
            SessionState::ReadyClean
        };
        self.model.status = if dirty {
            format!("{} dirty position(s).", cfg.dirty_count())
        } else {
            "Ready.".into()
        };
        Ok(())
    }

    pub fn export_profile(&self, path: PathBuf, include_serial: bool) -> Result<(), AppError> {
        let cfg = self
            .model
            .config
            .as_ref()
            .ok_or_else(|| AppError::Message("no configuration to export".into()))?;
        json::export_profile(&path, cfg, include_serial)?;
        Ok(())
    }

    pub fn import_profile(&mut self, path: PathBuf) -> Result<(), AppError> {
        let imported = json::import_profile(&path)?;
        if let (Some(dev), Some(var)) = (&self.model.selected_device, &self.model.variant) {
            let device_id = ProfileIdentity {
                vid: dev.vid,
                pid: dev.pid,
                serial: dev.serial.clone(),
                key_count: var.key_count,
                extra_count: var.extra_count,
                subtype: var.subtype,
            };
            match json::check_identity_compatible(&imported, &device_id) {
                Ok(IdentityImportNote::Exact) => {
                    self.model.status = "Profile imported.".into();
                }
                Ok(IdentityImportNote::SerialDiffers) => {
                    self.model.status =
                        "Profile imported with serial warning (same variant tuple).".into();
                }
                Err(e) => return Err(AppError::Profile(e)),
            }
        } else {
            self.model.status = "Profile imported (no device connected).".into();
        }
        self.model.config = Some(imported);
        // Import never writes automatically and does not mark dirty.
        if matches!(
            self.model.session_state,
            SessionState::ReadyClean | SessionState::ReadyDirty | SessionState::ReadOnlyUnknown
        ) {
            self.model.session_state = SessionState::ReadyClean;
        }
        Ok(())
    }

    pub fn export_diagnostics(&self, path: PathBuf) -> Result<(), AppError> {
        diagnostics::export_diagnostics(
            &path,
            DiagnosticInput {
                device: self.model.selected_device.as_ref(),
                variant: self.model.variant.as_ref(),
                session: self.model.session_state,
                descriptor: None,
                last_error_category: self.model.last_error_category.as_deref(),
            },
        )?;
        Ok(())
    }

    pub fn handle_worker_event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::Devices(list) => {
                self.model.devices = list;
                if self.model.selected_device.is_none()
                    && let Some(first) = self.model.devices.first()
                {
                    self.model.selected_device = Some(first.clone());
                }
            }
            WorkerEvent::Connected(id) => {
                self.model.selected_device = Some(id);
                self.model.status = "Connected.".into();
            }
            WorkerEvent::Identified(variant) => {
                let unknown = variant.support == SupportLevel::Unknown;
                self.model.variant = Some(variant.clone());
                // Seed empty config for UI geometry before verified read.
                if let Some(dev) = &self.model.selected_device {
                    let identity = ProfileIdentity {
                        vid: dev.vid,
                        pid: dev.pid,
                        serial: dev.serial.clone(),
                        key_count: variant.key_count,
                        extra_count: variant.extra_count,
                        subtype: variant.subtype,
                    };
                    let cfg = DeviceConfig::empty_for(identity, ConfigProvenance::Mock);
                    self.model.baseline = Some(cfg.clone());
                    self.model.config = Some(cfg);
                }
                self.model.status = if unknown {
                    format!(
                        "Unknown variant {} — read-only diagnostic mode.",
                        variant.geometry_id
                    )
                } else {
                    format!(
                        "Identified {} — read configuration to enable verified editing.",
                        variant.geometry_id
                    )
                };
            }
            WorkerEvent::ConfigRead(cfg) => {
                let _ = json::save_last_read_backup(&cfg);
                self.model.baseline = Some(cfg.clone());
                self.model.config = Some(cfg);
                self.model.status = "Configuration read.".into();
            }
            WorkerEvent::WriteVerified(cfg) => {
                self.model.baseline = Some(cfg.clone());
                self.model.config = Some(cfg);
                self.model.status = "Write verified.".into();
            }
            WorkerEvent::Session(s) => {
                self.model.session_state = s;
            }
            WorkerEvent::Failed(err) => {
                self.model.last_error_category = Some(err.category().into());
                self.model.status = format_status_error(&err);
            }
            WorkerEvent::Disconnected => {
                self.model.variant = None;
                self.model.config = None;
                self.model.baseline = None;
                self.model.session_state = SessionState::Disconnected;
                self.model.status = "Disconnected.".into();
            }
        }
    }
}

fn format_status_error(err: &AppError) -> String {
    match err {
        AppError::Device(crate::device::DeviceError::PermissionDenied)
        | AppError::Transport(crate::device::transport::TransportError::PermissionDenied) => {
            "Permission denied. Install the udev rule: sudo linux/setup-hid-permissions.sh \
             (copies data/udev/70-minikeyboard.rules to /etc/udev/rules.d/)."
                .into()
        }
        AppError::Protocol(crate::protocol::ProtocolError::UnverifiedReadProtocol) => {
            "Configuration read is not validated for this device tuple; Write remains disabled."
                .into()
        }
        AppError::Write(crate::protocol::WriteError::IndeterminateCommit) => {
            "Disconnected during write — outcome is indeterminate. Reconnect and read back."
                .into()
        }
        AppError::Profile(crate::profile::ProfileError::IdentityMismatch { profile, device }) => {
            format!("Profile mismatch: profile {profile} vs device {device}.")
        }
        other => other.to_string(),
    }
}

fn apply_mock_scenario(model: &mut AppModel, scenario: &str) {
    model.mock_scenario = Some(scenario.into());
    match scenario {
        "disconnected" => {
            model.session_state = SessionState::Disconnected;
            model.status = "Connect a supported Mini Keyboard to begin.".into();
        }
        "known" => {
            seed_known(model, false);
            model.session_state = SessionState::ReadyClean;
            model.status = "Mock known device — read configuration to enable editing.".into();
        }
        "unknown" => {
            seed_unknown(model);
            model.session_state = SessionState::ReadOnlyUnknown;
            model.status = "Mock unknown variant — read-only; export diagnostics.".into();
        }
        "dirty" => {
            seed_known(model, true);
            model.session_state = SessionState::ReadyDirty;
            model.status =
                "Mock dirty state — Write remains disabled: Read protocol has not been verified."
                    .into();
        }
        other => {
            model.status = format!("Unknown mock scenario '{other}'.");
        }
    }
}

fn seed_known(model: &mut AppModel, dirty: bool) {
    use crate::domain::geometry::resolve_variant;
    let variant = resolve_variant(6, 2, 0);
    let dev = DeviceIdentity {
        vid: 0x1189,
        pid: 0x8842,
        serial: Some("MOCKSERIAL".into()),
        path: "mock://known".into(),
        interface_number: 0,
        usage_page: 0xFF00,
        usage: 1,
        support: SupportLevel::Validated,
        product: Some("Mock Mini Keyboard".into()),
    };
    let identity = ProfileIdentity {
        vid: dev.vid,
        pid: dev.pid,
        serial: dev.serial.clone(),
        key_count: variant.key_count,
        extra_count: variant.extra_count,
        subtype: variant.subtype,
    };
    let mut cfg = DeviceConfig::empty_for(identity, ConfigProvenance::Mock);
    if dirty
        && let Some(pos) = cfg.layers[0].positions.get_mut(0)
    {
        pos.dirty = true;
        pos.action = Action::Opaque { mode: 1 };
    }
    model.devices = vec![dev.clone()];
    model.selected_device = Some(dev);
    model.variant = Some(variant);
    model.baseline = Some(cfg.clone());
    model.config = Some(cfg);
    model.selected_layer = 0;
    model.selected_position = Some(0);
}

fn seed_unknown(model: &mut AppModel) {
    use crate::domain::geometry::resolve_variant;
    let variant = resolve_variant(99, 9, 1);
    let dev = DeviceIdentity {
        vid: 0x1189,
        pid: 0x8842,
        serial: None,
        path: "mock://unknown".into(),
        interface_number: 0,
        usage_page: 0xFF00,
        usage: 1,
        support: SupportLevel::Validated,
        product: Some("Mock Unknown".into()),
    };
    let identity = ProfileIdentity {
        vid: dev.vid,
        pid: dev.pid,
        serial: None,
        key_count: variant.key_count,
        extra_count: variant.extra_count,
        subtype: variant.subtype,
    };
    let cfg = DeviceConfig::empty_for(identity, ConfigProvenance::Mock);
    model.devices = vec![dev.clone()];
    model.selected_device = Some(dev);
    model.variant = Some(variant);
    model.config = Some(cfg);
}

/// Shared controller handle for GTK callbacks.
pub type SharedController = Rc<RefCell<AppController>>;
