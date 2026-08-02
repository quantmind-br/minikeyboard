//! Data-driven keyboard geometries for every SPEC §4.1 tuple.

use super::config::SupportLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayKind {
    Key,
    Knob,
    Button,
    Lighting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionGeometry {
    pub logical_index: u8,
    pub row: u8,
    pub column: u8,
    pub col_span: u8,
    pub kind: DisplayKind,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Geometry {
    pub id: String,
    pub label: String,
    pub key_count: u8,
    pub extra_count: u8,
    pub subtype: u8,
    pub support: SupportLevel,
    pub positions: Vec<PositionGeometry>,
}

/// Resolved device variant after identify.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceVariant {
    pub key_count: u8,
    pub extra_count: u8,
    pub subtype: u8,
    pub geometry_id: String,
    pub support: SupportLevel,
    pub geometry: Geometry,
}

/// Known geometry tuples from SPEC §4.1.
/// Format: (key_count, extra_count) — subtype affects identity only.
const KNOWN_TUPLES: &[(u8, u8)] = &[
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 0),
    (2, 0),
    (3, 0),
    (3, 1),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
    (5, 0),
    (6, 0),
    (6, 1),
    (6, 2),
    (9, 2),
    (9, 3),
    (11, 3),
    (12, 2),
    (12, 3),
    (12, 4),
    (15, 3),
    (16, 0),
    (21, 1),
];

/// Wire layout of the validated read/write protocol: every layer carries
/// `WIRE_BASE_POSITIONS` ordinary records followed by `WIRE_EXTRA_GROUPS`
/// extra-control groups of three records (rotate-left, press, rotate-right).
pub const WIRE_BASE_POSITIONS: u8 = 15;
pub const WIRE_EXTRA_GROUPS: u8 = 3;

pub fn geometry_id(key_count: u8, extra_count: u8, subtype: u8) -> String {
    format!("k{key_count}-e{extra_count}-s{subtype}")
}

pub fn is_known_tuple(key_count: u8, extra_count: u8, _subtype: u8) -> bool {
    KNOWN_TUPLES
        .iter()
        .any(|&(k, e)| k == key_count && e == extra_count)
}

/// Build geometry for a variant tuple.
///
/// Provisional layout: row-major keys with `columns = min(max(key_count, 1), 6)`,
/// then `extra_count` knobs on the next row. Subtype never infers controls.
/// After identify capture, only the validated physical model is refined.
pub fn resolve_variant(key_count: u8, extra_count: u8, subtype: u8) -> DeviceVariant {
    let known = is_known_tuple(key_count, extra_count, subtype);
    // Known tuples stay Experimental (read-only) until capture validates geometry.
    let support = if known {
        SupportLevel::Experimental
    } else {
        SupportLevel::Unknown
    };

    let id = geometry_id(key_count, extra_count, subtype);
    let positions = provisional_positions(key_count, extra_count);
    let label = if known {
        format!("{key_count} keys + {extra_count} extra (s{subtype})")
    } else {
        format!("Unknown variant {key_count}+{extra_count} s{subtype}")
    };

    let geometry = Geometry {
        id: id.clone(),
        label,
        key_count,
        extra_count,
        subtype,
        support,
        positions,
    };

    DeviceVariant {
        key_count,
        extra_count,
        subtype,
        geometry_id: id,
        support,
        geometry,
    }
}

/// Mark the hardware-validated model as Validated once identify confirms it.
pub fn with_validated_support(mut variant: DeviceVariant, validated: bool) -> DeviceVariant {
    if validated && is_known_tuple(variant.key_count, variant.extra_count, variant.subtype) {
        variant.support = SupportLevel::Validated;
        variant.geometry.support = SupportLevel::Validated;
    }
    variant
}

/// Column count mirroring the physical key grids: macro pads with key counts
/// divisible by 3 use 3 columns (e.g. k12-e2 hardware is 4 rows x 3 columns),
/// multiples of 4 use 4 columns, everything else falls back to a single row
/// capped at 6 columns.
fn key_columns(keys: usize) -> u8 {
    match keys {
        0 => 1,
        n if n % 3 == 0 => 3,
        n if n % 4 == 0 => 4,
        n => n.clamp(1, 6) as u8,
    }
}

