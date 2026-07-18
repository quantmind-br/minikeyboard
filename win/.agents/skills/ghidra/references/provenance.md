# Provenance & Clean-Room Contract

## License status of the upstream source

The behavioral requirements for this skill were derived from the
`ByteLandTechnology/headless-ghidra` skill family. That repository carries **no
grant of license**:

- No `LICENSE`, `COPYING`, or `NOTICE` file exists at the repository root or in
  any subproject (verified at HEAD `9beaf6ca` and historical revisions).
- The npm package manifest declares `"license": "UNLICENSED"`
  (`ghidra-agent-cli/npm/main/package.json`).
- The Rust manifest `ghidra-agent-cli/Cargo.toml` declares **no** `license`
  field.
- GitHub API reports `license: null` for the repository.

Absent an explicit license, no permission is granted to copy, redistribute, or
create derivative works from the upstream code, scripts, fixtures, or
substantial prose.

## Clean-room rule

Everything shipped under `ghidra/` is an **independent, clean-room
reimplementation** built from observed behavior and public documentation:

- **No upstream Rust, Java, JavaScript, Python, shell, YAML, or Markdown is
  copied or paraphrased at the expression level.** Only the *ideas* (which
  operations are useful, what artifacts they should produce, the recovery order
  for single-function analysis) are reused — ideas are not copyrightable.
- All Python (`src/ghidra_skill/`), Java Ghidra scripts (`scripts/ghidra/`),
  fixtures (`tests/fixtures/`), operational text, help/errors, schemas, and eval
  prompts are written from scratch for this skill.
- Where the consolidated design diverges (JSON instead of YAML; Python instead
  of Rust; public state names instead of P0-P6; single skill instead of 15),
  the divergence is deliberate and documented in `upstream-matrix.md`.

If a license later appears upstream, this implementation remains clean-room and
is **not** a derivative work; no upstream expression will be back-ported.

## Behavioral sources consulted

| Source | Revision(s) | What was observed (behavior only) |
|---|---|---|
| `headless-ghidra/SKILL.md` | `9beaf6ca`, `309a1ee8` | orchestration flow, artifact layout, gate concept |
| `ghidra-agent-cli/SKILL.md` + `src/help.rs` | `9beaf6ca`, `e71c116a`, `c429d97` | command tree names, flag names, output-envelope shape, lock behavior |
| `headless-ghidra-{intake,baseline,evidence,discovery,batch-decompile,analyze-function}/SKILL.md` | `9beaf6ca` | per-phase inputs/outputs/exit expectations |
| `headless-ghidra-{scope,frida-verify}/SKILL.md` | `e71c116a`, `0aaee143` | absorbed capabilities (scope entries, Frida I/O verify) |
| `headless-ghidra-{auto-evolution,frida-evidence,frida-runtime-injection,progressive-decompilation,script-review}/SKILL.md` | `309a1ee8`, `30439c12` | improvement review, dynamic evidence import, runtime capture scenarios, progressive compare, script governance |
| upstream `ghidra-scripts/*.java`, `frida-scripts/*.js` | `9beaf6ca` | which Ghidra/Frida operations exist (names only; no source reused) |

The upstream tree was cloned to a scratch directory strictly for behavioral
observation and is not vendored into this repository.

## Toolchain / third-party notes

- Ghidra (Apache-2.0) is an external dependency invoked via
  `support/analyzeHeadless`; the skill never bundles or redistributes it.
- The Java scripts use `gson` already present in the Ghidra install
  (`Ghidra/Framework/Generic/lib/gson-2.13.2.jar`); no third-party JARs are
  vendored.
- Frida (wxWindows/LGPL variant) is an optional external dependency; the skill
  invokes it via the user's own installation and never bundles it.
- The Python CLI is stdlib-only (no runtime dependencies).
