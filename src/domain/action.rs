//! Semantic action types shared by codec authorization and UI gating.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct Modifiers: u8 {
        const CTRL  = 0x01;
        const SHIFT = 0x02;
        const ALT   = 0x04;
        const GUI   = 0x08;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Stroke {
    pub modifiers: Modifiers,
    pub usage: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerAction {
    PlayPause,
    Stop,
    PreviousTrack,
    NextTrack,
    Mute,
    VolumeUp,
    VolumeDown,
    Calculator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MouseAction {
    LeftClick,
    MiddleClick,
    RightClick,
    WheelUp,
    WheelDown,
}

/// Stable category key shared by codec authorization and UI gating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    Empty,
    Keyboard,
    Sequence,
    Consumer,
    Mouse,
    Lighting,
    Delay,
    Opaque,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Empty,
    Keyboard {
        modifiers: Modifiers,
        usage: u8,
    },
    Sequence {
        strokes: Vec<Stroke>,
    },
    Consumer {
        action: ConsumerAction,
    },
    Mouse {
        action: MouseAction,
        modifiers: Modifiers,
    },
    Lighting {
        mode: u8,
        color: u8,
    },
    Delay {
        delay_ms: u16,
    },
    Opaque {
        mode: u8,
    },
}

impl Action {
    pub fn kind(&self) -> ActionKind {
        match self {
            Self::Empty => ActionKind::Empty,
            Self::Keyboard { .. } => ActionKind::Keyboard,
            Self::Sequence { .. } => ActionKind::Sequence,
            Self::Consumer { .. } => ActionKind::Consumer,
            Self::Mouse { .. } => ActionKind::Mouse,
            Self::Lighting { .. } => ActionKind::Lighting,
            Self::Delay { .. } => ActionKind::Delay,
            Self::Opaque { .. } => ActionKind::Opaque,
        }
    }

    pub fn short_label(&self) -> String {
        match self {
            Self::Empty => "Empty".into(),
            Self::Keyboard { modifiers, usage } => {
                let key = key_label(*usage).unwrap_or("Key");
                if modifiers.is_empty() {
                    key.into()
                } else {
                    format!("{}+{}", modifiers_label(*modifiers), key)
                }
            }
            Self::Sequence { strokes } => format!("Macro×{}", strokes.len()),
            Self::Consumer { action } => format!("{action:?}"),
            Self::Mouse { action, modifiers } => {
                if modifiers.is_empty() {
                    format!("{action:?}")
                } else {
                    format!("{}+{action:?}", modifiers_label(*modifiers))
                }
            }
            Self::Lighting { mode, color } => format!("RGB m{mode} c{color}"),
            Self::Delay { delay_ms } => format!("{delay_ms} ms"),
            Self::Opaque { mode } => format!("Opaque/{mode}"),
        }
    }
}

fn modifiers_label(m: Modifiers) -> String {
    let mut parts = Vec::new();
    if m.contains(Modifiers::CTRL) {
        parts.push("Ctrl");
    }
    if m.contains(Modifiers::SHIFT) {
        parts.push("Shift");
    }
    if m.contains(Modifiers::ALT) {
        parts.push("Alt");
    }
    if m.contains(Modifiers::GUI) {
        parts.push("Win");
    }
    parts.join("+")
}

/// USB HID Keyboard/Keypad usage entry for the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyChoice {
    pub label: &'static str,
    pub usage: u8,
}

pub fn key_label(usage: u8) -> Option<&'static str> {
    KEY_CHOICES.iter().find(|k| k.usage == usage).map(|k| k.label)
}

