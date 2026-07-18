"""Artifact schema tests."""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from _util import run_cli
from ghidra_skill.artifacts import write_json, stamp


def _init(ws, binary, target="demo"):
    return run_cli("--workspace", str(ws), "init", str(binary), "--target", target)


def test_state_schema(workspace, elf_c):
    _init(workspace, elf_c)
    state = json.loads((workspace / "artifacts" / "demo" / "state.json").read_text())
    assert state["schema_version"] == 1
    assert state["target_id"] == "demo"
    assert set(state["binary"]) >= {"path", "sha256", "format"}
    assert len(state["binary"]["sha256"]) == 64
    assert state["scope"]["mode"] in ("full", "symbols", "addresses")
    assert isinstance(state["scope"]["entries"], list)
    assert state["status"] == "initialized"
    assert "created_at" in state and "updated_at" in state


def test_inspection_schema(workspace, elf_c):
    _init(workspace, elf_c)
    ip = workspace / "artifacts" / "demo" / "intake" / "inspection.json"
    doc = json.loads(ip.read_text())
    assert doc["schema_version"] == 1 and doc["target_id"] == "demo"
    for key in ("tool_versions", "format", "sections", "warnings"):
        assert key in doc


def test_third_party_schema(workspace, elf_c):
    _init(workspace, elf_c)
    run_cli("--workspace", str(workspace), "evidence", "third-party", "--target", "demo",
            "--library", "zlib", "--confidence", "medium", "--evidence", "e")
    doc = json.loads((workspace / "artifacts" / "demo" / "evidence" / "third-party.json").read_text())
    assert doc["schema_version"] == 1
    lib = doc["libraries"][0]
    assert lib["library"] == "zlib" and lib["confidence"] == "medium"
    assert lib["classification"] in ("observed", "inferred", "unresolved")


def test_gates_schema(workspace, elf_c):
    _init(workspace, elf_c)
    run_cli("--workspace", str(workspace), "validate", "--target", "demo")
    doc = json.loads((workspace / "artifacts" / "demo" / "gates" / "latest.json").read_text())
    assert doc["schema_version"] == 1
    assert set(doc["gates"]) == {"intake", "baseline", "evidence", "metadata", "decompilation"}
    for g in doc["gates"].values():
        assert g["status"] in ("passed", "failed", "not_applicable")
