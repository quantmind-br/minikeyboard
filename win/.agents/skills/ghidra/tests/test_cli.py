"""CLI contract tests (no real Ghidra)."""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from _util import run_cli
from ghidra_skill.artifacts import write_json, stamp


def _init(ws: Path, binary: Path, target="demo", scope="full"):
    return run_cli("--workspace", str(ws), "init", str(binary), "--target", target, "--scope", scope)


def _seed_baseline(ws: Path, target: str, group: str, items):
    base = ws / "artifacts" / target / "baseline"
    base.mkdir(parents=True, exist_ok=True)
    write_json(base / f"{group}.json", {"schema_version": 1, "program": "x",
               "count": len(items), group: items})


# ---- doctor ---------------------------------------------------------------

def test_doctor_envelope(workspace):
    code, data, _ = run_cli("--workspace", str(workspace), "doctor", "--format", "json")
    assert code == 0
    assert data["status"] == "ok"
    for key in ("ghidra", "java", "python", "binutils", "frida", "isolation"):
        assert key in data["data"]
    assert data["data"]["isolation"]["dynamic_ready"] is False  # no profile


# ---- init / inspect -------------------------------------------------------

def test_init_creates_workspace(workspace, elf_c):
    code, data, _ = _init(workspace, elf_c)
    assert code == 0 and data["status"] == "ok"
    assert (workspace / "artifacts" / "demo" / "state.json").is_file()
    assert data["data"]["format"] == "elf"
    assert len(data["data"]["sha256"]) == 64


def test_inspect_binary_envelope(workspace, elf_c):
    _init(workspace, elf_c)
    code, data, _ = run_cli("--workspace", str(workspace), "inspect", "--target", "demo")
    assert code == 0 and data["data"]["format"] == "elf"
    assert data["data"]["arch"]


def test_inspect_archive_partial(workspace, tmp_path):
    import subprocess, os, shutil
    if not (shutil.which("gcc") and shutil.which("ar")):
        pytest.skip("need gcc+ar")
    src = Path(__file__).parent / "fixtures" / "src" / "archive"
    oa, ob = tmp_path / "a.o", tmp_path / "b.o"
    env = dict(os.environ, LC_ALL="C")
    subprocess.run(["gcc", "-O1", "-c", "-o", str(oa), str(src / "member_a.c")], check=True, env=env)
    subprocess.run(["gcc", "-O1", "-c", "-o", str(ob), str(src / "member_b.c")], check=True, env=env)
    lib = tmp_path / "libp.a"
    subprocess.run(["ar", "rcs", str(lib), str(oa), str(ob)], check=True, env=env)
    bogus = tmp_path / "broken.o"
    bogus.write_bytes(b"not an object\n")
    subprocess.run(["ar", "q", str(lib), str(bogus)], check=True, env=env)
    code, data, _ = run_cli("--workspace", str(tmp_path / "ws"), "inspect", str(lib))
    assert code == 0 and data["data"]["format"] == "archive"
    members = {m["name"]: m for m in data["data"]["archive_members"]}
    assert members["broken.o"]["valid"] is False
    assert any(m["valid"] and m["symbols"] for m in data["data"]["archive_members"])


# ---- config scope ---------------------------------------------------------

def test_config_scope_roundtrip(workspace, elf_c):
    _init(workspace, elf_c)
    run_cli("--workspace", str(workspace), "config", "scope", "add", "--target", "demo", "main")
    code, data, _ = run_cli("--workspace", str(workspace), "config", "scope", "show", "--target", "demo")
    assert "main" in data["data"]["scope"]["entries"]
    run_cli("--workspace", str(workspace), "config", "scope", "remove", "--target", "demo", "main")
    _, data2, _ = run_cli("--workspace", str(workspace), "config", "scope", "show", "--target", "demo")
    assert "main" not in data2["data"]["scope"]["entries"]
    _, data3, _ = run_cli("--workspace", str(workspace), "config", "scope", "set",
                          "--target", "demo", "--mode", "symbols", "--entry", "foo")
    assert data3["data"]["scope"]["mode"] == "symbols" and "foo" in data3["data"]["scope"]["entries"]