/// Observable English labels aligned with the original UI vocabulary.
/// Usages follow USB HID Keyboard/Keypad page (0x07).
/// Linux evdev keycode -> HID keyboard usage (kernel `hid_keyboard` table,
/// inverted). Used by the shortcut recorder to translate captured key events.
pub fn hid_usage_from_evdev(code: u16) -> Option<u8> {
    const MAP: &[(u16, u8)] = &[
        // Letters
        (30, 0x04), (48, 0x05), (46, 0x06), (32, 0x07), (18, 0x08),
        (33, 0x09), (34, 0x0a), (35, 0x0b), (23, 0x0c), (36, 0x0d),
        (37, 0x0e), (38, 0x0f), (50, 0x10), (49, 0x11), (24, 0x12),
        (25, 0x13), (16, 0x14), (19, 0x15), (31, 0x16), (20, 0x17),
        (22, 0x18), (47, 0x19), (17, 0x1a), (45, 0x1b), (21, 0x1c),
        (44, 0x1d),
        // Digits 1..0
        (2, 0x1e), (3, 0x1f), (4, 0x20), (5, 0x21), (6, 0x22),
        (7, 0x23), (8, 0x24), (9, 0x25), (10, 0x26), (11, 0x27),
        // Control / punctuation
        (28, 0x28), (1, 0x29), (14, 0x2a), (15, 0x2b), (57, 0x2c),
        (12, 0x2d), (13, 0x2e), (26, 0x2f), (27, 0x30), (43, 0x31),
        (39, 0x33), (40, 0x34), (41, 0x35), (51, 0x36), (52, 0x37),
        (53, 0x38), (58, 0x39),
        // F1-F12
        (59, 0x3a), (60, 0x3b), (61, 0x3c), (62, 0x3d), (63, 0x3e),
        (64, 0x3f), (65, 0x40), (66, 0x41), (67, 0x42), (68, 0x43),
        (87, 0x44), (88, 0x45),
        // F13-F24
        (183, 0x68), (184, 0x69), (185, 0x6a), (186, 0x6b), (187, 0x6c),
        (188, 0x6d), (189, 0x6e), (190, 0x6f), (191, 0x70), (192, 0x71),
        (193, 0x72), (194, 0x73),
        // Navigation cluster
        (99, 0x46), (70, 0x47), (119, 0x48), (110, 0x49), (102, 0x4a),
        (104, 0x4b), (111, 0x4c), (107, 0x4d), (109, 0x4e), (106, 0x4f),
        (105, 0x50), (108, 0x51), (103, 0x52),
        // Keypad
        (69, 0x53), (98, 0x54), (55, 0x55), (74, 0x56), (78, 0x57),
        (96, 0x58), (79, 0x59), (80, 0x5a), (81, 0x5b), (75, 0x5c),
        (76, 0x5d), (77, 0x5e), (71, 0x5f), (72, 0x60), (73, 0x61),
        (82, 0x62), (83, 0x63),
        // ISO 102nd key, Menu
        (86, 0x64), (127, 0x65),
    ];
    MAP.iter().find(|&&(c, _)| c == code).map(|&(_, u)| u)
}

