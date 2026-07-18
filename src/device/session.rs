//! Session state machine — SPEC §7.4 invariants.

use std::fmt;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    Disconnected,
    Opening,
    Identifying,
    ReadyClean,
    ReadyDirty,
    ReadOnlyUnknown,
    Reading,
    Writing,
    ErrorRecoverable,
}

impl SessionState {
    pub fn rail_label(self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Opening => "Opening",
            Self::Identifying => "Identifying",
            Self::ReadyClean => "Ready",
            Self::ReadyDirty => "Ready (dirty)",
            Self::ReadOnlyUnknown => "Read-only",
            Self::Reading => "Reading",
            Self::Writing => "Writing",
            Self::ErrorRecoverable => "Error",
        }
    }

    pub fn allows_edit(self) -> bool {
        matches!(self, Self::ReadyClean | Self::ReadyDirty)
    }

    pub fn allows_write(self) -> bool {
        matches!(self, Self::ReadyDirty)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    BeginOpen,
    Opened,
    BeginIdentify,
    IdentifiedKnown,
    IdentifiedUnknown,
    BeginRead,
    ReadComplete { dirty: bool },
    EditApplied { has_dirty: bool },
    BeginWrite,
    WriteVerified,
    Fail,
    Disconnect,
    /// Serial changed on reconnect — discard dirty.
    SerialChanged,
    Revert,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("invalid transition from {from:?} on {event:?}")]
pub struct InvalidTransition {
    pub from: SessionState,
    pub event: String,
}

impl InvalidTransition {
    fn new(from: SessionState, event: &SessionEvent) -> Self {
        Self {
            from,
            event: format!("{event:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionMachine {
    state: SessionState,
}

impl Default for SessionMachine {
    fn default() -> Self {
        Self {
            state: SessionState::Disconnected,
        }
    }
}

impl SessionMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn transition(&mut self, event: SessionEvent) -> Result<SessionState, InvalidTransition> {
        use SessionEvent::*;
        use SessionState::*;

        let next = match (self.state, &event) {
            // Open path
            (Disconnected | ErrorRecoverable, BeginOpen) => Opening,
            (Opening, Opened) => Identifying, // auto-identify after open
            (Opening, BeginIdentify) => Identifying,
            (Identifying, IdentifiedKnown) => ReadyClean,
            (Identifying, IdentifiedUnknown) => ReadOnlyUnknown,

            // Read
            (ReadyClean | ReadyDirty | ReadOnlyUnknown, BeginRead) => Reading,
            (Reading, ReadComplete { dirty: false }) => ReadyClean,
            (Reading, ReadComplete { dirty: true }) => ReadyDirty,

            // We need support-aware read complete — handle via extra events:
            // For unknown, controller should emit IdentifiedUnknown again or use Fail path.
            // Explicit: after read from ReadOnlyUnknown context stay ReadOnlyUnknown.
            // Implemented by controller choosing event; machine accepts ReadyClean/Dirty only
            // from Reading. Controller maps unknown → stay via Fail? Better: special event.
            // Keep simple: controller calls transition with appropriate terminal after read.

            // Edit — reject during Reading/Writing
            (ReadyClean | ReadyDirty, EditApplied { has_dirty: true }) => ReadyDirty,
            (ReadyClean | ReadyDirty, EditApplied { has_dirty: false }) => ReadyClean,
            (Reading | Writing, EditApplied { .. }) => {
                return Err(InvalidTransition::new(self.state, &event));
            }

            // Write — only ReadyDirty
            (ReadyDirty, BeginWrite) => Writing,
            (Writing, WriteVerified) => ReadyClean,
            (s, BeginWrite) if s != ReadyDirty => {
                return Err(InvalidTransition::new(self.state, &event));
            }

            // Revert
            (ReadyDirty, Revert) => ReadyClean,

            // Failures / disconnect
            (_, Fail) => ErrorRecoverable,
            (_, Disconnect) => Disconnected,
            (_, SerialChanged) => Disconnected,

            _ => return Err(InvalidTransition::new(self.state, &event)),
        };

        self.state = next;
        Ok(self.state)
    }

    /// After a read while previously ReadOnlyUnknown, force read-only terminal.
    pub fn force_read_only(&mut self) {
        self.state = SessionState::ReadOnlyUnknown;
    }
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.rail_label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_identify_ready() {
        let mut m = SessionMachine::new();
        m.transition(SessionEvent::BeginOpen).unwrap();
        m.transition(SessionEvent::Opened).unwrap();
        assert_eq!(m.state(), SessionState::Identifying);
        m.transition(SessionEvent::IdentifiedKnown).unwrap();
        assert_eq!(m.state(), SessionState::ReadyClean);
    }

    #[test]
    fn only_ready_dirty_accepts_write() {
        let mut m = SessionMachine::new();
        m.transition(SessionEvent::BeginOpen).unwrap();
        m.transition(SessionEvent::Opened).unwrap();
        m.transition(SessionEvent::IdentifiedKnown).unwrap();
        assert!(m.transition(SessionEvent::BeginWrite).is_err());
        m.transition(SessionEvent::EditApplied { has_dirty: true })
            .unwrap();
        assert_eq!(m.state(), SessionState::ReadyDirty);
        m.transition(SessionEvent::BeginWrite).unwrap();
        assert_eq!(m.state(), SessionState::Writing);
    }

    #[test]
    fn reject_edit_while_writing() {
        let mut m = SessionMachine::new();
        m.transition(SessionEvent::BeginOpen).unwrap();
        m.transition(SessionEvent::Opened).unwrap();
        m.transition(SessionEvent::IdentifiedKnown).unwrap();
        m.transition(SessionEvent::EditApplied { has_dirty: true })
            .unwrap();
        m.transition(SessionEvent::BeginWrite).unwrap();
        assert!(m
            .transition(SessionEvent::EditApplied { has_dirty: true })
            .is_err());
    }

    #[test]
    fn unknown_never_ready_dirty_by_identify() {
        let mut m = SessionMachine::new();
        m.transition(SessionEvent::BeginOpen).unwrap();
        m.transition(SessionEvent::Opened).unwrap();
        m.transition(SessionEvent::IdentifiedUnknown).unwrap();
        assert_eq!(m.state(), SessionState::ReadOnlyUnknown);
        assert!(!m.state().allows_write());
    }

    #[test]
    fn serial_change_disconnects() {
        let mut m = SessionMachine::new();
        m.transition(SessionEvent::BeginOpen).unwrap();
        m.transition(SessionEvent::Opened).unwrap();
        m.transition(SessionEvent::IdentifiedKnown).unwrap();
        m.transition(SessionEvent::EditApplied { has_dirty: true })
            .unwrap();
        m.transition(SessionEvent::SerialChanged).unwrap();
        assert_eq!(m.state(), SessionState::Disconnected);
    }
}
