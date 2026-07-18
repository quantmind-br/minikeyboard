//! Lossless 50-byte record codec for the validated Mini Keyboard protocol.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::domain::action::{
    Action, ActionKind, ConsumerAction, Modifiers, MouseAction, Stroke,
};
use crate::domain::config::RECORD_SIZE;

const PAYLOAD_START: usize = 10;
const MAX_STROKES: usize = (RECORD_SIZE - PAYLOAD_START) / 2;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodecError {
    #[error("action kind {0:?} is not verified for encoding")]
    UnverifiedAction(ActionKind),

    #[error("opaque actions cannot be edited safely")]
    UnsupportedOpaqueEdit,

    #[error("lighting mode {0} out of range 0..=5")]
    LightingModeRange(u8),

    #[error("lighting color {0} out of range 0..=7")]
    LightingColorRange(u8),

    #[error("macro must contain 1..={MAX_STROKES} strokes")]
    SequenceLength,

    #[error("mouse wheel accepts at most one of Ctrl, Shift, or Alt")]
    MouseModifier,

    #[error("codec: {0}")]
    Message(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedRecord {
    pub action: Action,
    pub delay_ms: Option<u16>,
    pub raw: [u8; RECORD_SIZE],
}

/// Decode one 50-byte device record while retaining the original bytes.
pub fn decode_record(raw: [u8; RECORD_SIZE]) -> DecodedRecord {
    let delay_ms = delay_candidate(&raw);

    // The legacy clear operation preserves the identity/category prefix and zeroes bytes 4..49.
    if raw[4..].iter().all(|&byte| byte == 0) {
        return DecodedRecord {
            action: Action::Empty,
            delay_ms: None,
            raw,
        };
    }

    let action = match raw[3] {
        1 => decode_keyboard(&raw),
        2 => decode_consumer(&raw),
        3 => decode_mouse(&raw),
        5 if raw[0] == 0xFE && raw[1] == 0xB0 && raw[9] == 1 => {
            let packed = raw[11];
            let mode = (packed >> 4) & 0x0F;
            let color = packed & 0x0F;
            if mode <= 5 && color <= 7 {
                Action::Lighting { mode, color }
            } else {
                Action::Opaque { mode: raw[3] }
            }
        }
        mode => Action::Opaque { mode },
    };

    DecodedRecord {
        action,
        delay_ms,
        raw,
    }
}

fn decode_keyboard(raw: &[u8; RECORD_SIZE]) -> Action {
    let count = raw[9] as usize;
    if count == 0 {
        return Action::Empty;
    }
    if count > MAX_STROKES {
        return Action::Opaque { mode: raw[3] };
    }

    let strokes: Vec<Stroke> = (0..count)
        .map(|index| {
            let offset = PAYLOAD_START + index * 2;
            Stroke {
                modifiers: Modifiers::from_bits_retain(raw[offset]),
                usage: raw[offset + 1],
            }
        })
        .collect();

    if strokes.len() == 1 {
        Action::Keyboard {
            modifiers: strokes[0].modifiers,
            usage: strokes[0].usage,
        }
    } else {
        Action::Sequence { strokes }
    }
}

fn decode_consumer(raw: &[u8; RECORD_SIZE]) -> Action {
    if raw[9] != 1 {
        return Action::Opaque { mode: raw[3] };
    }
    let usage = u16::from_le_bytes([raw[10], raw[11]]);
    let action = match usage {
        0x00CD => ConsumerAction::PlayPause,
        0x00B7 => ConsumerAction::Stop,
        0x00B6 => ConsumerAction::PreviousTrack,
        0x00B5 => ConsumerAction::NextTrack,
        0x00E2 => ConsumerAction::Mute,
        0x00E9 => ConsumerAction::VolumeUp,
        0x00EA => ConsumerAction::VolumeDown,
        0x0192 => ConsumerAction::Calculator,
        _ => return Action::Opaque { mode: raw[3] },
    };
    Action::Consumer { action }
}

fn decode_mouse(raw: &[u8; RECORD_SIZE]) -> Action {
    // Hardware readback stores 4 (mouse payload length: buttons/x/y/wheel);
    // 1 appears in frames produced by older versions of this app.
    if raw[9] != 4 && raw[9] != 1 {
        return Action::Opaque { mode: raw[3] };
    }

    let modifiers = Modifiers::from_bits_retain(raw[10]);
    let action = match (raw[11], raw[14]) {
        (1, 0) => MouseAction::LeftClick,
        (4, 0) => MouseAction::MiddleClick,
        (2, 0) => MouseAction::RightClick,
        (0, 1) => MouseAction::WheelUp,
        (0, 0xFF) => MouseAction::WheelDown,
        _ => return Action::Opaque { mode: raw[3] },
    };
    Action::Mouse { action, modifiers }
}

fn delay_candidate(raw: &[u8; RECORD_SIZE]) -> Option<u16> {
    let value = u16::from_le_bytes([raw[4], raw[5]]);
    (value != 0).then_some(value)
}

fn clean_action(base: [u8; RECORD_SIZE], category: u8, delay_ms: Option<u16>) -> [u8; RECORD_SIZE] {
    let mut out = base;
    out[3] = category;
    out[4..].fill(0);
    if let Some(delay) = delay_ms {
        out[4..6].copy_from_slice(&delay.to_le_bytes());
    }
    out
}

/// Encode an action over an existing record, preserving position/layer identity bytes.
pub fn encode_record(
    base: [u8; RECORD_SIZE],
    action: &Action,
    delay_ms: Option<u16>,
    verified: &BTreeSet<ActionKind>,
) -> Result<[u8; RECORD_SIZE], CodecError> {
    let kind = action.kind();
    if matches!(action, Action::Opaque { .. }) {
        return Err(CodecError::UnsupportedOpaqueEdit);
    }
    if !verified.contains(&kind) {
        return Err(CodecError::UnverifiedAction(kind));
    }

    match action {
        Action::Empty => {
            let mut out = base;
            out[4..].fill(0);
            Ok(out)
        }
        Action::Keyboard { modifiers, usage } => {
            let mut out = clean_action(base, 1, delay_ms);
            out[9] = 1;
            out[10] = modifiers.bits();
            out[11] = *usage;
            Ok(out)
        }
        Action::Sequence { strokes } => {
            if strokes.is_empty() || strokes.len() > MAX_STROKES {
                return Err(CodecError::SequenceLength);
            }
            let mut out = clean_action(base, 1, delay_ms);
            out[9] = strokes.len() as u8;
            for (index, stroke) in strokes.iter().enumerate() {
                let offset = PAYLOAD_START + index * 2;
                out[offset] = stroke.modifiers.bits();
                out[offset + 1] = stroke.usage;
            }
            Ok(out)
        }
        Action::Consumer { action } => {
            let usage: u16 = match action {
                ConsumerAction::PlayPause => 0x00CD,
                ConsumerAction::Stop => 0x00B7,
                ConsumerAction::PreviousTrack => 0x00B6,
                ConsumerAction::NextTrack => 0x00B5,
                ConsumerAction::Mute => 0x00E2,
                ConsumerAction::VolumeUp => 0x00E9,
                ConsumerAction::VolumeDown => 0x00EA,
                ConsumerAction::Calculator => 0x0192,
            };
            let mut out = clean_action(base, 2, delay_ms);
            out[9] = 1;
            out[10..12].copy_from_slice(&usage.to_le_bytes());
            Ok(out)
        }
        Action::Mouse { action, modifiers } => {
            let allowed = Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT;
            if !allowed.contains(*modifiers) || modifiers.bits().count_ones() > 1 {
                return Err(CodecError::MouseModifier);
            }
            if !matches!(action, MouseAction::WheelUp | MouseAction::WheelDown)
                && !modifiers.is_empty()
            {
                return Err(CodecError::MouseModifier);
            }
            let mut out = clean_action(base, 3, delay_ms);
            // 4 = mouse payload length; the firmware canonicalizes this byte
            // on readback, so writing 4 keeps write-then-verify byte-exact.
            out[9] = 4;
            out[10] = modifiers.bits();
            match action {
                MouseAction::LeftClick => out[11] = 1,
                MouseAction::MiddleClick => out[11] = 4,
                MouseAction::RightClick => out[11] = 2,
                MouseAction::WheelUp => out[14] = 1,
                MouseAction::WheelDown => out[14] = 0xFF,
            }
            Ok(out)
        }
        Action::Lighting { mode, color } => {
            if *mode > 5 {
                return Err(CodecError::LightingModeRange(*mode));
            }
            if *color > 7 {
                return Err(CodecError::LightingColorRange(*color));
            }
            let layer = base[2];
            let mut out = clean_action(base, 5, delay_ms);
            out[0] = 0xFE;
            out[1] = 0xB0;
            out[2] = layer;
            out[9] = 1;
            out[11] = (mode << 4) | color;
            Ok(out)
        }
        Action::Delay { delay_ms } => {
            let mut out = base;
            out[4..6].copy_from_slice(&delay_ms.to_le_bytes());
            Ok(out)
        }
        Action::Opaque { .. } => Err(CodecError::UnsupportedOpaqueEdit),
    }
}

/// Action mappings recovered from the validated 1189:8842 implementation and hardware readback.
pub fn verified_action_kinds() -> BTreeSet<ActionKind> {
    BTreeSet::from([
        ActionKind::Empty,
        ActionKind::Keyboard,
        ActionKind::Sequence,
        ActionKind::Consumer,
        ActionKind::Mouse,
        ActionKind::Lighting,
        ActionKind::Delay,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_record(category: u8) -> [u8; RECORD_SIZE] {
        let mut raw = [0u8; RECORD_SIZE];
        raw[..4].copy_from_slice(&[7, 0, 2, category]);
        raw
    }

    #[test]
    fn decodes_cleared_record_with_identity_prefix() {
        assert_eq!(decode_record(identity_record(1)).action, Action::Empty);
    }

    #[test]
    fn keyboard_roundtrip_preserves_identity() {
        let base = identity_record(0x99);
        let action = Action::Keyboard {
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
            usage: 0x04,
        };
        let raw = encode_record(base, &action, Some(250), &verified_action_kinds()).unwrap();
        assert_eq!(&raw[..3], &[7, 0, 2]);
        assert_eq!(raw[3], 1);
        assert_eq!(decode_record(raw).action, action);
        assert_eq!(decode_record(raw).delay_ms, Some(250));
    }

    #[test]
    fn sequence_roundtrip() {
        let action = Action::Sequence {
            strokes: vec![
                Stroke { modifiers: Modifiers::empty(), usage: 0x29 },
                Stroke { modifiers: Modifiers::CTRL | Modifiers::SHIFT, usage: 0x19 },
            ],
        };
        let raw = encode_record(identity_record(1), &action, None, &verified_action_kinds()).unwrap();
        assert_eq!(raw[9], 2);
        assert_eq!(decode_record(raw).action, action);
    }

    #[test]
    fn consumer_vectors_roundtrip() {
        for action in [
            ConsumerAction::PlayPause,
            ConsumerAction::Stop,
            ConsumerAction::PreviousTrack,
            ConsumerAction::NextTrack,
            ConsumerAction::Mute,
            ConsumerAction::VolumeUp,
            ConsumerAction::VolumeDown,
            ConsumerAction::Calculator,
        ] {
            let semantic = Action::Consumer { action };
            let raw = encode_record(identity_record(2), &semantic, None, &verified_action_kinds()).unwrap();
            assert_eq!(decode_record(raw).action, semantic);
        }
    }

    #[test]
    fn mouse_vectors_roundtrip() {
        for action in [
            MouseAction::LeftClick,
            MouseAction::MiddleClick,
            MouseAction::RightClick,
            MouseAction::WheelUp,
            MouseAction::WheelDown,
        ] {
            let modifiers = if matches!(action, MouseAction::WheelUp) {
                Modifiers::CTRL
            } else {
                Modifiers::empty()
            };
            let semantic = Action::Mouse { action, modifiers };
            let raw = encode_record(identity_record(3), &semantic, None, &verified_action_kinds()).unwrap();
            assert_eq!(decode_record(raw).action, semantic);
        }
    }

    #[test]
    fn lighting_roundtrip() {
        let action = Action::Lighting { mode: 2, color: 3 };
        let raw = encode_record(identity_record(5), &action, None, &verified_action_kinds()).unwrap();
        assert_eq!(&raw[..3], &[0xFE, 0xB0, 2]);
        assert_eq!(decode_record(raw).action, action);
    }

    #[test]
    fn clear_preserves_routing_prefix() {
        let mut base = [0xFF; RECORD_SIZE];
        base[..4].copy_from_slice(&[3, 0, 1, 1]);
        let raw = encode_record(base, &Action::Empty, None, &verified_action_kinds()).unwrap();
        assert_eq!(&raw[..4], &[3, 0, 1, 1]);
        assert!(raw[4..].iter().all(|&byte| byte == 0));
        assert_eq!(decode_record(raw).action, Action::Empty);
    }
}
