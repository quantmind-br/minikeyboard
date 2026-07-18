# Upstream Skill Matrix (15/15)

Behavioral-requirements provenance for the consolidated `ghidra` skill. The
upstream source is the `ByteLandTechnology/headless-ghidra` skill family. This
matrix records **what each historical entry did** so the single `ghidra` skill
can cover every useful capability. No upstream code, script, fixture, or prose
is copied; see `provenance.md` for the clean-room contract.

## Source revisions

| Tag / ref | SHA | Skills present |
|---|---|---|
| catalog / `main` HEAD (v1.8.0) | `9beaf6ca0379c35ed162450a95e53fdd81048bef` | headless-ghidra, ghidra-agent-cli, intake, baseline, evidence, discovery, batch-decompile, analyze-function |
| pre-P0-P4 reorg | `309a1ee81931973f46b18b0ae59dadcfa91b891d` | headless-ghidra, intake, evidence, auto-evolution, frida-evidence, frida-runtime-injection, progressive-decompilation, script-review |
| frida-runtime-injection present | `30439c12c285593b8abab67e9241e89ed6c210ed` | (as above; runtime-injection confirmed) |
| scope sub-skill present | `e71c116a4c2511c270be2ed47ca0d46c95634ce0` | + ghidra-agent-cli, baseline, batch-decompile, discovery, frida-verify, scope |
| frida-verify created | `0aaee143235e2744f65e4c2b796d77e716453c17` | frida-verify (P6) present, no ghidra-agent-cli yet |
| frida-verify removed / docs overhaul | `c429d97781bb0fa7f4d2ec67a57d1a355879d9b0` | frida-verify absent; absorbed into baseline/runtime + compare |

## Catalog vs. current source

The `skills.sh` catalog lists **15** skill entries for this family. The current
upstream source at HEAD (`9beaf6ca`, v1.8.0) contains only **8** of them
(orchestrator, the 5 P0-P4 phase skills, single-function analysis, and the
`ghidra-agent-cli` helper). The remaining **7** entries exist only in earlier
revisions: `scope` and `frida-verify` were dropped, and `auto-evolution`,
`frida-evidence`, `frida-runtime-injection`, `progressive-decompilation`, and
`script-review` predate the P0-P4 reorganization. `frida-verify` survives in
intermediate versions and is then removed (no deprecated alias in HEAD).

Per-entry catalog URL (base
`https://github.com/ByteLandTechnology/headless-ghidra/tree/<sha>/<subdir>`):

| # | Entry | Catalog URL |
|---|---|---|
| 1 | headless-ghidra | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra |
| 2 | ghidra-agent-cli | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/ghidra-agent-cli |
| 3 | intake | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-intake |
| 4 | baseline | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-baseline |
| 5 | evidence | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-evidence |
| 6 | discovery | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-discovery |
| 7 | batch-decompile | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-batch-decompile |
| 8 | analyze-function | https://github.com/ByteLandTechnology/headless-ghidra/tree/9beaf6ca/headless-ghidra-analyze-function |
| 9 | scope | https://github.com/ByteLandTechnology/headless-ghidra/tree/e71c116a/headless-ghidra-scope |
| 10 | auto-evolution | https://github.com/ByteLandTechnology/headless-ghidra/tree/309a1ee8/headless-ghidra-auto-evolution |
| 11 | frida-verify | https://github.com/ByteLandTechnology/headless-ghidra/tree/0aaee143/headless-ghidra-frida-verify |
| 12 | frida-evidence | https://github.com/ByteLandTechnology/headless-ghidra/tree/309a1ee8/headless-ghidra-frida-evidence |
| 13 | frida-runtime-injection | https://github.com/ByteLandTechnology/headless-ghidra/tree/30439c12/headless-ghidra-frida-runtime-injection |
| 14 | progressive-decompilation | https://github.com/ByteLandTechnology/headless-ghidra/tree/309a1ee8/headless-ghidra-progressive-decompilation |
| 15 | script-review | https://github.com/ByteLandTechnology/headless-ghidra/tree/309a1ee8/headless-ghidra-script-review |

