# ghidra skill — development

Clean-room headless Ghidra reverse-engineering skill. User-facing docs are in
`SKILL.md` and `references/`. This file is for development and validation only.

## Layout

- `SKILL.md` — the single trigger surface.
- `capabilities.json` — source of truth for capabilities (33 entries).
- `src/ghidra_skill/` — stdlib-only Python CLI (`ghidra` / `python -m ghidra_skill`).
- `scripts/ghidra/*.java` — clean-room Ghidra scripts (Gson from the Ghidra install).
- `scripts/validate_capabilities.py` — capabilities/CLI/docs consistency gate.
- `scripts/build_fixtures.py` — deterministic clean-room fixture builder.
- `references/` — on-demand docs.
- `tests/` — unit, contract, real-Ghidra, and Frida tests.
- `evals/` — skill evaluation harness.

## Validation

```bash
# static contracts (no real Ghidra)
python -m pytest tests/test_cli.py tests/test_capabilities.py tests/test_artifacts.py \
  tests/test_install_guidance.py tests/test_english_only.py tests/test_script_safety.py \
  tests/test_fake_headless.py -q
python scripts/validate_capabilities.py --check
python -m ghidra_skill --help
python -m ghidra_skill doctor --format json

# fixtures
python scripts/build_fixtures.py --out "$(mktemp -d)"

# real Ghidra (needs Ghidra 12.x / JDK 21+)
GHIDRA_HOME=/opt/ghidra python -m pytest tests/test_real_ghidra.py -m requires_ghidra -q

# frida (deterministic with or without Frida)
python -m pytest tests/test_frida.py -q

# evals
python -m pytest evals/test_eval_contract.py -q
```

Run from this directory. Tests write artifacts only to temp dirs; no command
writes Ghidra projects inside the skill.

## Regenerating the SKILL capabilities table

```bash
python scripts/validate_capabilities.py --sync    # rewrite the generated region
python scripts/validate_capabilities.py --check    # verify it matches
```

## Provenance

This skill is an independent, clean-room reimplementation of the behavioral
requirements of the upstream `headless-ghidra` skill family, which carries no
license grant. No upstream code, scripts, fixtures, or substantial prose are
copied. See `references/provenance.md` and `references/upstream-matrix.md`.
