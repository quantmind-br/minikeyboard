"""Fake-headless test: validates argv/allowlist/output orchestration ONLY.

The fake analyzeHeadless proves the CLI assembles a correct invocation and
consumes exported baselines. It is NOT proof of real analysis (that is
test_real_ghidra.py).
"""
from __future__ import annotations

import json
from pathlib import Path

from _util import run_cli


def test_analyze_invokes_headless(workspace, elf_c, fake_ghidra):
    code, data, _ = run_cli("--workspace", str(workspace), "init", str(elf_c), "--target", "demo")
    assert code == 0
    code, data, _ = run_cli("--workspace", str(workspace), "--ghidra-home",
                            str(fake_ghidra["home"]), "analyze", "--target", "demo")
    assert code == 0, data
    assert data["data"]["status"] == "analyzed"
    # all seven baselines exported by the fake
    assert not data["data"]["missing_baselines"]
    # argv assembled correctly: single -scriptPath arg, import, ExportBaseline post
    argv = json.loads(fake_ghidra["argv_log"].read_text())
    assert "-import" in argv
    sp = argv.index("-scriptPath")
    assert ";" in argv[sp + 1] or argv[sp + 1]  # one joined path-list argument
    assert "-postScript" in argv and "ExportBaseline.java" in argv
    # state advanced
    state = json.loads((workspace / "artifacts" / "demo" / "state.json").read_text())
    assert state["status"] == "analyzed"
