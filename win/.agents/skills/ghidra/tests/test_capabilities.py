"""Capabilities contract tests: run the validator and check invariants."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent


def test_validator_passes():
    r = subprocess.run([sys.executable, "scripts/validate_capabilities.py", "--check"],
                       cwd=ROOT, capture_output=True, text=True)
    assert r.returncode == 0, r.stdout + r.stderr


def test_capabilities_cover_15_15():
    caps = json.loads((ROOT / "capabilities.json").read_text())["capabilities"]
    # every capability maps to at least one CLI command and a test
    for c in caps:
        assert c["cliCommands"], c["id"]
        assert c["tests"], c["id"]
    # matrix documents all 15 upstream entries
    matrix = (ROOT / "references" / "upstream-matrix.md").read_text()
    import re
    entries = re.findall(r"^### \d+\. ", matrix, re.M)
    assert len(entries) == 15


def test_cli_registry_matches_capabilities():
    sys.path.insert(0, str(ROOT / "src"))
    from ghidra_skill.cli import command_paths
    registered = set(command_paths())
    caps = json.loads((ROOT / "capabilities.json").read_text())["capabilities"]
    declared = {cmd for c in caps for cmd in c["cliCommands"]}
    assert declared <= registered, declared - registered


def test_skill_region_in_sync():
    r = subprocess.run([sys.executable, "scripts/validate_capabilities.py", "--check"],
                       cwd=ROOT, capture_output=True, text=True)
    assert "region diverges" not in (r.stdout + r.stderr)
