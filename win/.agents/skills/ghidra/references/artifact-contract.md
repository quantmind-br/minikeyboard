# Artifact contract

JSON is canonical (`schema_version: 1`). All paths are under
`<workspace>/artifacts/<target-id>/`. Writes are atomic (temp + `os.replace`);
JSONL streams are append-only under the target lock. Every record is stamped
with `schema_version`, `target_id`, `source`, and `observed_at`.

## Layout

```
targets/<id>/projects/                     Ghidra project (created by analyze)
artifacts/<id>/
  state.json
  execution-log.jsonl
  intake/
    inspection.json
    analyze.log  analyze-script.log  analyze-manifest.json
  baseline/
    functions.json  callgraph.json  types.json  vtables.json
    constants.json  strings.json  imports.json
  evidence/
    third-party.json
  metadata/
    renames.json  signatures.json  types.json
    apply.log  verify.json  apply-records/<group>-apply.json
  decompilation/
    functions/<fn-id>/{source.c,record.json,analysis.json}
    batches/<run-id>.json
  analysis/
    functions/<fn-id>/steps.json
  runtime/
    captures/<run-id>/{manifest.json,events.jsonl}
    imported/evidence.json  compare.json
  scripts/
    <name>.java  runs/<run-id>.{json,log}
  reports/
    <run-id>/{report.md,report.json}
    <run-id>-compare.json  <run-id>-improve.json
  gates/
    latest.json
  locks/
    <id>.lock
```

## state.json

```json
{
  "schema_version": 1,
  "target_id": "demo",
  "binary": { "path": "/abs/path", "sha256": "<64 hex>", "format": "elf" },
  "scope": { "mode": "full|symbols|addresses", "entries": [] },
  "status": "initialized|analyzed|enriched|decompiled|validated|failed",
  "created_at": "...Z", "updated_at": "...Z"
}
```

Loaded state is validated: `target_id` must match the requested id,
`schema_version == 1`, `status` a public state, `binary.path` a non-empty
string, `binary.sha256` 64 lowercase hex, `binary.format` present (nullable),
`scope.mode` valid, `scope.entries` a list of strings.

## intake/inspection.json

Tool versions, `format`/`arch`/`endian`, `sections`, `exports`, `imports`,
`archive_members` (with `valid`/`ordinal`/`warning`), and `warnings`. Invalid or
duplicate archive members are recorded, never silently dropped.

## baseline/*.json

Each file wraps a list under its group key with `schema_version`, `program`, and
`count`. `callgraph.json` holds `{caller, callee, caller_name, callee_name}`
edges (direct calls only).

## metadata

`renames.json` / `signatures.json` keyed by `address`; `types.json` keyed by
`name`. Each entry carries `provenance`. `apply` writes
`apply-records/<group>-apply.json` and `verify.json` (verified/mismatched).

## decompilation

`functions/<fn-id>/record.json` has `status` (`succeeded|failed`),
`source_path`, and the Ghidra `manifest`. `batches/<run-id>.json` summarizes
`succeeded/failed/skipped/total` and preserves per-item results.

## runtime

`captures/<run-id>/manifest.json` records scenario, trust, isolation, and the
`events.jsonl` path. Events are only ever real observations; no synthetic events
are written.

## gates/latest.json

`overall` plus per-gate status for `intake`, `baseline`, `evidence`,
`metadata`, `decompilation`. A gate not exercised by the requested flow is
`not_applicable`, never `passed`.