## Divergence notes

- The `skills.sh` catalog lists **15** entries, but the current source
  (`9beaf6ca`, v1.8.0) ships only **8**; the README/installer advertises those
  8 installable skills. The other 7 are catalog-only history.
- `frida-verify` was introduced at `0aaee14` as a **P6** sub-skill, survived
  through the P0-P6 pipeline, and was **removed** by `c429d97`; its I/O
  verification role was folded into runtime capture + compare. No deprecated
  alias remains in HEAD `help.rs`.
- `scope` was a standalone **P0.5** sub-skill at `e71c116a`. At HEAD it is only a
  `ghidra-agent-cli scope {show,set,add-entry,remove-entry}` container, no
  longer an independent skill. Confirmed in HEAD `help.rs` (scope container at
  lines 538-572).
- Upstream artifact format is **YAML-first**; the CLI global default is
  `--format yaml`. The consolidated skill standardizes on **JSON** as canonical
  (automation-friendly, stdlib-only), a deliberate divergence.
- Upstream helper is a **Rust** CLI distributed via npm (`UNLICENSED`). The
  consolidated skill reimplements the useful surface as a **stdlib-only Python**
  CLI. No Rust is reused.
- Upstream exposes phase state `P0..P6`. The consolidated skill uses public
  states `initialized|analyzed|enriched|decompiled|validated|failed` and
  translates any imported legacy phase names in `validate` output.

## Entry-by-entry matrix

### 1. headless-ghidra (orchestrator)
- **SHA:** `9beaf6ca` (also `309a1ee8`).
- **Capabilities:** workflow coordination P0-P4; dispatch rules; gate checks;
  review-pause management; shared artifact contract under `artifacts/<id>/`.
  Performs zero analysis itself.
- **Artifacts:** `pipeline-state.yaml`, routing only.
- **Overlap:** meta-layer over all phase skills.
- **Consolidation decision:** absorbed as the `ghidra` SKILL.md workflow section
  (`doctor -> init -> analyze -> inspect/query -> decompile -> compare/report`)
  and the public state machine. No separate orchestrator skill.
- **Local test:** `tests/test_cli.py` (state transitions), `tests/test_capabilities.py`.

### 2. ghidra-agent-cli (helper CLI)
- **SHA:** `9beaf6ca` (also `e71c116a`, `c429d97`).
- **Capabilities:** command tree, workspace layout, artifact semantics, output
  envelope, locking, format flags, Ghidra/Frida/gate/progress operations.
- **Artifacts:** all artifact writes and reads.
- **Overlap:** the shared interface every phase skill calls.
- **Consolidation decision:** reimplemented clean-room as the Python `ghidra`
  CLI (`src/ghidra_skill/`), JSON envelope `{status,message,data,artifacts}`,
  exit codes `0/1/2/32`, per-target exclusive lock, atomic writes.
- **Local test:** `tests/test_cli.py`, `tests/test_artifacts.py`, `tests/test_fake_headless.py`.

### 3. headless-ghidra-intake (P0)
- **SHA:** `9beaf6ca`.
- **Capabilities:** workspace init; Ghidra discovery; binary/archive inspection;
  scope definition; SHA identity; P0 gate.
- **Artifacts:** `pipeline-state.yaml`, `scope.yaml`, `targets/<id>/ghidra-projects/`, `intake/`.
- **Overlap:** absorbs `scope` (P0.5).
- **Consolidation decision:** `ghidra init`, `ghidra inspect`, `ghidra doctor`,
  `ghidra config scope`. Produces `state.json` + `intake/inspection.json`.
- **Local test:** `tests/test_cli.py`, `tests/test_artifacts.py`, `tests/test_real_ghidra.py`.

### 4. headless-ghidra-baseline (P1)
- **SHA:** `9beaf6ca`.
- **Capabilities:** Ghidra import + auto-analysis; baseline export (functions,
  callgraph, types, vtables, constants, strings, imports); runtime/hotpath
  observation groundwork; P1 gate.