pub const KEY_CHOICES: &[KeyChoice] = &[
    KeyChoice { label: "A", usage: 0x04 },
    KeyChoice { label: "B", usage: 0x05 },
    KeyChoice { label: "C", usage: 0x06 },
    KeyChoice { label: "D", usage: 0x07 },
    KeyChoice { label: "E", usage: 0x08 },
    KeyChoice { label: "F", usage: 0x09 },
    KeyChoice { label: "G", usage: 0x0A },
    KeyChoice { label: "H", usage: 0x0B },
    KeyChoice { label: "I", usage: 0x0C },
    KeyChoice { label: "J", usage: 0x0D },
    KeyChoice { label: "K", usage: 0x0E },
    KeyChoice { label: "L", usage: 0x0F },
    KeyChoice { label: "M", usage: 0x10 },
    KeyChoice { label: "N", usage: 0x11 },
    KeyChoice { label: "O", usage: 0x12 },
    KeyChoice { label: "P", usage: 0x13 },
    KeyChoice { label: "Q", usage: 0x14 },
    KeyChoice { label: "R", usage: 0x15 },
    KeyChoice { label: "S", usage: 0x16 },
    KeyChoice { label: "T", usage: 0x17 },
    KeyChoice { label: "U", usage: 0x18 },
    KeyChoice { label: "V", usage: 0x19 },
    KeyChoice { label: "W", usage: 0x1A },
    KeyChoice { label: "X", usage: 0x1B },
    KeyChoice { label: "Y", usage: 0x1C },
    KeyChoice { label: "Z", usage: 0x1D },
    KeyChoice { label: "1", usage: 0x1E },
    KeyChoice { label: "2", usage: 0x1F },
    KeyChoice { label: "3", usage: 0x20 },
    KeyChoice { label: "4", usage: 0x21 },
    KeyChoice { label: "5", usage: 0x22 },
    KeyChoice { label: "6", usage: 0x23 },
    KeyChoice { label: "7", usage: 0x24 },
    KeyChoice { label: "8", usage: 0x25 },
    KeyChoice { label: "9", usage: 0x26 },
    KeyChoice { label: "0", usage: 0x27 },
    KeyChoice { label: "Enter", usage: 0x28 },
    KeyChoice { label: "Escape", usage: 0x29 },
    KeyChoice { label: "Backspace", usage: 0x2A },
    KeyChoice { label: "Tab", usage: 0x2B },
    KeyChoice { label: "Space", usage: 0x2C },
    KeyChoice { label: "F1", usage: 0x3A },
    KeyChoice { label: "F2", usage: 0x3B },
    KeyChoice { label: "F3", usage: 0x3C },
    KeyChoice { label: "F4", usage: 0x3D },
    KeyChoice { label: "F5", usage: 0x3E },
    KeyChoice { label: "F6", usage: 0x3F },
    KeyChoice { label: "F7", usage: 0x40 },
    KeyChoice { label: "F8", usage: 0x41 },
    KeyChoice { label: "F9", usage: 0x42 },
    KeyChoice { label: "F10", usage: 0x43 },
    KeyChoice { label: "F11", usage: 0x44 },
    KeyChoice { label: "F12", usage: 0x45 },
    KeyChoice { label: "F13", usage: 0x68 },
    KeyChoice { label: "F14", usage: 0x69 },
    KeyChoice { label: "F15", usage: 0x6a },
    KeyChoice { label: "F16", usage: 0x6b },
    KeyChoice { label: "F17", usage: 0x6c },
    KeyChoice { label: "F18", usage: 0x6d },
    KeyChoice { label: "F19", usage: 0x6e },
    KeyChoice { label: "F20", usage: 0x6f },
    KeyChoice { label: "F21", usage: 0x70 },
    KeyChoice { label: "F22", usage: 0x71 },
    KeyChoice { label: "F23", usage: 0x72 },
    KeyChoice { label: "F24", usage: 0x73 },
    KeyChoice { label: "Print Screen", usage: 0x46 },
    KeyChoice { label: "Scroll Lock", usage: 0x47 },
    KeyChoice { label: "Pause", usage: 0x48 },
    KeyChoice { label: "Insert", usage: 0x49 },
    KeyChoice { label: "Home", usage: 0x4A },
    KeyChoice { label: "Page Up", usage: 0x4B },
    KeyChoice { label: "Delete", usage: 0x4C },
    KeyChoice { label: "End", usage: 0x4D },
    KeyChoice { label: "Page Down", usage: 0x4E },
    KeyChoice { label: "Right", usage: 0x4F },
    KeyChoice { label: "Left", usage: 0x50 },
    KeyChoice { label: "Down", usage: 0x51 },
    KeyChoice { label: "Up", usage: 0x52 },
    KeyChoice { label: "Num Lock", usage: 0x53 },
    KeyChoice { label: "Caps Lock", usage: 0x39 },
    KeyChoice { label: "Keypad /", usage: 0x54 },
    KeyChoice { label: "Keypad *", usage: 0x55 },
    KeyChoice { label: "Keypad -", usage: 0x56 },
    KeyChoice { label: "Keypad +", usage: 0x57 },
    KeyChoice { label: "Keypad Enter", usage: 0x58 },
    KeyChoice { label: "Keypad 1", usage: 0x59 },
    KeyChoice { label: "Keypad 2", usage: 0x5A },
    KeyChoice { label: "Keypad 3", usage: 0x5B },
    KeyChoice { label: "Keypad 4", usage: 0x5C },
    KeyChoice { label: "Keypad 5", usage: 0x5D },
    KeyChoice { label: "Keypad 6", usage: 0x5E },
    KeyChoice { label: "Keypad 7", usage: 0x5F },
    KeyChoice { label: "Keypad 8", usage: 0x60 },
    KeyChoice { label: "Keypad 9", usage: 0x61 },
    KeyChoice { label: "Keypad 0", usage: 0x62 },
    KeyChoice { label: "Keypad .", usage: 0x63 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_choices_cover_required_set() {
        assert!(KEY_CHOICES.iter().any(|k| k.label == "A" && k.usage == 0x04));
        assert!(KEY_CHOICES.iter().any(|k| k.label == "F12"));
        assert!(KEY_CHOICES.iter().any(|k| k.label == "Caps Lock"));
        assert!(KEY_CHOICES.iter().any(|k| k.label == "Keypad Enter"));
    }

    #[test]
    fn evdev_to_hid_usage() {
        assert_eq!(hid_usage_from_evdev(30), Some(0x04)); // KEY_A
        assert_eq!(hid_usage_from_evdev(1), Some(0x29)); // KEY_ESC
        assert_eq!(hid_usage_from_evdev(88), Some(0x45)); // KEY_F12
        assert_eq!(hid_usage_from_evdev(103), Some(0x52)); // KEY_UP
        assert_eq!(hid_usage_from_evdev(183), Some(0x68)); // KEY_F13
        assert_eq!(hid_usage_from_evdev(194), Some(0x73)); // KEY_F24
        assert_eq!(hid_usage_from_evdev(465), None);
        assert!(KEY_CHOICES.iter().any(|k| k.label == "Escape"));
    }

    #[test]
    fn action_kind_mapping() {
        assert_eq!(Action::Empty.kind(), ActionKind::Empty);
        assert_eq!(
            Action::Keyboard {
                modifiers: Modifiers::CTRL,
                usage: 0x04
            }
            .kind(),
            ActionKind::Keyboard
        );
        assert_eq!(Action::Opaque { mode: 7 }.kind(), ActionKind::Opaque);
    }
}
