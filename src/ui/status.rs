//! Human-readable status / recovery strings (English UI).

use crate::app::AppModel;
use crate::device::session::SessionState;
use crate::domain::config::SupportLevel;

pub fn empty_state_message() -> &'static str {
    "Connect a supported Mini Keyboard to begin."
}

pub fn badge_class(model: &AppModel) -> &'static str {
    match model.session_state {
        SessionState::Disconnected => "connection-badge",
        SessionState::ErrorRecoverable => "connection-badge error",
        SessionState::ReadOnlyUnknown => "connection-badge readonly",
        SessionState::ReadyClean
        | SessionState::ReadyDirty
        | SessionState::Identifying
        | SessionState::Opening
        | SessionState::Reading
        | SessionState::Writing => "connection-badge connected",
    }
}

pub fn badge_text(model: &AppModel) -> String {
    match model.session_state {
        SessionState::Disconnected => "Disconnected".into(),
        SessionState::ReadOnlyUnknown => "Read-only".into(),
        SessionState::ReadyDirty => "Dirty".into(),
        SessionState::ErrorRecoverable => "Error".into(),
        other => other.rail_label().into(),
    }
}

pub fn write_blocked_reason(model: &AppModel) -> Option<String> {
    if model.write_enabled() {
        return None;
    }
    if model.session_state != SessionState::ReadyDirty {
        return Some("Write requires dirty validated configuration.".into());
    }
    if !matches!(
        model.read_capability,
        crate::protocol::read_config::ReadCapability::Verified { .. }
    ) {
        return Some("Read protocol has not been verified.".into());
    }
    if let Some(dev) = &model.selected_device
        && dev.support != SupportLevel::Validated
    {
        return Some("Device is not on the validated allowlist.".into());
    }
    Some("Write is disabled.".into())
}

/// Protocol rail steps for the signature visual element.
pub fn rail_steps(state: SessionState) -> Vec<(&'static str, RailTone)> {
    use RailTone::*;
    let active = |s: SessionState, label: &'static str, tone: RailTone| -> (&'static str, RailTone) {
        if state == s {
            (label, tone)
        } else {
            (label, Idle)
        }
    };
    // Show condensed pipeline: Disconnected → Identifying → Ready/Read-only → Writing/Verified
    let mut steps = Vec::new();
    steps.push(match state {
        SessionState::Disconnected => ("Disconnected", Active),
        _ => ("Disconnected", Idle),
    });
    steps.push(match state {
        SessionState::Opening | SessionState::Identifying => ("Identifying", Active),
        SessionState::Disconnected => ("Identifying", Idle),
        _ => ("Identifying", Idle),
    });
    steps.push(match state {
        SessionState::ReadyClean | SessionState::ReadyDirty | SessionState::Reading => {
            ("Ready", Ok)
        }
        SessionState::ReadOnlyUnknown => ("Read-only", Warn),
        _ => ("Ready/Read-only", Idle),
    });
    steps.push(match state {
        SessionState::Writing => ("Writing", Active),
        SessionState::ErrorRecoverable => ("Error", Error),
        SessionState::ReadyClean if false => ("Verified", Ok),
        _ => ("Writing/Verified", Idle),
    });
    let _ = active;
    steps
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailTone {
    Idle,
    Active,
    Warn,
    Error,
    Ok,
}

impl RailTone {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Idle => "protocol-rail-step",
            Self::Active => "protocol-rail-step active",
            Self::Warn => "protocol-rail-step warn",
            Self::Error => "protocol-rail-step error",
            Self::Ok => "protocol-rail-step ok",
        }
    }
}