- **Artifacts:** `baseline/*.yaml`, `runtime/*`.
- **Overlap:** runtime capture shared with frida-runtime-injection / frida-verify.
- **Consolidation decision:** `ghidra analyze` (+ `ExportBaseline.java`) writing
  seven `baseline/*.json` files; `ghidra list ...` reads them. Runtime capture
  routed through `ghidra frida capture`.
- **Local test:** `tests/test_real_ghidra.py` (seven exports), `tests/test_artifacts.py`.

### 5. headless-ghidra-evidence (P2)
- **SHA:** `9beaf6ca` (also `309a1ee8`).
- **Capabilities:** identify/rule-out third-party libraries; record pristine
  sources (path/hash/version/confidence/evidence); classify functions; P2 gate.
- **Artifacts:** `third-party/identified.yaml`, `third-party/pristine/...`.
- **Overlap:** feeds metadata (discovery).
- **Consolidation decision:** `ghidra evidence third-party ...` writing
  `evidence/third-party.json`; records sources pristine by path/hash without
  modifying them.
- **Local test:** `tests/test_cli.py`, `tests/test_artifacts.py`.

### 6. headless-ghidra-discovery (P3)
- **SHA:** `9beaf6ca`.
- **Capabilities:** enrich names, signatures, types, constants, strings, hotpath
  metadata; apply renames/signatures via serialized CLI; verify; P3 gate.
- **Artifacts:** `metadata/*.yaml`, `metadata/apply-records/`.
- **Overlap:** apply/verify shared with batch-decompile.
- **Consolidation decision:** `ghidra metadata rename|signature|types` +
  `ghidra metadata apply` (+ `ApplyMetadata.java`, `VerifyMetadata.java`)
  writing `metadata/{renames,signatures,types}.json`; conflict needs `--force`.
- **Local test:** `tests/test_cli.py`, `tests/test_real_ghidra.py` (apply+verify).

### 7. headless-ghidra-batch-decompile (P4)
- **SHA:** `9beaf6ca`.
- **Capabilities:** apply enriched metadata; decompile selected batch through
  Ghidra; per-function substitution records; partial-success batch; P4 gate.
- **Artifacts:** `substitution/functions/<fn_id>/`.
- **Overlap:** single-function path overlaps analyze-function.
- **Consolidation decision:** `ghidra decompile --function ... [--batch FILE]`
  (+ `DecompileFunction.java`) writing
  `decompilation/functions/<fn-id>/{source.c,record.json,analysis.json}` and
  `decompilation/batches/<run-id>.json`; preserves successes on partial failure.
- **Local test:** `tests/test_real_ghidra.py` (batch two-success one-failure).

### 8. headless-ghidra-analyze-function (single-function)
- **SHA:** `9beaf6ca`.
- **Capabilities:** strict-order single-function analysis
  `types -> constants -> vtables -> identity/signature -> decompile`; unique
  selector resolution; per-step provenance.
- **Artifacts:** per-function analysis tree.
- **Overlap:** reuses decompile + baseline reads.
- **Consolidation decision:** `ghidra function analyze --target ID SELECTOR`
  writing `analysis/functions/<fn-id>/steps.json` + report; refuses ambiguous
  selectors.
- **Local test:** `tests/test_real_ghidra.py` (five-step report).

### 9. headless-ghidra-scope (P0.5, historical)
- **SHA:** `e71c116a`.
- **Capabilities:** populate scope entries (functions / address ranges /
  symbols); scope mode `manual|auto|mixed`; P0.5 gate.
- **Artifacts:** `scope.yaml`.
- **Overlap:** fully inside intake.
- **Consolidation decision:** **history-only**. Absorbed into
  `ghidra config scope {show,set,add,remove}` and `state.json` `scope`. No P0.5
  public command. Legacy `P0.5` state translated to `intake` by `validate`.
- **Local test:** `tests/test_cli.py` (config scope), `tests/test_capabilities.py`.

