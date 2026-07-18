#!/usr/bin/env python3
"""Validate the capabilities contract for the ghidra skill.

Fails (exit 1) when any of these diverge:
  - a shipped/shipped-partial capability has no existing entrypoint file
  - a shipped/shipped-partial capability names a test that does not exist
  - an external-required capability lacks a dependency and diagnostic
  - a CLI command in capabilities.json is not registered by the CLI
  - a SKILL/help operation is announced without a backing capability
  - a Markdown link in SKILL.md / references points at a missing file
  - the generated <!-- capabilities:start --> region in SKILL.md diverges from
    capabilities.json

Usage: python scripts/validate_capabilities.py --check
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CAPS = ROOT / "capabilities.json"
SKILL = ROOT / "SKILL.md"

START = "<!-- capabilities:start -->"
END = "<!-- capabilities:end -->"
VALID_STATUS = {"shipped", "shipped-partial", "external-required"}


def load_caps() -> dict:
    return json.loads(CAPS.read_text())


def render_block(doc: dict) -> str:
    lines = ["| Capability | Status | CLI | Safety tier |", "|---|---|---|---|"]
    for c in doc["capabilities"]:
        cli = "; ".join(f"`ghidra {x}`" for x in c.get("cliCommands", []))
        lines.append(f"| {c.get('id')} | {c.get('status')} | {cli} | {c.get('safetyTier')} |")
    return "\n".join(lines)


def registered_cli_commands() -> set[str] | None:
    """Import the CLI registry and return the set of registered command paths.

    Returns None if the CLI package is not importable yet (scaffolding phase).
    """
    src = ROOT / "src"
    sys.path.insert(0, str(src))
    try:
        from ghidra_skill.cli import command_paths  # type: ignore
    except Exception:
        return None
    finally:
        try:
            sys.path.remove(str(src))
        except ValueError:
            pass
    return set(command_paths())


def check() -> list[str]:
    errors: list[str] = []
    doc = load_caps()
    caps = doc["capabilities"]

    ids = [c.get("id") for c in caps if c.get("id")]
    for dup in {i for i in ids if ids.count(i) > 1}:
        errors.append(f"duplicate capability id: {dup}")

    REQUIRED = ["id", "status", "entrypoint", "cliCommands", "inputs",
                "outputs", "safetyTier", "limitations", "tests"]
    VALID_TIERS = {"static-read", "static-ghidra", "dynamic-owned", "dynamic-untrusted"}
    for c in caps:
        cid = c.get("id", "<no-id>")
        for field in REQUIRED:
            if field not in c:
                errors.append(f"{cid}: missing required field {field!r}")
        status = c.get("status")
        if status not in VALID_STATUS:
            errors.append(f"{cid}: invalid status {status!r}")
            continue
        if c.get("safetyTier") not in VALID_TIERS:
            errors.append(f"{cid}: invalid safetyTier {c.get('safetyTier')!r}")

        # entrypoint must exist for ALL capabilities (shipped* run it;
        # external-required must contain the code that returns external-required)
        entry = ROOT / c.get("entrypoint", "")
        if not c.get("entrypoint") or not entry.is_file():
            errors.append(f"{cid}: entrypoint missing: {c.get('entrypoint')!r}")

        # every capability MUST name at least one test, and each named test
        # (file + optional ::symbol) must exist. external-required tests are the
        # deterministic diagnostic tests for Ghidra/Frida/PyGhidra absence.
        tests = c.get("tests", [])
        if not tests:
            errors.append(f"{cid}: no tests named")
        for t in tests:
            tpath, _, tname = t.partition("::")
            tfile = ROOT / tpath
            if not tfile.is_file():
                errors.append(f"{cid}: named test file missing: {tpath}")
                continue
            if tname:
                body = tfile.read_text()
                if not re.search(rf"^\s*def {re.escape(tname)}\s*\(", body, re.M):
                    errors.append(f"{cid}: named test not found: {t}")

        if status == "external-required":
            if not c.get("dependency"):
                errors.append(f"{cid}: external-required without dependency")
            if not c.get("diagnostic"):
                errors.append(f"{cid}: external-required without diagnostic")

    # CLI command mapping
    declared = {cmd for c in caps for cmd in c.get("cliCommands", [])}
    registered = registered_cli_commands()
    if registered is None:
        errors.append("CLI not importable yet: cannot verify command mapping (scaffold the CLI)")
    else:
        for cmd in sorted(declared):
            if cmd not in registered:
                errors.append(f"capability CLI command not registered: {cmd!r}")
        # announced operations must have a backing capability
        for cmd in sorted(registered):
            if cmd not in declared:
                errors.append(f"CLI command announced without capability: {cmd!r}")

    # generated region matches JSON
    skill_text = SKILL.read_text()
    if START not in skill_text or END not in skill_text:
        errors.append("SKILL.md missing capabilities markers")
    else:
        region = skill_text.split(START, 1)[1].split(END, 1)[0].strip()
        if region != render_block(doc).strip():
            errors.append("SKILL.md capabilities region diverges from capabilities.json")

    # markdown links resolve
    errors.extend(check_links())

    return errors


def check_links() -> list[str]:
    errors: list[str] = []
    md_files = [SKILL] + sorted((ROOT / "references").glob("*.md"))
    link_re = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
    for md in md_files:
        if not md.is_file():
            continue
        for m in link_re.finditer(md.read_text()):
            target = m.group(1)
            if target.startswith(("http://", "https://", "#", "mailto:")):
                continue
            target = target.split("#", 1)[0].split("?", 1)[0]
            if not target:
                continue
            resolved = (md.parent / target).resolve()
            if not resolved.exists():
                errors.append(f"{md.relative_to(ROOT)}: broken link -> {m.group(1)}")
    return errors


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true", help="run all checks")
    ap.add_argument("--sync", action="store_true", help="rewrite SKILL.md region from JSON")
    args = ap.parse_args()

    if args.sync:
        doc = load_caps()
        text = SKILL.read_text()
        new = text.split(START, 1)[0] + START + "\n" + render_block(doc) + "\n" + END + text.split(END, 1)[1]
        SKILL.write_text(new)
        print("synced SKILL.md capabilities region")
        return 0

    errors = check()
    if errors:
        print(f"capabilities validation FAILED ({len(errors)} issue(s)):", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    doc = load_caps()
    print(f"capabilities OK: {len(doc['capabilities'])} capabilities, links resolved, region in sync")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
