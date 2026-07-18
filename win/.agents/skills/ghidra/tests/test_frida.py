"""Frida tests: deterministic with or without Frida installed."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

import pytest

from _util import run_cli
from ghidra_skill import frida as frida_mod
from ghidra_skill.context import Context
from ghidra_skill.envelope import ExternalRequired, ValidationError
from ghidra_skill.workspace import Workspace


def _frida_present() -> bool:
    from ghidra_skill.doctor import check_frida
    return check_frida()["found"]


def _ctx(tmp_path, elf_c):
    ws = Workspace(tmp_path / "ws")
    ws.create_target("demo", elf_c, "full", [])
    return Context(workspace=ws)


def test_frida_doctor_envelope(workspace, elf_c):
    code, data, _ = run_cli("--workspace", str(workspace), "frida", "doctor")
    assert code == 0 and data["status"] == "ok"
    assert "frida" in data["data"] and "isolation" in data["data"]


def test_frida_capture_external_required(tmp_path, elf_c, monkeypatch):
    ctx = _ctx(tmp_path, elf_c)
    monkeypatch.setattr(frida_mod, "check_frida",
                        lambda: {"cli": False, "version": None, "python_module": False, "found": False})
    with pytest.raises(ExternalRequired):
        frida_mod.capture(ctx, "demo", trusted=True, isolation_profile=None, scenario="io")


def test_frida_capture_owned_fixture(tmp_path, elf_c, monkeypatch):
    ctx = _ctx(tmp_path, elf_c)
    monkeypatch.setattr(frida_mod, "check_frida",
                        lambda: {"cli": True, "version": "17.0", "python_module": True, "found": True})
    # trusted/owned fixture: allowed with manifest, no synthetic events
    res = frida_mod.capture(ctx, "demo", trusted=True, isolation_profile=None, scenario="io")
    manifest = json.loads(Path(res["manifest"]).read_text())
    assert manifest["trusted"] is True and manifest["scenario"] == "io"
    events = Path(res["events"])
    assert events.is_file() and events.read_text() == ""  # no fabricated events


def test_frida_capture_untrusted_refused_without_profile(tmp_path, elf_c, monkeypatch):
    ctx = _ctx(tmp_path, elf_c)
    monkeypatch.setattr(frida_mod, "check_frida",
                        lambda: {"cli": True, "version": "17.0", "python_module": True, "found": True})
    with pytest.raises(ValidationError):
        frida_mod.capture(ctx, "demo", trusted=False, isolation_profile=None, scenario="io")


def test_frida_import_evidence(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    manifest = tmp_path / "ext.json"
    manifest.write_text(json.dumps({"events": [{"fn": "main", "ts": 1}]}))
    res = frida_mod.import_evidence(ctx, "demo", manifest_path=str(manifest))
    doc = json.loads(Path(res["imported"]).read_text())
    assert doc["classification"] == "observed"
    assert doc["source_manifest"].endswith("ext.json")


def test_frida_compare_conflict(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    res = frida_mod.compare(ctx, "demo", static_ref="baseline", runtime_ref="cap-1")
    doc = json.loads(Path(res["compare"]).read_text())
    for k in ("observed", "inferred", "unresolved", "conflicts"):
        assert k in doc
    assert doc["static_ref"] == "baseline" and doc["runtime_ref"] == "cap-1"