# ---- list / show ----------------------------------------------------------

def test_list_functions_reads_baseline(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "functions",
                   [{"id": "0x1", "name": "main", "address": "0x1"}])
    code, data, _ = run_cli("--workspace", str(workspace), "list", "functions", "--target", "demo")
    assert code == 0 and data["data"]["count"] == 1
    assert data["data"]["functions"][0]["name"] == "main"


def test_list_types_reads_baseline(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "types", [{"name": "int"}])
    code, data, _ = run_cli("--workspace", str(workspace), "list", "types", "--target", "demo")
    assert code == 0 and data["data"]["types"][0]["name"] == "int"


def test_list_constants_reads_baseline(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "constants", [{"address": "0x1", "value": "42"}])
    code, data, _ = run_cli("--workspace", str(workspace), "list", "constants", "--target", "demo")
    assert code == 0 and data["data"]["constants"][0]["value"] == "42"


def test_list_strings_reads_baseline(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "strings", [{"address": "0x1", "value": "hello"}])
    code, data, _ = run_cli("--workspace", str(workspace), "list", "strings", "--target", "demo")
    assert code == 0 and data["data"]["strings"][0]["value"] == "hello"


def test_callgraph_callers_callees(workspace, elf_c):
    _init(workspace, elf_c)
    edges = [{"caller": "0xmain", "callee": "0xbridge"},
             {"caller": "0xbridge", "callee": "0xhot"}]
    _seed_baseline(workspace, "demo", "callgraph", edges)
    # callees of main, transitive -> bridge, hot
    code, data, _ = run_cli("--workspace", str(workspace), "list", "callgraph",
                            "--target", "demo", "--callees", "--selector", "0xmain", "--transitive")
    assert code == 0
    assert "0xbridge" in data["data"]["functions"] and "0xhot" in data["data"]["functions"]
    # callers of hot -> bridge
    _, data2, _ = run_cli("--workspace", str(workspace), "list", "callgraph",
                          "--target", "demo", "--callers", "--selector", "0xhot")
    assert "0xbridge" in data2["data"]["functions"]
    assert "xrefs not implemented" in data2["data"]["note"]


def test_show_function_unique_selector(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "functions", [
        {"id": "0x1", "name": "dup", "address": "0x1"},
        {"id": "0x2", "name": "dup", "address": "0x2"},
        {"id": "0x3", "name": "uniq", "address": "0x3"}])
    code, data, _ = run_cli("--workspace", str(workspace), "show", "function", "--target", "demo", "uniq")
    assert code == 0 and data["data"]["function"]["address"] == "0x3"
    # ambiguous
    code2, data2, _ = run_cli("--workspace", str(workspace), "show", "function", "--target", "demo", "dup")
    assert code2 == 2 and data2["status"] == "error"


# ---- evidence -------------------------------------------------------------

def test_evidence_third_party_record(workspace, elf_c):
    _init(workspace, elf_c)
    code, data, _ = run_cli("--workspace", str(workspace), "evidence", "third-party",
                            "--target", "demo", "--library", "zlib", "--version", "1.3.1",
                            "--confidence", "high", "--evidence", "string inflate")
    assert code == 0 and data["data"]["entry"]["library"] == "zlib"
    _, listed, _ = run_cli("--workspace", str(workspace), "evidence", "third-party",
                           "--target", "demo", "--list")
    assert listed["data"]["libraries"][0]["library"] == "zlib"


# ---- metadata -------------------------------------------------------------

def test_metadata_record(workspace, elf_c):
    _init(workspace, elf_c)
    code, data, _ = run_cli("--workspace", str(workspace), "metadata", "rename",
                            "--target", "demo", "--address", "0x1160",
                            "--new-name", "parse", "--provenance", "xref")
    assert code == 0 and data["data"]["entry"]["new_name"] == "parse"
    assert (workspace / "artifacts" / "demo" / "metadata" / "renames.json").is_file()


