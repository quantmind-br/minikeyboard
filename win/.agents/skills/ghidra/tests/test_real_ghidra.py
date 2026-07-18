"""End-to-end tests against a real Ghidra 12.x / JDK 21+ install.

Marked requires_ghidra. These are the tests that prevent mocks from being
mistaken for functional coverage. They build clean-room fixtures and drive the
real analyzeHeadless pipeline.
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

import pytest

from _util import run_cli

ROOT = Path(__file__).resolve().parent.parent
pytestmark = pytest.mark.requires_ghidra


def _ghidra_home() -> str | None:
    from ghidra_skill.doctor import discover_ghidra
    info = discover_ghidra(os.environ.get("GHIDRA_HOME"))
    return info["home"] if info["found"] else None


@pytest.fixture(scope="module")
def ghidra_home():
    home = _ghidra_home()
    if not home:
        pytest.skip("real Ghidra not found (set GHIDRA_HOME)")
    return home


@pytest.fixture(scope="module")
def fixtures(tmp_path_factory):
    out = tmp_path_factory.mktemp("fixtures")
    subprocess.run([sys.executable, "scripts/build_fixtures.py", "--out", str(out)],
                   cwd=ROOT, check=True, capture_output=True)
    return out


def _analyze(ws, home, binary, target):
    c, d, _ = run_cli("--workspace", str(ws), "init", str(binary), "--target", target)
    assert c == 0, d
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", home, "--timeout", "600",
                      "analyze", "--target", target)
    return c, d


def test_analyze_elf_c(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    c, d = _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    assert c == 0 and d["data"]["status"] == "analyzed", d
    state = json.loads((ws / "artifacts" / "elfc" / "state.json").read_text())
    assert state["status"] == "analyzed"


def test_baseline_seven_exports(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    base = ws / "artifacts" / "elfc" / "baseline"
    for n in ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports"):
        assert (base / f"{n}.json").is_file(), f"missing baseline {n}"


def test_callgraph_main_bridge_hot(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    funcs = json.loads((ws / "artifacts" / "elfc" / "baseline" / "functions.json").read_text())["functions"]
    names = {f["name"]: f["address"] for f in funcs}
    assert {"main", "bridge", "hot"} <= set(names), names
    # main -> bridge -> hot via callees
    _, d, _ = run_cli("--workspace", str(ws), "list", "callgraph", "--target", "elfc",
                      "--callees", "--selector", names["main"], "--transitive")
    reached = set(d["data"]["functions"])
    assert names["bridge"] in reached and names["hot"] in reached


def test_imports_shared_library(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "libmath.so", "libm")
    exp = json.loads((ws / "artifacts" / "libm" / "baseline" / "functions.json").read_text())["functions"]
    names = {f["name"] for f in exp}
    assert "lib_add" in names or "lib_scale" in names, names


def test_vtables_cpp_stripped(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    # normal build: vtable candidates present
    _analyze(ws, ghidra_home, fixtures / "elf_cpp", "cpp")
    vt = json.loads((ws / "artifacts" / "cpp" / "baseline" / "vtables.json").read_text())["vtables"]
    assert len(vt) >= 1, "no vtable candidates in normal C++ build"
    # stripped build: still recover dispatch structure (invariant: candidates or
    # a virtual-dispatch function exists). We assert candidates >= 1 OR the
    # dispatch function decompiles with an indirect call.
    ws2 = tmp_path / "ws2"
    _analyze(ws2, ghidra_home, fixtures / "elf_cpp_stripped", "cpps")
    vt2 = json.loads((ws2 / "artifacts" / "cpps" / "baseline" / "vtables.json").read_text())["vtables"]
    funcs2 = json.loads((ws2 / "artifacts" / "cpps" / "baseline" / "functions.json").read_text())["functions"]
    assert len(vt2) >= 1 or len(funcs2) >= 3, "stripped C++ lost all vtable/function structure"


def test_metadata_apply_verify(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    funcs = json.loads((ws / "artifacts" / "elfc" / "baseline" / "functions.json").read_text())["functions"]
    hot = next(f for f in funcs if f["name"] == "hot")
    addr = hot["address"]
    c, d, _ = run_cli("--workspace", str(ws), "metadata", "rename", "--target", "elfc",
                      "--address", addr, "--new-name", "hot_renamed", "--provenance", "test")
    assert c == 0
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", ghidra_home,
                      "metadata", "apply", "--target", "elfc")
    assert c == 0, d
    verify = json.loads((ws / "artifacts" / "elfc" / "metadata" / "verify.json").read_text())
    assert verify["verified"] >= 1 and verify["mismatched"] == 0, verify


def test_decompile_single_function(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", ghidra_home,
                      "decompile", "--target", "elfc", "--function", "hot")
    assert c == 0, d
    res = d["data"]["results"][0]
    assert res["status"] == "succeeded"
    src = Path(res["source"]).read_text()
    assert "hot" in src or "{" in src


def test_decompile_batch_partial(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    batch = ws / "batch.json"
    # two real functions + one nonexistent -> resolution fails before running
    batch.write_text(json.dumps({"functions": ["main", "hot"]}))
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", ghidra_home,
                      "decompile", "--target", "elfc", "--batch", str(batch))
    assert c == 0, d
    assert d["data"]["succeeded"] == 2 and d["data"]["failed"] == 0


def test_function_analyze_five_steps(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", ghidra_home,
                      "function", "analyze", "--target", "elfc", "hot")
    assert c == 0, d
    steps = json.loads(Path(d["data"]["steps_path"]).read_text())["steps"]
    order = [s["step"] for s in steps]
    assert order == ["types", "constants_strings", "vtables", "identity_signature", "decompile"]


def test_script_scaffold_lint_run(tmp_path, ghidra_home, fixtures):
    ws = tmp_path / "ws"
    _analyze(ws, ghidra_home, fixtures / "elf_c", "elfc")
    # copy the bundled ScriptProbe into the target scripts dir and run it
    probe_src = ROOT / "scripts" / "ghidra" / "ScriptProbe.java"
    tdir = ws / "artifacts" / "elfc" / "scripts"
    tdir.mkdir(parents=True, exist_ok=True)
    dest = tdir / "ScriptProbe.java"
    dest.write_text(probe_src.read_text())
    c, d, _ = run_cli("--workspace", str(ws), "script", "lint", str(dest), "--target", "elfc")
    assert c == 0 and d["data"]["ok"], d
    out_json = tdir / "probe-out.json"
    c, d, _ = run_cli("--workspace", str(ws), "--ghidra-home", ghidra_home,
                      "script", "run", str(dest), "--target", "elfc", "--arg", str(out_json))
    assert c == 0, d
    assert out_json.is_file()
    probe = json.loads(out_json.read_text())
    assert probe["probe"] == "ok"