fn provisional_positions(key_count: u8, extra_count: u8) -> Vec<PositionGeometry> {
    let mut positions = Vec::new();
    let keys = key_count as usize;
    let columns = key_columns(keys);

    // Knobs sit above the key grid, matching the physical device.
    let knob_rows: u8 = if extra_count > 0 { 1 } else { 0 };

    let wire_extras =
        key_count <= WIRE_BASE_POSITIONS && extra_count <= WIRE_EXTRA_GROUPS;

    // Span keys across the knob columns when they divide evenly so the two
    // blocks line up (k12-e2: 6 knob records over 3 key columns, span 2).
    let knob_cols: u8 = if wire_extras { extra_count * 3 } else { extra_count };
    let key_span: u8 = if keys > 0 && knob_cols > columns && knob_cols.is_multiple_of(columns) {
        knob_cols / columns
    } else {
        1
    };

    // Extra controls (knobs) map to the wire's extra groups: three records
    // per control — rotate-left, press, rotate-right — starting at logical
    // index WIRE_BASE_POSITIONS. Hardware capture (k12-e2 factory layout:
    // VolumeDown / PlayPause / VolumeUp at wire 16..18) confirms the order.
    for e in 0..extra_count {
        if wire_extras {
            let base = WIRE_BASE_POSITIONS + e * 3;
            for (offset, glyph) in ["◀", "⏺", "▶"].iter().enumerate() {
                positions.push(PositionGeometry {
                    logical_index: base + offset as u8,
                    row: 0,
                    column: e * 3 + offset as u8,
                    col_span: 1,
                    kind: DisplayKind::Knob,
                    label: format!("E{} {glyph}", e + 1),
                });
            }
        } else {
            positions.push(PositionGeometry {
                logical_index: key_count + e,
                row: 0,
                column: e,
                col_span: 1,
                kind: DisplayKind::Knob,
                label: format!("E{}", e + 1),
            });
        }
    }

    let key_rows = if keys == 0 { 0 } else { (keys as u8).div_ceil(columns) };
    for i in 0..keys {
        let idx = i as u8;
        // Wire-to-physical mapping. Hardware capture on k12-e2
        // (tools/identify-layout.py) shows the key matrix is wired
        // column-major, bottom to top: wire 0 = bottom-left, wire 3 =
        // top-left, wire 8 = bottom-right, wire 11 = top-right.
        let (row, col) = if keys == 12 && columns == 3 {
            (key_rows - 1 - idx % key_rows, idx / key_rows)
        } else {
            (idx / columns, idx % columns)
        };
        positions.push(PositionGeometry {
            logical_index: idx,
            row: knob_rows + row,
            column: col * key_span,
            col_span: key_span,
            kind: DisplayKind::Key,
            // Label follows the physical reading order (K1 = top-left).
            label: format!("K{}", row * columns + col + 1),
        });
    }

    if positions.is_empty() {
        positions.push(PositionGeometry {
            logical_index: 0,
            row: 0,
            column: 0,
            col_span: 1,
            kind: DisplayKind::Key,
            label: "?".into(),
        });
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_tuple_ids() {
        let v = resolve_variant(6, 2, 0);
        assert_eq!(v.geometry_id, "k6-e2-s0");
        assert_eq!(v.support, SupportLevel::Experimental);
        // 6 keys + 2 knobs x 3 wire records each.
        assert_eq!(v.geometry.positions.len(), 12);
        let knob = v
            .geometry
            .positions
            .iter()
            .find(|p| p.kind == DisplayKind::Knob)
            .unwrap();
        assert_eq!(knob.logical_index, WIRE_BASE_POSITIONS);
    }

    #[test]
    fn unknown_tuple() {
        let v = resolve_variant(99, 9, 1);
        assert_eq!(v.support, SupportLevel::Unknown);
        assert!(v.geometry_id.starts_with("k99-e9-s1"));
    }

    #[test]
    fn all_spec_tuples_known() {
        for &(k, e) in KNOWN_TUPLES {
            assert!(is_known_tuple(k, e, 0), "missing {k}+{e}");
        }
    }

    #[test]
    fn twelve_keys_form_three_column_grid() {
        let v = resolve_variant(12, 0, 0);
        let keys: Vec<_> = v
            .geometry
            .positions
            .iter()
            .filter(|p| p.kind == DisplayKind::Key)
            .collect();
        assert_eq!(keys.iter().map(|p| p.column).max().unwrap(), 2);
        assert_eq!(keys.iter().map(|p| p.row).max().unwrap(), 3);
    }

    #[test]
    fn k12_e2_matches_physical_layout() {
        // Physical device: two knobs on top, 12 keys in 4 rows x 3 columns.
        let v = resolve_variant(12, 2, 0);
        let knobs: Vec<_> = v
            .geometry
            .positions
            .iter()
            .filter(|p| p.kind == DisplayKind::Knob)
            .collect();
        assert_eq!(knobs.len(), 6);
        assert!(knobs.iter().all(|p| p.row == 0));
        assert_eq!(knobs.iter().map(|p| p.column).max().unwrap(), 5);
        let keys: Vec<_> = v
            .geometry
            .positions
            .iter()
            .filter(|p| p.kind == DisplayKind::Key)
            .collect();
        assert_eq!(keys.len(), 12);
        // Keys start below the knob row and span 2 grid columns each to
        // align with the 6 knob columns: 4 rows x 3 columns.
        assert!(keys.iter().all(|p| p.row >= 1));
        assert_eq!(keys.iter().map(|p| p.row).max().unwrap(), 4);
        assert!(keys.iter().all(|p| p.col_span == 2));
        // Column-major bottom-up wiring, per hardware capture:
        // wire 0 = bottom-left (K10), wire 3 = top-left (K1),
        // wire 8 = bottom-right (K12), wire 11 = top-right (K3).
        assert_eq!((keys[0].row, keys[0].column, keys[0].label.as_str()), (4, 0, "K10"));
        assert_eq!((keys[3].row, keys[3].column, keys[3].label.as_str()), (1, 0, "K1"));
        assert_eq!((keys[8].row, keys[8].column, keys[8].label.as_str()), (4, 4, "K12"));
        assert_eq!((keys[11].row, keys[11].column, keys[11].label.as_str()), (1, 4, "K3"));
    }
}