def test_metadata_conflict_needs_force(workspace, elf_c):
    _init(workspace, elf_c)
    args = ("--workspace", str(workspace), "metadata", "rename", "--target", "demo",
            "--address", "0x1160", "--new-name", "a", "--provenance", "p")
    assert run_cli(*args)[0] == 0
    code, data, _ = run_cli(*args)  # duplicate address without --force
    assert code == 2 and data["status"] == "error"
    assert run_cli(*args, "--force")[0] == 0


# ---- compare --------------------------------------------------------------

def test_compare_requires_inputs(workspace, elf_c):
    _init(workspace, elf_c)
    # argparse requires all flags; missing ones -> usage error exit 2
    code, data, _ = run_cli("--workspace", str(workspace), "compare", "--target", "demo",
                            "--reason", "r")
    assert code == 2


def test_compare_preserves_conflict(workspace, elf_c):
    _init(workspace, elf_c)
    code, data, _ = run_cli("--workspace", str(workspace), "compare", "--target", "demo",
                            "--reason", "verify parser", "--question", "does it bound-check",
                            "--boundary", "parse_header", "--fallback", "keep static",
                            "--compare", "runtime-capture-1")
    assert code == 0
    rec = json.loads(Path(data["data"]["record"]).read_text())
    # all six explicit inputs are persisted verbatim (progressive-decompilation contract)
    assert rec["reason"] == "verify parser"
    assert rec["question"] == "does it bound-check"
    assert rec["boundary"] == "parse_header"
    assert rec["fallback"] == "keep static"
    assert rec["compare"] == "runtime-capture-1"
    # observed/inferred/unresolved/conflicts surfaces all present and separate
    for k in ("observed", "inferred", "unresolved", "conflicts"):
        assert k in rec and isinstance(rec[k], list)
    assert rec["schema_version"] == 1 and rec["target_id"] == "demo"


# ---- report / improve / validate ------------------------------------------

def test_report_separates_evidence(workspace, elf_c):
    _init(workspace, elf_c)
    _seed_baseline(workspace, "demo", "functions", [{"name": "main", "address": "0x1"}])
    code, data, _ = run_cli("--workspace", str(workspace), "report", "--target", "demo")
    assert code == 0
    doc = json.loads(Path(data["data"]["report_json"]).read_text())
    assert "observed" in doc and "inferred" in doc and "unresolved" in doc
    assert any("functions" in o for o in doc["observed"])


def test_improve_review_classifies(workspace, elf_c):
    _init(workspace, elf_c)
    code, data, _ = run_cli("--workspace", str(workspace), "improve", "review",
                            "--target", "demo", "--candidate", "reuse export helper",
                            "--classification", "accepted", "--evidence", "seen twice")
    assert code == 0 and data["data"]["classification"] == "accepted"


def test_validate_gates(workspace, elf_c):
    _init(workspace, elf_c)
    # No analyze yet: baseline not exercised -> not_applicable, not passed.
    _, data, _ = run_cli("--workspace", str(workspace), "validate", "--target", "demo")
    gates = data["data"]["gates"]
    assert gates["intake"] == "passed"
    assert gates["baseline"] == "not_applicable"
    assert data["data"]["overall"] == "passed"  # nothing failed

    # Partial baseline (missing some of the seven) -> baseline FAILED, overall failed.
    for g in ("functions", "callgraph"):
        _seed_baseline(workspace, "demo", g, [{"name": "x", "address": "0x1"}])
    _, data2, _ = run_cli("--workspace", str(workspace), "validate", "--target", "demo")
    assert data2["data"]["gates"]["baseline"] == "failed"
    assert data2["data"]["overall"] == "failed"

    # Complete baseline (all seven) -> baseline PASSED.
    for g in ("types", "vtables", "constants", "strings", "imports"):
        _seed_baseline(workspace, "demo", g, [])
    _, data3, _ = run_cli("--workspace", str(workspace), "validate", "--target", "demo")
    assert data3["data"]["gates"]["baseline"] == "passed"