### 10. headless-ghidra-auto-evolution (historical)
- **SHA:** `309a1ee8`.
- **Capabilities:** mine completed work for reusable improvements; resolve
  overlap; classify promotion `accepted|deferred|rejected` with evidence.
- **Artifacts:** Markdown review records.
- **Overlap:** maintenance discipline, not analysis.
- **Consolidation decision:** `ghidra improve review --target ID`. Never
  self-edits the skill; emits a reviewable candidate record with
  status/evidence/overlap/proposed-destination. Promotion is a separate
  maintenance change.
- **Local test:** `tests/test_cli.py`, `evals/evals.json` (auto-evolution review).

### 11. headless-ghidra-frida-verify (P6, created then removed)
- **SHA:** created `0aaee143`, removed `c429d97`.
- **Capabilities:** hook original function via Frida, record I/O, run
  reconstructed function on same inputs, compare case-by-case; gate basis;
  three-source test inputs (runtime-recorded / fuzz / manual).
- **Artifacts:** verification/compare records.
- **Overlap:** compare + runtime capture.
- **Consolidation decision:** **history-only** (no HEAD alias). I/O-record and
  compare folded into `ghidra frida capture` + `ghidra compare`. Frida stays
  opt-in and isolation-gated per `security.md`.
- **Local test:** `tests/test_frida.py` (capture/compare + external-required path).

### 12. headless-ghidra-frida-evidence (historical)
- **SHA:** `309a1ee8`.
- **Capabilities:** import-only Frida evidence normalization; replay/verify
  expectations; static-vs-dynamic conflict adjudication; provenance.
- **Artifacts:** normalized runtime evidence manifests.
- **Overlap:** evidence + compare + runtime.
- **Consolidation decision:** `ghidra frida import-evidence` + `ghidra compare`
  preserving observed/inferred/unresolved and static-vs-dynamic conflict.
- **Local test:** `tests/test_frida.py`, `tests/test_cli.py` (compare conflict).

### 13. headless-ghidra-frida-runtime-injection (historical)
- **SHA:** `30439c12`.
- **Capabilities:** reproducible CLI/headless Frida capture; reusable common
  scripts (signature, decomp-compare, call-tree, dispatch/vtable, hotpath/
  coverage); capture-manifest handoff; audit gates.
- **Artifacts:** `runtime/captures/<run-id>/{manifest.json,events.jsonl}`.
- **Overlap:** baseline runtime, frida-verify, frida-evidence.
- **Consolidation decision:** `ghidra frida {doctor,capture,trace,compare,
  import-evidence}`; manifest-driven; covers all five runtime scenarios; hard
  timeout + isolation for untrusted targets; never fabricates traces.
- **Local test:** `tests/test_frida.py`.

### 14. headless-ghidra-progressive-decompilation (Stage 6, historical)
- **SHA:** `309a1ee8`.
- **Capabilities:** one selected outside-in decompilation step at the frontier;
  incremental compare for the replaced boundary; carry-forward caveats; audit.
- **Artifacts:** per-step compare records.
- **Overlap:** decompile + compare.
- **Consolidation decision:** `ghidra compare` progressive mode requiring
  explicit target/reason/question/boundary/fallback/compare inputs.
- **Local test:** `tests/test_cli.py`, `evals/evals.json` (single-function order).

### 15. headless-ghidra-script-review (historical)
- **SHA:** `309a1ee8`.
- **Capabilities:** reusable headless Ghidra script authoring/registration/
  review; deterministic I/O; manifest-generation review; naming expectations.
- **Artifacts:** tracked script examples + review records.
- **Overlap:** script authoring discipline.
- **Consolidation decision:** `ghidra script {scaffold,lint,run}`; Java default;
  Python only when `doctor` proves PyGhidra; scripts confined to allowlisted
  dirs; symlink/traversal rejected; lint checks metadata/headless APIs.
- **Local test:** `tests/test_script_safety.py`, `tests/test_real_ghidra.py`
  (scaffold/lint/run).
